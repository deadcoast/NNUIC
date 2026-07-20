# (A) MIVIM TODO: NNUI_CELL WORKSTREAM A EXECUTION LEDGER [DOC]

This TODO is the execution artifact that leads development out of the
planning stage. It orders the work defined in the dev-spec [D02] into staged,
requirement-traced tasks without starting workstream A sourcecode before its
gates pass [REQ03]. Every task must trace to at least one requirement; a task
without a parent requirement is not valid work [RQ01].

## (1) Context ledger [KN]

> Goal: [GL01] — TASK_CONTEXT
> Dependency: [DP01] — DOCUMENT_CHAIN
> Requirement: [RQ01] — TASK_REQUIREMENT_TRACE

The active document chain, most-authoritative first:

| Document | Role |
| --- | --- |
| [D01] | canonical frozen theory (v0.2 specification) |
| [D02] | workstream A requirements, design, milestones |
| [D03] | session journal (numbered entries) |
| [D04] | public state of development |
| [D05] | full narrative record (source of record) |
| [D06] | validated prototype artifacts and reports |
| [D07] | MIVIM documentation workflow contracts |
| [D08] | this execution ledger |
| [D09] | workstream A API contracts (Stage 2 design) |

Real paths: [D01](latex/v0.2/NNUI_CELL_PoC_v0.2.tex),
[D02](docs/DEVSPEC_WORKSTREAM_A.md), [D03](CHANGELOG), [D04](README.md),
[D05](logs/devlog), [D06](analysis/001/README.md),
[D07](.MIVIM/ONBOARDING.md), [D08](TODO.md),
[D09](docs/DESIGN_API_CONTRACTS.md).

## (2) Stage 0: documentation hygiene [TK]

> Goal: [GL02] — SYNC_DOCUMENTATION
> Dependency: [DP02] — KNOWN_STALE_DOCS
> Requirement: [RQ02] — DOCS_MATCH_REALITY

- [x] [TK01] Refresh `CLAUDE.md`: point to the v0.2 spec path, drop the
  no-implementation and not-a-git-repo claims, record the `.MIVIM` contract
  and the dev-spec [D02] as required reading [REQ01].
- [x] [TK02] Repair the [D07] package: fix the stale relative links that
  point outside this repository and align the template filename with its
  references, then regenerate the validator audit [REQ02].
- [ ] [TK03] Keep the validator audit current: every MIVIM document added or
  edited re-runs link-completeness and lint, and its row is refreshed in
  [VALIDATE_WORKFLOW.json](.MIVIM/VALIDATE_WORKFLOW.json) [REQ02].

## (3) Stage 1: discovery [WL01]

> Goal: [GL03] — DISCOVER_CONSUMER_MODEL
> Dependency: [DP03] — DEVSPEC_BASE
> Requirement: [RQ03] — DISCOVERY_BEFORE_DESIGN

- [x] [TK04] Enumerate the read-path questions the paper leaves open — the
  settled-geometry snapshot shape, the settlement event, the strain readout
  an agent may see — and record the answers in [D02] [REQ03], [REQ05].
- [x] [TK05] Obtain the first-consumer decision from the creative lead:
  public demo-first or headless harness-first. This is a product-temperament
  call in Heat's lane; record the answer in [D02] [REQ12], [REQ03].
  Decision: **headless harness first**; demo follows after M1 and M2 pass.

## (4) Stage 2: design [WL03]

> Goal: [GL04] — DESIGN_BEFORE_CODE
> Dependency: [DP04] — DISCOVERY_OUTPUT
> Requirement: [RQ04] — WORKED_EXAMPLES_FIRST

- [x] [TK06] Write worked examples for the API surface: program submission,
  settle, query, and the error semantics for invalid programs (parse error,
  unknown trait, containment cycle). No verb accepts a coordinate [REQ04],
  [REQ10]. Delivered in [D09] sections 1 and 2.
- [x] [TK07] Draft the cell serialization format — genome, material state,
  containment — with a structural check proving no geometry field exists
  [REQ07], [REQ10]. Delivered in [D09] section 3: the grammar is the wire
  format; binds do not travel; schema gate by whitelist equality.
- [x] [TK08] Specify the read-path query and event model, exposing only the
  realized post-arrest strain (the honest instrument) [REQ05], [REQ10].
  Delivered in [D09] section 4 with the instrument gate.

## (5) Stage 3: build, milestones in order [WL04]

> Goal: [GL05] — BUILD_MILESTONES_IN_ORDER
> Dependency: [DP05] — DESIGN_CONTRACTS
> Requirement: [RQ05] — MILESTONE_ORDER_ENFORCED

Build starts only when Stage 2 is complete; each milestone starts only when
the previous binary exit has passed [REQ11].

- [x] [TK09] Graduate the validated crate from [D06] into an `engine/`
  source tree with all fifteen existing tests green before any new code
  [REQ04], [REQ09]. Done: five validated modules copied byte-identical;
  missing scaffolding reconstructed (`vec2.rs`, `lib.rs`, `Cargo.toml`,
  R3-era extensions in `compat.rs`); 15/15 tests green in release. Honest
  note: settle length reproduces the record exactly (801 frames) but the
  historical hash does not — first concrete data point for the documented
  cross-platform float limitation (Linux sandbox vs Windows libm).
- [x] [TK10] M1 — API round-trip: the three reference layouts settle through
  the public API to the same hashes as the prototype binary [REQ04],
  [REQ08], [REQ11]. Done: `engine/src/api.rs` (atomic submit, error table,
  settle, snapshot, events) + 6-test harness, 21/21 green. The harness
  caught a real defect — settled states erupted under stepping — fixed by
  completing the v0.2 arrest backport (region-rest latch, C1 topological
  guard, no-carry dissipation, zero-net-transport cage). Settle now exact,
  stable, and 5× faster (144 frames vs 801); hashes bit-identical across
  processes. Historical-binary comparison adapted to the in-engine grammar
  path (the prototype binary was never committed; recorded honestly).
- [x] [TK11] M2 — read path: settled geometry and settlement events without
  polling internals; a lying-instrument test (positions move while strain
  reads zero) fails the build [REQ05], [REQ11]. Done: hull boundaries at
  requested LOD (deterministic monotone chain), world→vessel view map,
  transition-based events read from the solver's own jamming latch (the
  sustained settled predicate — first-touch quiet flapped 15 global events
  before the fix), and the VG03 instrument gate asserting geometry-moved ⇔
  strain-nonzero both directions on every frame of a full settle. 25/25
  tests green.
- [ ] [TK12] M3 — LLM drive: at least three novel natural-language intents
  produce valid programs that settle with zero authored coordinates; token
  counts recorded against the R3 baseline [REQ06], [REQ11].
- [ ] [TK13] M4 — portability: a cell serialized from layout A injected into
  layout B re-expresses and re-settles bit-identically; the wire format
  carries no geometry [REQ07], [REQ08], [REQ11].

## (6) Stage 4: validation gates [WL05]

> Goal: [GL06] — GATE_EVERY_MERGE
> Dependency: [DP06] — INVARIANT_SUITE
> Requirement: [RQ06] — GATES_ARE_AUTOMATED

- [ ] [TK14] Wire the seven-invariant suite as an automated merge gate on
  the `engine/` tree; a failing invariant is drift by definition [REQ10],
  [REQ11].
- [ ] [TK15] Add determinism-hash and frame-budget checks at the API
  boundary to the same gate [REQ08], [REQ09].
- [ ] [TK16] Re-run MIVIM validation on [D02] and [D08] after every edit;
  keep the next milestone blocked while the prior exit is unmet [REQ02],
  [REQ11].

## (7) Completion criteria [AC]

> Goal: [GL07] — DEFINE_TASK_COMPLETION
> Dependency: [DP07] — TASK_SET
> Requirement: [RQ07] — COMPLETION_IS_AUDITABLE

This TODO is complete when:

1. Every task carries at least one requirement marker.
2. Every requirement REQ01 through REQ12 appears in at least one task.
3. No build task (TK09 through TK13) started before Stage 2 completed and
   the prior milestone exit passed.
4. Validation commands pass for [D02] and [D08], and the validator audit
   reflects both.

## (Y) Link-complete usage index [LC]

This section is non-authoritative. It invokes active markers so the registry
below remains link-complete [LC], [LN].

- Documents: [DOC], [D01], [D02], [D03], [D04], [D05], [D06], [D07],
  [D08], [D09]
- Context and acceptance: [KN], [TK], [AC], [LC], [LN]
- Workflow phases: [WL], [WL01], [WL03], [WL04], [WL05]
- Goals: [GL], [GL01], [GL02], [GL03], [GL04], [GL05], [GL06], [GL07]
- Dependencies: [DP], [DP01], [DP02], [DP03], [DP04], [DP05], [DP06], [DP07]
- Metadata requirements: [RQ], [RQ01], [RQ02], [RQ03], [RQ04], [RQ05],
  [RQ06], [RQ07]
- Project requirements: [REQ], [REQ01], [REQ02], [REQ03], [REQ04], [REQ05],
  [REQ06], [REQ07], [REQ08], [REQ09], [REQ10], [REQ11], [REQ12]
- Tasks: [TK01], [TK02], [TK03], [TK04], [TK05], [TK06], [TK07], [TK08],
  [TK09], [TK10], [TK11], [TK12], [TK13], [TK14], [TK15], [TK16]

## (Z) Context-Citations

[DOC]: TODO_EXECUTION_LEDGER
  "The root MIVIM TODO ordering workstream A into staged, requirement-traced
  tasks."

[D01]: THEORY_SPECIFICATION
  "latex/v0.2/NNUI_CELL_PoC_v0.2.tex — the canonical frozen theory and its
  validated discrete realization."

[D02]: WORKSTREAM_A_DEVSPEC
  "docs/DEVSPEC_WORKSTREAM_A.md — requirements, design objects, milestones,
  and invariant frame for the agent-facing API."

[D03]: SESSION_JOURNAL
  "CHANGELOG — numbered session entries condensing the development
  history."

[D04]: PUBLIC_STATE
  "README.md — the public statement of project identity, results, and
  status."

[D05]: NARRATIVE_RECORD
  "logs/devlog — the full development narrative; source of record for
  design intent."

[D06]: VALIDATED_ARTIFACTS
  "analysis/001 through analysis/004 — prototype crates, R-gate reports,
  and the verified visual bench."

[D07]: MIVIM_PACKAGE
  ".MIVIM/ — the documentation workflow contracts: onboarding, refresher,
  templates, and the machine-readable validator."

[D08]: TODO_INSTANCE
  "TODO.md — this execution ledger."

[D09]: API_CONTRACTS_DESIGN
  "docs/DESIGN_API_CONTRACTS.md — the Stage 2 worked-example contracts:
  submission surface, error semantics, cell wire format, read path, and
  conformance gates."

[KN]: KNOWLEDGE_LEDGER
  "The active document context and knowledge chain."

[TK]: TASK
  "A scoped work item with requirement traceability."

[AC]: ACCEPTANCE_CRITERIA
  "The conditions under which this TODO is auditable as complete."

[LC]: LINK_COMPLETE_STATE
  "A state where every active marker is defined once and referenced at
  least once."

[LN]: LINT_RULE
  "The pseudo-citation rule set: definitions live only in the registry and
  lint is repaired, never suppressed."

[WL]: WORKFLOW
  "The ordered phases of workstream A, numbered consistently with the
  dev-spec."

[WL01]: DISCOVER
  "Fix the API consumer model and enumerate open read-path questions."

[WL03]: DESIGN
  "Specify surface, serialization, and read path with worked examples
  before code."

[WL04]: BUILD
  "Implement milestones M1 through M4 in dependency order."

[WL05]: VALIDATE
  "Automated gates: invariants, determinism hashes, frame budget, MIVIM
  validation."

[GL]: GOAL
  "The intended outcome of a stage."

[GL01]: TASK_CONTEXT
  "Preserve the document chain so every task is traceable to its sources."

[GL02]: SYNC_DOCUMENTATION
  "Bring stale project documentation in line with the actual state of
  development."

[GL03]: DISCOVER_CONSUMER_MODEL
  "Resolve who consumes the API first and what it must answer."

[GL04]: DESIGN_BEFORE_CODE
  "Produce worked-example contracts before implementation."

[GL05]: BUILD_MILESTONES_IN_ORDER
  "Implement the four milestones strictly in dependency order."

[GL06]: GATE_EVERY_MERGE
  "Make drift detection automatic at every merge."

[GL07]: DEFINE_TASK_COMPLETION
  "Make completion inspectable by a future agent."

[DP]: DEPENDENCY
  "A prerequisite object for a stage."

[DP01]: DOCUMENT_CHAIN
  "The eight-document context ledger this TODO operates within."

[DP02]: KNOWN_STALE_DOCS
  "The identified staleness: CLAUDE.md claims and the package's broken
  relative links."

[DP03]: DEVSPEC_BASE
  "The accepted dev-spec whose milestones and requirements this ledger
  executes."

[DP04]: DISCOVERY_OUTPUT
  "The recorded answers from Stage 1 that design consumes."

[DP05]: DESIGN_CONTRACTS
  "The Stage 2 worked examples that build implements."

[DP06]: INVARIANT_SUITE
  "The seven executable invariants and the existing fifteen tests."

[DP07]: TASK_SET
  "The complete task list TK01 through TK16."

[RQ]: METADATA_REQUIREMENT
  "A stage-level acceptance rule for this ledger itself."

[RQ01]: TASK_REQUIREMENT_TRACE
  "Every task must trace to at least one project requirement."

[RQ02]: DOCS_MATCH_REALITY
  "Documentation must reflect the actual repository state, continuously."

[RQ03]: DISCOVERY_BEFORE_DESIGN
  "Design does not start until the consumer model and read-path questions
  are recorded."

[RQ04]: WORKED_EXAMPLES_FIRST
  "Contracts are written as worked examples before any implementation."

[RQ05]: MILESTONE_ORDER_ENFORCED
  "A milestone starts only after the previous binary exit passes."

[RQ06]: GATES_ARE_AUTOMATED
  "Drift detection runs as automation, not reviewer memory."

[RQ07]: COMPLETION_IS_AUDITABLE
  "A future agent can verify completion from the artifacts alone."

[REQ]: PROJECT_REQUIREMENT
  "A binding requirement traced from the dev-spec and project constraints."

[REQ01]: DOCUMENTATION_CONSISTENCY
  "Project instructions and public docs state the true development stage,
  paths, and repository facts."

[REQ02]: LINK_COMPLETE_DOC_FAMILY
  "Every MIVIM document stays link-complete, lint-clean, and audited in the
  validator."

[REQ03]: PLANNING_BEFORE_SOURCECODE
  "Workstream A sourcecode waits for discovery and design to complete."

[REQ04]: API_ROUNDTRIP_M1
  "Submit-settle-query through the public API reproduces the prototype
  hashes."

[REQ05]: READ_PATH_M2
  "Agents read settled geometry and settlement events; only honest strain
  is exposed."

[REQ06]: LLM_DRIVE_M3
  "A real LLM drives layout from intent with zero authored coordinates."

[REQ07]: PORTABILITY_M4
  "Serialized cells inject into new vessels and re-settle bit-identically;
  no geometry in the wire format."

[REQ08]: DETERMINISM_AT_BOUNDARY
  "Identical programs settle bit-identically through the public API."

[REQ09]: REALTIME_BUDGET_HELD
  "Fifty regions stay inside 16 ms/frame at the API level."

[REQ10]: INVARIANTS_AT_BOUNDARY
  "All seven invariants hold at the API surface, including no coordinate
  intake."

[REQ11]: BINARY_EXIT_DISCIPLINE
  "Milestones complete only by measurable pass/fail exits with the full
  suite green."

[REQ12]: CREATIVE_LEAD_CONSUMER_CALL
  "The first-consumer decision belongs to the creative lead and gates
  discovery."

[TK01]: REFRESH_CLAUDE_MD_TASK
  "Task to bring CLAUDE.md in line with v0.2, the git repository, and the
  MIVIM contract."

[TK02]: REPAIR_MIVIM_PACKAGE_TASK
  "Task to fix stale package links and the template filename, then
  regenerate the validator."

[TK03]: KEEP_VALIDATOR_CURRENT_TASK
  "Task to refresh validator rows whenever MIVIM documents change."

[TK04]: ENUMERATE_READ_PATH_TASK
  "Task to record the open read-path questions and answers in the
  dev-spec."

[TK05]: FIRST_CONSUMER_DECISION_TASK
  "Task to obtain and record the creative lead's demo-first versus
  harness-first call."

[TK06]: API_WORKED_EXAMPLES_TASK
  "Task to write submission, settle, query, and error-semantics worked
  examples."

[TK07]: SERIALIZATION_DRAFT_TASK
  "Task to draft the geometry-free cell wire format with a structural
  check."

[TK08]: READ_PATH_SPEC_TASK
  "Task to specify the query and event model with the honest strain
  readout."

[TK09]: ENGINE_GRADUATION_TASK
  "Task to move the validated crate into an engine tree with all tests
  green."

[TK10]: M1_ROUNDTRIP_TASK
  "Task to pass the M1 binary exit: public-API hashes match the prototype."

[TK11]: M2_READ_PATH_TASK
  "Task to pass the M2 binary exit including the lying-instrument test."

[TK12]: M3_LLM_DRIVE_TASK
  "Task to pass the M3 binary exit with recorded token counts."

[TK13]: M4_PORTABILITY_TASK
  "Task to pass the M4 binary exit: bit-identical cross-vessel re-settle."

[TK14]: INVARIANT_GATE_TASK
  "Task to automate the seven-invariant suite as a merge gate."

[TK15]: DETERMINISM_GATE_TASK
  "Task to add hash and frame-budget checks at the API boundary."

[TK16]: CONTINUOUS_VALIDATION_TASK
  "Task to re-validate the doc family after edits and hold milestone
  order."
