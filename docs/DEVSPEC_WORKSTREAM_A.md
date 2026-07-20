# DEVSPEC: NNUI_CELL Workstream A — Agent-Facing API [DOC]

This is a populated MIVIM dev-spec created from the blank specimen
[TEMPLATE_MIVIM](../.MIVIM/TEMPLATES/TEMPLATE_MIVIM) [A1] under the operating
contract of [ONBOARDING](../.MIVIM/ONBOARDING.md) [A2] with the steering note
[REFRESHER](../.MIVIM/REFRESHER.md) [A3]. It preserves the pseudo-citation
format model [FM]: compact markers reference above, the bottom registry defines
once below, and the file is maintained link-complete [LC]. The expected reader
is a local CLI agent [CLI] executing the SLIME-UI / NNUI_CELL pipeline.

## (0) Document role [GL01]

> Goal: [GL01] — DEFINE_THE_DEVSPEC_OUTCOME
> Dependency: [DP01] — MIVIM_PACKAGE_CONTRACT
> Requirement: [RQ01] — LINK_COMPLETE_STRUCTURE

This document is the planning-stage artifact for **workstream A: the
agent-facing API** — the stated goal of the whole project. The pipeline
position is: specification (v0.2, done) then R1–R4 prototype (done) then
visual demo (done) then **this plan** then development. Per the project's
pipeline standard, planning supports development; no code for workstream A is
written before this spec is accepted.

The goal family [GL], dependency family [DP], and requirement family [RQ]
carry each section's metadata in reference form; their definitions live only
in the registry [RQ01].

## (1) Project identity [PRJ01]

> Goal: [GL02] — CAPTURE_PROJECT_IDENTITY
> Dependency: [DP02] — FROZEN_THEORY_BASE
> Requirement: [RQ02] — REPLACE_TEMPLATE_TOKENS

All template identity tokens are replaced with real project content per
[PRJ] and [RQ02]:

| Field | Value |
| --- | --- |
| Project name | SLIME-UI / NNUI_CELL (MIVIM Development) |
| Project type | Continuum-field layout engine (rheology + genetics) |
| Owners / roles | Claude (FABLE5), master dev; Heat, creative lead |
| Primary outcome | Agent-driven API: material and containment, no coordinates |
| Current lifecycle stage | Planning for workstream A [PRJ03] |
| Non-goals | OS integration, multi-user editing, CSS interop, scale beyond the measured envelope [IG01] |
| Key constraints | Bit-identical determinism [CN01]; real-time frame budget [CN02]; the seven invariants [IV] |

## (2) Context intake [WH]

> Goal: [GL03] — BUILD_CONTEXTUAL_BOOTSTRAP
> Dependency: [DP03] — REPO_ARTIFACT_TRAIL
> Requirement: [RQ03] — WHO_WHAT_WHERE_WHY_HOW_COMPLETE

### (2.1) Who [WH01]

A local CLI agent [CLI] acting as master developer, and the human creative
lead who holds design authority. Sub-agents may be dispatched for scoped
tasks, but their output is checked against the invariants [IV] because
reductive simplification is the project's primary failure mode [DN03].

### (2.2) What [WH02]

The development specification for the public, agent-facing API over the
validated NNUI_CELL engine: program submission in the agent grammar, the
settle loop, the read path, and the portable cell serialization format
[DS02], [DS04], [DS05].

### (2.3) Where [WH03]

Repository `slime-ui`. The canonical theory is
[NNUI_CELL_PoC_v0.2.tex](../latex/v0.2/NNUI_CELL_PoC_v0.2.tex); the validated
prototype sources live under [analysis/001](../analysis/001/README.md) and
[analysis/002](../analysis/002/README.md); the corroborating visual bench is
under [analysis/004](../analysis/004/README.md). Workstream A graduates the
prototype into an `engine/` source tree [DS06].

### (2.4) Why [WH04]

The token-economy and zero-re-authoring claims only become product value
through an API surface an LLM can emit cheaply and trust deterministically.
The measured floor is 2.43x cheaper than equivalent markup [EV04]; the
structural claim (zero re-authoring on change) and cross-surface portability
follow from the model but are not yet demonstrated through a public interface
[EV02].

### (2.5) How [WH05]

MIVIM-format documentation with binary-exit milestones in the style of the
R-gates: every milestone [TK] states an exit criterion that either passes or
fails, invariant tests gate every merge [CN03], and all validation follows
the lint discipline [LN] without configuration changes.

## (3) Constraints, guardrails, and invariants [CN]

> Goal: [GL04] — PIN_ANTIDRIFT_BOUNDARIES
> Dependency: [DP04] — EXECUTABLE_INVARIANTS
> Requirement: [RQ04] — GUARDRAILS_ARE_EXPLICIT

Required constraints:

- Determinism is preserved through the API layer: identical programs settle
  to bit-identical states, as measured in R2 [CN01].
- The real-time budget holds: 50 regions inside 16 ms/frame at the API level,
  not only in the raw solver [CN02].
- A milestone is complete only when its binary exit criterion passes and the
  full invariant test suite is green [CN03].

Prohibited behaviors [DN]:

- Nothing stochastic enters the genetic path; gene transfer is opt-in only,
  never proximity-triggered [DN01].
- The API accepts no authored coordinates. There is no input field, verb, or
  escape hatch that positions a cell directly; geometry is always a solver
  output [DN02].
- Sub-agent output is never merged unchecked; every contribution is audited
  against the invariants before acceptance [DN03].

Ignored scope [IG]: OS integration, multi-user editing, CSS interop, and
performance beyond the measured envelope remain deferred stage-5 questions
[IG01].

The seven executable invariants [IV] are the binding anti-drift boundary.
They are restated here as the acceptance frame every milestone inherits:

| Marker | Invariant | Enforcement hook |
| --- | --- | --- |
| [IV01] | No authored positions | Static check + perturbation test |
| [IV02] | Real constitutive law per frame | Flow-index sweep monotonicity |
| [IV03] | Locks are solidity only | No lock special-casing in AST |
| [IV04] | Threshold-terminated settling, honestly measured | Post-arrest strain readout |
| [IV05] | One medium | Removing a component type removes no layout path |
| [IV06] | No animation system | Freezing the solver freezes all motion |
| [IV07] | Metaphor reduces to mechanism | Bit-identity; no stochastic call in genetic path |

## (4) Workflow roadmap [WL]

> Goal: [GL05] — PLAN_THE_WORKFLOW
> Dependency: [DP05] — ORDERED_PHASES
> Requirement: [RQ05] — WORKFLOW_HAS_PHASE_OWNERSHIP

| Phase | Marker | Concrete work |
| --- | --- | --- |
| Discover | [WL01] | Fix the API consumer model: what an agent must submit, read back, and be notified of; enumerate the read-path questions the paper leaves open |
| Design | [WL03] | Specify the API surface [DS02], the serialization format [DS04], and the read path [DS05] with worked examples before code |
| Build | [WL04] | Graduate the crate to `engine/` [DS06]; implement milestones in order [TK01] to [TK04] |
| Validate | [WL05] | Binary exits per milestone; invariant suite green; determinism hash checks at the API boundary [CN01] |

The blank specimen's research phase (`WL02` in the template) is deliberately
removed rather than left ornamental: theory is frozen at v0.2 and the
empirical questions were already answered by the executed R-gates [EV04]. Its
registry entry is omitted per the unused-phase rule of the template.

## (5) Evidence base [RS]

> Goal: [GL06] — GROUND_CLAIMS_IN_MEASURED_RESULTS
> Dependency: [DP06] — REAL_SOURCE_MATERIAL
> Requirement: [RQ06] — DO_NOT_FABRICATE_SOURCES

Evidence levels [EV] are assigned honestly:

- **Validated results [EV04]** — R1 (1.2–1.4 ms/frame at 50 regions; 11.8 ms
  at 400), R2 (100 bit-identical settles, hash `0xdb5295e7d7bce689`), R3
  (68/47/52 tokens, 2.43x vs markup), R4 (registry extension with core
  untouched), and the arrest-mechanism verification (exact zero strain, zero
  flicker). Sources: [RS02], [RS03], [RS04], folded normatively into [RS01].
- **Internal reasoning [EV02]** — cross-surface cell portability: it follows
  from no-stored-geometry [IV01] plus determinism [CN01], but has not been
  demonstrated through a public interface. Milestone [TK04] converts this to
  a measured result.
- **Working assumptions [EV01]** — cross-hardware float determinism is
  achievable at acceptable cost; token savings compound over a session. Both
  remain unverified and are stated as such in the paper's limitations.
- No claim in this dev-spec currently carries external-source support
  [EV03]; the constitutive-law lineage (Herschel–Bulkley, Papanastasiou,
  XPBD) is recorded inside the paper itself [RS01]. New external sources get
  numbered records only when actually read [RQ06].

## (6) Design: the API surface [DS]

> Goal: [GL07] — SPECIFY_THE_AGENT_FACING_API
> Dependency: [DP07] — GRAMMAR_AND_SOLVER_BASE
> Requirement: [RQ07] — API_PRESERVES_INVARIANTS

Design summary [DS01]: promote the validated prototype from experiment
artifact to a versioned public API with four connected objects:

- **API surface [DS02]** — the agent grammar (two verb families: rheological
  and genetic) becomes the submission contract; error semantics are defined
  for invalid programs (parse errors, unknown traits, containment cycles);
  every accepted program yields a deterministic settled state. No verb
  accepts a coordinate [DN02].
- **Cell serialization format [DS04]** — a cell's complete wire form is
  genome plus material state plus containment. Geometry is structurally
  absent from the format, which is what makes cells host-portable [EV02].
- **Read path [DS05]** — the genuinely unspecified piece. An agent must be
  able to query settled geometry, subscribe to the settled event, and read
  the honest strain measurement [IV04] without touching solver internals.
- **Engine packaging [DS06]** — the crate moves from `analysis/` to
  `engine/` with the 15 existing tests as merge gates; the invariant suite
  becomes CI, which is the anti-drift machinery running automatically [DN03].

Acceptance criteria [DS03]: each milestone below carries a binary exit; all
seven invariants [IV] hold at the API boundary, not merely inside the solver.

### (6.1) Read-path discovery record [WL01]

Discovery output for the read path [DS05], recorded per Stage 1 of the root
execution ledger. These answers hold at internal-reasoning strength [EV02]
until milestone M2 validates them:

1. **Snapshot shape.** A settled-geometry snapshot exposes, per cell:
   centroid, bounding extent, and a boundary polyline at a requested level
   of detail, plus container occupancy. Every value is a read-only solver
   output; nothing in the snapshot is writable [IV01].
2. **Settlement signal.** A settled event fires when the strain predicate
   crosses threshold, per cell and globally. The settled state carries its
   determinism hash so agents can compare layouts across runs and machines
   [CN01].
3. **Strain exposure.** The only motion figure exposed is the realized
   post-arrest strain, per cell and as a global maximum — the honest
   instrument, never a damped or zeroed variable [IV04].
4. **Mid-settle reads.** Snapshots may be taken at any tick; the settled
   flag distinguishes transient geometry from final geometry, so agents
   never mistake motion for rest.
5. **Identity and traits.** Cell identifiers are stable across re-settles.
   Agents may read a cell's trait set — agents author genomes by injecting
   assets — while end-user surfaces touch only expressed phenotype.
6. **Read-only guarantee.** No query mutates solver state, so query order
   can never affect settlement; reads sit entirely outside the determinism
   boundary [CN01].

**First consumer (creative-lead decision, closing Stage 1):** the headless
test harness leads. The harness drives the API through round-trip, hash,
and read-path proofs before anything visual; the demo is built on top only
after milestones M1 and M2 pass [TK01], [TK02]. This mirrors how the
R-gates were validated and keeps visuals out of the contract-hardening
path [DN03].

## (7) Milestones [TK]

> Goal: [GL08] — SEQUENCE_BINARY_MILESTONES
> Dependency: [DP08] — MILESTONE_DEPENDENCY_CHAIN
> Requirement: [RQ08] — BINARY_EXIT_CRITERIA

Milestones are ordered; each depends on the previous [DP08]. Named M1–M4 in
prose, tracked as task markers:

- **M1 — API round-trip [TK01].** Public API: submit program, settle, query.
  *Exit:* the three R3 reference layouts settle through the public API to the
  same hashes as through the prototype binary; all existing tests green.
- **M2 — Read path [TK02].** Query and event model per [DS05].
  *Exit:* an agent can obtain settled geometry and detect settlement without
  polling internals; the exposed strain is the realized post-arrest value
  [IV04]; a lying-instrument test (positions move while strain reads zero)
  fails the build.
- **M3 — LLM drive [TK03].** A real LLM emits grammar from natural-language
  intent. *Exit:* at least three novel layout intents produce valid programs
  that parse and settle with zero authored coordinates [IV01], [DN02]; token
  counts recorded against the R3 baseline [EV04].
- **M4 — Serialization and portability [TK04].** The wire format of [DS04]
  round-trips. *Exit:* a cell serialized out of layout A and injected into
  layout B re-expresses and re-settles bit-identically across repeated runs
  [CN01]; the format contains no geometry field, checked structurally.

## (8) Validation and handoff [OUT]

> Goal: [GL09] — VALIDATE_BEFORE_HANDOFF
> Dependency: [DP09] — SHELL_SCAN_AND_LINT_TOOLS
> Requirement: [RQ09] — REPORT_VALIDATION_STATUS

This document is validated with the package's shell workflow before every
handoff [LN]:

```sh
# 1. Illegal body definitions (blockquote-shaped).
grep -nE '^>\s*\[[A-Za-z0-9]+\]:' docs/DEVSPEC_WORKSTREAM_A.md

# 2. Zero-number placeholder IDs.
grep -nE '\[[A-Z]{1,6}00\]' docs/DEVSPEC_WORKSTREAM_A.md

# 3. All live reference definitions (must be registry-only).
grep -nE '^\[[A-Za-z0-9]+\]:' docs/DEVSPEC_WORKSTREAM_A.md

# 4. Lint without configuration changes.
npx --yes markdownlint-cli2 docs/DEVSPEC_WORKSTREAM_A.md
```

MD053 failures are repaired in the document, never suppressed [MD053]. The
handoff artifact is this file [OUT01] plus a validation summary stating lint
status, link-completeness, and structural changes [OUT02], with the audit
row added to [VALIDATE_WORKFLOW.json](../.MIVIM/VALIDATE_WORKFLOW.json).

## (Y) Link-complete usage index [LI]

This section is non-authoritative. It invokes every active marker so the
registry definitions below remain needed by the Markdown reference system.

- Package and format markers: [DOC], [A1], [A2], [A3], [CLI], [FM], [LC],
  [LI], [LN], [MD053]
- Context markers: [WH], [WH01], [WH02], [WH03], [WH04], [WH05], [PRJ],
  [PRJ01], [PRJ03]
- Goal markers: [GL], [GL01], [GL02], [GL03], [GL04], [GL05], [GL06],
  [GL07], [GL08], [GL09]
- Dependency markers: [DP], [DP01], [DP02], [DP03], [DP04], [DP05], [DP06],
  [DP07], [DP08], [DP09]
- Requirement markers: [RQ], [RQ01], [RQ02], [RQ03], [RQ04], [RQ05], [RQ06],
  [RQ07], [RQ08], [RQ09]
- Boundary markers: [CN], [CN01], [CN02], [CN03], [DN], [DN01], [DN02],
  [DN03], [IG], [IG01]
- Invariant markers: [IV], [IV01], [IV02], [IV03], [IV04], [IV05], [IV06],
  [IV07]
- Workflow markers: [WL], [WL01], [WL03], [WL04], [WL05]
- Evidence markers: [RS], [RS01], [RS02], [RS03], [RS04], [EV], [EV01],
  [EV02], [EV03], [EV04]
- Design markers: [DS], [DS01], [DS02], [DS03], [DS04], [DS05], [DS06]
- Task markers: [TK], [TK01], [TK02], [TK03], [TK04]
- Output markers: [OUT], [OUT01], [OUT02]

## (Z) Context-Citations

[DOC]: DEVSPEC_WORKSTREAM_A
  "The workstream A development specification: the planning-stage artifact
  for the NNUI_CELL agent-facing API."

[A1]: TEMPLATE_MIVIM
  "The reusable blank pseudo-citation template this document was populated
  from."

[A2]: AGENT_ONBOARDING
  "The full fresh-agent ingestion guide for the pseudo-citation package and
  its operating rules."

[A3]: ACTIVE_SESSION_REFRESHER
  "The compact steering note used during active sessions to hold the core
  format rules."

[CLI]: LOCAL_CLI_AGENT
  "A local shell-capable agent such as Claude Code operating directly on
  repository files as master developer."

[FM]: FORMAT_MODEL
  "The pseudo-citation design where compact body markers reference complete
  bottom-registry definitions."

[LC]: LINK_COMPLETE_STATE
  "A state where every active marker is defined once, referenced at least
  once, placeholder-free, and lint-clean."

[LI]: LINK_INDEX
  "The marker for the non-authoritative usage-index section that anchors
  active definitions."

[LN]: LINT_RULE
  "The discipline of repairing pseudo-citation syntax in the document rather
  than suppressing markdownlint."

[MD053]: LINK_IMAGE_REFERENCE_DEFINITION_FAILURE
  "A markdownlint failure caused by duplicate, unused, or accidentally
  parsed reference definitions."

[WH]: WHO_WHAT_WHERE_WHY_HOW
  "The contextual bootstrap that makes this document ingestible without
  hidden conversation history."

[WH01]: WHO_CONTEXT
  "The readers and actors: the CLI master-developer agent, the human
  creative lead, and audited sub-agents."

[WH02]: WHAT_CONTEXT
  "The object of this spec: the public agent-facing API over the validated
  NNUI_CELL engine."

[WH03]: WHERE_CONTEXT
  "The repository locations of the theory, prototype, bench, and the target
  engine source tree."

[WH04]: WHY_CONTEXT
  "The purpose: convert measured token economy and structural
  zero-re-authoring into product value through a public interface."

[WH05]: HOW_CONTEXT
  "The method: MIVIM documentation, binary-exit milestones, invariant tests
  as merge gates, lint discipline."

[PRJ]: PROJECT_CONTEXT
  "The SLIME-UI / NNUI_CELL project: a continuum-field layout engine
  replacing the box model, driven by agents declaring material state."

[PRJ01]: PROJECT_IDENTITY
  "The populated identity table naming the project, owners, outcome, stage,
  non-goals, and constraints."

[PRJ03]: PROJECT_LIFECYCLE_STAGE
  "The current stage: planning for workstream A, after specification and
  validation stages completed."

[GL]: GOAL
  "The intended outcome of a section, milestone, validation pass, or
  handoff."

[GL01]: DEFINE_THE_DEVSPEC_OUTCOME
  "State what this document is for and where it sits in the project
  pipeline before detail is added."

[GL02]: CAPTURE_PROJECT_IDENTITY
  "Record the project identity so the spec stands alone without hidden
  context."

[GL03]: BUILD_CONTEXTUAL_BOOTSTRAP
  "Complete who, what, where, why, and how for a fresh agent."

[GL04]: PIN_ANTIDRIFT_BOUNDARIES
  "Make constraints, prohibitions, deferred scope, and the seven invariants
  explicit and enforceable."

[GL05]: PLAN_THE_WORKFLOW
  "Lay out the ordered phases from discovery through validation with owned
  work."

[GL06]: GROUND_CLAIMS_IN_MEASURED_RESULTS
  "Attach honest evidence levels and real source records to every claim the
  spec relies on."

[GL07]: SPECIFY_THE_AGENT_FACING_API
  "Define the API surface, serialization format, read path, and packaging
  before implementation."

[GL08]: SEQUENCE_BINARY_MILESTONES
  "Order the build into milestones whose exit criteria pass or fail with no
  middle state."

[GL09]: VALIDATE_BEFORE_HANDOFF
  "Run the shell scans and lint pass and report status before returning
  work."

[DP]: DEPENDENCY
  "A prerequisite, support object, or input needed to satisfy the related
  requirement."

[DP01]: MIVIM_PACKAGE_CONTRACT
  "This document depends on the template, onboarding guide, and refresher
  that define the format it must preserve."

[DP02]: FROZEN_THEORY_BASE
  "Identity rests on the v0.2 specification: theory is frozen and only its
  realization is extended."

[DP03]: REPO_ARTIFACT_TRAIL
  "Context depends on the traceable artifact chain: paper, prototype
  crates, reports, and bench."

[DP04]: EXECUTABLE_INVARIANTS
  "The guardrails depend on the seven invariants existing as automated
  tests, not prose intentions."

[DP05]: ORDERED_PHASES
  "The roadmap depends on phase ownership so discovery, design, build, and
  validation stay distinct."

[DP06]: REAL_SOURCE_MATERIAL
  "Evidence depends on source records that exist in the repository and can
  be reproduced."

[DP07]: GRAMMAR_AND_SOLVER_BASE
  "The API design depends on the validated grammar lowering and the R1
  solver it drives."

[DP08]: MILESTONE_DEPENDENCY_CHAIN
  "Each milestone depends on the previous one: round-trip before read path,
  read path before LLM drive, all before portability."

[DP09]: SHELL_SCAN_AND_LINT_TOOLS
  "Validation depends on grep-style scans and markdownlint run without
  configuration changes."

[RQ]: REQUIREMENT
  "A necessary condition or acceptance rule required to achieve the
  correlated goal."

[RQ01]: LINK_COMPLETE_STRUCTURE
  "Every active marker is defined once in the registry and referenced at
  least once in body or index."

[RQ02]: REPLACE_TEMPLATE_TOKENS
  "No double-brace template tokens remain; all identity fields carry real
  project content."

[RQ03]: WHO_WHAT_WHERE_WHY_HOW_COMPLETE
  "The bootstrap answers all five context questions without relying on
  conversation history."

[RQ04]: GUARDRAILS_ARE_EXPLICIT
  "Constraints, do-not rules, ignored scope, and invariants are stated
  directly enough for another agent to enforce."

[RQ05]: WORKFLOW_HAS_PHASE_OWNERSHIP
  "Each phase states the work it owns and the output it produces; unused
  phases are removed, not kept ornamentally."

[RQ06]: DO_NOT_FABRICATE_SOURCES
  "Numbered source records exist only for real, present material; future
  needs stay as prose."

[RQ07]: API_PRESERVES_INVARIANTS
  "The public interface must uphold all seven invariants at its boundary,
  including rejecting authored coordinates."

[RQ08]: BINARY_EXIT_CRITERIA
  "Every milestone exit either passes or fails measurably; no milestone is
  declared done by judgment call."

[RQ09]: REPORT_VALIDATION_STATUS
  "Handoffs state lint status, link-completeness, and structural changes
  explicitly."

[CN]: CONSTRAINT
  "A rule or acceptance condition that must remain true throughout
  workstream A."

[CN01]: DETERMINISM_PRESERVED
  "Identical programs settle to bit-identical states through the public
  API, extending the R2 result to the API boundary."

[CN02]: REALTIME_BUDGET
  "Fifty regions settle inside 16 ms/frame at the API level, preserving the
  R1 result."

[CN03]: ACCEPTANCE_THRESHOLD
  "A milestone is complete only when its binary exit passes and the full
  invariant suite is green."

[DN]: DO_NOT
  "A negative constraint that blocks drift or unsafe behavior."

[DN01]: NO_STOCHASTIC_GENETIC_PATH
  "Nothing random enters the genetic path; gene transfer is a deliberate,
  opt-in, deterministic operation."

[DN02]: NO_COORDINATE_INTAKE
  "The API exposes no field, verb, or escape hatch that authors a position;
  geometry is always solved."

[DN03]: NO_REDUCTIVE_SIMPLIFICATION
  "Sub-agent output is audited against the invariants before merge; watered
  down contributions are rejected."

[IG]: IGNORE_RULE
  "An explicit exclusion that removes a direction from active
  consideration."

[IG01]: DEFERRED_STAGE5_SCOPE
  "OS integration, multi-user editing, CSS interop, and beyond-envelope
  performance stay deferred until after workstream A."

[IV]: INVARIANT
  "The family of seven executable invariants from the v0.2 specification
  that pin the design against reductive drift."

[IV01]: NO_AUTHORED_POSITIONS
  "No code path assigns a coordinate directly; every position is a solver
  output."

[IV02]: REAL_CONSTITUTIVE_LAW
  "Apparent viscosity is evaluated per frame from the constitutive
  equation; no easing or spring preset stands in for it."

[IV03]: LOCKS_ARE_SOLIDITY
  "There is no boolean lock; all locking flows through the solidity field
  approaching one."

[IV04]: HONEST_THRESHOLD_SETTLING
  "Settling ends by strain-rate threshold measured on realized post-arrest
  displacement, never a timer or a zeroed velocity variable."

[IV05]: ONE_MEDIUM
  "No component owns private layout logic outside the solver."

[IV06]: NO_ANIMATION_SYSTEM
  "The renderer is a stateless view of solver state; no tweens, timelines,
  or durations exist."

[IV07]: METAPHOR_REDUCES_TO_MECHANISM
  "Every biological term maps to a defined deterministic operator; the
  genetic path contains no stochastic call."

[WL]: WORKFLOW
  "The ordered phases that move workstream A from discovery to validated
  delivery."

[WL01]: DISCOVER
  "Fix the API consumer model and enumerate the open read-path questions
  before design."

[WL03]: DESIGN
  "Specify the API surface, serialization format, and read path with worked
  examples before code."

[WL04]: BUILD
  "Graduate the crate to the engine tree and implement the milestones in
  dependency order."

[WL05]: VALIDATE
  "Check binary exits, invariant suite, and determinism hashes at the API
  boundary."

[RS]: RESEARCH_SOURCE
  "A real source record present in this repository; external sources gain
  records only when actually read."

[RS01]: RESEARCH_SOURCE
  "TITLE: NNUI_CELL Proof of Concept and Formal Specification v0.2
  SOURCE_TYPE: Specification paper (LaTeX/PDF)
  AUTHOR_OR_ORG: MIVIM Development
  YEAR_OR_DATE: July 2026
  LINK_OR_DOI: ../latex/v0.2/NNUI_CELL_PoC_v0.2.tex
  USED_FOR: Canonical theory, corrections C1-C4, arrest mechanism, invariants
  CONTEXT_WEIGHT: HIGH
  LINKED_CONTEXT: the invariant family, the milestone exits, the API design"

[RS02]: RESEARCH_SOURCE
  "TITLE: R1+R2 report — real-time settling and bit-identical determinism
  SOURCE_TYPE: Captured experiment output with reproducible crate sources
  AUTHOR_OR_ORG: MIVIM Development
  YEAR_OR_DATE: July 2026
  LINK_OR_DOI: ../analysis/001/R1R2_REPORT.txt
  USED_FOR: Frame-cost and determinism evidence behind CN01 and CN02
  CONTEXT_WEIGHT: HIGH
  LINKED_CONTEXT: determinism constraint, real-time budget, milestone M1"

[RS03]: RESEARCH_SOURCE
  "TITLE: R3+R4 reports — agent grammar token economy and expression registry
  SOURCE_TYPE: Captured experiment outputs with reproducible crate sources
  AUTHOR_OR_ORG: MIVIM Development
  YEAR_OR_DATE: July 2026
  LINK_OR_DOI: ../analysis/002/R3_REPORT.txt and ../analysis/002/R4_REPORT.txt
  USED_FOR: Token-economy baseline and registry extensibility behind M1 and M3
  CONTEXT_WEIGHT: HIGH
  LINKED_CONTEXT: the API surface, milestone M3 token comparison"

[RS04]: RESEARCH_SOURCE
  "TITLE: Workstream B visual bench and verification record
  SOURCE_TYPE: Self-contained browser bench with programmatic verification
  AUTHOR_OR_ORG: MIVIM Development
  YEAR_OR_DATE: July 2026
  LINK_OR_DOI: ../analysis/004/README.md
  USED_FOR: Arrest-mechanism corroboration and the honest-strain requirement
  CONTEXT_WEIGHT: MEDIUM
  LINKED_CONTEXT: invariant four, the read-path lying-instrument test in M2"

[EV]: EVIDENCE_LEVEL
  "A label communicating how strongly a claim is supported."

[EV01]: ASSUMPTION
  "A working assumption not yet verified, such as affordable cross-hardware
  determinism."

[EV02]: INTERNAL_REASONING
  "A claim supported by the model's own logic but not yet demonstrated
  through a public interface, such as cell portability."

[EV03]: EXTERNAL_SUPPORT
  "A claim supported by real external sources; currently none are attached
  directly to this spec."

[EV04]: VALIDATED_RESULT
  "A claim supported by executed experiments with reproducible artifacts,
  such as the four R-gate results."

[DS]: DESIGN_OBJECT
  "A stable design-level object of the workstream A architecture."

[DS01]: DESIGN_SUMMARY
  "Promote the validated prototype to a versioned public API: submission,
  settle, read path, serialization, packaging."

[DS02]: API_SURFACE
  "The agent grammar as submission contract with defined error semantics
  and deterministic settled output; no coordinate intake."

[DS03]: ACCEPTANCE_CRITERIA
  "Binary milestone exits plus all seven invariants holding at the API
  boundary."

[DS04]: CELL_SERIALIZATION_FORMAT
  "The portable wire form of a cell: genome, material state, containment;
  structurally free of geometry."

[DS05]: READ_PATH_MODEL
  "The query and event surface: settled geometry, settlement events, and
  the honest post-arrest strain readout."

[DS06]: ENGINE_PACKAGING
  "The prototype crate graduates to an engine source tree with the
  invariant tests as continuous merge gates."

[TK]: TASK_OBJECT
  "A scoped milestone of workstream A with a binary exit criterion."

[TK01]: MILESTONE_API_ROUNDTRIP
  "M1: submit-settle-query through the public API reproduces the prototype
  hashes for the three reference layouts."

[TK02]: MILESTONE_READ_PATH
  "M2: agents obtain settled geometry and settlement events; exposed strain
  is the realized post-arrest value; lying instruments fail the build."

[TK03]: MILESTONE_LLM_DRIVE
  "M3: a real LLM turns natural-language intent into valid programs that
  settle with zero authored coordinates; token counts recorded."

[TK04]: MILESTONE_SERIALIZATION_PORTABILITY
  "M4: a serialized cell injected into a different layout re-expresses and
  re-settles bit-identically; the format carries no geometry."

[OUT]: OUTPUT
  "The final artifact and reporting expected at each handoff."

[OUT01]: FINAL_ARTIFACT
  "This dev-spec file, maintained link-complete, plus the milestone
  deliverables it defines."

[OUT02]: VALIDATION_SUMMARY
  "A concise statement of lint status, link-completeness, and structural
  changes accompanying every handoff."
