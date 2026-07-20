# SLIME-UI / NNUI_CELL

**A continuum field model for self-settling user interfaces** — replacing the
box model with a single continuous medium governed by a rheological
constitutive law (yield-stress fluid) and a deterministic genotype→phenotype
expression layer.

MIVIM Development — *Standards in Novel Design*.

## Naming

| Name | Role |
| --- | --- |
| **SLIME** | The application identity |
| **NNLG** (non-Newtonian Liquid Glass) | The method name |
| **NNUI_CELL** | The formal model in the paper |

## The idea

Contemporary UI is thousands of independent boxes, each carrying private
geometry and style — which is why assets clash, layout precedes design, and
every adaptation re-authors geometry. NNUI_CELL removes the primitive: there
is **one continuous medium**, and a "component" is a region of that medium
distinguished only by its local material state and its skin. Layout is never
authored; it is always **solved**.

Two interlocking axes fully specify a region (a *cell*):

- **Rheological axis** — *how it flows.* Every region is a yield-stress fluid
  under a solidity-modulated Herschel–Bulkley law with rate-dependent yield
  and thixotropic settling. One dial `S ∈ [0,1)` spans freely-flowing →
  locked (locking is the limit S→1, not a flag). "Settled" is a measurable
  strain-rate predicate, not an animation duration.
- **Genetic axis** — *what it becomes.* Every cell carries a genome (absorbed
  data traits) that deterministically expresses a phenotype (rendered
  controls) through an extensible registry. A dropped CSV becomes a chart
  because its traits express one — no stochastic operator anywhere.

The end goal is a layout engine that **LLM agents drive by declaring material
properties and containment instead of coordinates** — measured at 2.43×
cheaper in tokens than equivalent markup, with zero re-authoring when content
or viewport changes.

### Use case: cross-surface portability (the standardization argument)

Consider an application that exists on several surfaces with no shared UI
standard between them — Claude Code or Codex running as a native desktop app,
a web app, an IDE extension, and a CLI. Each surface re-implements the same
interface in a different box model, and nothing moves between them.

If every surface hosted the same NNUIC medium, a "window" would be a cell —
and a cell's complete description is its genome plus a handful of material
scalars and a containment relation. **No geometry is stored, because geometry
is never authored** (Invariant 1). Dragging a Claude panel from the IDE to
the web is then just injecting that serialized cell into a different vessel:
it re-expresses its phenotype and re-settles under the new container's
pressure — same genotype, different phenotype-in-context — and determinism
(R2) guarantees it settles identically on every host. Box-model UIs cannot do
this precisely because their identity is entangled with host-specific
geometry.

This is the kind of standardization nobody thinks they need until it is shown
working. It is recorded here as *motivating use case*: actual OS/host
integration remains explicitly deferred (a stage-5 question) until the
agent-facing API exists.

## Status (July 2026)

The specification is at **v0.2** (post-validation revision). All four
proof-of-concept research gates have been executed on a dependency-free
headless Rust prototype and **passed**:

| Gate | Question | Result |
| --- | --- | --- |
| R1 | Real-time settling (XPBD) | **PASS** — 1.2–1.4 ms/frame, 50 regions |
| R2 | Determinism | **PASS** — 100 settles bit-identical |
| R3 | Agent grammar / tokens | **PASS** — 68/47/52 tokens, 2.43× cheaper |
| R4 | Expression registry | **PASS** — 4th type added, core untouched |

Detail: R1 scaling holds to 400 regions / 10,000 particles at 11.8 ms
against the 16 ms budget. R2's settled-state hash (FNV-1a
`0xdb5295e7d7bce689`) reproduces across separate process invocations. R3's
layouts (dashboard/article/settings, all under 80 tokens) lower end-to-end
into the solver, so "settles unambiguously" is demonstrated, not asserted.
R4 added PNG→image-viewer through the public registry alone.

15 automated tests green. A browser-based visual bench corroborates the
thesis on screen: one medium pours into a vessel, takes its shape, and
settles to exact zero strain with zero flicker (verified by screenshot
capture and programmatic image analysis).

**Pipeline position:** specification → R1–R4 prototype *(done)* → visual demo
*(done)* → **agent-facing API (workstream A — next)** → integration.

**Honest limitations** (see paper §12): prototype scale only (no true rigid
content inclusions yet); constants tuned, not derived; cross-hardware float
determinism untested; token economy measured narrowly; accessibility an open
design.

## Repository layout

```text
engine/                              graduated engine crate (workstream A)
latex/v0.2/NNUI_CELL_PoC_v0.2.tex   canonical formal specification (v0.2, current)
latex/v0.1/                          v0.1 spec (superseded; theory frozen here)
analysis/001/                        R1+R2 headless Rust prototype + report
analysis/002/                        R3 grammar + R4 expression registry + reports
analysis/003/                        visual bench v1 (rejected drift example)
analysis/004/                        visual bench final + verification record
logs/devlog                          master development log (source of record)
docs/research/methods.md             early rheology/terminology brainstorm
CHANGELOG                            condensed session journal
```

## Anti-drift: the seven invariants

The design is pinned against reductive drift by seven executable invariants
(paper §9) — each an automated acceptance test, not an intention:

1. **No authored positions** — every position is a solver output.
2. **Real constitutive law** — η_app evaluated per frame; no easing/spring
   presets masquerading as physics.
3. **Locks are solidity** — no boolean lock flag; all locking is `set_solidity`.
4. **Threshold-terminated settling, honestly measured** — never a timer; the
   strain readout is realized post-arrest displacement.
5. **One medium** — no component owns private layout logic outside the solver.
6. **No animation system** — the renderer is a stateless view of solver state.
7. **Metaphor reduces to mechanism** — every biological term maps to a defined
   deterministic operator; nothing stochastic in the genetic path; gene
   transfer is opt-in only, never proximity-triggered.

## Building the paper

```bash
cd latex/v0.2
pdflatex NNUI_CELL_PoC_v0.2.tex
pdflatex NNUI_CELL_PoC_v0.2.tex   # second pass resolves cross-references
```

Requires beyond a base LaTeX install: `algorithm2e`, `mathtools`, `booktabs`,
`titlesec`, `listings`, `enumitem`.

## Roles

- **Claude (FABLE5)** — master developer: research, technical design, rigor.
- **Heat (R. Forrestall)** — creative lead and user-feature executive: design
  authority and creative direction.

Pipeline standard: research → paper → theory → planning → development →
application. No stage-skipping. Deferred by design (do not build): OS
integration, multi-user editing, CSS interop, performance beyond the measured
scale.
