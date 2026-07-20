//! Diagnose the settle-idempotency failure: does the dashboard layout
//! actually reach the settled predicate within budget, and what does one
//! extra step do at "rest"?

use nnui_solver::api::SEED;
use nnui_solver::grammar;

fn main() {
    let src = include_str!("../layouts/dashboard.nnui");
    let l = grammar::parse(src).unwrap();
    let mut w = l.to_world(SEED);
    let frames = w.settle(1500);
    println!("frames={frames} (budget 1500) max_speed={:.5} eps={}",
        w.max_speed(), w.eps_settle);
    for (r, reg) in w.regions.iter().enumerate() {
        println!("region {r}: xi={:.4} vy={:.4}", reg.mat.xi,
            w.yield_vel * reg.mat.xi);
    }
    let h1 = w.state_hash();
    w.step();
    println!("after 1 extra step: max_speed={:.5} hash_same={}",
        w.max_speed(), w.state_hash() == h1);
    let mut moved = 0;
    for i in 0..w.pos.len() {
        if w.pos[i].sub(w.prev[i]).len() > 0.0 {
            moved += 1;
        }
    }
    println!("particles with nonzero realized displacement: {moved}/{}",
        w.pos.len());
}
