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

    /// Build a LAYOUT world from a parsed program in a vessel-shaped
    /// container: E_fill enabled (the medium claims the whole vessel;
    /// sizes solved from S), deliberate binds lowered to E_cohesion, and
    /// jittered-grid spawn (C4). World height is fixed at 60 medium
    /// units; width follows the vessel's aspect so the settled screen IS
    /// the vessel, edge to edge.
    pub fn from_layout(
        layout: &crate::grammar::Layout,
        vessel_w: f64,
        vessel_h: f64,
        seed: u64,
    ) -> Self {
        let mats = layout.to_materials();
        let h = 60.0;
        let aspect = (vessel_w / vessel_h).clamp(0.2, 5.0);
        let w = h * aspect;
        let mut world = World::from_materials(&mats, 5, w, h, seed);
        world.fill_enabled = true;
        world.set_area_targets();
        let idx = |id: &str| layout.cells.iter().position(|c| c.id == id);
        for op in &layout.ops {
            if let crate::grammar::Op::Bind(a, b) = op {
                if let (Some(ra), Some(rb)) = (idx(a), idx(b)) {
                    world.binds.push((ra, rb));
                }
            }
        }
        world
    }
}
