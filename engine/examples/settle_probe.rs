//! Probe the 50-region settle trajectory under the v0.2 arrest backport:
//! max realized strain over time, quiet-stretch lengths, and latch counts.

use nnui_solver::solver::World;

fn main() {
    let mut w = World::new_packed(50, 5, 60.0, 52.0, 0xABCDEF);
    let mut quiet = 0usize;
    let mut best_quiet = 0usize;
    for f in 0..6000 {
        w.step();
        let m = w.max_speed();
        if m < w.eps_settle {
            quiet += 1;
            best_quiet = best_quiet.max(quiet);
        } else {
            quiet = 0;
        }
        if f % 250 == 249 || (f > 700 && f < 1200 && f % 50 == 49) {
            let resting = w.rest_t.iter().filter(|&&t| t >= 24).count();
            println!(
                "f={:5}  max={:9.4}  quiet={:4} best={:4}  resting={}/{}",
                f + 1,
                m,
                quiet,
                best_quiet,
                resting,
                w.regions.len()
            );
        }
    }
}
