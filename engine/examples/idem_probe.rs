//! Replicates m1_hashes_reproduce_and_settle_is_idempotent to find where
//! the D2 constraint formulation breaks settle idempotence.

use nnui_solver::api::{Engine, Extent, Lod};

fn main() {
    let src = include_str!("../layouts/dashboard.nnui");
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(src).unwrap();
    let h1 = e.settle();
    let s1 = e.snapshot(Lod::Extents);
    println!("settle#1: hash={:?} settled={}", h1, s1.settled);
    let w = e.world_ref();
    println!(
        "  fill_enabled={} fill_done={} rest_t={:?}",
        w.fill_enabled,
        w.fill_done,
        w.rest_t
    );
    let h2 = e.settle();
    let s2 = e.snapshot(Lod::Extents);
    println!("settle#2: hash={:?} settled={}", h2, s2.settled);
    let w = e.world_ref();
    println!(
        "  fill_done={} rest_t={:?} idempotent={}",
        w.fill_done,
        w.rest_t,
        h1 == h2
    );
    // strain over 10 observation ticks
    for f in 0..10 {
        let r = e.tick();
        println!("tick {f}: max_strain={:.5} settled={}", r.max_strain, r.settled);
    }
}
