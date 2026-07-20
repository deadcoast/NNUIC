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
        // contacts: non-penetration is stiff for every material. Two regions do
        // not interpenetrate regardless of how soft each is; material softness
        // lives in cohesion/turgor (how a region deforms), not in whether
        // regions can occupy the same space.
        for (a, b, rest) in self.build_contacts() {
            cs.push(Constraint { kind: CKind::Contact { a, b, rest }, alpha: self.alpha_contact, lambda: 0.0 });
        }
        // walls
        for p in 0..self.pos.len() {
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
        match c.kind {
            CKind::Dist { a, b, rest } => {
                let d = self.pos[a].sub(self.pos[b]);
                let len = d.len();
                let cval = len - rest;
                let n = d.norm_or(Vec2::new(1.0, 0.0));
                let wsum = self.inv_mass[a] + self.inv_mass[b];
                let denom = wsum + alpha_tilde;
                if denom == 0.0 {
                    return;
                }
                let dl = (-cval - alpha_tilde * c.lambda) / denom;
                c.lambda += dl;
                self.pos[a] = self.pos[a].add(n.scale(self.inv_mass[a] * dl));
                self.pos[b] = self.pos[b].add(n.scale(-self.inv_mass[b] * dl));
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
                let wsum = self.inv_mass[a] + self.inv_mass[b];
                let denom = wsum + alpha_tilde;
                if denom == 0.0 {
                    return;
                }
                let dl = (-cval - alpha_tilde * c.lambda) / denom;
                c.lambda += dl;
                self.pos[a] = self.pos[a].add(n.scale(self.inv_mass[a] * dl));
                self.pos[b] = self.pos[b].add(n.scale(-self.inv_mass[b] * dl));
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
    pub fn step(&mut self) {
        let dt = self.dt;
        let dt2 = dt * dt;

        // strain rate from current velocities; update thixotropic structure
        let sr = self.region_strain_rate();
        for r in 0..self.regions.len() {
            let next = rheology::step_structure(&self.regions[r].mat, sr[r], dt);
            self.regions[r].mat.xi = next;
        }

        // predict (no gravity; layout forces only). Overdamped: only a small
        // fraction `damping` of last frame's motion is carried as momentum, so
        // the medium relaxes (gradient flow) instead of sloshing.
        for i in 0..self.pos.len() {
            self.prev[i] = self.pos[i];
            if !self.no_carry[i] {
                self.pos[i] =
                    self.pos[i].add(self.vel[i].scale(self.damping * dt));
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
            if rest - d > CONTACT_TOL {
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
            let quiet = |k: usize, v: f64| -> bool {
                (arrestable[k] && v < vy) || self.rattle[k]
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

    /// Max particle speed — the strain-rate proxy for the settled predicate.
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
        let mut quiet = 0usize;
        for f in 0..max_frames {
            self.step();
            if self.max_speed() < self.eps_settle {
                quiet += 1;
                if quiet > REST_FRAMES as usize {
                    return f + 1;
                }
            } else {
                quiet = 0;
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

// helper: shallow-copy a CKind (all Copy fields) so we can project without
// borrowing cs mutably and self mutably at once.
fn replace_kind(k: &CKind) -> CKind {
    match *k {
        CKind::Dist { a, b, rest } => CKind::Dist { a, b, rest },
        CKind::Turgor { p, cx, cy, rest } => CKind::Turgor { p, cx, cy, rest },
        CKind::Contact { a, b, rest } => CKind::Contact { a, b, rest },
        CKind::Wall { p, nx, ny, offset } => CKind::Wall { p, nx, ny, offset },
    }
}
