//! Watches the dashboard layout world frame by frame under the D2
//! constraint formulation: when does fill close, what does max_speed do
//! afterwards, and do the regions ever latch?

use nnui_solver::api::SEED;
use nnui_solver::grammar;
use nnui_solver::solver::{World, REST_FRAMES};

fn main() {
    let src = include_str!("../layouts/dashboard.nnui");
    let layout = grammar::parse(src).unwrap();
    let mut w = World::from_layout(&layout, 1280.0, 800.0, SEED);
    let frames = w.settle(30000);
    println!(
        "vessel area = {:.0}; area targets = {:?} (sum {:.0}); occupancy = {:.3}; max_speed at end = {:.3}",
        w.w * w.h,
        w.area_target,
        w.area_target.iter().sum::<f64>(),
        w.occupancy(),
        w.max_speed()
    );
    let latched =
        w.rest_t.iter().filter(|&&t| t >= REST_FRAMES).count();
    println!("settled in {frames} frames, latched={latched}/4");
    // pairwise SAT depth (min over x, y, centroid axis, its perp) at rest
    let n = w.regions.len();
    let centroid = |r: usize| {
        let reg = &w.regions[r];
        let mut c = nnui_solver::vec2::Vec2::zero();
        for k in reg.first..reg.first + reg.count {
            c = c.add(w.pos[k]);
        }
        c.scale(1.0 / reg.count as f64)
    };
    for a in 0..n {
        for b in (a + 1)..n {
            let u = centroid(b).sub(centroid(a));
            let ul = u.len();
            let u = if ul > 1e-9 {
                u.scale(1.0 / ul)
            } else {
                nnui_solver::vec2::Vec2::new(1.0, 0.0)
            };
            let axes = [
                nnui_solver::vec2::Vec2::new(1.0, 0.0),
                nnui_solver::vec2::Vec2::new(0.0, 1.0),
                u,
                nnui_solver::vec2::Vec2::new(-u.y, u.x),
            ];
            let mut depth = f64::INFINITY;
            for ax in axes {
                let proj = |r: usize| {
                    let reg = &w.regions[r];
                    let (mut lo, mut hi) =
                        (f64::INFINITY, f64::NEG_INFINITY);
                    for k in reg.first..reg.first + reg.count {
                        let t = w.pos[k].dot(ax);
                        lo = lo.min(t);
                        hi = hi.max(t);
                    }
                    (lo, hi)
                };
                let (alo, ahi) = proj(a);
                let (blo, bhi) = proj(b);
                depth = depth.min(ahi.min(bhi) - alo.max(blo));
            }
            println!("pair ({a},{b}): SAT depth = {depth:8.3}");
        }
    }
    println!("spacing ~ region0 target_radius = {}", w.regions[0].target_radius);
}
