# NNUI_CELL — R1+R2 Prototype

Headless Rust solver proving two claims from the Proof of Concept
(`NNUI_CELL_PoC_v0.1`, §9):

- **R1** — the XPBD continuum-region solver settles **50 regions in < 16 ms/frame**.
- **R2** — the settled state is **bit-identical across 100 runs**.

It also demonstrates three of the paper's seven invariants directly as passing
tests, and exercises the full constitutive law (§4–§5) rather than a stand-in.

This is a *proof of concept*: it exists to accept or refute the approach, not to
be a production engine. It has **no UI and no dependencies** — pure `std`, by
design, so R2 determinism is not at the mercy of any external crate.

---

## Results (reproduced from `R1R2_REPORT.txt`)

```out
config: 50 regions x 5x5 = 1250 particles, container 60x52

R1  steady-state cost: 1.36 ms/frame   (budget 16.0)      => PASS
R1  settles in 801 frames (max_speed 0 < eps 2.0e-2)

R2  100 independent runs, settled-state FNV-1a hash:
R2  hash = 0xdb5295e7d7bce689
R2  all 100 identical                                     => PASS

scaling probe (steady-state ms/frame, density held constant):
     50 regions ( 1250 particles):  1.44 ms/frame
    100 regions ( 2500 particles):  2.93 ms/frame
    200 regions ( 5000 particles):  5.78 ms/frame
    400 regions (10000 particles): 11.83 ms/frame
```

R1 passes with ~13× headroom. The scaling probe shows even 400 regions
(10,000 particles) stays inside the real-time budget. The same settled-state
hash appears across separate process invocations, so determinism holds
run-to-run **and** process-to-process.

`settled.svg` / `settled.png` show a settled arrangement: 50 regions tiled
across the container (min inter-region centroid distance ≈ 4.9 for ~4-unit
blobs — properly spaced, not collapsed), with the near-locked region drawn in
red.

## Build & run

```bash
cargo run   --release --bin r1r2       # the R1+R2 report
cargo run   --release --bin snapshot > settled.svg   # settled-state SVG
cargo test  --release                  # invariants (below), ~20s
```

## Invariants as tests (`tests/invariants.rs`)

All pass. These convert anti-drift intent (paper §8) into automated checks:

| Test                            | Invariant | What it asserts                                                                                          |
|---------------------------------|-----------|----------------------------------------------------------------------------------------------------------|
| `i2_constitutive_law_is_real`   | I2        | flow index `n` changes apparent viscosity; higher solidity `S` monotonically lowers compliance (stiffer) |
| `i3_locked_region_holds_still`  | I3        | a near-locked (`S≈1`) isolated region does not drift — locking is the `S→1` limit, not a flag            |
| `i4_settling_reaches_threshold` | I4        | settling terminates by the strain-rate threshold, and actually reaches it                                |
| `r2_bit_identical_determinism`  | I7 / R2   | 20 independent settles produce identical state hashes                                                    |
| `confinement_holds`             | —         | every particle stays inside the container through settling                                               |

Plus four `rheology` unit tests: yield diverges as `S→1`, compliance → 0 when
locked, `n` changes behaviour, structure `ξ` rebuilds at rest.

## Model as implemented (map to the paper)

- `src/rheology.rs` — the constitutive law. Solidity-modulated Herschel–Bulkley
  yield `τ_y(S)=τ_y0·(S/(1-S))^κ` (§4.1); dual-rate effective yield with impact
  hardening (§4.2); Papanastasiou-regularized apparent viscosity `η_app` (§4.1);
  the viscosity→compliance map `α = α_base/(1+η)` (§5.2); thixotropic structure
  kinetics `dξ/dt` (§4.3).
- `src/solver.rs` — the XPBD world (§5). Each region is a 5×5 particle blob held
  by internal cohesion + turgor constraints; regions interact by stiff
  non-penetration contacts and are confined by container walls. Compliance per
  constraint comes from the region's material state each frame. Settling driver
  is overdamped relaxation from a jittered-grid start; termination is the
  strain-rate predicate (§4.4, eq. 9); the long-tail creep of frictionless
  packing is arrested by the thixotropic yield mechanism (§4.3/§4.5).
- Determinism (§5.4): fixed `dt`, fixed constraint order, index-order reductions,
  seeded `splitmix64` init, single thread, contacts sorted before use.

## What implementation taught us (corrections to fold back into the model)

The paper was internally coherent, but building it surfaced four things the
math alone did not. These are genuine refinements, recorded so the next revision
of the spec carries them.

1. **Non-penetration must be stiff for every material.** Scaling contact
   compliance by softness (the first, "obvious" reading) jams the system: soft
   regions interpenetrate and never separate. Material softness belongs in
   *internal deformation* (cohesion/turgor), not in whether two regions may
   occupy the same space. Contacts are now stiff regardless of `S`.
2. **"Overdamped" means little carried momentum, not heavy damping.** Retaining
   most of the velocity each frame is *under*-damped and sloshes. Overdamped
   gradient flow carries only a small momentum fraction (`damping ≈ 0.2`).
3. **The thixotropic `ξ` mechanism is load-bearing, not decorative.**
   Frictionless frustrated packings creep indefinitely; every production physics
   engine solves this with a sleep system. The paper already contains the
   physically-correct one — structure rebuilds at rest, raising yield until
   sub-yield creep is arrested. Wiring `ξ` into actual motion-arrest is what
   makes the medium settle cleanly. The metaphor turned out to be the mechanism.
4. **Initial conditions matter enormously.** A jammed clump is pathological;
   a jittered grid makes settling a well-posed relaxation from near a solution.

## Honest caveats

- **Cross-machine determinism is not tested.** Only within-machine, 100-run (and
  process-to-process) bit-identity is shown. The harder cross-architecture clause
  of R2 may require fixed-point arithmetic; the paper flags this (§11).
- **Settle duration is ~801 frames** (~13 s of relaxation wall-time). This is a
  one-time background settle; the figure that governs real-time feel is the
  *per-frame* cost (1.36 ms), which is the passing number. Settle duration was
  not an exit criterion.
- **One particle blob per region at 5×5 resolution.** Higher-resolution regions
  and true content inclusions (rigid text/image bodies, §4.3 as shape-matching)
  are the natural next fidelity step, deferred by design.
- **Parameters are tuned, not derived.** Container density, solve iterations,
  and the yield-arrest scale were tuned to a clean operating point; a principled
  calibration is future work (the paper flagged realism-vs-perf as open).

## File manifest

```text
Cargo.toml            crate manifest (release profile tuned for determinism)
src/vec2.rs           deterministic 2D f64 vector math
src/rheology.rs       constitutive law + viscosity→compliance + thixotropy
src/solver.rs         XPBD world, constraints, step(), settle(), state hash
src/lib.rs            module wiring
src/bin/r1r2.rs       the R1+R2 report
src/bin/snapshot.rs   settled-state SVG emitter
tests/invariants.rs   invariants + R2 as automated tests
R1R2_REPORT.txt       captured report output
settled.svg / .png    settled-state visualization
```

MIVIM Development — *Standards in Novel Design*. R1+R2 prototype, July 2026.
