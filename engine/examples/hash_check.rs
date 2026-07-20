//! Lineage check: does the reconstructed crate reproduce the R1/R2 report's
//! recorded settled-state hash (0xdb5295e7d7bce689) for the same
//! configuration? Matching proves the reconstruction is faithful to the
//! validated prototype; a mismatch would mean determinism holds within this
//! crate but the historical lineage was broken by reconstruction.

use nnui_solver::solver::World;

fn main() {
    let mut w = World::new_packed(50, 5, 60.0, 52.0, 0xABCDEF);
    let frames = w.settle(4000);
    println!("frames  : {frames}");
    println!("hash    : {:#018x}", w.state_hash());
    println!("recorded: 0xdb5295e7d7bce689");
}
