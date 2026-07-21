//! Probe the E_fill phase: occupancy, strain, resting count, and per-cell
//! extents over a full settle of the dashboard layout in a 16:10 vessel.

use nnui_solver::api::{Engine, Extent, Lod};

fn main() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(include_str!("../layouts/dashboard.nnui")).unwrap();
    for f in 0..1800 {
        let r = e.tick();
        if f % 120 == 119 || (r.settled && f % 30 == 0) {
            let s = e.snapshot(Lod::Extents);
            let areas: Vec<String> = s
                .cells
                .iter()
                .map(|c| {
                    format!("{}={:.0}", c.name, c.extent.w * c.extent.h)
                })
                .collect();
            println!(
                "f={f:5} strain={:8.3} settled={} areas[{}]",
                r.max_strain,
                r.settled,
                areas.join(" ")
            );
            if r.settled {
                break;
            }
        }
    }
    let s = e.snapshot(Lod::Extents);
    let total: f64 =
        s.cells.iter().map(|c| c.extent.w * c.extent.h).sum();
    println!(
        "world {:.0}x{:.0}  claimed={:.0}%  settled={}",
        s.world.w,
        s.world.h,
        100.0 * total / (s.world.w * s.world.h),
        s.settled
    );
}
