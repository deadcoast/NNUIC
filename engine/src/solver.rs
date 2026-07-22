//! Deterministic XPBD solver for a continuum of regions (paper §5).
//!
//! Discretization (R1 choice): each region is a small blob of particles held by
//! internal cohesion + turgor (size) constraints. Regions interact by
//! non-penetration contacts and are confined by container walls. Every
//! constraint's compliance is set each frame from the region's material state
//! via `rheology::compliance`, so one field S spans liquid→locked.
//!
//! Determinism (paper §5.4 / R2): fixed dt, fixed constraint order, index-order
//! reductions, seeded init PRNG, single thread. No wall-clock enters state.

use crate::rheology::{self, Material};
use crate::vec2::Vec2;

/// Deterministic splitmix64 → uniform f64 in [0,1). Used only for init layout.
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    #[inline]
    fn unit(&mut self) -> f64 {
        // 53-bit mantissa → deterministic double in [0,1)
        (self.next_u64() >> 11) as f64 * (1.0 / ((1u64 << 53) as f64))
    }
}

#[derive(Clone, Copy)]
pub struct Region {
    pub mat: Material,
    pub target_radius: f64, // per-particle turgor radius (region size handle)
    pub first: usize,
    pub count: usize,
}

enum CKind {
    Dist { a: usize, b: usize, rest: f64 },      // bilateral cohesion
    Turgor { p: usize, cx: f64, cy: f64, rest: f64 }, // bilateral, toward centroid
    Contact { a: usize, b: usize, rest: f64 },   // unilateral non-penetration
    Wall { p: usize, nx: f64, ny: f64, offset: f64 }, // unilateral confinement
    /// D2: per-region area constraint (the paper's volume/occupancy
    /// constraint) over the ordered hull vertices. Fill is SOLVED in the
    /// projection loop — its multiplier accumulates, so elastic pullback
    /// cannot starve growth, and wrapping is self-defeating (enclosed
    /// foreign space counts against the wrapper's own area budget).
    Area { hull: Vec<usize>, target: f64 },
    /// D2 completion: pairwise region overlap as an IN-LOOP unilateral
    /// constraint — the projection depth of the two regions along their
    /// centroid axis, re-evaluated every sweep, resolved in translation
    /// mode. Lives in the same Gauss–Seidel solve as the area
    /// constraints, so the equilibrium is negotiated, not raced.
    Overlap { ra: usize, rb: usize },
}

struct Constraint {
    kind: CKind,
    alpha: f64,
    lambda: f64,
}

/// C1 guard tolerance: interpenetration beyond this is a topological
/// violation still being resolved — such matter is not arrestable (the
/// deadband and latch may only grab geometrically valid matter; freezing a
/// wedge makes non-penetration infinitely soft at rest, negating C1).
const CONTACT_TOL: f64 = 0.05;
/// D2 overlap slack: pairwise hull interpenetration below this is
/// tolerated (the constraint stays inactive). Center hulls of properly
/// tiled matter are separated by a full contact rest (~2 particle
/// radii), so any positive SAT depth is already interleaving; the slack
/// just keeps the constraint from chattering at the boundary.
const OVERLAP_SLACK: f64 = 0.25;
/// Arrest threshold for the hull-level C1 guard: a pair deeper than this
/// is a topological violation still being resolved — not arrestable.
/// Sits above OVERLAP_SLACK so there is hysteresis between where the
/// constraint pushes to (≤ slack) and where the latch may engage.
const OVERLAP_ARREST: f64 = 0.5;
/// Consecutive all-sub-yield frames before a region latches into rest
/// (~0.4 s at 60 fps) — the region-granular realization of the v0.2
/// jamming latch (paper §7.3). Public: the API layer reads the latch as
/// the sustained settled predicate (a region is settled iff latched).
pub const REST_FRAMES: u32 = 24;
/// Zero-net-transport threshold for the rattle cage (paper §7.3): a
/// particle whose per-tick corrections stay high but whose net transport
/// over a REST_FRAMES window is below this is caged — macroscopically at
/// rest — and is arrested even supra-yield. This is what stabilizes a
/// particle wedged in a cavity narrower than itself, whose overlap is
/// topologically unresolvable; the held residual is bounded internal
/// prestress, exactly what a yield-stress medium sustains at rest.
const R_CAGE_NET: f64 = 0.25;

pub struct World {
    pub pos: Vec<Vec2>,
    pub prev: Vec<Vec2>,
    pub vel: Vec<Vec2>,
    pub inv_mass: Vec<f64>,
    pub radius: Vec<f64>,
    /// Per-region consecutive all-sub-yield frame count; a region with
    /// `rest_t >= REST_FRAMES` is latched at rest: rigid below yield, its
    /// material constraints held (not projected), waking only on a
    /// supra-yield disturbance (paper v0.2 §7).
    pub rest_t: Vec<u32>,
    /// E_fill (paper eq. 6): when enabled, the medium expands into void
    /// until the vessel is claimed — "the slime fills the bowl". Growth is
    /// a quasi-static displacement (C2) that yields against S, so soft
    /// matter claims more space: SIZES ARE SOLVED from material state.
    pub fill_enabled: bool,
    /// The fill phase ends at PRESSURE STALL — when expansion no longer
    /// realizes new claimed area — not at an area target. Pressed against
    /// walls and each other, cells flatten into tiles and corners are
    /// claimed (tessellation, REQ15). Latched here; released by resize
    /// or injection.
    pub fill_done: bool,
    last_occ: f64,
    occ_stall: u32,
    /// Deliberate bind pairs (region indices): E_cohesion across the
    /// marked interface — bound cells pack together and move as a group.
    pub binds: Vec<(usize, usize)>,
    /// Per-region area targets (the paper's volume/occupancy constraint):
    /// shares of the vessel solved from softness, (1-S)², so material
    /// state determines size. Empty for legacy (non-layout) worlds.
    pub area_target: Vec<f64>,
    /// C2 refinement: particles whose realized velocity reversed direction
    /// last frame carry no momentum into the next predict (oscillation in
    /// an overdamped medium dissipates; carried momentum is what sustains
    /// deterministic limit cycles).
    pub no_carry: Vec<bool>,
    /// Cage window position markers (paper §7.3, zero-net-transport form).
    pub cage_mark: Vec<Vec2>,
    /// Particles currently classified as caged rattle.
    pub rattle: Vec<bool>,
    /// Frame counter (drives the cage window cadence).
    pub frame: u64,
    pub turgor_rest: Vec<f64>, // per-particle rest distance to its region centroid
    pub region_of: Vec<usize>,
    pub regions: Vec<Region>,
    pub w: f64,
    pub h: f64,
    // params
    pub dt: f64,
    pub n_solve: usize,
    pub alpha_base: f64,
    pub alpha_wall: f64,
    pub alpha_contact: f64, // non-penetration is stiff regardless of material
    pub damping: f64,
    pub eps_settle: f64,
    pub yield_vel: f64, // structure-gated arrest scale (thixotropic re-solidification)
    pub use_turgor: bool,
    // persistent cohesion topology (a,b,rest)
    cohesion: Vec<(usize, usize, f64)>,
    cell: f64, // spatial grid cell size for contacts
}

impl World {
    /// Build a world of `n_regions` blobs, `side`×`side` particles each,
    /// clumped near the container centre and overlapping, so settling = spread &
    /// pack to fill the container (E_fill emergent). Deterministic from `seed`.
    pub fn new_packed(n_regions: usize, side: usize, w: f64, h: f64, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let spacing = 1.0; // particle rest spacing within a blob
        let prad = 0.5; // particle radius
        let target_radius = spacing; // turgor rest = neighbour spacing

        let mut pos = Vec::new();
        let mut region_of = Vec::new();
        let mut regions = Vec::new();
        let mut cohesion = Vec::new();

        // Jittered grid placement: regions start roughly laid out with small
        // random offsets, so settling is a well-posed relaxation (nudge apart,
        // tidy shapes) that converges — not an explosive de-jamming. Mild jitter
        // creates real settling work (some neighbours overlap and must separate).
        let cols = (n_regions as f64).sqrt().ceil() as usize;
        let rows = (n_regions + cols - 1) / cols;
        let cellx = w / cols as f64;
        let celly = h / rows as f64;
        let jit = 0.35 * cellx.min(celly);

        for r in 0..n_regions {
            let first = pos.len();
            let col = r % cols;
            let row = r / cols;
            let ox = (col as f64 + 0.5) * cellx + (rng.unit() - 0.5) * 2.0 * jit;
            let oy = (row as f64 + 0.5) * celly + (rng.unit() - 0.5) * 2.0 * jit;

            // lay a side×side blob
            for j in 0..side {
                for i in 0..side {
                    let px = ox + (i as f64 - (side as f64 - 1.0) * 0.5) * spacing;
                    let py = oy + (j as f64 - (side as f64 - 1.0) * 0.5) * spacing;
                    pos.push(Vec2::new(px, py));
                    region_of.push(r);
                }
            }
            // cohesion: right + down neighbours (deterministic grid mesh) + diagonals
            for j in 0..side {
                for i in 0..side {
                    let idx = first + j * side + i;
                    if i + 1 < side {
                        let b = first + j * side + (i + 1);
                        cohesion.push((idx, b, spacing));
                    }
                    if j + 1 < side {
                        let b = first + (j + 1) * side + i;
                        cohesion.push((idx, b, spacing));
                    }
                    if i + 1 < side && j + 1 < side {
                        let b = first + (j + 1) * side + (i + 1);
                        cohesion.push((idx, b, spacing * std::f64::consts::SQRT_2));
                    }
                }
            }

            // vary solidity a little across regions so the test isn't uniform;
            // one region is deliberately near-locked to exercise contrast.
            let s = if r == 0 { 0.985 } else { 0.15 + 0.5 * rng.unit() };
            regions.push(Region {
                mat: Material::fluidish(s),
                target_radius,
                first,
                count: side * side,
            });
        }

        let n = pos.len();
        // per-particle turgor rest = initial distance to region centroid, so
        // turgor and cohesion agree at rest (no perpetual conflict).
        let mut turgor_rest = vec![0.0; n];
        for reg in &regions {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for k in reg.first..reg.first + reg.count {
                sx += pos[k].x;
                sy += pos[k].y;
            }
            let inv = 1.0 / reg.count as f64;
            let c = Vec2::new(sx * inv, sy * inv);
            for k in reg.first..reg.first + reg.count {
                turgor_rest[k] = pos[k].sub(c).len();
            }
        }
        World {
            prev: pos.clone(),
            vel: vec![Vec2::zero(); n],
            inv_mass: vec![1.0; n],
            radius: vec![prad; n],
            rest_t: vec![0; regions.len()],
            fill_enabled: false,
            fill_done: false,
            last_occ: 0.0,
            occ_stall: 0,
            binds: Vec::new(),
            area_target: Vec::new(),
            no_carry: vec![false; n],
            cage_mark: pos.clone(),
            rattle: vec![false; n],
            frame: 0,
            turgor_rest,
            region_of,
            regions,
            pos,
            w,
            h,
            dt: 1.0 / 60.0,
            n_solve: 12,
            alpha_base: 1.0e-2,
            alpha_wall: 1.0e-6,    // stiff container walls
            alpha_contact: 1.0e-6, // stiff non-penetration between regions
            damping: 0.2,       // overdamped: little carried momentum (paper §4.4)
            eps_settle: 2.0e-2,
            yield_vel: 0.30,
            use_turgor: true,
            cohesion,
            cell: 1.0, // ~ 2×particle radius
        }
    }

    fn region_centroids(&self) -> Vec<Vec2> {
        // fixed index-order sum → deterministic float accumulation
        let mut c = vec![Vec2::zero(); self.regions.len()];
        for r in 0..self.regions.len() {
            let reg = &self.regions[r];
            let mut sx = 0.0;
            let mut sy = 0.0;
            for k in reg.first..reg.first + reg.count {
                sx += self.pos[k].x;
                sy += self.pos[k].y;
            }
            let inv = 1.0 / reg.count as f64;
            c[r] = Vec2::new(sx * inv, sy * inv);
        }
        c
    }

    fn region_strain_rate(&self) -> Vec<f64> {
        let mut sr = vec![0.0; self.regions.len()];
        for r in 0..self.regions.len() {
            let reg = &self.regions[r];
            let mut s = 0.0;
            for k in reg.first..reg.first + reg.count {
                s += self.vel[k].len();
            }
            let mean_speed = s / reg.count as f64;
            sr[r] = mean_speed / reg.target_radius;
        }
        sr
    }

    /// Deterministic contact pair generation via a uniform grid, then sorted.
    fn build_contacts(&self) -> Vec<(usize, usize, f64)> {
        use std::collections::HashMap;
        let inv = 1.0 / self.cell;
        let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for i in 0..self.pos.len() {
            let gx = (self.pos[i].x * inv).floor() as i64;
            let gy = (self.pos[i].y * inv).floor() as i64;
            grid.entry((gx, gy)).or_default().push(i);
        }
        let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..self.pos.len() {
            let gx = (self.pos[i].x * inv).floor() as i64;
            let gy = (self.pos[i].y * inv).floor() as i64;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let Some(bucket) = grid.get(&(gx + dx, gy + dy)) {
                        for &j in bucket {
                            if j <= i {
                                continue;
                            }
                            if self.region_of[i] == self.region_of[j] {
                                continue; // same region handled by cohesion
                            }
                            let rest = self.radius[i] + self.radius[j];
                            let d = self.pos[i].sub(self.pos[j]).len();
                            if d < rest * 1.05 {
                                pairs.push((i, j, rest));
                            }
                        }
                    }
                }
            }
        }
        // sort → order independent of HashMap traversal → deterministic
        pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        pairs
    }

    fn assemble(&self, centroids: &[Vec2], sr: &[f64]) -> Vec<Constraint> {
        let mut cs: Vec<Constraint> = Vec::new();

        // cohesion. A latched-at-rest region is rigid below yield: its
        // material constraints are HELD, not projected — a yield-stress
        // material at rest sustains its stored stress without relaxing it
        // (paper v0.2 §7). Re-projecting stored violations at rest-stiffened
        // compliance is what re-excites a settled packing.
        for &(a, b, rest) in &self.cohesion {
            let r = self.region_of[a];
            if self.rest_t[r] >= REST_FRAMES {
                continue;
            }
            let alpha = rheology::compliance(&self.regions[r].mat, sr[r], self.alpha_base);
            cs.push(Constraint { kind: CKind::Dist { a, b, rest }, alpha, lambda: 0.0 });
        }
        // turgor (toward region centroid) — held for resting regions too
        if self.use_turgor {
        for r in 0..self.regions.len() {
            if self.rest_t[r] >= REST_FRAMES {
                continue;
            }
            let reg = &self.regions[r];
            let alpha = rheology::compliance(&reg.mat, sr[r], self.alpha_base);
            let c = centroids[r];
            for k in reg.first..reg.first + reg.count {
                cs.push(Constraint {
                    kind: CKind::Turgor { p: k, cx: c.x, cy: c.y, rest: self.turgor_rest[k] },
                    alpha,
                    lambda: 0.0,
                });
            }
        }
        }
        // D2: fill as per-region AREA constraints (the paper's
        // volume/occupancy constraint) — solved jointly with everything
        // else, so growth cannot be starved and wrapping is
        // self-defeating. Held (skipped) for resting regions like all
        // material constraints.
        if self.fill_enabled {
            const ALPHA_AREA: f64 = 1.0e-4;
            for r in 0..self.regions.len() {
                if self.rest_t[r] >= REST_FRAMES
                    || r >= self.area_target.len()
                {
                    continue;
                }
                let reg = &self.regions[r];
                let hull = convex_hull_indices(
                    &self.pos,
                    reg.first,
                    reg.count,
                );
                // NOTE (pour shape, measured): ramped effective targets
                // were tried twice — growth relative to own size (2%/f)
                // and synchronized target-share increments — and both
                // WORSENED the worst-pair jam (3.2% → 22% → 33.5% hull
                // overlap): the jam is seeded by spawn placement, not
                // pour speed, and a slow pour gives the seeded engulfment
                // longer to consolidate. The instant target with the
                // XPBD compliance as the rate limit is the best-measured
                // pour (see CHANGELOG 027).
                cs.push(Constraint {
                    kind: CKind::Area {
                        hull,
                        target: self.area_target[r],
                    },
                    alpha: ALPHA_AREA,
                    lambda: 0.0,
                });
            }
            // D2 completion: overlap constraints for awake pairs whose
            // bounding boxes intersect — axis fixed per frame, depth
            // re-evaluated every sweep in projection (in-loop, so the
            // equilibrium with the area constraints is negotiated).
            const ALPHA_OVERLAP: f64 = 1.0e-3;
            let n_r = self.regions.len();
            let mut boxes = Vec::with_capacity(n_r);
            for r in 0..n_r {
                let reg = &self.regions[r];
                let (mut minx, mut miny) =
                    (f64::INFINITY, f64::INFINITY);
                let (mut maxx, mut maxy) =
                    (f64::NEG_INFINITY, f64::NEG_INFINITY);
                for k in reg.first..reg.first + reg.count {
                    let p = self.pos[k];
                    minx = minx.min(p.x);
                    miny = miny.min(p.y);
                    maxx = maxx.max(p.x);
                    maxy = maxy.max(p.y);
                }
                boxes.push((minx, miny, maxx, maxy));
            }
            for a in 0..n_r {
                for b in (a + 1)..n_r {
                    if self.rest_t[a] >= REST_FRAMES
                        && self.rest_t[b] >= REST_FRAMES
                    {
                        continue;
                    }
                    let (ax0, ay0, ax1, ay1) = boxes[a];
                    let (bx0, by0, bx1, by1) = boxes[b];
                    if ax1 < bx0 || bx1 < ax0 || ay1 < by0 || by1 < ay0
                    {
                        continue;
                    }
                    cs.push(Constraint {
                        kind: CKind::Overlap { ra: a, rb: b },
                        alpha: ALPHA_OVERLAP,
                        lambda: 0.0,
                    });
                }
            }

            // D2: binds as E_cohesion across the INTERFACE — a distance
            // constraint pulling the nearest particle pair of the two
            // regions to contact rest, constant soft compliance (never
            // stiffness-scaled, C7). Adjacency is the demand, never
            // superposition: a rest-0 anchor on lattice-coincident
            // particles drags bound cells INTO each other (measured:
            // SAT depth 26 on a bound pair — near-total overlap the
            // exclusion constraint then fights forever). With area
            // protected by its own constraint, translation is the only
            // compliant path — one interface spring transports bulk.
            const ALPHA_BIND: f64 = 0.3;
            for &(ra, rb) in &self.binds {
                if self.rest_t[ra] >= REST_FRAMES
                    && self.rest_t[rb] >= REST_FRAMES
                {
                    continue;
                }
                let a = self.regions[ra];
                let b = self.regions[rb];
                // deterministic nearest pair: index-ordered scan, strict
                // improvement keeps the first minimum
                let (mut ka, mut kb, mut best) =
                    (a.first, b.first, f64::INFINITY);
                for i in a.first..a.first + a.count {
                    for j in b.first..b.first + b.count {
                        let d = self.pos[i].sub(self.pos[j]).len();
                        if d < best {
                            best = d;
                            ka = i;
                            kb = j;
                        }
                    }
                }
                cs.push(Constraint {
                    kind: CKind::Dist {
                        a: ka,
                        b: kb,
                        rest: self.radius[ka] + self.radius[kb],
                    },
                    alpha: ALPHA_BIND,
                    lambda: 0.0,
                });
            }
        }

        // contacts: non-penetration is stiff for every material. Two regions do
        // not interpenetrate regardless of how soft each is; material softness
        // lives in cohesion/turgor (how a region deforms), not in whether
        // regions can occupy the same space.
        // Held for latched pairs like every other constraint: a
        // cage-latched jam holds its residual contact violation as
        // bounded prestress (§7.3); re-projecting it at rest is what
        // erupts a settled packing (measured: 4.9 u/s flicker at
        // observation frame 0). A contact with an AWAKE side stays —
        // flowing matter must not pass through latched matter.
        for (a, b, rest) in self.build_contacts() {
            if self.rest_t[self.region_of[a]] >= REST_FRAMES
                && self.rest_t[self.region_of[b]] >= REST_FRAMES
            {
                continue;
            }
            cs.push(Constraint { kind: CKind::Contact { a, b, rest }, alpha: self.alpha_contact, lambda: 0.0 });
        }
        // walls — held for latched regions (same rest semantics)
        for p in 0..self.pos.len() {
            if self.rest_t[self.region_of[p]] >= REST_FRAMES {
                continue;
            }
            let r = self.radius[p];
            cs.push(Constraint { kind: CKind::Wall { p, nx: 1.0, ny: 0.0, offset: r }, alpha: self.alpha_wall, lambda: 0.0 });
            cs.push(Constraint { kind: CKind::Wall { p, nx: -1.0, ny: 0.0, offset: r - self.w }, alpha: self.alpha_wall, lambda: 0.0 });
            cs.push(Constraint { kind: CKind::Wall { p, nx: 0.0, ny: 1.0, offset: r }, alpha: self.alpha_wall, lambda: 0.0 });
            cs.push(Constraint { kind: CKind::Wall { p, nx: 0.0, ny: -1.0, offset: r - self.h }, alpha: self.alpha_wall, lambda: 0.0 });
        }
        cs
    }

    #[inline]
    fn project(&mut self, c: &mut Constraint, dt2: f64) {
        let alpha_tilde = c.alpha / dt2;
        if let CKind::Area { ref hull, target } = c.kind {
            // D2: XPBD area constraint over the ordered hull. One scalar
            // multiplier per region; polygon-area gradients on vertices.
            let hull = hull.clone();
            let n = hull.len();
            if n < 3 {
                return;
            }
            let mut area = 0.0;
            for i in 0..n {
                let p = self.pos[hull[i]];
                let q = self.pos[hull[(i + 1) % n]];
                area += p.x * q.y - q.x * p.y;
            }
            area *= 0.5;
            let cval = area - target;
            let mut denom = alpha_tilde;
            let mut grads = Vec::with_capacity(n);
            for i in 0..n {
                let prev = self.pos[hull[(i + n - 1) % n]];
                let next = self.pos[hull[(i + 1) % n]];
                let g = Vec2::new(
                    0.5 * (next.y - prev.y),
                    0.5 * (prev.x - next.x),
                );
                denom += self.inv_mass[hull[i]] * g.dot(g);
                grads.push(g);
            }
            if denom <= 0.0 {
                return;
            }
            let dl = (-cval - alpha_tilde * c.lambda) / denom;
            c.lambda += dl;
            for i in 0..n {
                let k = hull[i];
                self.pos[k] = self.pos[k]
                    .add(grads[i].scale(self.inv_mass[k] * dl));
            }
            return;
        }
        if let CKind::Overlap { ra, rb } = c.kind {
            // Exact convex-polygon SAT: for convex hulls a separating
            // axis, if one exists, is an edge normal of one of the two
            // hulls. Testing arbitrary axes both misses real overlap and
            // pushes pairs that do not overlap (measured: sustained
            // false-positive pushing broke settle at tight slack).
            // Resolution is along the minimum-translation axis.
            let (depth, dir) = self.pair_sat(ra, rb);
            if depth <= OVERLAP_SLACK {
                return; // separated (or within slack): inactive
            }
            let a = Region { ..self.regions[ra] };
            let b = Region { ..self.regions[rb] };
            let cval = depth - OVERLAP_SLACK;
            let na = a.count as f64;
            let nb = b.count as f64;
            // a latched region is rigid at rest: infinite mass here —
            // the awake side does all the moving
            let wa = if self.rest_t[ra] >= REST_FRAMES {
                0.0
            } else {
                1.0 / na
            };
            let wb = if self.rest_t[rb] >= REST_FRAMES {
                0.0
            } else {
                1.0 / nb
            };
            if wa + wb == 0.0 {
                return;
            }
            let denom = wa + wb + alpha_tilde;
            let dl = (-cval - alpha_tilde * c.lambda) / denom;
            c.lambda += dl;
            // XPBD signs: dl < 0 for a violated (positive) C, and the
            // gradients are +dir/na for A, -dir/nb for B — so A moves
            // AGAINST dir (away from b) and B along it.
            if wa > 0.0 {
                let da = dir.scale(dl / na);
                for k in a.first..a.first + a.count {
                    self.pos[k] = self.pos[k].add(da);
                }
            }
            if wb > 0.0 {
                let db = dir.scale(-dl / nb);
                for k in b.first..b.first + b.count {
                    self.pos[k] = self.pos[k].add(db);
                }
            }
            return;
        }
        // Rigid at rest (§7): a latched region's particles have infinite
        // mass in every surviving pairwise constraint — awake matter
        // flows AROUND settled matter, it does not shove it supra-yield
        // and tear the latch open (measured: 13 CellSettled events for 4
        // cells — settle-event flapping during consolidation).
        let eff = |w: &World, k: usize| -> f64 {
            if w.rest_t[w.region_of[k]] >= REST_FRAMES {
                0.0
            } else {
                w.inv_mass[k]
            }
        };
        match c.kind {
            CKind::Area { .. } => {}
            CKind::Overlap { .. } => {}
            CKind::Dist { a, b, rest } => {
                let d = self.pos[a].sub(self.pos[b]);
                let len = d.len();
                let cval = len - rest;
                let n = d.norm_or(Vec2::new(1.0, 0.0));
                let (wa, wb) = (eff(self, a), eff(self, b));
                let wsum = wa + wb;
                let denom = wsum + alpha_tilde;
                if wsum == 0.0 || denom == 0.0 {
                    return;
                }
                let dl = (-cval - alpha_tilde * c.lambda) / denom;
                c.lambda += dl;
                self.pos[a] = self.pos[a].add(n.scale(wa * dl));
                self.pos[b] = self.pos[b].add(n.scale(-wb * dl));
            }
            CKind::Turgor { p, cx, cy, rest } => {
                let anchor = Vec2::new(cx, cy);
                let d = self.pos[p].sub(anchor);
                let len = d.len();
                let cval = len - rest;
                let n = d.norm_or(Vec2::new(1.0, 0.0));
                let wsum = self.inv_mass[p];
                let denom = wsum + alpha_tilde;
                if denom == 0.0 {
                    return;
                }
                let dl = (-cval - alpha_tilde * c.lambda) / denom;
                c.lambda += dl;
                self.pos[p] = self.pos[p].add(n.scale(self.inv_mass[p] * dl));
            }
            CKind::Contact { a, b, rest } => {
                let d = self.pos[a].sub(self.pos[b]);
                let len = d.len();
                let cval = len - rest;
                if cval >= 0.0 {
                    return; // separated: unilateral, inactive
                }
                let n = d.norm_or(Vec2::new(1.0, 0.0));
                let (wa, wb) = (eff(self, a), eff(self, b));
                let wsum = wa + wb;
                let denom = wsum + alpha_tilde;
                if wsum == 0.0 || denom == 0.0 {
                    return;
                }
                let dl = (-cval - alpha_tilde * c.lambda) / denom;
                c.lambda += dl;
                self.pos[a] = self.pos[a].add(n.scale(wa * dl));
                self.pos[b] = self.pos[b].add(n.scale(-wb * dl));
            }
            CKind::Wall { p, nx, ny, offset } => {
                let n = Vec2::new(nx, ny);
                let cval = self.pos[p].dot(n) - offset;
                if cval >= 0.0 {
                    return; // inside: inactive
                }
                let wsum = self.inv_mass[p];
                let denom = wsum + alpha_tilde;
                if denom == 0.0 {
                    return;
                }
                let dl = (-cval - alpha_tilde * c.lambda) / denom;
                c.lambda += dl;
                self.pos[p] = self.pos[p].add(n.scale(self.inv_mass[p] * dl));
            }
        }
    }

    /// One deterministic solver tick (paper §5.3).
    /// Fraction of the vessel claimed by matter (bounding-extent
    /// estimate). Drives the E_fill phase toward its target.
    pub fn occupancy(&self) -> f64 {
        let mut area = 0.0;
        for reg in &self.regions {
            let (mut minx, mut miny) = (f64::INFINITY, f64::INFINITY);
            let (mut maxx, mut maxy) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
            for k in reg.first..reg.first + reg.count {
                let p = self.pos[k];
                minx = minx.min(p.x);
                miny = miny.min(p.y);
                maxx = maxx.max(p.x);
                maxy = maxy.max(p.y);
            }
            area += (maxx - minx).max(0.0) * (maxy - miny).max(0.0);
        }
        area / (self.w * self.h)
    }

    /// Fraction of the vessel boundary within reach of matter. The fill
    /// phase may not stall while wall spans remain unclaimed — edge to
    /// edge is part of E_fill's definition, not a nice-to-have.
    pub fn wall_coverage(&self) -> f64 {
        let seg = 2.0;
        let reach = 3.0;
        let nx = (self.w / seg).ceil() as usize;
        let ny = (self.h / seg).ceil() as usize;
        let mut cov = vec![false; 2 * nx + 2 * ny];
        for p in &self.pos {
            if p.y < reach {
                cov[(p.x / seg).floor().clamp(0.0, nx as f64 - 1.0) as usize] =
                    true;
            }
            if p.y > self.h - reach {
                cov[nx
                    + (p.x / seg).floor().clamp(0.0, nx as f64 - 1.0) as usize] =
                    true;
            }
            if p.x < reach {
                cov[2 * nx
                    + (p.y / seg).floor().clamp(0.0, ny as f64 - 1.0) as usize] =
                    true;
            }
            if p.x > self.w - reach {
                cov[2 * nx
                    + ny
                    + (p.y / seg).floor().clamp(0.0, ny as f64 - 1.0) as usize] =
                    true;
            }
        }
        cov.iter().filter(|&&c| c).count() as f64 / cov.len() as f64
    }

    /// Viewport change: the vessel reshapes and the medium re-settles
    /// with no further program call (paper §9, zero-cost adaptation).
    /// The whole body is under imposed shear: latches release, structure
    /// erodes, and the fill phase re-runs in the new vessel.
    pub fn resize(&mut self, w: f64, h: f64) {
        self.w = w;
        self.h = h;
        for r in 0..self.regions.len() {
            self.rest_t[r] = 0;
            self.regions[r].mat.xi *= 0.3;
        }
        for k in 0..self.cage_t_len() {
            self.rattle[k] = false;
            self.cage_mark[k] = self.pos[k];
        }
        // the fill phase re-runs in the reshaped vessel: area shares are
        // re-solved from the same material state over the new vessel area
        self.fill_done = false;
        self.last_occ = 0.0;
        self.occ_stall = 0;
        if !self.area_target.is_empty() {
            self.set_area_targets();
        }
    }

    /// Solve area shares from softness: share_r ∝ (1-S_r)², normalized
    /// over the vessel. Material state determines size (§9); nothing is
    /// authored.
    pub fn set_area_targets(&mut self) {
        // Targets are CENTER-HULL areas, so they must share the domain
        // hull centers can actually reach: the vessel inset by the
        // particle radius band (walls hold centers a radius off the
        // boundary; matter covers the band, centers cannot). Shares of
        // the full vessel command more hull than fits — overlap becomes
        // the only satisfying state (measured: hulls 1.2–1.9× vessel).
        // Radius depends on dilated spacing which depends on the target,
        // so fix-point the inset (converges in a couple of rounds).
        let n: usize = self.regions.iter().map(|r| r.count).sum();
        let mut inset = self.w * self.h;
        for _ in 0..3 {
            let spacing = (inset / n.max(1) as f64).sqrt();
            let r = (0.5 * spacing).clamp(0.5, 6.0);
            inset = (self.w - 2.0 * r).max(1.0)
                * (self.h - 2.0 * r).max(1.0);
        }
        let weights: Vec<f64> = self
            .regions
            .iter()
            .map(|r| ((1.0 - r.mat.s).max(0.1)).powi(2))
            .collect();
        let sum: f64 = weights.iter().sum();
        self.area_target =
            weights.iter().map(|w| inset * w / sum).collect();
    }

    fn cage_t_len(&self) -> usize {
        self.pos.len()
    }

    pub fn step(&mut self) {
        let dt = self.dt;
        let dt2 = dt * dt;

        // E_fill phase (paper eq. 6): active until PRESSURE STALL — the
        // medium keeps claiming space until expansion stops realizing new
        // area, so cells flatten against walls and each other and corners
        // are claimed (tessellation). While filling, the frame is
        // effectively accelerating (the boot pour): ξ rebuild is
        // suspended (§7.2) so the medium flows freely into void.
        if self.fill_enabled && !self.fill_done && self.frame % 15 == 0 {
            let occ = self.occupancy();
            let stalled = occ > 0.985 || occ - self.last_occ < 0.0004;
            if stalled {
                self.occ_stall += 1;
                // normal exit: growth stalled AND the boundary is claimed
                // edge to edge; hard exit: long stall regardless (safety)
                let walls = self.wall_coverage();
                if (self.occ_stall >= 6 && walls > 0.9)
                    || self.occ_stall >= 40
                {
                    self.fill_done = true;
                }
            } else {
                self.occ_stall = 0;
            }
            self.last_occ = occ;
        }
        let filling = self.fill_enabled && !self.fill_done;

        // strain rate from current velocities; update thixotropic structure
        let sr = self.region_strain_rate();
        for r in 0..self.regions.len() {
            let mat = self.regions[r].mat;
            let next = if filling {
                // rebuild suspended; shear still erodes structure
                (mat.xi - mat.a * mat.xi * sr[r] * dt).clamp(0.0, 1.0)
            } else {
                rheology::step_structure(&mat, sr[r], dt)
            };
            self.regions[r].mat.xi = next;
        }

        // Continuum conservation under dilation (layout worlds): a dilated
        // lattice is not a porous point-cloud — effective particle radius
        // scales with lattice spacing, so cells cannot finger through each
        // other's interiors; they must tile. Radius follows spacing
        // smoothly (spacing grows ~1%/frame during fill), and the contact
        // search grid follows the radius.
        if self.fill_enabled {
            let mut sum = vec![0.0; self.regions.len()];
            let mut cnt = vec![0u32; self.regions.len()];
            for &(a, _b, rest) in &self.cohesion {
                let r = self.region_of[a];
                sum[r] += rest;
                cnt[r] += 1;
            }
            let mut maxr: f64 = 0.5;
            for r in 0..self.regions.len() {
                if cnt[r] == 0 {
                    continue;
                }
                // mean cohesion rest ≈ 1.14 × lattice spacing (diagonals)
                let spacing = sum[r] / cnt[r] as f64 / 1.14;
                let pr = (0.5 * spacing).clamp(0.5, 6.0);
                if pr > maxr {
                    maxr = pr;
                }
                let reg = Region { ..self.regions[r] };
                for k in reg.first..reg.first + reg.count {
                    self.radius[k] = pr;
                }
            }
            self.cell = 2.0 * maxr;
        }

        let centroids_pre = self.region_centroids();

        // predict (layout forces only, C2: quasi-static displacements).
        // Overdamped: only a small fraction of last frame's motion carries.
        for i in 0..self.pos.len() {
            self.prev[i] = self.pos[i];
            if !self.no_carry[i] {
                self.pos[i] =
                    self.pos[i].add(self.vel[i].scale(self.damping * dt));
            }
        }
        // D2 (addendum v0.3): fill and binding are constraints, solved in
        // the projection loop (see assemble); no drives remain here.
        let _ = filling;

        // E_cohesion (addendum v0.3 design): interface tension as
        // boundary curvature flow. Boundary particles are detected by
        // same-region neighbour deficit; the discrete curvature is the
        // boundary Laplacian κ_i = mean(boundary neighbours) − x_i; the
        // tension step is a bounded quasi-static displacement along κ
        // (C2). Flat interfaces carry no force — this cannot fight wall
        // conformity or tiling; bulges retract, dents fill, shard arms
        // collapse. Terminates naturally as κ→0 under deadband and latch.
        if self.fill_enabled {
            const B_NEIGHBOURS: u32 = 6;
            const W_TENSION: f64 = 4.0;
            const C_MAX: f64 = 3.0;
            use std::collections::HashMap;
            let inv = 1.0 / self.cell;
            let mut tgrid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
            for i in 0..self.pos.len() {
                let gx = (self.pos[i].x * inv).floor() as i64;
                let gy = (self.pos[i].y * inv).floor() as i64;
                tgrid.entry((gx, gy)).or_default().push(i);
            }
            let span = 3i64;
            let mut boundary = vec![false; self.pos.len()];
            for r in 0..self.regions.len() {
                if self.rest_t[r] >= REST_FRAMES {
                    continue;
                }
                let reg = Region { ..self.regions[r] };
                let spacing = 2.0 * self.radius[reg.first];
                let rn2 = (1.7 * spacing).powi(2);
                for k in reg.first..reg.first + reg.count {
                    let p = self.pos[k];
                    let gx = (p.x * inv).floor() as i64;
                    let gy = (p.y * inv).floor() as i64;
                    let mut cnt = 0u32;
                    for dy in -span..=span {
                        for dx in -span..=span {
                            if let Some(b) = tgrid.get(&(gx + dx, gy + dy)) {
                                for &j in b {
                                    if j != k && self.region_of[j] == r {
                                        let d = self.pos[j].sub(p);
                                        if d.dot(d) < rn2 {
                                            cnt += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    boundary[k] = cnt < B_NEIGHBOURS;
                }
            }
            let mut disp = vec![Vec2::zero(); self.pos.len()];
            for r in 0..self.regions.len() {
                if self.rest_t[r] >= REST_FRAMES {
                    continue;
                }
                let reg = Region { ..self.regions[r] };
                let spacing = 2.0 * self.radius[reg.first];
                let rb2 = (2.2 * spacing).powi(2);
                for k in reg.first..reg.first + reg.count {
                    if !boundary[k] {
                        continue;
                    }
                    let p = self.pos[k];
                    // interfacial tension acts on FREE surfaces; matter
                    // pressed against a wall is wetting the vessel, not
                    // free boundary — and a corner-filling tip is high
                    // curvature, so tension there would evacuate the
                    // very wedges E_fill claims
                    let wn = 2.5;
                    if p.x < wn
                        || p.y < wn
                        || p.x > self.w - wn
                        || p.y > self.h - wn
                    {
                        continue;
                    }
                    let gx = (p.x * inv).floor() as i64;
                    let gy = (p.y * inv).floor() as i64;
                    let (mut sx, mut sy, mut nb) = (0.0, 0.0, 0u32);
                    for dy in -span..=span {
                        for dx in -span..=span {
                            if let Some(b) = tgrid.get(&(gx + dx, gy + dy)) {
                                for &j in b {
                                    if j != k
                                        && self.region_of[j] == r
                                        && boundary[j]
                                    {
                                        let d = self.pos[j].sub(p);
                                        if d.dot(d) < rb2 {
                                            sx += self.pos[j].x;
                                            sy += self.pos[j].y;
                                            nb += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if nb >= 2 {
                        let m =
                            Vec2::new(sx / nb as f64, sy / nb as f64);
                        let kappa = m.sub(p);
                        let mag =
                            (W_TENSION * kappa.len()).min(C_MAX) * dt;
                        disp[k] = kappa.norm_or(Vec2::zero()).scale(mag);
                    }
                }
            }
            for k in 0..self.pos.len() {
                self.pos[k] = self.pos[k].add(disp[k]);
            }
        }

        // Post-yield flow is plastic (n < 1 realization): while a layout
        // region is awake, its material rest lengths follow the realized
        // geometry at a rate that yields against S. Stiff matter keeps its
        // shape; soft matter flows into the space it claims. At rest the
        // rests hold — elastic below yield.
        if self.fill_enabled {
            const PLASTIC: f64 = 6.0;
            // The plastic ratchet is BOUNDED by the solved share: rests
            // exist to realize the region's occupancy target, never to
            // memorize transient overshoot. Unbounded rests let natural
            // matter exceed the share; summed over regions that is more
            // matter than vessel (measured: 1.6×), separation becomes
            // infeasible, and the C1 guard then rightly never lets the
            // over-packed medium arrest.
            let cap_spacing: Vec<f64> = (0..self.regions.len())
                .map(|r| {
                    if r < self.area_target.len() {
                        (self.area_target[r]
                            / self.regions[r].count as f64)
                            .sqrt()
                    } else {
                        f64::INFINITY
                    }
                })
                .collect();
            for i in 0..self.cohesion.len() {
                let (a, b, rest) = self.cohesion[i];
                let r = self.region_of[a];
                if self.rest_t[r] >= REST_FRAMES {
                    continue;
                }
                let s = self.regions[r].mat.s;
                let d = self.pos[a].sub(self.pos[b]).len();
                let k = (PLASTIC * (1.0 - s) * dt).min(1.0);
                // mean cohesion rest ≈ 1.14 × lattice spacing
                let cap = 1.14 * cap_spacing[r];
                self.cohesion[i] =
                    (a, b, (rest + (d - rest) * k).min(cap));
            }
            for r in 0..self.regions.len() {
                if self.rest_t[r] >= REST_FRAMES {
                    continue;
                }
                let reg = Region { ..self.regions[r] };
                let s = reg.mat.s;
                let k = (PLASTIC * (1.0 - s) * dt).min(1.0);
                let c = centroids_pre[r];
                // corner reach of a square tile of the target area
                let cap = if r < self.area_target.len() {
                    0.75 * self.area_target[r].sqrt()
                } else {
                    f64::INFINITY
                };
                for p in reg.first..reg.first + reg.count {
                    let d = self.pos[p].sub(c).len();
                    self.turgor_rest[p] = (self.turgor_rest[p]
                        + (d - self.turgor_rest[p]) * k)
                        .min(cap);
                }
            }
        }

        let centroids = self.region_centroids();
        let mut cs = self.assemble(&centroids, &sr);

        // Gauss–Seidel sweeps in fixed constraint order
        for _ in 0..self.n_solve {
            for k in 0..cs.len() {
                let mut c = Constraint { kind: replace_kind(&cs[k].kind), alpha: cs[k].alpha, lambda: cs[k].lambda };
                self.project(&mut c, dt2);
                cs[k].lambda = c.lambda;
            }
        }

        // velocity = true per-frame motion (undamped); this is what the settled
        // predicate measures. Damping is applied at predict time, not here.
        // A direction reversal marks the particle no-carry for the next
        // predict (C2: carried momentum sustains limit cycles).
        for i in 0..self.pos.len() {
            let new_v = self.pos[i].sub(self.prev[i]).scale(1.0 / dt);
            self.no_carry[i] = new_v.dot(self.vel[i]) < 0.0;
            self.vel[i] = new_v;
        }

        // Cage window (paper §7.3): every REST_FRAMES frames, classify each
        // particle by net transport over the window. High per-tick motion
        // with near-zero net transport is caged rattle — macroscopically at
        // rest.
        self.frame += 1;
        if self.frame % REST_FRAMES as u64 == 0 {
            for k in 0..self.pos.len() {
                let net = self.pos[k].sub(self.cage_mark[k]).len();
                self.rattle[k] = net < R_CAGE_NET;
                self.cage_mark[k] = self.pos[k];
            }
        }

        // C1 guard: matter interpenetrating beyond tolerance (or violating a
        // wall) is a topological violation still being resolved. It is NOT
        // arrestable — reverting it would freeze the wedge and make
        // non-penetration soft at rest, exactly what correction C1 forbids.
        let mut arrestable = vec![true; self.pos.len()];
        for (a, b, rest) in self.build_contacts() {
            let d = self.pos[a].sub(self.pos[b]).len();
            // tolerance proportional to the contact rest: with dilated
            // radii an absolute tolerance misclassifies ordinary residual
            // prestress as topological violation
            if rest - d > (CONTACT_TOL * rest).max(CONTACT_TOL) {
                arrestable[a] = false;
                arrestable[b] = false;
            }
        }
        for p in 0..self.pos.len() {
            let r = self.radius[p];
            if self.pos[p].x < r - CONTACT_TOL
                || self.pos[p].x > self.w - r + CONTACT_TOL
                || self.pos[p].y < r - CONTACT_TOL
                || self.pos[p].y > self.h - r + CONTACT_TOL
            {
                arrestable[p] = false;
            }
        }
        // C1 guard, hull level (D2): a region pair whose convex hulls
        // interpenetrate beyond OVERLAP_ARREST is a topological
        // violation still being resolved — not arrestable while it still
        // moves. Without this the deadband arrests the sub-yield
        // separation drift and the latch freezes the interleave in
        // (measured: 5.4% hull overlap latched on dashboard). Cage
        // precedence is the SAME as the contact guard's (§7.3): a jammed
        // pair at zero net transport is macroscopically at rest — the
        // held overlap is bounded internal prestress, and clearing the
        // cage here livelocks the whole medium on one stuck pair
        // (measured: 30k frames, no latch, depths frozen).
        if !filling {
            let n_r = self.regions.len();
            for a in 0..n_r {
                for b in (a + 1)..n_r {
                    let (depth, _) = self.pair_sat(a, b);
                    if depth <= OVERLAP_ARREST {
                        continue;
                    }
                    for reg in [&self.regions[a], &self.regions[b]] {
                        for k in reg.first..reg.first + reg.count {
                            arrestable[k] = false;
                        }
                    }
                }
            }
        }

        // v0.2 arrest mechanism (paper §7), region-granular. Below yield a
        // Bingham-class material does not flow at all: sub-yield frame
        // displacement is REVERTED and the material holds the residual
        // stress at rest. The gate scales with ξ (structure ratchet). A
        // region whose particles are all sub-yield for REST_FRAMES
        // consecutive frames latches into rest: rigid, its material
        // constraints held (dropped from assembly), waking only when a
        // supra-yield disturbance or a topological violation (C1 guard)
        // arrives. Strain is measured on realized post-arrest displacement
        // (Invariant 4, sharpened): a settled state is exactly still and
        // stable under continued stepping.
        //
        // Phase gate (§7.2): the fill phase is supra-yield BY DEFINITION —
        // the frame is accelerating (the boot pour), so the deadband must
        // not arrest actively driven matter and no region may latch
        // mid-pour. Without this gate a stiff cell whose slow growth
        // realizes below v_y is reverted every frame, reads quiet, and
        // latches at spawn size, locked out of the fill it never did
        // (measured: kanban's head frozen at area 17 while columns grew).
        if filling {
            for r in 0..self.regions.len() {
                self.rest_t[r] = 0;
            }
        } else {
        for r in 0..self.regions.len() {
            // v_y = v0·ξ·(1 + c_ξ·ξ): the structure ratchet widens the
            // arrest band as ξ matures (bench-validated shape, paper v0.2
            // §7.2), closing the band of creep just above the bare yield
            // that the flat gate leaves open. Sheared matter (ξ→0) keeps an
            // essentially open gate and flows freely.
            let xi = self.regions[r].mat.xi;
            let vy = self.yield_vel * xi * (1.0 + 6.0 * xi);
            let reg = Region { ..self.regions[r] };
            // A particle is quiet when sub-yield and geometrically valid,
            // OR when it is caged rattle (zero net transport — the cage
            // overrides the C1 guard for topologically unresolvable
            // wedges; the held overlap is bounded internal prestress).
            // Latch hysteresis: the guards gate ENTRY into rest, never
            // continuation — a latched region wakes only on supra-yield
            // motion (§7). Without this, the first cage recompute after
            // settling reads pre-latch consolidation transport, drops
            // rattle, and tears a held jam open (measured: 37.8 u/s
            // eruption at observation frame 1).
            let latched = self.rest_t[r] >= REST_FRAMES;
            let quiet = |k: usize, v: f64| -> bool {
                ((arrestable[k] || latched) && v < vy)
                    || self.rattle[k]
            };
            let mut all_quiet = true;
            for k in reg.first..reg.first + reg.count {
                if !quiet(k, self.vel[k].len()) {
                    all_quiet = false;
                    break;
                }
            }
            if all_quiet {
                // below yield everywhere: no flow — hold rigid
                for k in reg.first..reg.first + reg.count {
                    self.pos[k] = self.prev[k];
                    self.vel[k] = Vec2::zero();
                }
                if self.rest_t[r] < u32::MAX {
                    self.rest_t[r] += 1;
                }
                if self.rest_t[r] == REST_FRAMES {
                    // §7.3 as written: the latch SETS ξ = 1. Structure is
                    // fully rebuilt at rest — and settle history cannot
                    // leak into exported material state.
                    self.regions[r].mat.xi = 1.0;
                }
            } else {
                // flowing (or disturbed / interpenetrating): the region is
                // awake; quiet particles still hit the deadband or cage.
                for k in reg.first..reg.first + reg.count {
                    if quiet(k, self.vel[k].len()) {
                        self.pos[k] = self.prev[k];
                        self.vel[k] = Vec2::zero();
                    }
                }
                self.rest_t[r] = 0;
            }
        }
        }
    }

    /// Max particle speed — the strain-rate proxy for the settled predicate.
    /// Exact convex-polygon SAT for a region pair: the minimum interval
    /// overlap over every edge normal of both particle-center hulls
    /// (exact for convex shapes — a separating axis, if one exists, is
    /// among these normals). Returns (depth, dir): depth > 0 is the
    /// minimum-translation interpenetration, depth < 0 the separation
    /// margin; dir is the resolution axis oriented from `ra` toward
    /// `rb`. Deterministic: hulls and edge order are index-stable.
    pub fn pair_sat(&self, ra: usize, rb: usize) -> (f64, Vec2) {
        let reg_a = &self.regions[ra];
        let reg_b = &self.regions[rb];
        let ha = convex_hull(
            &self.pos[reg_a.first..reg_a.first + reg_a.count],
        );
        let hb = convex_hull(
            &self.pos[reg_b.first..reg_b.first + reg_b.count],
        );
        if ha.len() < 3 || hb.len() < 3 {
            // degenerate (collinear) region: no polygon to test
            return (f64::NEG_INFINITY, Vec2::new(1.0, 0.0));
        }
        let mut depth = f64::INFINITY;
        let mut dir = Vec2::new(1.0, 0.0);
        for hull in [&ha, &hb] {
            let m = hull.len();
            for i in 0..m {
                let e = hull[(i + 1) % m].sub(hull[i]);
                let l = e.len();
                if l < 1.0e-9 {
                    continue;
                }
                let axis = Vec2::new(-e.y / l, e.x / l);
                let proj = |h: &[Vec2]| {
                    let (mut lo, mut hi) =
                        (f64::INFINITY, f64::NEG_INFINITY);
                    for p in h {
                        let t = p.dot(axis);
                        lo = lo.min(t);
                        hi = hi.max(t);
                    }
                    (lo, hi)
                };
                let (alo, ahi) = proj(&ha);
                let (blo, bhi) = proj(&hb);
                let ov = ahi.min(bhi) - alo.max(blo);
                if ov < depth {
                    depth = ov;
                    dir = axis;
                }
            }
        }
        // orient the axis from a toward b (centroid order)
        let cent = |h: &[Vec2]| {
            let mut c = Vec2::zero();
            for p in h {
                c = c.add(*p);
            }
            c.scale(1.0 / h.len() as f64)
        };
        if cent(&hb).sub(cent(&ha)).dot(dir) < 0.0 {
            dir = dir.scale(-1.0);
        }
        (depth, dir)
    }

    pub fn max_speed(&self) -> f64 {
        let mut m = 0.0;
        for i in 0..self.vel.len() {
            let s = self.vel[i].len();
            if s > m {
                m = s;
            }
        }
        m
    }

    /// Run to settled or `max_frames`. Returns frames taken.
    ///
    /// v0.2 sharpened Invariant 4: settled is exact AND stable — the
    /// predicate requires the strain threshold to hold over a sustained
    /// window (one full cage period), not merely on first touch. This is
    /// what guarantees every particle's jamming latch has matured, so a
    /// settled state cannot erupt when stepped again.
    pub fn settle(&mut self, max_frames: usize) -> usize {
        for f in 0..max_frames {
            self.step();
            // The pour phase is supra-yield by definition (§7.2): matter
            // still claiming space cannot be settled, however quiet the
            // stalled tail looks. Speed-based quiet alone is NOT the
            // predicate — it can trigger between fill close and latch
            // maturity, returning a state that still consolidates.
            if self.fill_enabled && !self.fill_done {
                continue;
            }
            let latched =
                self.rest_t.iter().all(|&t| t >= REST_FRAMES);
            if latched && self.max_speed() < self.eps_settle {
                return f + 1;
            }
        }
        max_frames
    }

    /// FNV-1a hash over the raw little-endian bits of every particle position.
    /// This is the R2 fingerprint of a settled state.
    pub fn state_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for p in &self.pos {
            for b in p.x.to_le_bytes().iter().chain(p.y.to_le_bytes().iter()) {
                h ^= *b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        h
    }

    pub fn n_particles(&self) -> usize {
        self.pos.len()
    }
}

/// Deterministic convex hull (monotone chain), CCW.
#[allow(dead_code)]
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
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], p)
                <= 0.0
        {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<Vec2> = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], p)
                <= 0.0
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

/// Shoelace area of a polygon.
#[allow(dead_code)]
fn poly_area(p: &[Vec2]) -> f64 {
    let n = p.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    for i in 0..n {
        let q = p[(i + 1) % n];
        a += p[i].x * q.y - q.x * p[i].y;
    }
    a.abs() / 2.0
}

/// Sutherland–Hodgman clip of `subject` against convex CCW `clip`.
#[allow(dead_code)]
fn clip_convex(subject: &[Vec2], clip: &[Vec2]) -> Vec<Vec2> {
    let mut out: Vec<Vec2> = subject.to_vec();
    let n = clip.len();
    for i in 0..n {
        if out.is_empty() {
            break;
        }
        let a = clip[i];
        let b = clip[(i + 1) % n];
        let inside = |p: Vec2| -> bool {
            (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x) >= 0.0
        };
        let input = std::mem::take(&mut out);
        let m = input.len();
        for j in 0..m {
            let cur = input[j];
            let prev = input[(j + m - 1) % m];
            let cur_in = inside(cur);
            let prev_in = inside(prev);
            if cur_in != prev_in {
                let d1 = Vec2::new(cur.x - prev.x, cur.y - prev.y);
                let d2 = Vec2::new(b.x - a.x, b.y - a.y);
                let denom = d1.x * d2.y - d1.y * d2.x;
                if denom.abs() > 1e-12 {
                    let t = ((a.x - prev.x) * d2.y
                        - (a.y - prev.y) * d2.x)
                        / denom;
                    out.push(Vec2::new(
                        prev.x + d1.x * t,
                        prev.y + d1.y * t,
                    ));
                }
            }
            if cur_in {
                out.push(cur);
            }
        }
    }
    out
}

// helper: shallow-copy a CKind (all Copy fields) so we can project without
// borrowing cs mutably and self mutably at once.
fn replace_kind(k: &CKind) -> CKind {
    match k {
        CKind::Dist { a, b, rest } => {
            CKind::Dist { a: *a, b: *b, rest: *rest }
        }
        CKind::Turgor { p, cx, cy, rest } => {
            CKind::Turgor { p: *p, cx: *cx, cy: *cy, rest: *rest }
        }
        CKind::Contact { a, b, rest } => {
            CKind::Contact { a: *a, b: *b, rest: *rest }
        }
        CKind::Wall { p, nx, ny, offset } => {
            CKind::Wall { p: *p, nx: *nx, ny: *ny, offset: *offset }
        }
        CKind::Area { hull, target } => {
            CKind::Area { hull: hull.clone(), target: *target }
        }
        CKind::Overlap { ra, rb } => {
            CKind::Overlap { ra: *ra, rb: *rb }
        }
    }
}

/// Convex hull of a region's particles returning GLOBAL indices, CCW
/// (monotone chain over (position, index), deterministic).
fn convex_hull_indices(pos: &[Vec2], first: usize, count: usize) -> Vec<usize> {
    let mut pts: Vec<(f64, f64, usize)> = (first..first + count)
        .map(|i| (pos[i].x, pos[i].y, i))
        .collect();
    pts.sort_by(|a, b| {
        a.0.partial_cmp(&b.0).unwrap().then(a.1.partial_cmp(&b.1).unwrap())
    });
    if pts.len() < 3 {
        return pts.into_iter().map(|p| p.2).collect();
    }
    let cross = |o: (f64, f64, usize), a: (f64, f64, usize), b: (f64, f64, usize)| {
        (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
    };
    let mut lower: Vec<(f64, f64, usize)> = Vec::new();
    for &p in &pts {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0
        {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper: Vec<(f64, f64, usize)> = Vec::new();
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
    lower.into_iter().map(|p| p.2).collect()
}
