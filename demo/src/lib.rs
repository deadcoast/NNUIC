//! NNUI_CELL integration demo — the real engine compiled to WASM, driven
//! exclusively through the public API (ledger REQ13). The renderer on the
//! JS side is a stateless view of `Snapshot` data (Invariant 6). Assets
//! are real bytes digested through the R4 registry, so every cell's
//! phenotype (chart / player / reader / image-viewer / germline panel) is
//! genuinely expressed, not painted on. The vessel can be reshaped at
//! runtime: the medium re-settles with NO further program call — the §9
//! zero-cost-adaptation claim, live.

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
    static GEOM: RefCell<Vec<f64>> = RefCell::new(vec![0.0; GEOM_CAP]);
}

fn phenotype_code(kind: &str) -> f64 {
    match kind {
        "chart" => 1.0,
        "player" => 2.0,
        "reader" => 3.0,
        "image-viewer" => 4.0,
        _ => 0.0, // germline panel
    }
}

/// Register real byte samples for every asset the reference programs
/// hold, so cells digest genuine content through the R4 registry.
fn register_assets(e: &mut Engine) {
    let csv: &[u8] = b"month,value\njan,120\nfeb,140\nmar,160\n";
    for name in ["revenue", "kpi1", "kpi2", "kpi3"] {
        e.register_asset(name, csv.to_vec());
    }
    e.register_asset("film", b"\x00\x00\x00\x18ftypisom\0\0\0\0".to_vec());
    let png: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR";
    for name in ["c1", "c2", "c3", "c4"] {
        e.register_asset(name, png.to_vec());
    }
    let txt = |s: &str| s.as_bytes().to_vec();
    e.register_asset("title", txt("Board"));
    e.register_asset("backlog", txt("todo items live here"));
    e.register_asset("active", txt("in progress work"));
    e.register_asset("finished", txt("done and shipped"));
    e.register_asset("convos", txt("conversation list"));
    e.register_asset("transcript", txt("the running dialogue"));
    e.register_asset("draft", txt("compose a message"));
}

/// Load an embedded agent program. Returns the cell count (0 = failure).
#[no_mangle]
pub extern "C" fn demo_load(layout: u32, vessel_w: f64, vessel_h: f64) -> u32 {
    let src = LAYOUTS[layout as usize % LAYOUTS.len()];
    let mut e = Engine::new(Extent::viewport(vessel_w, vessel_h));
    register_assets(&mut e);
    match e.submit(src) {
        Ok(_) => {
            let n = e.snapshot(Lod::Extents).cells.len() as u32;
            ENGINE.with(|slot| *slot.borrow_mut() = Some(e));
            n
        }
        Err(_) => 0,
    }
}

/// Reshape the vessel (§9): the medium re-settles, no program call.
#[no_mangle]
pub extern "C" fn demo_resize(vessel_w: f64, vessel_h: f64) {
    ENGINE.with(|slot| {
        if let Some(e) = slot.borrow_mut().as_mut() {
            e.resize_vessel(Extent::viewport(vessel_w, vessel_h));
        }
    });
}

/// One solver frame; returns realized post-arrest max strain.
#[no_mangle]
pub extern "C" fn demo_tick() -> f64 {
    ENGINE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let e = slot.as_mut().expect("demo_load first");
        e.tick().max_strain
    })
}

/// 1 when the sustained settled predicate (the latch) holds.
#[no_mangle]
pub extern "C" fn demo_settled() -> u32 {
    ENGINE.with(|slot| {
        let slot = slot.borrow();
        let e = slot.as_ref().expect("demo_load first");
        e.snapshot(Lod::Extents).settled as u32
    })
}

/// Serialize the Snapshot into the geometry buffer:
/// `[scale, ox, oy, world_w, world_h, n_cells,` then per cell
/// `[n_pts, cx, cy, settled, strain, phenotype_code, x0, y0, ...]]`.
/// World-space values; JS applies the vessel map. Returns f64 count.
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
            push(s.world.w, &mut i);
            push(s.world.h, &mut i);
            push(s.cells.len() as f64, &mut i);
            for c in &s.cells {
                push(c.boundary.len() as f64, &mut i);
                push(c.centroid.x, &mut i);
                push(c.centroid.y, &mut i);
                push(c.settled as u32 as f64, &mut i);
                push(c.strain, &mut i);
                push(phenotype_code(&c.phenotype.kind), &mut i);
                // hull vertices are particle CENTERS; matter extends
                // one material radius beyond — inflate so the drawn cell
                // is the true material extent (seams stay seams)
                for p in &c.boundary {
                    let dx = p.x - c.centroid.x;
                    let dy = p.y - c.centroid.y;
                    let d = (dx * dx + dy * dy).sqrt().max(1e-9);
                    let f = (d + c.radius) / d;
                    push(c.centroid.x + dx * f, &mut i);
                    push(c.centroid.y + dy * f, &mut i);
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
