# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

SLIME-UI / NNUI_CELL is a research project (MIVIM Development) developing a novel UI paradigm: replacing the box model with a single continuous medium governed by a rheological constitutive law (yield-stress fluid, inspired by non-Newtonian fluids) plus a deterministic genotype→phenotype expression layer. The end goal is a layout engine that LLM agents drive by declaring material properties and containment instead of coordinates.

There is **no implementation yet**. The repository currently contains theory and specification artifacts. Naming: **SLIME** is the application identity, **NNLG** (non-Newtonian Liquid Glass) is the method name, **NNUI_CELL** is the formal model in the paper.

## Roles and working style

- Claude is the **master developer** — owns research, technical design, and rigor.
- The user ("Heat", R. Forrestall) is the **creative lead and user-feature executive** — owns design authority and creative direction. Direct questions to them should concern creative/feature decisions, not technical ones.
- The project follows a strict pipeline: research → paper → theory → planning → development → application. Do not skip stages. **Theory is currently frozen** at `NNUI_CELL_PoC_v0.1`; the next planned artifact is the R1+R2 headless Rust prototype (settle 50 regions in <16 ms/frame, bit-identical determinism).

## Anti-drift (load-bearing constraint)

The core project risk, repeatedly emphasized by the user: novel design decays into "what works" when work is watered down — especially by sub-agents that simplify what they don't understand. Concretely:

- The seven executable invariants in §8 of the paper are binding (no authored positions, real constitutive law evaluated per frame, locks are solidity only, threshold-terminated settling, one medium, no animation system, every biological metaphor reduces to a deterministic mechanism).
- Nothing stochastic may enter the genetic path; gene transfer is opt-in only, never proximity-triggered.
- Explicitly deferred (do not research or build): OS integration, multi-user editing, CSS interop, performance beyond 50 regions.
- If dispatching sub-agents, give them the full design context and check their output for reductive simplification.

## Repository contents

- `NNUI_CELL_PoC_v0.1.tex` / `.pdf` — the canonical formal specification (constitutive model, XPBD solver design, genetic operators, agent grammar, invariants, experiment plan R1–R4). Edit the `.tex`; the PDF is its build output.
- `logs/chatlog` — original brainstorm and the collaboration log establishing intent, roles, and definitions. Primary source for design intent.
- `research/methods.md` — early brainstorm notes (rheology background, terminology).
- `CHANGELOG` — session journal with numbered entries (`## YYYY-MM-DD: NNN`). Append a new entry summarizing substantive work done in a session.

## Commands

Build the paper (run twice to resolve cross-references):

```
pdflatex NNUI_CELL_PoC_v0.1.tex
pdflatex NNUI_CELL_PoC_v0.1.tex
```

Required LaTeX packages beyond a base install: `algorithm2e`, `mathtools`, `booktabs`, `titlesec`, `listings`, `enumitem`.

Note: this directory is not a git repository.
