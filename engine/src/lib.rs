//! NNUI_CELL engine — graduated from the R1–R4 prototype (TK09, dev-spec DS06).
//!
//! Four modules are byte-identical copies of the sources in `analysis/001`
//! and `analysis/002`; `solver.rs` additionally carries the **v0.2 arrest
//! mechanism backport** (yield deadband with position revert, structure
//! ratchet, jamming latch — paper v0.2 §7, normative), which the R1-era
//! prototype predates. The M1 harness forced it: without the latch a
//! settled state erupted when stepped, violating sharpened Invariant 4.
//! Reconstructed scaffolding (the packaged crate never reached the repo):
//! this file, `vec2.rs`, `Cargo.toml`, and the two R3-era extensions in
//! `compat.rs`. All of it is recorded honestly in the ledger.

pub mod vec2;
pub mod rheology;
pub mod solver;
pub mod genetics;
pub mod grammar;
pub mod api;
mod compat;
