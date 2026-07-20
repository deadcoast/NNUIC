//! Reproduce the demo-exposed chat flicker: settle the chat layout, then
//! observe 600 ticks. Any nonzero strain after settle is an eruption.

use nnui_solver::api::{Engine, Extent};

fn main() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(include_str!("../layouts/m3_chat.nnui")).unwrap();
    e.settle();
    let mut eruptions = 0;
    let mut max = 0.0f64;
    for t in 0..600 {
        let r = e.tick();
        if r.max_strain > 0.0 {
            eruptions += 1;
            if r.max_strain > max {
                max = r.max_strain;
            }
            if eruptions <= 5 {
                println!("t={t}: strain={:.4} settled={}", r.max_strain,
                    r.settled);
            }
        }
    }
    println!("eruption frames: {eruptions}/600, peak strain {max:.4}");
}
