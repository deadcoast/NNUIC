//! The public agent-facing API (workstream A, milestone M1).
//!
//! Realizes the submission-surface and error-semantics contracts of
//! `docs/DESIGN_API_CONTRACTS.md` (DS02): an engine object over the
//! unchanged, validated R3 grammar. Submission is atomic — a program either
//! applies in full or is rejected in full with engine state untouched.
//!
//! There is no way to author a coordinate through this API: the grammar has
//! no coordinate production (VG01), and the only geometry accepted is the
//! vessel extent — the environment's boundary condition, not a cell
//! position. M1 note, recorded honestly: at prototype fidelity the solve
//! container is sized by the R1-tuned density (the validated
//! `Layout::to_world` path, which the M1 hash-equality exit depends on);
//! the vessel extent is carried as view metadata until the read path maps
//! world space to vessel space in M2.
//!
//! Strain in every view is the realized post-arrest velocity the solver
//! produced after its thixotropic arrest gate — the honest instrument
//! (IV04). Queries are detached copies; nothing returned here can mutate
//! solver state.

use crate::genetics::{Ecosystem, Genome, Phenotype};
use crate::grammar::{self, Layout, Mask, Op};
use crate::solver::{World, REST_FRAMES};
use crate::vec2::Vec2;

/// Canonical deterministic seed for layout lowering (same as the validated
/// prototype path).
pub const SEED: u64 = 0xABCDEF;
/// Settle frame budget, identical to the validated grammar path.
const MAX_SETTLE_FRAMES: usize = 8000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramId(pub u64);

/// Stable cell identity: the declaration index within the active program.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateHash(pub u64);

/// Vessel extent — the environment's boundary condition (view metadata at
/// M1 fidelity; see module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Extent {
    pub w: f64,
    pub h: f64,
}

impl Extent {
    pub fn viewport(w: f64, h: f64) -> Self {
        Extent { w, h }
    }
}

/// Rejection reasons (DS02 error table). Submission is atomic: every
/// variant leaves engine state untouched.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubmitError {
    /// E-PARSE — malformed program text (includes duplicate declarations).
    Parse(String),
    /// E-REF — an op names an undeclared cell (or binds a cell to itself).
    UnknownRef(String),
    /// E-CYCLE — containment forms a cycle.
    ContainmentCycle(String),
    /// E-ASSET — an asset reference with a content-hash annotation that
    /// cannot be resolved. Bare symbolic refs are NOT errors: an unknown
    /// asset type sequences to germline (R4 semantics). This variant
    /// becomes reachable when hash-annotated refs arrive with M4.
    UnresolvableAsset(String),
    /// E-RANGE — a material dial outside its domain.
    OutOfRange(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TickReport {
    pub tick: u64,
    pub max_strain: f64,
    pub settled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cause {
    ContentChange,
    ViewportChange,
    Injection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    CellSettled(CellId),
    GlobalSettled(StateHash),
    Resettling(Cause),
}

/// Level of detail for boundary geometry (M2 read path).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lod {
    /// Centroid + bounding extent only; `boundary` stays empty.
    Extents,
    /// Full deterministic convex hull of the region's particles.
    Hull,
}

/// World-space → vessel-space mapping (uniform scale, centered). The
/// solver's geometry is intrinsic medium units; presenting it inside the
/// vessel is a stateless view transform (Invariant 6) that the renderer —
/// or an agent — applies via this map.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VesselMap {
    pub scale: f64,
    pub ox: f64,
    pub oy: f64,
}

impl VesselMap {
    pub fn apply(&self, p: Vec2) -> Vec2 {
        Vec2::new(p.x * self.scale + self.ox, p.y * self.scale + self.oy)
    }
}

#[derive(Clone, Debug)]
pub struct CellView {
    pub id: CellId,
    /// The grammar identifier — the identity an agent declared.
    pub name: String,
    pub centroid: Vec2,
    pub extent: Extent,
    pub boundary: Vec<Vec2>, // empty at Lod::Extents
    pub traits: Vec<String>,
    pub phenotype: Phenotype,
    pub settled: bool,
    /// Realized post-arrest strain (max particle speed in the region).
    pub strain: f64,
    /// Effective material radius: matter extends this far beyond the
    /// hull of particle centers (continuum conservation under dilation).
    pub radius: f64,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub tick: u64,
    pub settled: bool,
    /// Present only when settled — motion is never hashed.
    pub hash: Option<StateHash>,
    pub vessel: Extent,
    /// The solver's intrinsic world extent.
    pub world: Extent,
    /// World→vessel view transform for this snapshot.
    pub map: VesselMap,
    pub cells: Vec<CellView>,
}

pub struct Engine {
    vessel: Extent,
    eco: Ecosystem,
    /// Registered asset bytes: `hold` refs resolve here and are digested
    /// through the real R4 registry (genome → phenotype). Unregistered
    /// refs express germline, per the unknown-type semantics.
    assets: std::collections::BTreeMap<String, Vec<u8>>,
    layout: Option<Layout>,
    world: Option<World>,
    events: Vec<Event>,
    tick: u64,
    next_program: u64,
    // settlement transition tracking (M2): events fire on state changes,
    // never repeatedly — agents consume events instead of polling.
    cell_settled: Vec<bool>,
    global_settled: bool,
}

impl Engine {
    pub fn new(vessel: Extent) -> Engine {
        Engine {
            vessel,
            eco: {
                // the full registry: three core types plus the R4
                // extension type, registered through the public path
                let mut eco = Ecosystem::core();
                eco.register_type(
                    Box::new(crate::genetics::detect_image),
                    Box::new(crate::genetics::express_image),
                );
                eco
            },
            assets: std::collections::BTreeMap::new(),
            layout: None,
            world: None,
            events: Vec::new(),
            tick: 0,
            next_program: 1,
            cell_settled: Vec::new(),
            global_settled: false,
        }
    }

    /// Atomic submission: parse, validate, lower. On any error the previous
    /// program, world, and pending events are untouched.
    pub fn submit(&mut self, src: &str) -> Result<ProgramId, SubmitError> {
        let layout = grammar::parse(src).map_err(SubmitError::Parse)?;
        validate(&layout)?;
        // deterministic lowering into the vessel: E_fill on, binds real
        let world =
            World::from_layout(&layout, self.vessel.w, self.vessel.h, SEED);
        // -- commit point: nothing above mutated self --
        let n_cells = layout.cells.len();
        self.layout = Some(layout);
        self.world = Some(world);
        self.tick = 0;
        self.events.clear();
        self.cell_settled = vec![false; n_cells];
        self.global_settled = false;
        self.events.push(Event::Resettling(Cause::Injection));
        let id = ProgramId(self.next_program);
        self.next_program += 1;
        Ok(id)
    }

    /// Run the solver to the settled predicate (or the frame budget) and
    /// return the settled-state hash. Idempotent at rest; settlement
    /// events fire on transitions only.
    pub fn settle(&mut self) -> StateHash {
        let world = self.world.as_mut().expect("no program submitted");
        let frames = world.settle(MAX_SETTLE_FRAMES);
        self.tick += frames as u64;
        let hash = StateHash(world.state_hash());
        self.emit_transitions();
        hash
    }

    /// One solver frame. Settlement events fire on transitions, so an
    /// agent driving the loop consumes events instead of polling
    /// internals.
    pub fn tick(&mut self) -> TickReport {
        let world = self.world.as_mut().expect("no program submitted");
        world.step();
        self.tick += 1;
        let max_strain = world.max_speed();
        let settled = world.rest_t.iter().all(|&t| t >= REST_FRAMES);
        self.emit_transitions();
        TickReport { tick: self.tick, max_strain, settled }
    }

    /// Emit settlement events on state transitions (deterministic order:
    /// cells in id order, then global). Settled is the solver's own
    /// sustained predicate — a region is settled iff its jamming latch is
    /// engaged (v0.2 sharpened I4: exact AND stable) — never a first-touch
    /// quiet frame, so events cannot flap during consolidation. A latched
    /// region waking (genuine disturbance) clears its flag so a later
    /// re-settle re-emits.
    fn emit_transitions(&mut self) {
        let world = self.world.as_ref().expect("no program submitted");
        let mut all = true;
        for r in 0..world.regions.len() {
            let settled = world.rest_t[r] >= REST_FRAMES;
            if settled {
                if !self.cell_settled[r] {
                    self.cell_settled[r] = true;
                    self.events.push(Event::CellSettled(CellId(r as u64)));
                }
            } else {
                self.cell_settled[r] = false;
                all = false;
            }
        }
        if all {
            if !self.global_settled {
                self.global_settled = true;
                let h = StateHash(world.state_hash());
                self.events.push(Event::GlobalSettled(h));
            }
        } else {
            self.global_settled = false;
        }
    }

    /// Detached, read-only view of the current state.
    pub fn snapshot(&self, lod: Lod) -> Snapshot {
        let world = self.world.as_ref().expect("no program submitted");
        let layout = self.layout.as_ref().expect("no program submitted");
        let mut cells = Vec::new();
        for (r, reg) in world.regions.iter().enumerate() {
            let (mut sx, mut sy) = (0.0, 0.0);
            let (mut minx, mut miny) = (f64::INFINITY, f64::INFINITY);
            let (mut maxx, mut maxy) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
            let mut strain = 0.0f64;
            for k in reg.first..reg.first + reg.count {
                let p = world.pos[k];
                sx += p.x;
                sy += p.y;
                minx = minx.min(p.x);
                miny = miny.min(p.y);
                maxx = maxx.max(p.x);
                maxy = maxy.max(p.y);
                let s = world.vel[k].len();
                if s > strain {
                    strain = s;
                }
            }
            let inv = 1.0 / reg.count as f64;
            let cell = &layout.cells[r];
            let boundary = match lod {
                Lod::Extents => Vec::new(),
                Lod::Hull => {
                    convex_hull(&world.pos[reg.first..reg.first + reg.count])
                }
            };
            // Held assets with registered bytes are digested through the
            // real R4 registry (genome → phenotype). Unresolved refs and
            // germline cells express germline — the unknown-type
            // semantics, not an error.
            let genome = match cell.content.as_deref() {
                Some(asset) => match self.assets.get(asset) {
                    Some(bytes) => self.eco.sequence(bytes),
                    None => Genome::new(),
                },
                None => Genome::new(),
            };
            let phenotype = self.eco.express(&genome);
            cells.push(CellView {
                id: CellId(r as u64),
                name: cell.id.clone(),
                centroid: Vec2::new(sx * inv, sy * inv),
                extent: Extent { w: maxx - minx, h: maxy - miny },
                boundary,
                traits: genome.into_iter().collect(),
                phenotype,
                // settled = latched: the solver's sustained predicate,
                // never a first-touch quiet frame.
                settled: world.rest_t[r] >= REST_FRAMES,
                strain,
                radius: world.radius[reg.first],
            });
        }
        let settled = world.rest_t.iter().all(|&t| t >= REST_FRAMES);
        let world_extent = Extent { w: world.w, h: world.h };
        let scale = (self.vessel.w / world.w).min(self.vessel.h / world.h);
        let map = VesselMap {
            scale,
            ox: (self.vessel.w - world.w * scale) * 0.5,
            oy: (self.vessel.h - world.h * scale) * 0.5,
        };
        Snapshot {
            tick: self.tick,
            settled,
            hash: if settled {
                Some(StateHash(world.state_hash()))
            } else {
                None
            },
            vessel: self.vessel,
            world: world_extent,
            map,
            cells,
        }
    }

    /// Drain pending events (delivered in deterministic order).
    pub fn poll_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    /// Register asset bytes for a `hold` reference. Held cells digest
    /// their asset through the real R4 registry — genome sequenced from
    /// bytes, phenotype expressed deterministically.
    pub fn register_asset(&mut self, name: &str, bytes: Vec<u8>) {
        self.assets.insert(name.to_string(), bytes);
    }

    /// Viewport change (paper §9): the vessel reshapes and the medium
    /// re-settles with NO further program call — zero-cost adaptation.
    pub fn resize_vessel(&mut self, vessel: Extent) {
        self.vessel = vessel;
        if let Some(world) = self.world.as_mut() {
            let h = 60.0;
            let aspect = (vessel.w / vessel.h).clamp(0.2, 5.0);
            world.resize(h * aspect, h);
            self.cell_settled.iter_mut().for_each(|s| *s = false);
            self.global_settled = false;
            self.events.push(Event::Resettling(Cause::ViewportChange));
        }
    }

    /// Export a cell as a portable record (DS04). The wire format is the
    /// agent grammar itself: a single valid `cell` declaration carrying
    /// identifier, material state (current S, n, ξ — material evolves;
    /// geometry does not exist in the format), D, and content provenance.
    /// Binds do not travel: gene transfer is opt-in per boundary, and an
    /// auto-rebinding record would smuggle proximity coupling past that
    /// rule — the injecting agent re-issues `bind` ops deliberately.
    pub fn export_cell(&self, id: CellId) -> CellRecord {
        let world = self.world.as_ref().expect("no program submitted");
        let layout = self.layout.as_ref().expect("no program submitted");
        let r = id.0 as usize;
        let cell = &layout.cells[r];
        let mat = world.regions[r].mat;
        let content = match &cell.content {
            Some(asset) => format!("hold {asset}"),
            None => "germline".to_string(),
        };
        // canonical numbers: Rust's f64 Display is shortest round-trip
        let source = format!(
            "cell {} {{ S:{} n:{} xi:{} D:{} {} }}",
            cell.id, mat.s, mat.n, mat.xi, cell.d, content
        );
        let settled = world.rest_t.iter().all(|&t| t >= REST_FRAMES);
        CellRecord {
            version: 1,
            origin: if settled {
                Some(StateHash(world.state_hash()))
            } else {
                None
            },
            source,
        }
    }

    /// Inject a portable record into the active layout (DS04). The record
    /// must be exactly one cell declaration — no ops (binds do not
    /// travel), no duplicate identifier. At prototype fidelity the
    /// augmented layout re-lowers deterministically (spawn placement is
    /// part of the injection contract, correction C4); the medium then
    /// re-settles under local pressure — same genotype, phenotype
    /// re-expressed in context.
    pub fn inject_record(
        &mut self,
        rec: &CellRecord,
    ) -> Result<CellId, SubmitError> {
        if rec.version != 1 {
            return Err(SubmitError::Parse(format!(
                "unsupported cellrecord version {}",
                rec.version
            )));
        }
        let fragment = grammar::parse(&rec.source).map_err(SubmitError::Parse)?;
        if fragment.cells.len() != 1 || !fragment.ops.is_empty() {
            return Err(SubmitError::Parse(
                "a cellrecord must be exactly one cell declaration with no \
                 ops (binds do not travel)"
                    .to_string(),
            ));
        }
        let mut augmented = self
            .layout
            .as_ref()
            .expect("no program submitted")
            .clone();
        augmented.cells.push(fragment.cells.into_iter().next().unwrap());
        validate(&augmented)?;
        let world =
            World::from_layout(&augmented, self.vessel.w, self.vessel.h, SEED);
        // -- commit point --
        let n_cells = augmented.cells.len();
        self.layout = Some(augmented);
        self.world = Some(world);
        self.tick = 0;
        self.events.clear();
        self.cell_settled = vec![false; n_cells];
        self.global_settled = false;
        self.events.push(Event::Resettling(Cause::Injection));
        Ok(CellId((n_cells - 1) as u64))
    }
}

/// A portable cell record (DS04): version, origin fingerprint, and the
/// grammar fragment. Structurally geometry-free — the format has no field
/// that could carry a position, extent, or settled shape.
#[derive(Clone, Debug, PartialEq)]
pub struct CellRecord {
    pub version: u32,
    /// Settled-state hash of the exporting world, when it was settled.
    pub origin: Option<StateHash>,
    /// A single valid `cell` declaration in the agent grammar.
    pub source: String,
}

impl CellRecord {
    /// Serialize to wire text.
    pub fn to_text(&self) -> String {
        let origin = match self.origin {
            Some(h) => format!("origin-hash {:#018x}\n", h.0),
            None => String::new(),
        };
        format!("cellrecord v{}\n{}{}\n", self.version, origin, self.source)
    }

    /// Parse wire text back into a record.
    pub fn from_text(text: &str) -> Result<CellRecord, SubmitError> {
        let mut lines = text.lines();
        let head = lines.next().unwrap_or_default();
        let version: u32 = head
            .strip_prefix("cellrecord v")
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| {
                SubmitError::Parse("missing cellrecord header".to_string())
            })?;
        let mut origin = None;
        let mut source_lines: Vec<&str> = Vec::new();
        for line in lines {
            if let Some(h) = line.strip_prefix("origin-hash ") {
                let h = h.trim_start_matches("0x");
                let v = u64::from_str_radix(h, 16).map_err(|_| {
                    SubmitError::Parse("bad origin-hash".to_string())
                })?;
                origin = Some(StateHash(v));
            } else if !line.trim().is_empty() {
                source_lines.push(line);
            }
        }
        Ok(CellRecord { version, origin, source: source_lines.join("\n") })
    }
}

/// Deterministic convex hull (Andrew's monotone chain). Points are sorted
/// by (x, y) with total order; the result is counter-clockwise and
/// identical for identical inputs — no hash-map traversal, no epsilon
/// tuning.
fn convex_hull(points: &[Vec2]) -> Vec<Vec2> {
    let mut pts: Vec<Vec2> = points.to_vec();
    pts.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap()
            .then(a.y.partial_cmp(&b.y).unwrap())
    });
    pts.dedup();
    if pts.len() < 3 {
        return pts;
    }
    let cross = |o: Vec2, a: Vec2, b: Vec2| -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    };
    let mut lower: Vec<Vec2> = Vec::new();
    for &p in &pts {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0
        {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<Vec2> = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0
        {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

/// Static validation over the parsed layout (DS02 error table). Runs before
/// any engine state changes, preserving atomicity.
fn validate(layout: &Layout) -> Result<(), SubmitError> {
    use std::collections::{BTreeMap, BTreeSet};

    // duplicate declarations are malformed programs
    let mut ids: BTreeSet<&str> = BTreeSet::new();
    for c in &layout.cells {
        if !ids.insert(c.id.as_str()) {
            return Err(SubmitError::Parse(format!(
                "duplicate cell id '{}'",
                c.id
            )));
        }
    }

    // material dial domains (E-RANGE)
    for c in &layout.cells {
        let dial = |name: &str, v: f64, lo: f64, hi: f64| {
            if v < lo || v > hi {
                Err(SubmitError::OutOfRange(format!(
                    "cell '{}': {} = {} outside [{}, {}]",
                    c.id, name, v, lo, hi
                )))
            } else {
                Ok(())
            }
        };
        dial("S", c.s, 0.0, 1.0)?;
        dial("xi", c.xi, 0.0, 1.0)?;
        dial("D", c.d, 0.0, 1.0)?;
        if c.n <= 0.0 {
            return Err(SubmitError::OutOfRange(format!(
                "cell '{}': n = {} must be positive",
                c.id, c.n
            )));
        }
    }

    // reference checks (E-REF) and containment edges for cycle detection
    let known = |id: &str| ids.contains(id);
    let mut parents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for op in &layout.ops {
        match op {
            Op::Solidity(mask, v) => {
                if !(0.0..=1.0).contains(v) {
                    return Err(SubmitError::OutOfRange(format!(
                        "solidity value {} outside [0, 1]",
                        v
                    )));
                }
                match mask {
                    Mask::All => {}
                    Mask::One(id) | Mask::Not(id) => {
                        if !known(id) {
                            return Err(SubmitError::UnknownRef(format!(
                                "solidity mask names unknown cell '{}'",
                                id
                            )));
                        }
                    }
                }
            }
            Op::Bind(a, b) => {
                if !known(a) || !known(b) {
                    return Err(SubmitError::UnknownRef(format!(
                        "bind references unknown cell ('{}', '{}')",
                        a, b
                    )));
                }
                if a == b {
                    return Err(SubmitError::UnknownRef(format!(
                        "bind requires two distinct cells, got '{}' twice",
                        a
                    )));
                }
            }
            Op::Contain(children, container) => {
                for ch in children {
                    if !known(ch) {
                        return Err(SubmitError::UnknownRef(format!(
                            "contain lists unknown cell '{}'",
                            ch
                        )));
                    }
                    parents.entry(ch.as_str()).or_default().push(container);
                }
                // container may be a declared cell or an external vessel
                // name (e.g. "viewport"); only declared cells participate
                // in cycle detection.
            }
        }
    }

    // cycle detection over child -> container edges among declared cells
    fn dfs<'a>(
        node: &'a str,
        parents: &BTreeMap<&'a str, Vec<&'a str>>,
        seen: &mut Vec<&'a str>,
    ) -> Option<String> {
        if seen.contains(&node) {
            return Some(format!("containment cycle through '{}'", node));
        }
        seen.push(node);
        if let Some(ps) = parents.get(node) {
            for p in ps {
                if let Some(e) = dfs(p, parents, seen) {
                    return Some(e);
                }
            }
        }
        seen.pop();
        None
    }
    for c in &layout.cells {
        let mut seen = Vec::new();
        if let Some(e) = dfs(c.id.as_str(), &parents, &mut seen) {
            return Err(SubmitError::ContainmentCycle(e));
        }
    }
    Ok(())
}
