//! R3 — the agent-facing declarative grammar (paper §7).
//!
//! An agent declares material intent and containment, never coordinates. This
//! module parses that grammar into an AST and *lowers it into the R1 solver*,
//! so "settles unambiguously" is demonstrated end-to-end: parse → build World →
//! settle → deterministic hash. The grammar is not a mock; it drives the same
//! XPBD solver that passed R1/R2.
//!
//! Grammar (EBNF):
//!   program  := { cell | op }
//!   cell     := "cell" id "{" { dial } [content] "}"
//!   dial     := ("S" | "n" | "xi" | "D") ":" num   // any subset; unstated = default
//!   content  := "hold" id | "germline"
//!   op       := "solidity" mask num | "bind" id id | "contain" "{" id+ "}" "in" id
//!   mask     := "all" | id | "not" id
//!
//! Dials are optional: an agent states only the material intent that differs
//! from the default (S=0.5, n=0.8, xi=1.0, D=0.5). This is the terse form the
//! token-economy claim depends on — a cell that just holds an asset is
//! `cell k1 { hold kpi1 }`.

use crate::rheology::Material;
use crate::solver::World;

#[derive(Debug, Clone)]
pub struct Cell {
    pub id: String,
    pub s: f64,
    pub n: f64,
    pub xi: f64,
    pub d: f64,
    pub content: Option<String>, // asset id or None (germline)
}

#[derive(Debug, Clone)]
pub enum Mask {
    All,
    One(String),
    Not(String),
}

#[derive(Debug, Clone)]
pub enum Op {
    Solidity(Mask, f64),
    Bind(String, String),
    Contain(Vec<String>, String),
}

#[derive(Debug, Clone, Default)]
pub struct Layout {
    pub cells: Vec<Cell>,
    pub ops: Vec<Op>,
}

// ---- lexer ----
fn lex(src: &str) -> Vec<String> {
    let mut toks = Vec::new();
    let mut cur = String::new();
    for ch in src.chars() {
        match ch {
            '{' | '}' | ':' => {
                if !cur.is_empty() {
                    toks.push(std::mem::take(&mut cur));
                }
                toks.push(ch.to_string());
            }
            c if c.is_whitespace() => {
                if !cur.is_empty() {
                    toks.push(std::mem::take(&mut cur));
                }
            }
            // comments: strip from '/' '/' to end of line handled below
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        toks.push(cur);
    }
    toks
}

fn strip_comments(src: &str) -> String {
    src.lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

struct Cursor {
    toks: Vec<String>,
    i: usize,
}
impl Cursor {
    fn peek(&self) -> Option<&str> {
        self.toks.get(self.i).map(|s| s.as_str())
    }
    fn next(&mut self) -> Option<String> {
        let t = self.toks.get(self.i).cloned();
        self.i += 1;
        t
    }
    fn expect(&mut self, want: &str) -> Result<(), String> {
        match self.next() {
            Some(t) if t == want => Ok(()),
            other => Err(format!("expected '{want}', got {other:?}")),
        }
    }
    fn num(&mut self) -> Result<f64, String> {
        let t = self.next().ok_or("expected number, got EOF")?;
        t.parse::<f64>().map_err(|_| format!("bad number '{t}'"))
    }
    fn ident(&mut self) -> Result<String, String> {
        self.next().ok_or_else(|| "expected identifier, got EOF".into())
    }
}

/// Parse the grammar into a Layout AST.
pub fn parse(src: &str) -> Result<Layout, String> {
    let clean = strip_comments(src);
    let mut c = Cursor { toks: lex(&clean), i: 0 };
    let mut layout = Layout::default();

    while let Some(t) = c.peek() {
        match t {
            "cell" => layout.cells.push(parse_cell(&mut c)?),
            "solidity" | "bind" | "contain" => layout.ops.push(parse_op(&mut c)?),
            other => return Err(format!("unexpected token '{other}'")),
        }
    }
    Ok(layout)
}

fn parse_cell(c: &mut Cursor) -> Result<Cell, String> {
    c.expect("cell")?;
    let id = c.ident()?;
    c.expect("{")?;
    let (mut s, mut n, mut xi, mut d) = (0.5, 0.8, 1.0, 0.5);
    let mut content = None;
    loop {
        match c.peek() {
            Some("}") => {
                c.next();
                break;
            }
            Some("hold") => {
                c.next();
                content = Some(c.ident()?);
            }
            Some("germline") => {
                c.next();
                content = None;
            }
            Some("S") | Some("n") | Some("xi") | Some("D") => {
                let key = c.next().unwrap();
                c.expect(":")?;
                let v = c.num()?;
                match key.as_str() {
                    "S" => s = v,
                    "n" => n = v,
                    "xi" => xi = v,
                    "D" => d = v,
                    _ => unreachable!(),
                }
            }
            other => return Err(format!("in cell '{id}': unexpected {other:?}")),
        }
    }
    Ok(Cell { id, s, n, xi, d, content })
}

fn parse_op(c: &mut Cursor) -> Result<Op, String> {
    match c.next().as_deref() {
        Some("solidity") => {
            let mask = match c.next().as_deref() {
                Some("all") => Mask::All,
                Some("not") => Mask::Not(c.ident()?),
                Some(id) => Mask::One(id.to_string()),
                None => return Err("solidity: expected mask".into()),
            };
            let v = c.num()?;
            Ok(Op::Solidity(mask, v))
        }
        Some("bind") => Ok(Op::Bind(c.ident()?, c.ident()?)),
        Some("contain") => {
            c.expect("{")?;
            let mut ids = Vec::new();
            while let Some(t) = c.peek() {
                if t == "}" {
                    break;
                }
                ids.push(c.ident()?);
            }
            c.expect("}")?;
            c.expect("in")?;
            let container = c.ident()?;
            Ok(Op::Contain(ids, container))
        }
        other => Err(format!("bad op '{other:?}'")),
    }
}

impl Layout {
    fn index_of(&self, id: &str) -> Option<usize> {
        self.cells.iter().position(|c| c.id == id)
    }

    /// Lower to per-region materials, applying solidity ops (which reduce to
    /// set_solidity — paper §8, I3). bind/contain are structural/genetic
    /// metadata and do not alter the settle geometry in this prototype.
    pub fn to_materials(&self) -> Vec<Material> {
        let mut mats: Vec<Material> =
            self.cells.iter().map(|c| Material::from_dials(c.s, c.n, c.xi)).collect();
        for op in &self.ops {
            if let Op::Solidity(mask, v) = op {
                match mask {
                    Mask::All => {
                        for m in &mut mats {
                            m.s = *v;
                        }
                    }
                    Mask::One(id) => {
                        if let Some(k) = self.index_of(id) {
                            mats[k].s = *v;
                        }
                    }
                    Mask::Not(id) => {
                        let keep = self.index_of(id);
                        for (k, m) in mats.iter_mut().enumerate() {
                            if Some(k) != keep {
                                m.s = *v;
                            }
                        }
                    }
                }
            }
        }
        mats
    }

    /// Build a settleable World. Container is sized to hold the cells at the
    /// R1-tuned density so settling is well-posed.
    pub fn to_world(&self, seed: u64) -> World {
        let mats = self.to_materials();
        let n = mats.len().max(1);
        let cols = (n as f64).sqrt().ceil() as usize;
        let rows = (n + cols - 1) / cols;
        let cellsz = 7.3; // matches R1's tuned per-region spacing
        let w = cols as f64 * cellsz;
        let h = rows as f64 * cellsz;
        World::from_materials(&mats, 5, w, h, seed)
    }

    /// Parse convenience: settle and return (frames, state hash). A single
    /// deterministic hash for the same source is what "settles unambiguously"
    /// means operationally.
    pub fn settle_hash(&self, seed: u64) -> (usize, u64) {
        let mut w = self.to_world(seed);
        let f = w.settle(1500);
        (f, w.state_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DASH: &str = r#"
        cell main { S:0.3 n:0.8 xi:1 D:0.6 hold revenue }
        cell k1 { S:0.5 n:0.8 xi:1 D:0.4 hold kpi1 }
        cell k2 { S:0.5 n:0.8 xi:1 D:0.4 hold kpi2 }
        bind k1 k2
        contain { main k1 k2 } in viewport
    "#;

    #[test]
    fn parses_dashboard() {
        let l = parse(DASH).unwrap();
        assert_eq!(l.cells.len(), 3);
        assert_eq!(l.cells[0].id, "main");
        assert_eq!(l.cells[0].content.as_deref(), Some("revenue"));
        assert_eq!(l.ops.len(), 2);
    }

    #[test]
    fn lowers_and_settles_deterministically() {
        let l = parse(DASH).unwrap();
        let a = l.settle_hash(0xABCDEF);
        let b = l.settle_hash(0xABCDEF);
        assert_eq!(a, b, "same source must settle to the same state");
    }

    #[test]
    fn solidity_op_reduces_to_set_solidity() {
        let src = "cell a { S:0.2 n:0.8 xi:1 D:0.5 } cell b { S:0.2 n:0.8 xi:1 D:0.5 } solidity a 1.0";
        let l = parse(src).unwrap();
        let mats = l.to_materials();
        assert!(mats[0].s > 0.99, "cell a should be locked");
        assert!(mats[1].s < 0.5, "cell b untouched");
    }
}
