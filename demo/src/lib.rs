//! NNUI_CELL integration demo — the real engine compiled to WASM, driven
//! exclusively through the public API (ledger REQ13). The renderer on the
//! JS side is a stateless view of `Snapshot` data (Invariant 6): this
//! module exports layout submission, ticking, and a flattened geometry
//! buffer containing only what `Snapshot` exposes — centroid, hull,
//! settled flag, strain, and the vessel map. No solver internals cross
//! the boundary.
//!
//! The four programs are the R3 dashboard plus the three M3 LLM-emitted
//! layouts — the demo literally replays agent-authored programs.

use nnui_solver::api::{Engine, Extent, Lod};
use std::cell::RefCell;

const LAYOUTS: [&str; 4] = [
    include_str!("../../engine/layouts/dashboard.nnui"),
    include_str!("../../engine/layouts/m3_kanban.nnui"),
    include_str!("../../engine/layouts/m3_gallery.nnui"),
    include_str!("../../engine/layouts/m3_chat.nnui"),
];

const GEOM_CAP: usize = 16384;

thread_local! {
    static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
    // fixed-capacity geometry buffer: allocated once, pointer stable
    static GEOM: RefCell<Vec<f64>> =
        RefCell::new(vec![0.0; GEOM_CAP]);
}

/// Load one of the embedded agent programs. Returns the cell count, or 0
/// on failure (which the harness-tested API makes impossible for these
/// programs).
#[no_mangle]
pub extern "C" fn demo_load(layout: u32, vessel_w: f64, vessel_h: f64) -> u32 {
    let src = LAYOUTS[layout as usize % LAYOUTS.len()];
    let mut e = Engine::new(Extent::viewport(vessel_w, vessel_h));
    match e.submit(src) {
        Ok(_) => {
            let n = e.snapshot(Lod::Extents).cells.len() as u32;
            ENGINE.with(|slot| *slot.borrow_mut() = Some(e));
            n
        }
        Err(_) => 0,
    }
}

/// One solver frame through the public API. Returns the realized
/// post-arrest max strain (the honest instrument).
#[no_mangle]
pub extern "C" fn demo_tick() -> f64 {
    ENGINE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let e = slot.as_mut().expect("demo_load first");
        e.tick().max_strain
    })
}

/// 1 when the engine's sustained settled predicate holds (the latch, not
/// first-touch quiet).
#[no_mangle]
pub extern "C" fn demo_settled() -> u32 {
    ENGINE.with(|slot| {
        let slot = slot.borrow();
        let e = slot.as_ref().expect("demo_load first");
        e.snapshot(Lod::Extents).settled as u32
    })
}

/// Serialize the current Snapshot into the geometry buffer. Layout:
/// `[scale, ox, oy, n_cells,` then per cell
/// `[n_pts, cx, cy, settled, strain, x0, y0, x1, y1, ...]]`
/// (all coordinates world-space; JS applies the vessel map).
/// Returns the number of f64 values written.
#[no_mangle]
pub extern "C" fn demo_refresh_geom() -> u32 {
    ENGINE.with(|slot| {
        let slot = slot.borrow();
        let e = slot.as_ref().expect("demo_load first");
        let s = e.snapshot(Lod::Hull);
        GEOM.with(|g| {
            let mut g = g.borrow_mut();
            let mut i = 0usize;
            let mut push = |v: f64, i: &mut usize| {
                if *i < GEOM_CAP {
                    g[*i] = v;
                    *i += 1;
                }
            };
            push(s.map.scale, &mut i);
            push(s.map.ox, &mut i);
            push(s.map.oy, &mut i);
            push(s.cells.len() as f64, &mut i);
            for c in &s.cells {
                push(c.boundary.len() as f64, &mut i);
                push(c.centroid.x, &mut i);
                push(c.centroid.y, &mut i);
                push(c.settled as u32 as f64, &mut i);
                push(c.strain, &mut i);
                for p in &c.boundary {
                    push(p.x, &mut i);
                    push(p.y, &mut i);
                }
            }
            i as u32
        })
    })
}

/// Stable pointer to the geometry buffer.
#[no_mangle]
pub extern "C" fn demo_geom_ptr() -> *const f64 {
    GEOM.with(|g| g.borrow().as_ptr())
}
