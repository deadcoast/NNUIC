//! R4 — the genetic axis: sequencing, registry, expression (paper §6).
//!
//! Exit criterion (PoC §9): three data types (tabular, video, plain text)
//! sequenced and expressed to *distinct* phenotypes, headless, plus a fourth
//! added *without core change*.
//!
//! The design point that makes "no core change" real rather than cosmetic:
//! BOTH detection and expression are registry-driven. `sequence` runs a list of
//! registered detectors; `express` runs a list of registered constructors. The
//! core loops never mention a concrete type. Adding a type is two pushes onto
//! those lists via a public method — the core functions are untouched. If
//! detection were hardcoded in `sequence`, a fourth type would force a core
//! edit; it doesn't here, and the R4 test proves it by only calling the public
//! `register_type`.
//!
//! Determinism (Invariant I7): detectors and constructors run in fixed
//! registration order; the genome is a sorted set; first matching constructor
//! wins. Same bytes → same phenotype, every time.

use std::collections::BTreeSet;

/// A genome is a sorted set of typed trait tokens absorbed from an asset.
pub type Genome = BTreeSet<String>;

/// The rendered manifestation: which control surface a cell grows, and its skin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Phenotype {
    pub kind: String,
    pub controls: Vec<String>,
    pub skin: String,
}

impl Phenotype {
    fn new(kind: &str, controls: &[&str], skin: &str) -> Self {
        Phenotype {
            kind: kind.to_string(),
            controls: controls.iter().map(|s| s.to_string()).collect(),
            skin: skin.to_string(),
        }
    }
    /// The germline phenotype — what an un-sequenced cell expresses. There is no
    /// empty cell (paper §6.1); an unrecognized asset still yields this.
    pub fn germline() -> Self {
        Phenotype::new("panel", &["resize"], "bare")
    }
}

/// Inspects raw bytes; returns the traits it recognizes, or None if it doesn't
/// apply. Pure and deterministic.
type Detector = Box<dyn Fn(&[u8]) -> Option<Vec<String>>>;

/// Builds a phenotype if it recognizes the genome's signature, else None.
type Constructor = Box<dyn Fn(&Genome) -> Option<Phenotype>>;

/// The expression ecosystem: sequencer (detectors) + registry (constructors).
pub struct Ecosystem {
    detectors: Vec<Detector>,
    constructors: Vec<Constructor>,
}

impl Ecosystem {
    /// Empty ecosystem — knows no types. Everything expresses germline.
    pub fn empty() -> Self {
        Ecosystem { detectors: Vec::new(), constructors: Vec::new() }
    }

    /// Register one data type: its byte detector and its phenotype constructor.
    /// This is the ONLY entry point for extension. The core `sequence`/`express`
    /// loops below never change. Adding the fourth type in R4 is a call to this.
    pub fn register_type(&mut self, detector: Detector, constructor: Constructor) {
        self.detectors.push(detector);
        self.constructors.push(constructor);
    }

    /// seq: Bytes → Genome (paper §6.1). Runs every detector in fixed order and
    /// unions their traits. Core loop — type-agnostic.
    pub fn sequence(&self, bytes: &[u8]) -> Genome {
        let mut g = Genome::new();
        for d in &self.detectors {
            if let Some(ts) = d(bytes) {
                for t in ts {
                    g.insert(t);
                }
            }
        }
        g
    }

    /// express: Genome → Phenotype (paper §6.2). First constructor that
    /// recognizes the genome wins; otherwise germline. Core loop — type-agnostic.
    pub fn express(&self, g: &Genome) -> Phenotype {
        for c in &self.constructors {
            if let Some(p) = c(g) {
                return p;
            }
        }
        Phenotype::germline()
    }

    /// Full path: absorb an asset and manifest it.
    pub fn digest(&self, bytes: &[u8]) -> Phenotype {
        self.express(&self.sequence(bytes))
    }

    /// The three core types (paper §9 exit set): tabular, video, plain text.
    pub fn core() -> Self {
        let mut e = Ecosystem::empty();
        e.register_type(Box::new(detect_tabular), Box::new(express_tabular));
        e.register_type(Box::new(detect_video), Box::new(express_video));
        e.register_type(Box::new(detect_text), Box::new(express_text));
        e
    }
}

// ---- core type: TABULAR (CSV-like) ----
fn detect_tabular(bytes: &[u8]) -> Option<Vec<String>> {
    let s = std::str::from_utf8(bytes).ok()?;
    let lines: Vec<&str> = s.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.len() < 2 {
        return None;
    }
    if !lines[0].contains(',') {
        return None;
    }
    let cols = lines[0].split(',').count();
    // every row has the same column count → structured table
    if !lines.iter().all(|l| l.split(',').count() == cols) {
        return None;
    }
    let mut traits = vec!["tabular".to_string()];
    // numeric column detection (data-row cells that parse as numbers)
    let numeric = lines[1..]
        .iter()
        .any(|l| l.split(',').any(|c| c.trim().parse::<f64>().is_ok()));
    if numeric {
        traits.push("numeric".to_string());
    }
    Some(traits)
}
fn express_tabular(g: &Genome) -> Option<Phenotype> {
    if g.contains("tabular") {
        Some(Phenotype::new(
            "chart",
            &["axis-select", "aggregate", "sort", "filter"],
            "data",
        ))
    } else {
        None
    }
}

// ---- core type: VIDEO (ISO base media / MP4) ----
fn detect_video(bytes: &[u8]) -> Option<Vec<String>> {
    // ISO-BMFF: bytes 4..8 == "ftyp"
    if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" {
        Some(vec!["video".to_string(), "temporal".to_string()])
    } else {
        None
    }
}
fn express_video(g: &Genome) -> Option<Phenotype> {
    if g.contains("video") {
        Some(Phenotype::new(
            "player",
            &["play-pause", "scrub", "volume", "fullscreen"],
            "media",
        ))
    } else {
        None
    }
}

// ---- core type: PLAIN TEXT (prose) ----
fn detect_text(bytes: &[u8]) -> Option<Vec<String>> {
    let s = std::str::from_utf8(bytes).ok()?;
    if s.trim().is_empty() {
        return None;
    }
    // prose, not a table: reject if it has the CSV structure detect_tabular wants
    let lines: Vec<&str> = s.lines().filter(|l| !l.trim().is_empty()).collect();
    let looks_tabular = lines.len() >= 2 && lines[0].contains(',');
    if looks_tabular {
        return None;
    }
    if s.chars().any(|c| c.is_alphabetic()) {
        Some(vec!["prose".to_string(), "text".to_string()])
    } else {
        None
    }
}
fn express_text(g: &Genome) -> Option<Phenotype> {
    if g.contains("prose") {
        Some(Phenotype::new(
            "reader",
            &["scroll", "font-size", "find"],
            "document",
        ))
    } else {
        None
    }
}

// ---- the FOURTH type, defined OUTSIDE the core, to prove extensibility. ----
// Nothing above this line changes when this is added. It is registered by the
// caller via `register_type`. (Kept here for reuse by the bin and the test.)

/// Detector for PNG raster images (magic bytes).
pub fn detect_image(bytes: &[u8]) -> Option<Vec<String>> {
    const PNG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    if bytes.len() >= 8 && bytes[..8] == PNG {
        Some(vec!["raster".to_string(), "image".to_string()])
    } else {
        None
    }
}
/// Constructor for the image-viewer phenotype.
pub fn express_image(g: &Genome) -> Option<Phenotype> {
    if g.contains("raster") {
        Some(Phenotype::new("image-viewer", &["zoom", "pan", "fit"], "media"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_core_types_express_distinctly() {
        let eco = Ecosystem::core();
        let csv = b"name,score\nalice,91\nbob,84\n";
        let mp4 = b"\x00\x00\x00\x18ftypisom\xff\xff\xff\xff";
        let txt = "The medium is the message. It flows, then it settles.".as_bytes();

        let p_csv = eco.digest(csv);
        let p_mp4 = eco.digest(mp4);
        let p_txt = eco.digest(txt);

        assert_eq!(p_csv.kind, "chart");
        assert_eq!(p_mp4.kind, "player");
        assert_eq!(p_txt.kind, "reader");
        // distinct
        assert_ne!(p_csv, p_mp4);
        assert_ne!(p_mp4, p_txt);
        assert_ne!(p_csv, p_txt);
    }

    #[test]
    fn expression_is_deterministic() {
        let eco = Ecosystem::core();
        let csv = b"a,b\n1,2\n3,4\n";
        assert_eq!(eco.digest(csv), eco.digest(csv));
    }

    #[test]
    fn fourth_type_added_without_core_change() {
        // Default core does NOT know PNG → germline.
        let mut eco = Ecosystem::core();
        let png = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR";
        assert_eq!(eco.digest(png).kind, "panel"); // germline

        // Add the fourth type via the PUBLIC extension point only. No function
        // in the core module above was edited to make this work.
        eco.register_type(Box::new(detect_image), Box::new(express_image));

        let p = eco.digest(png);
        assert_eq!(p.kind, "image-viewer");
        // still distinct from the three core phenotypes
        assert_eq!(p.controls, vec!["zoom", "pan", "fit"]);
    }
}
