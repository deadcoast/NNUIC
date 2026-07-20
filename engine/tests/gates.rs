//! Stage 4 merge gates (ledger TK14/TK15): the checks every merge must
//! pass beyond the invariant suite — the real-time budget at reference
//! scale and determinism at the API boundary (the latter also covered by
//! the M1/M4 harnesses; asserted here so the gate is self-contained).

use nnui_solver::api::{Engine, Extent};
use nnui_solver::solver::World;
use std::time::Instant;

const DASH: &str = include_str!("../layouts/dashboard.nnui");

/// R1 budget held at the reference scale: 50 regions inside 16 ms/frame
/// during active settling (steady-state cost; the settled state is
/// cheaper still because latched regions drop their constraints).
#[test]
fn gate_frame_budget_50_regions() {
    let mut w = World::new_packed(50, 5, 60.0, 52.0, 0xABCDEF);
    // warm-up: get past the initial transient
    for _ in 0..30 {
        w.step();
    }
    let frames = 100;
    let t0 = Instant::now();
    for _ in 0..frames {
        w.step();
    }
    let ms_per_frame = t0.elapsed().as_secs_f64() * 1000.0 / frames as f64;
    assert!(
        ms_per_frame < 16.0,
        "frame budget violated: {ms_per_frame:.2} ms/frame (budget 16)"
    );
}

/// Determinism at the API boundary: full submit→settle runs reproduce the
/// hash bit-identically.
#[test]
fn gate_api_boundary_determinism() {
    let hash = |src: &str| {
        let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
        e.submit(src).unwrap();
        e.settle().0
    };
    let h = hash(DASH);
    for _ in 0..5 {
        assert_eq!(hash(DASH), h, "API-boundary hash diverged");
    }
}
