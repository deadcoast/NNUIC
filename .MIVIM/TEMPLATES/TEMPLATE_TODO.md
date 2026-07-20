# (A) MIVIM TODO: SPINALTAP CONSTRAINT ENGINE DOCSPEC [DOC]

This TODO is the third [WL04] docspec artifact. It turns requirements and
design into ordered work without starting runtime sourcecode [D04], [D05],
[WL05].

Every task must trace to at least one requirement. A task without a parent
requirement is not valid work for this phase [RQ01].

## (1) Context ledger [KN]

> Goal: [GL01] - TASK_CONTEXT
> Dependency: [DP01] - DOCSPEC_PACKAGE
> Requirement: [RQ01] - TASK_REQUIREMENT_TRACE

Task context:

| Document | Role |
| --- | --- |
| [D01] | method and guardrails |
| [D02] | evidence and decisions |
| [D03] | proposal scope and modules |
| [D04] | requirements |
| [D05] | design |
| [D06] | TODO |

## (2) Stage 1: docspec alignment [TK]

> Goal: [GL02] - ALIGN_DOCSPEC
> Dependency: [DP02] - REQUIREMENTS_AND_DESIGN
> Requirement: [RQ02] - DOCSPEC_IS_CONSISTENT

- [x] [TK01] Verify that [D04], [D05], and [D06] all preserve the document
  chain [REQ01].
- [x] [TK02] Confirm the work remains a research prototype architecture and
  sourcecode remains blocked until docspec validation passes [REQ02], [REQ03].
- [x] [TK03] Confirm each design module [MG01] through [MG06] has a matching
  requirement [REQ04], [REQ05], [REQ06], [REQ07], [REQ08], [REQ09].

## (3) Stage 2: module contract refinement [TK]

> Goal: [GL03] - REFINE_CONTRACTS
> Dependency: [DP03] - MODULE_SET
> Requirement: [RQ03] - CONTRACTS_ARE_IMPLEMENTABLE

- [x] [TK04] Expand [DC01] with exact fields for source authority, claim fit,
  originality distance, validation reach, influence weight, and risk flag
  [REQ04].
- [x] [TK05] Expand [DC02] with exact fields for elapsed time, fixed packet
  size, packet count, accumulator remainder, interpolation state, and overflow
  risk [REQ05].
- [x] [TK06] Expand [DC03] with exact fields for owner, channel, ordered
  signal bundle, constraint envelope, validation hook, and residual risk
  [REQ06].
- [x] [TK07] Expand [DC04] with exact fields for local state slice, constraint
  set, time packet, bounded delta, and residual risk [REQ07].
- [x] [TK08] Expand [DC05] with exact fields for geometry, lighting, texture,
  color, fog, alpha, depth, camera, animation cadence, and artifact limits
  [REQ08].
- [x] [TK09] Expand [DC06] with exact fields for claim, specimen plan, failure
  path, promotion state, and evidence tier [REQ09].

## (4) Stage 3: validation specimen planning [TK]

> Goal: [GL04] - PLAN_VALIDATION_SPECIMENS
> Dependency: [DP04] - VALIDATION_GATES
> Requirement: [RQ04] - SPECIMENS_BEFORE_SOURCECODE

- [x] [TK10] Define the [VG01] originality specimen: one source-pressured claim
  is scored and either allowed, down-weighted, or blocked [REQ10].
- [x] [TK11] Define the [VG02] time specimen: raw elapsed time becomes fixed
  packets and solvers receive no raw dt [REQ11].
- [x] [TK12] Define the [VG03] signal specimen: one bounded delta routes
  through a signal bundle before state publication [REQ12].
- [x] [TK13] Define the [VG04] solver specimen: a vertebra refuses state
  outside its local slice [REQ13].
- [x] [TK14] Define the [VG05] style specimen: a style profile becomes
  measurable limits [REQ14].
- [x] [TK15] Define the [VG06] proof specimen: a claim cannot advance without
  specimen plan and failure path [REQ15].

## (5) Stage 4: future source planning gate [TK]

> Goal: [GL05] - PREPARE_SOURCE_PLAN_GATE
> Dependency: [DP05] - VALIDATED_DOCSPEC
> Requirement: [RQ05] - SOURCE_PLAN_REQUIRES_EVIDENCE

- [x] [TK16] Re-run link-completeness and markdownlint for [D04], [D05], and
  [D06] [REQ03].
- [x] [TK17] Write a sourcecode implementation plan only after [TK01] through
  [TK16] are complete [REQ03], [REQ15].
- [x] [TK18] Keep [WL05] blocked if any requirement lacks a design owner,
  contract, validation gate, or task [REQ01].

Output: `docs/sourcecode_plan.md`. Traceability audit found no requirement
without owner, contract, validation gate, or task. [WL05] remains not started
pending explicit approval [REQ03].

## (6) Completion criteria [AC]

> Goal: [GL06] - DEFINE_TASK_COMPLETION
> Dependency: [DP06] - TASK_SET
> Requirement: [RQ06] - COMPLETION_IS_AUDITABLE

This TODO is ready for execution when:

1. Every task has a requirement marker.
2. Every requirement from [D04] appears in at least one task.
3. No task starts runtime sourcecode.
4. Validation commands pass for [D04], [D05], and [D06].

## (Y) Link-complete usage index [LC]

This section is non-authoritative. It invokes active markers so the registry
below remains link-complete [LC], [LN].

- Documents: [DOC], [D01], [D02], [D03], [D04], [D05], [D06], [WL04], [WL05]
- Context and acceptance: [KN], [TK], [AC], [LC], [LN]
- Modules and contracts: [MG01], [MG02], [MG03], [MG04], [MG05], [MG06],
  [DC01], [DC02], [DC03], [DC04], [DC05], [DC06]
- Validation: [VG01], [VG02], [VG03], [VG04], [VG05], [VG06]
- Requirements: [REQ01], [REQ02], [REQ03], [REQ04], [REQ05], [REQ06], [REQ07],
  [REQ08], [REQ09], [REQ10], [REQ11], [REQ12], [REQ13], [REQ14], [REQ15]
- Tasks: [TK01], [TK02], [TK03], [TK04], [TK05], [TK06], [TK07], [TK08],
  [TK09], [TK10], [TK11], [TK12], [TK13], [TK14], [TK15], [TK16], [TK17],
  [TK18]
- Goals: [GL01], [GL02], [GL03], [GL04], [GL05], [GL06]
- Dependencies: [DP01], [DP02], [DP03], [DP04], [DP05], [DP06]
- Requirement metadata: [RQ01], [RQ02], [RQ03], [RQ04], [RQ05], [RQ06]

## (Z) Context-Citations

[DOC]: TODO_DOCUMENT
  "The WL04 TODO artifact for the SPINALTAP Constraint Engine docspec."

[D01]: RESEARCH_PLAN_DOCUMENT
  "docs/research_plan.md, the approved method and guardrail source."

[D02]: LIVE_RESEARCH_DOCUMENT
  "docs/live_research.md, the evidence and decision ledger."

[D03]: PROPOSAL_DOCUMENT
  "docs/proposal.md, the proposal source."

[D04]: REQUIREMENTS_DOCUMENT
  "docs/requirements.md, the companion requirements document."

[D05]: DESIGN_DOCUMENT
  "docs/design.md, the companion design document."

[D06]: TODO_DOCUMENT_INSTANCE
  "docs/TODO.md, the current TODO document."

[WL04]: DEVELOPMENT_DOCSPEC
  "The workflow phase for requirements, design, and TODO artifacts."

[WL05]: DEVELOPMENT_SOURCECODE
  "The blocked sourcecode phase."

[KN]: KNOWLEDGE_LEDGER
  "The active document context and knowledge chain."

[TK]: TASK
  "A scoped docspec work item with requirement traceability."

[AC]: ACCEPTANCE_CRITERIA
  "The conditions for TODO readiness and later execution."

[LC]: LINK_COMPLETE_STATE
  "A state where every active marker is defined once and referenced."

[LN]: LINT_RULE
  "The pseudo-citation rule that definitions live in the registry."

[MG01]: CONTEXT_WEIGHT_GOVERNOR_MODULE
  "Module for source influence and replication-risk scoring."

[MG02]: TIME_PACKET_PUMP_MODULE
  "Module for converting elapsed time into fixed simulation packets."

[MG03]: SIGNAL_CONSTRAINT_SPINE_MODULE
  "Module for routing ordered signal bundles."

[MG04]: VERTEBRA_DELTA_WORKER_MODULE
  "Module family for local solvers and bounded deltas."

[MG05]: STYLE_CONSTRAINT_SURFACE_MODULE
  "Module for measurable style limits."

[MG06]: SOFT_PROOF_LAB_MODULE
  "Module for specimen plans and failure paths."

[DC01]: CONTEXT_WEIGHT_CONTRACT
  "Contract for source influence and replication risk."

[DC02]: TIME_PACKET_CONTRACT
  "Contract for fixed simulation packets and interpolation state."

[DC03]: SIGNAL_BUNDLE_CONTRACT
  "Contract for ordered signal bundles."

[DC04]: VERTEBRA_DELTA_CONTRACT
  "Contract for local state, constraints, bounded delta, and residual risk."

[DC05]: STYLE_LIMIT_CONTRACT
  "Contract for measurable style limits."

[DC06]: PROOF_SPECIMEN_CONTRACT
  "Contract for specimen plan, failure path, and promotion state."

[VG01]: ORIGINALITY_GATE
  "Gate requiring source influence and originality review."

[VG02]: TIME_GATE
  "Gate requiring fixed packet use."

[VG03]: SIGNAL_GATE
  "Gate requiring signal bundle routing."

[VG04]: SOLVER_GATE
  "Gate requiring local solver ownership."

[VG05]: STYLE_GATE
  "Gate requiring measurable style limits."

[VG06]: PROOF_GATE
  "Gate requiring specimen and failure path."

[REQ01]: ENGINE_IDENTITY_REQUIREMENT
  "Requirement that the project preserve SPINALTAP identity and traceability."

[REQ02]: RESEARCH_PROTOTYPE_REQUIREMENT
  "Requirement that the first target remain a research prototype architecture."

[REQ03]: SOURCECODE_BLOCK_REQUIREMENT
  "Requirement that sourcecode wait until docspec validation."

[REQ04]: CONTEXT_WEIGHT_REQUIREMENT
  "Requirement for context-weight influence and risk output."

[REQ05]: TIME_PACKET_REQUIREMENT
  "Requirement for fixed time packets before solver use."

[REQ06]: SIGNAL_SPINE_REQUIREMENT
  "Requirement for state mutation through signal bundles."

[REQ07]: VERTEBRA_REQUIREMENT
  "Requirement for local solver ownership and bounded deltas."

[REQ08]: STYLE_CONSTRAINT_REQUIREMENT
  "Requirement for measurable style limits."

[REQ09]: SOFT_PROOF_REQUIREMENT
  "Requirement for specimen plans and failure paths."

[REQ10]: ORIGINALITY_VALIDATION_REQUIREMENT
  "Requirement for high-influence source originality review."

[REQ11]: TIME_VALIDATION_REQUIREMENT
  "Requirement for solver-facing fixed time packets."

[REQ12]: SIGNAL_VALIDATION_REQUIREMENT
  "Requirement for signal-bundled state mutation."

[REQ13]: SOLVER_VALIDATION_REQUIREMENT
  "Requirement for local state ownership."

[REQ14]: STYLE_VALIDATION_REQUIREMENT
  "Requirement for measurable style claims."

[REQ15]: PROOF_VALIDATION_REQUIREMENT
  "Requirement for runnable specimen plans and failure paths."

[TK01]: VERIFY_DOCUMENT_CHAIN_TASK
  "Task to verify that requirements, design, and TODO preserve document
  chain."

[TK02]: CONFIRM_SOURCE_BLOCK_TASK
  "Task to confirm sourcecode remains blocked."

[TK03]: CONFIRM_MODULE_REQUIREMENTS_TASK
  "Task to confirm each module has a matching requirement."

[TK04]: EXPAND_CONTEXT_WEIGHT_CONTRACT_TASK
  "Task to expand the context-weight contract."

[TK05]: EXPAND_TIME_PACKET_CONTRACT_TASK
  "Task to expand the time-packet contract."

[TK06]: EXPAND_SIGNAL_CONTRACT_TASK
  "Task to expand the signal-bundle contract."

[TK07]: EXPAND_VERTEBRA_CONTRACT_TASK
  "Task to expand the vertebra contract."

[TK08]: EXPAND_STYLE_CONTRACT_TASK
  "Task to expand the style-limit contract."

[TK09]: EXPAND_PROOF_CONTRACT_TASK
  "Task to expand the proof-specimen contract."

[TK10]: ORIGINALITY_SPECIMEN_TASK
  "Task to define the originality validation specimen."

[TK11]: TIME_SPECIMEN_TASK
  "Task to define the time validation specimen."

[TK12]: SIGNAL_SPECIMEN_TASK
  "Task to define the signal validation specimen."

[TK13]: SOLVER_SPECIMEN_TASK
  "Task to define the solver validation specimen."

[TK14]: STYLE_SPECIMEN_TASK
  "Task to define the style validation specimen."

[TK15]: PROOF_SPECIMEN_TASK
  "Task to define the proof validation specimen."

[TK16]: VALIDATE_DOCSPEC_TASK
  "Task to rerun lint and link-completeness checks."

[TK17]: WRITE_SOURCE_PLAN_TASK
  "Task to write a future sourcecode implementation plan after docspec
  validation."

[TK18]: KEEP_SOURCE_BLOCKED_TASK
  "Task to keep sourcecode blocked if traceability is incomplete."

[GL01]: TASK_CONTEXT
  "Preserve task context and document chain."

[GL02]: ALIGN_DOCSPEC
  "Align requirements, design, and TODO."

[GL03]: REFINE_CONTRACTS
  "Refine module contracts before sourcecode."

[GL04]: PLAN_VALIDATION_SPECIMENS
  "Plan validation specimens before sourcecode."

[GL05]: PREPARE_SOURCE_PLAN_GATE
  "Prepare the gate for a future source plan."

[GL06]: DEFINE_TASK_COMPLETION
  "Define auditable completion criteria."

[DP01]: DOCSPEC_PACKAGE
  "The requirements, design, and TODO package."

[DP02]: REQUIREMENTS_AND_DESIGN
  "The upstream requirements and design documents."

[DP03]: MODULE_SET
  "The module set requiring contract refinement."

[DP04]: VALIDATION_GATES
  "The validation gates requiring specimen planning."

[DP05]: VALIDATED_DOCSPEC
  "The validated docspec package needed before source planning."

[DP06]: TASK_SET
  "The complete task set."

[RQ01]: TASK_REQUIREMENT_TRACE
  "Every task must trace to a requirement."

[RQ02]: DOCSPEC_IS_CONSISTENT
  "Requirements, design, and TODO must stay consistent."

[RQ03]: CONTRACTS_ARE_IMPLEMENTABLE
  "Contracts must be concrete enough for implementation planning."

[RQ04]: SPECIMENS_BEFORE_SOURCECODE
  "Validation specimens must be planned before sourcecode."

[RQ05]: SOURCE_PLAN_REQUIRES_EVIDENCE
  "Source planning requires validated docspec evidence."

[RQ06]: COMPLETION_IS_AUDITABLE
  "Completion criteria must be inspectable by a future agent."
