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

```
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

## Workstream B — visual demo (`demo/nnui_cell_bench.html`)

A self-contained interactive bench (open the file in any browser — no server, no
dependencies) that makes the ooze/settle visible to the eye. The in-browser
physics is a faithful transcription of the validated Rust solver — same
constitutive constants, same viscosity→compliance bridge, same thixotropic yield
arrest — so it is a *view of solver state*, not an animation (Invariant 6: no
tweens, no timelines). The Rust crate remains the reference of record; this is
workstream B, a demo, explicitly not the end goal.

What it shows, all solver-driven:
- **Scatter & settle** — regions ooze from a jittered start and settle; the
  strain-rate meter drains to zero and `STATE` flips `OOZING → SETTLED`, which is
  literally the settle predicate crossing threshold.
- **Solidity dial / Lock** — one slider spans liquid → locked; at S→1 the medium
  vitrifies (shifts amber) and stops yielding. Locking is the top of the dial.
- **Shake** — displaces the settled medium and lets you watch it heal back.
- **Stir** — drag in the tank; sustained force makes the medium yield and flow.
- **Inject cell** — a new cell drops in and settles among the others.

The physics was verified headless before shipping (Node): scatter settles in
~137 frames, shake heals in ~127, a locked medium moves &lt;0.1 units under a hard
kick, and no state goes NaN. Honest note surfaced during the build:
impact-hardening (fast input stiffening) is in the law but is a high-strain-rate
regime, so at hand speed the medium shear-thins and flows; the demo says so
rather than overclaiming a flashy flick.

## R3 — agent grammar & token economy (`R3_REPORT.txt`)

The grammar is parsed and **lowered into the R1 solver**, so "settles
unambiguously" is shown end-to-end (parse → build World → settle → hash), not
asserted. Token cost is measured with a real LLM tokenizer.

```
settle (parse -> World -> settle -> hash, x2 must match):
  dashboard  4 cells  settled@255  hash 0x2cc3b2c865f4e8ea  reproducible PASS
  article    3 cells  settled@162  hash 0x2e48ae36d2999efa  reproducible PASS
  settings   3 cells  settled@97   hash 0xd9a28d59c004b275  reproducible PASS

tokens (tiktoken cl100k_base / GPT-4; markup = minimal functional HTML+CSS):
  layout      NNUI  markup  ratio   <80?
  dashboard     68     152  2.24x   yes
  article       47     126  2.68x   yes
  settings      52     127  2.44x   yes
  TOTAL        167     405  2.43x
```

Both exit conditions pass: each layout is under 80 NNUI tokens, and the grammar
is **2.43× cheaper** than the equivalent hand-authored markup. The comparison is
deliberately fair — the markup baseline is compact, idiomatic HTML+CSS that
produces the same layout intent (see `r3_layouts/*.html`), and both sides rely
on their own defaults (NNUI's unstated dials; the browser's default styles).
The measured generation saving is the *floor* of the paper's claim; the larger
argument — that a change to content or viewport triggers **zero** re-authoring
because geometry is solved, not authored — is structural and not counted here.

## R4 — expression registry (`R4_REPORT.txt`)

Three data types sequence to distinct phenotypes; a fourth is added through the
public registry with the core untouched.

```
CSV -> traits[numeric,tabular]   -> phenotype 'chart'        (axis/aggregate/sort/filter)
MP4 -> traits[temporal,video]    -> phenotype 'player'       (play/scrub/volume/fullscreen)
TXT -> traits[prose,text]        -> phenotype 'reader'       (scroll/font-size/find)
PNG -> (unknown) germline 'panel'  ... register_type(...) ...  -> phenotype 'image-viewer'
```

`R4a` distinct phenotypes, `R4b` 4th unknown before registration (germline),
`R4c` 4th expresses after a single `register_type` call, `R4d` the three core
types still work unchanged — all PASS. The design point that makes "no core
change" *real*: both detection and expression are registry-driven, so adding a
type is two pushes onto those lists via a public method; no function in the core
`sequence`/`express` path is edited. Determinism holds — same bytes, same
phenotype, every time.

## Build & run

```bash
cargo run   --release --bin r1r2       # the R1+R2 report
cargo run   --release --bin r3         # R3 grammar: parse + settle each layout
python3     r3_tokens.py               # R3 token measurement (needs tiktoken)
cargo run   --release --bin r4         # R4 expression registry
cargo run   --release --bin snapshot > settled.svg   # settled-state SVG
cargo test  --release                  # all 15 tests (~25s)
```

Open `demo/nnui_cell_bench.html` in a browser for the visual bench (workstream B).

## Invariants as tests (`tests/invariants.rs`)

All pass. These convert anti-drift intent (paper §8) into automated checks:

| Test | Invariant | What it asserts |
|------|-----------|-----------------|
| `i2_constitutive_law_is_real` | I2 | flow index `n` changes apparent viscosity; higher solidity `S` monotonically lowers compliance (stiffer) |
| `i3_locked_region_holds_still` | I3 | a near-locked (`S≈1`) isolated region does not drift — locking is the `S→1` limit, not a flag |
| `i4_settling_reaches_threshold` | I4 | settling terminates by the strain-rate threshold, and actually reaches it |
| `r2_bit_identical_determinism` | I7 / R2 | 20 independent settles produce identical state hashes |
| `confinement_holds` | — | every particle stays inside the container through settling |

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

```
Cargo.toml            crate manifest (release profile tuned for determinism)
src/vec2.rs           deterministic 2D f64 vector math
src/rheology.rs       constitutive law + viscosity→compliance + thixotropy
src/solver.rs         XPBD world, constraints, step(), settle(), from_materials()
src/genetics.rs       R4: sequencing, registry, expression (registry-driven)
src/grammar.rs        R3: lexer, parser, AST, lowering to a settleable World
src/lib.rs            module wiring
src/bin/r1r2.rs       R1+R2 report
src/bin/r3.rs         R3 settle report (grammar → solver)
src/bin/r4.rs         R4 expression-registry report
src/bin/snapshot.rs   settled-state SVG emitter
tests/invariants.rs   R1/R2 invariants as tests (genetics/grammar tests are inline)
r3_layouts/*.nnui     three agent-grammar layouts
r3_layouts/*.html     fair minimal-markup equivalents for token comparison
r3_tokens.py          token measurement (tiktoken cl100k_base)
demo/nnui_cell_bench.html  workstream B: interactive visual bench (self-contained)
R1R2_REPORT.txt       R1+R2 captured output
R3_REPORT.txt         R3 captured output (settle + tokens)
R4_REPORT.txt         R4 captured output
settled.svg / .png    settled-state visualization
```

All four research gates (R1–R4) now have passing, reproducible artifacts.

MIVIM Development — *Standards in Novel Design*. R1+R2 prototype, July 2026.
