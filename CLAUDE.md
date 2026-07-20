# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

SLIME-UI / NNUI_CELL is a research project (MIVIM Development) developing a novel UI paradigm: replacing the box model with a single continuous medium governed by a rheological constitutive law (yield-stress fluid) plus a deterministic genotype→phenotype expression layer. The end goal is a layout engine that LLM agents drive by declaring material properties and containment instead of coordinates.

**State of development:** the specification is at **v0.2** (post-validation revision); all four research gates (R1–R4) passed on the headless prototype, and **workstream A (the agent-facing API) is complete** — milestones M1–M4 all passed through the `engine/` crate (35/35 tests green: atomic grammar submission, event-driven read path with honest strain, LLM drive, portable geometry-free cell records). The v0.2 arrest mechanism is now normative solver code. Merge gates are automated via `.githooks/pre-commit`. Governing documents: `docs/DEVSPEC_WORKSTREAM_A.md`, `docs/DESIGN_API_CONTRACTS.md`, `TODO.md` (ledger complete). Next stage: integration / demo on top of the API (harness-first decision: demo only after M1–M2, which are passed).

Naming: **SLIME** is the application identity, **NNLG** (non-Newtonian Liquid Glass) is the method name, **NNUI_CELL** is the formal model in the paper.

## Roles and working style

- Claude is the **master developer** — owns research, technical design, and rigor.
- The user ("Heat", R. Forrestall) is the **creative lead and user-feature executive** — owns design authority and creative direction. Direct questions to them should concern creative/feature decisions, not technical ones.
- The project follows a strict pipeline: research → paper → theory → planning → development → application. Do not skip stages. **Theory is frozen at v0.2**; only its discrete realization may be sharpened by measurement. Milestones are built strictly in order with binary exit criteria.

## MIVIM documentation contract (required)

All substantive project documents use the MIVIM pseudo-citation format. Before creating or editing any of them, read `.MIVIM/ONBOARDING.md` (full contract) or `.MIVIM/REFRESHER.md` (active-session reminder). Blank templates live in `.MIVIM/TEMPLATES/`.

- Core rule: reference markers above, define each once in `(Z) Context-Citations`, keep a `(Y)` usage index, never leave zero-number placeholders, and repair markdownlint failures in the document — never in the config.
- After any MIVIM document edit: run the link-completeness audit and `npx --yes markdownlint-cli2 <file>`, and refresh the file's row in `.MIVIM/VALIDATE_WORKFLOW.json`.
- `TODO.md` (root) is the execution ledger: tasks trace to requirements, milestones stay blocked until the prior binary exit passes.

## Anti-drift (load-bearing constraint)

The core project risk, repeatedly emphasized by the user: novel design decays into "what works" when work is watered down — especially by sub-agents that simplify what they don't understand. Concretely:

- The seven executable invariants in §9 of the paper are binding (no authored positions, real constitutive law evaluated per frame, locks are solidity only, threshold-terminated settling honestly measured, one medium, no animation system, every biological metaphor reduces to a deterministic mechanism).
- Nothing stochastic may enter the genetic path; gene transfer is opt-in only, never proximity-triggered. The API accepts no authored coordinates.
- Explicitly deferred (do not research or build): OS integration, multi-user editing, CSS interop, performance beyond the measured envelope.
- If dispatching sub-agents, give them the full design context and check their output for reductive simplification before merging.

## Repository contents

- `latex/v0.2/NNUI_CELL_PoC_v0.2.tex` / `.pdf` — the canonical formal specification (current). Edit the `.tex`; the PDF is its build output. `latex/v0.1/` is superseded.
- `docs/DEVSPEC_WORKSTREAM_A.md` — the workstream A dev-spec (MIVIM format): API surface, serialization, read path, milestones M1–M4.
- `TODO.md` — the MIVIM execution ledger (staged, requirement-traced tasks).
- `engine/` — the graduated engine crate (workstream A build tree). `cargo test --release` must stay 15/15 green; the invariant suite is the merge gate.
- `analysis/001`–`004` — validated prototype artifacts: R1+R2 solver crate, R3 grammar + R4 genetics, and the visual bench (003 is the rejected v1, kept as the canonical drift example). Read-only history; the live code is `engine/`.
- `logs/devlog` — the full development narrative; primary source for design intent.
- `docs/research/methods.md` — early brainstorm notes (rheology background, terminology).
- `.MIVIM/` — documentation workflow contracts and templates.
- `CHANGELOG` — session journal with numbered entries (`## YYYY-MM-DD: NNN`). Append a new entry summarizing substantive work done in a session.

## Commands

Build the paper (run twice to resolve cross-references):

```sh
cd latex/v0.2
pdflatex NNUI_CELL_PoC_v0.2.tex
pdflatex NNUI_CELL_PoC_v0.2.tex
```

Required LaTeX packages beyond a base install: `algorithm2e`, `mathtools`, `booktabs`, `titlesec`, `listings`, `enumitem`.

Validate a MIVIM document:

```sh
npx --yes markdownlint-cli2 <file>.md
grep -nE '^>\s*\[[A-Za-z0-9]+\]:' <file>.md   # illegal body definitions
grep -nE '\[[A-Z]{1,6}00\]' <file>.md          # zero placeholders
```

Note: this directory **is** a git repository (branch `main`). Commit or push only when the user asks.
