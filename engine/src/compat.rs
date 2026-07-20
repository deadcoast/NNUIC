//! R3-era extensions the committed R1-era sources predate. The validated
//! files `rheology.rs` and `solver.rs` are byte-identical to `analysis/001`;
//! `grammar.rs` (from `analysis/002`) calls `Material::from_dials` and
//! `World::from_materials`, which lived in the un-committed packaged crate.
//! They are reconstructed here as separate impl blocks so the validated
//! sources stay untouched.

use crate::rheology::Material;
use crate::solver::World;

impl Material {
    /// Build a material from the grammar's dials, with the prototype's fixed
    /// thixotropic constants (rebuild timescale and breakdown coefficient
    /// match `Material::fluidish`).
    pub fn from_dials(s: f64, n: f64, xi: f64) -> Self {
        Material { s, n, xi, t_r: 0.25, a: 6.0 }
    }
}

impl World {
    /// Build a world with one region per supplied material, using the same
    /// deterministic jittered-grid placement as `new_packed`, then applying
    /// the given materials. Placement consumes the seed identically, so the
    /// layout is reproducible for the same inputs.
    pub fn from_materials(
        mats: &[Material],
        side: usize,
        w: f64,
        h: f64,
        seed: u64,
    ) -> Self {
        let mut world = World::new_packed(mats.len().max(1), side, w, h, seed);
        for (r, m) in mats.iter().enumerate() {
            world.regions[r].mat = *m;
        }
        world
    }
}
