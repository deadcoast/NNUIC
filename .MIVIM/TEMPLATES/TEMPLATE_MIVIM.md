# TEMPLATE_MIVIM: Blank Pseudo-Citation Document Template [A1]

This file is a link-complete blank template for building a context-dense
Markdown document that uses compact pseudo-citation markers in the body and one
source-of-truth registry at the bottom [TPL]. It is intentionally project
neutral: replace `{{TEMPLATE_TOKENS}}`, expand the numbered IDs that are truly
needed, remove sections that are not useful, and keep the Markdown reference
system lint-clean [LC].

Use this file when a new, unrelated project needs the same contextual knowledge
map pattern without inheriting domain-specific content from a prior project
[A1]. For operating instructions, pair it with the fresh-agent guide in
[ONBOARDING.md](../ONBOARDING.md#0-fresh-agent-bootstrap-a2) [A2] and the compact
session reminder in [REFRESHER.md](../REFRESHER.md#0-active-session-refresher-a3)
[A3].

## (0) Template role [TPL01]

> Goal: [GL01] — DEFINE_THE_DOCUMENT_OUTCOME
> Dependency: [DP01] — CONTEXT_CITATION_REGISTRY
> Requirement: [RQ01] — LINK_COMPLETE_STRUCTURE

This template exists to create a reusable pseudo-source document where the
upper body is a compact knowledge map [BD] and the bottom registry is the only
place that creates live reference definitions [RG]. The body may mention goals,
dependencies, requirements, constraints, workflows, sources, evidence, design
objects, and task outputs by short markers; the registry explains those markers
once with complete contextual definitions [FM].

Do not treat this file as a final populated document. Treat it as a controlled
starting point [TPL02]. Each section below contains population prompts that must
be replaced with project-specific substance before the document is considered
complete [RQ02].

## (1) Project identity [PRJ01]

> Goal: [GL02] — CAPTURE_PROJECT_IDENTITY
> Dependency: [DP02] — PROJECT_CONTEXT_INPUTS
> Requirement: [RQ02] — REPLACE_TEMPLATE_TOKENS

Populate the project identity before adding detailed theory, research, design,
or implementation content [PRJ]. A fresh agent should be able to understand the
project without external memory after this section is completed [WH].

| Field | Populate with project-specific content |
| --- | --- |
| Project name | `{{PROJECT_NAME}}` |
| Project type | `{{PROJECT_TYPE_OR_DOMAIN}}` |
| Primary owner or intended agent role | `{{OWNER_OR_AGENT_ROLE}}` |
| Primary outcome | `{{PRIMARY_OUTCOME}}` |
| Current lifecycle stage | `{{LIFECYCLE_STAGE}}` |
| Non-goals | `{{EXPLICIT_NON_GOALS}}` |
| Key constraints | `{{KEY_CONSTRAINTS}}` |

The project identity should name what is being built, what problem it solves,
what boundaries matter, and what the agent must not assume [WH01], [WH02],
[WH03].

## (2) Context intake [WH]

> Goal: [GL03] — BUILD_CONTEXTUAL_BOOTSTRAP
> Dependency: [DP03] — USER_SUPPLIED_CONTEXT
> Requirement: [RQ03] — WHO_WHAT_WHERE_WHY_HOW_COMPLETE

Use the WHO / WHAT / WHERE / WHY / HOW structure to make the document ingestible
for a fresh local CLI agent [WH]. Replace each token with dense, direct context.
Do not leave ambiguous prose that depends on conversation history [RQ04].

### (2.1) Who [WH01]

`{{WHO_IS_THE_DOCUMENT_FOR}}`

Name the expected reader, tool, agent, team, or system. State whether the file is
for a human, a local CLI agent, a build process, a research pass, or another
specific consumer [CLI].

### (2.2) What [WH02]

`{{WHAT_THE_PROJECT_OR_DOCUMENT_IS}}`

Define the project, artifact, system, theory, feature, or task. Include the
central object and the exact kind of completion being requested [PRJ].

### (2.3) Where [WH03]

`{{WHERE_THE_WORK_LIVES_OR_APPLIES}}`

State the repository path, document family, section boundaries, runtime context,
platform, environment, or scope of application [DP02].

### (2.4) Why [WH04]

`{{WHY_THE_WORK_MATTERS}}`

Explain the purpose, pressure, design reason, research reason, operational need,
or strategic outcome [GL].

### (2.5) How [WH05]

`{{HOW_THE_WORK_SHOULD_BE_DONE}}`

State the method: editing protocol, validation sequence, source requirements,
research standard, output format, and lint expectations [LN].

## (3) Goal, dependency, and requirement map [GL]

> Goal: [GL04] — MAP_SECTION_OBJECTIVES
> Dependency: [DP04] — TRACEABLE_METADATA
> Requirement: [RQ04] — SECTION_METADATA_COMPLETE

Every substantial section should carry local metadata that references existing
or newly created registry entries [DP04]. The metadata must invoke IDs; it must
not define them in the body [LN01].

Correct section metadata shape:

```md
> Goal: [GLnn] — SHORT_GOAL_LABEL
> Dependency: [DPnn] — SHORT_DEPENDENCY_LABEL
> Requirement: [RQnn] — SHORT_REQUIREMENT_LABEL
```

Incorrect section metadata shape:

```md
> [GLnn]: TODO ""
> [DPnn]: TODO ""
> [RQnn]: TODO ""
```

The incorrect shape creates live Markdown reference definitions outside the
registry and can trigger MD053 through duplicate or unused definitions [MD053].

## (4) Constraints, guardrails, and exclusions [CN]

> Goal: [GL05] — DEFINE_BOUNDARIES
> Dependency: [DP05] — KNOWN_LIMITS_AND_RISKS
> Requirement: [RQ05] — GUARDRAILS_ARE_EXPLICIT

Use constraints to state what must remain true [CN]. Use do-not markers for
negative rules that prevent drift [DN]. Use ignore markers only when a concept,
style, source, or direction must be excluded from active relevance [IG].

Populate the active boundary set:

- Required constraints: `{{REQUIRED_CONSTRAINTS}}` [CN01]
- Prohibited behaviors: `{{PROHIBITED_BEHAVIORS}}` [DN01]
- Ignored concepts or sources: `{{IGNORED_CONCEPTS_OR_SOURCES}}` [IG01]
- Risk notes: `{{KNOWN_RISKS_OR_FAILURE_MODES}}` [CN02]
- Acceptance threshold: `{{WHAT_COUNTS_AS_GOOD_ENOUGH}}` [CN03]

Constraints should be specific enough that another agent can enforce them
without asking for hidden context [RQ05].

## (5) Workflow roadmap [WL]

> Goal: [GL06] — PLAN_THE_WORKFLOW
> Dependency: [DP06] — ORDERED_PHASES
> Requirement: [RQ06] — WORKFLOW_HAS_PHASE_OWNERSHIP

Use workflow markers to describe the ordered path from context intake to final
output [WL]. Keep the phase model small unless the project truly needs more
states.

| Phase | Marker | Populate with concrete work |
| --- | --- | --- |
| Discover | [WL01] | `{{DISCOVERY_ACTIONS_AND_QUESTIONS}}` |
| Research | [WL02] | `{{RESEARCH_AND_VERIFICATION_ACTIONS}}` |
| Design | [WL03] | `{{DESIGN_DECISIONS_AND_ARTIFACTS}}` |
| Build | [WL04] | `{{IMPLEMENTATION_OR_AUTHORING_ACTIONS}}` |
| Validate | [WL05] | `{{LINT_TEST_REVIEW_AND_HANDOFF_ACTIONS}}` |

If the project does not need a phase, remove the body reference and its registry
entry, or redefine the phase honestly [LN02].

## (6) Research and evidence intake [RS]

> Goal: [GL07] — ATTACH_EVIDENCE_WITH_CONTEXT_WEIGHT
> Dependency: [DP07] — REAL_SOURCE_MATERIAL
> Requirement: [RQ07] — DO_NOT_FABRICATE_SOURCES

Use research-source entries only when an actual source exists [RS]. Do not
pre-create numbered research-source IDs as fake placeholders. Until a real
source exists, keep the source request as plain prose or a fenced template block
[DN01].

Source-record pattern for later population:

```md
[RSnn]: RESEARCH_SOURCE
  "TITLE: {{SOURCE_TITLE}}
  SOURCE_TYPE: {{PAPER_BOOK_DOCS_ARTICLE_REPO_OTHER}}
  AUTHOR_OR_ORG: {{AUTHOR_OR_ORG}}
  YEAR_OR_DATE: {{YEAR_OR_DATE}}
  LINK_OR_DOI: {{LINK_OR_DOI}}
  USED_FOR: {{WHAT_THIS_SOURCE_SUPPORTS}}
  CONTEXT_WEIGHT: {{LOW_MEDIUM_HIGH}}
  LINKED_CONTEXT: {{RELATED_MARKERS}}"
```

Use evidence markers to label support strength without pretending that a claim
is stronger than it is [EV].

- [EV01] for unverified working assumptions.
- [EV02] for plausible internal reasoning.
- [EV03] for externally supported claims.
- [EV04] for validated implementation or test results.

## (7) Design, tasks, and handoff [DS]

> Goal: [GL08] — TURN_CONTEXT_INTO_ACTIONABLE_OUTPUT
> Dependency: [DP08] — DECISIONS_AND_ARTIFACTS
> Requirement: [RQ08] — OUTPUT_IS_REVIEWABLE

Use design markers for stable architecture, document, interface, or proposal
objects [DS]. Use task markers for scoped work items that can be assigned,
implemented, or reviewed [TK]. Use output markers to define what the final agent
handoff must contain [OUT].

Populate the delivery structure:

- Design summary: `{{DESIGN_SUMMARY}}` [DS01]
- Interface or document structure: `{{STRUCTURE_OR_INTERFACE}}` [DS02]
- Acceptance criteria: `{{ACCEPTANCE_CRITERIA}}` [DS03]
- Task list: `{{TASKS_TO_COMPLETE}}` [TK01], [TK02], [TK03]
- Final handoff: `{{EXPECTED_FINAL_OUTPUT}}` [OUT01], [OUT02]

## (8) Population protocol for agents [TPL02]

> Goal: [GL09] — CONVERT_TEMPLATE_TO_LINK_COMPLETE_DOCUMENT
> Dependency: [DP09] — AGENT_EDITING_DISCIPLINE
> Requirement: [RQ09] — COMPLETE_WITHOUT_LINTER_SUPPRESSION

A local CLI agent should populate this template in this order [CLI]:

1. Replace the identity and context tokens before expanding technical details.
2. Keep body references as references only; never create `[ID]:` definitions
   above `(Z) Context-Citations` [LN01].
3. Create new numbered IDs only when a real concept exists [LN03].
4. Add each new active ID to the registry exactly once [LN04].
5. Add each active ID to the usage index or a meaningful body location [UI].
6. Remove any unused definitions instead of keeping ornamental tags [LN05].
7. Run markdownlint without suppressing MD053 [LN06].
8. Return the completed file with a concise statement of validation status
   [OUT].

## (9) Validation and lint protocol [LN]

> Goal: [GL10] — VALIDATE_LINK_COMPLETENESS
> Dependency: [DP10] — MARKDOWN_REFERENCE_RULES
> Requirement: [RQ10] — LINT_CLEAN_FINAL_STATE

Use shell-oriented validation before handoff:

```sh
# Find live reference definitions. They should appear only in the registry.
grep -nE '^\[[A-Za-z0-9]+\]:' TEMPLATE_MIVIM.md

# Find illegal blockquote definitions above the registry.
grep -nE '^>\s*\[[A-Za-z0-9]+\]:' TEMPLATE_MIVIM.md

# Find zero-number placeholder IDs.
grep -nE '\[[A-Z]{1,6}00\]' TEMPLATE_MIVIM.md

# Run lint without changing project configuration.
npx --yes markdownlint-cli2 TEMPLATE_MIVIM.md
```

Repair the document, not the linter configuration [RQ10].

## (Y) Link-complete usage index [LI]

This section is non-authoritative. It invokes every active pseudo-citation
marker in this template so the registry definitions below are needed by the
Markdown reference system [UI].

- Package and format markers: [A1], [A2], [A3], [DOC], [TPL], [TPL01],
  [TPL02], [TPL03], [FM], [BD], [RG], [UI], [LI], [LC], [CLI]
- Context markers: [WH], [WH01], [WH02], [WH03], [WH04], [WH05], [PRJ],
  [PRJ01], [PRJ02], [PRJ03]
- Goal markers: [GL], [GL01], [GL02], [GL03], [GL04], [GL05], [GL06], [GL07],
  [GL08], [GL09], [GL10]
- Dependency markers: [DP], [DP01], [DP02], [DP03], [DP04], [DP05], [DP06],
  [DP07], [DP08], [DP09], [DP10]
- Requirement markers: [RQ], [RQ01], [RQ02], [RQ03], [RQ04], [RQ05], [RQ06],
  [RQ07], [RQ08], [RQ09], [RQ10]
- Boundary markers: [CN], [CN01], [CN02], [CN03], [DN], [DN01], [IG], [IG01]
- Workflow and evidence markers: [WL], [WL01], [WL02], [WL03], [WL04],
  [WL05], [RS], [EV], [EV01], [EV02], [EV03], [EV04]
- Design, task, output, and lint markers: [DS], [DS01], [DS02], [DS03], [TK],
  [TK01], [TK02], [TK03], [OUT], [OUT01], [OUT02], [LN], [LN01], [LN02],
  [LN03], [LN04], [LN05], [LN06], [MD053]

## (Z) Context-Citations

[A1]: TEMPLATE_MIVIM
  "The reusable blank pseudo-citation template used to create a new
  context-dense, link-complete Markdown document for an unrelated project."

[A2]: AGENT_ONBOARDING
  "The companion onboarding guide that teaches a fresh local CLI agent how to
  understand, populate, lint, and repair this pseudo-citation format."

[A3]: ACTIVE_SESSION_REFRESHER
  "The compact reminder file used during an active session to steer an agent
  back to the core pseudo-citation operating rules."

[DOC]: DOCUMENT
  "The working document being authored from this template after project-specific
  tokens are replaced with real context."

[TPL]: TEMPLATE_MODEL
  "The blank but structurally complete source/citation model used as a starting
  point for a new document."

[TPL01]: TEMPLATE_ROLE
  "The section that states what the template is, how it should be used, and why
  it is not itself a populated final document."

[TPL02]: POPULATION_PROTOCOL
  "The ordered editing method for converting the blank template into a
  project-specific, link-complete document."

[TPL03]: TEMPLATE_TOKEN
  "A non-citation placeholder written with double braces, such as
  {{PROJECT_NAME}}, that must be replaced during population."

[FM]: FORMAT_MODEL
  "The pseudo-citation design where compact body markers point to complete
  bottom-registry definitions."

[BD]: BODY_LAYER
  "The upper document layer that acts as a compact contextual knowledge map and
  uses markers without defining them."

[RG]: REGISTRY_LAYER
  "The bottom source-of-truth layer where active markers are defined exactly
  once with Markdown reference-definition syntax."

[UI]: USAGE_INDEX
  "A non-authoritative section that invokes active markers so registry
  definitions remain needed by markdownlint."

[LI]: LINK_INDEX
  "The marker for the usage-index section that anchors active definitions
  without becoming a source of truth."

[LC]: LINK_COMPLETE_STATE
  "A document state where every active marker is defined once, referenced at
  least once, free of zero placeholders, and lint-clean."

[CLI]: LOCAL_CLI_AGENT
  "A local shell-capable editing agent such as Codex, Claude Code, or another
  repository-aware assistant."

[WH]: WHO_WHAT_WHERE_WHY_HOW
  "The contextual bootstrap structure that explains the intended reader, object,
  location, purpose, and method of the document."

[WH01]: WHO_CONTEXT
  "The template field that defines the intended reader, agent, team, or system
  responsible for consuming the document."

[WH02]: WHAT_CONTEXT
  "The template field that defines the project, artifact, system, feature,
  theory, or task being worked on."

[WH03]: WHERE_CONTEXT
  "The template field that defines the repository, environment, platform,
  section, or operational scope where the work applies."

[WH04]: WHY_CONTEXT
  "The template field that explains the purpose, pressure, strategic reason, or
  expected benefit behind the work."

[WH05]: HOW_CONTEXT
  "The template field that states the method, validation path, source standard,
  output format, and editing rules."

[PRJ]: PROJECT_CONTEXT
  "The project-specific identity and background context required before a fresh
  agent can make informed edits."

[PRJ01]: PROJECT_IDENTITY
  "The section that names the project, domain, owner role, outcome, lifecycle
  stage, non-goals, and constraints."

[PRJ02]: PROJECT_SCOPE
  "The boundary of what the populated document is allowed to affect, modify,
  assert, or deliver."

[PRJ03]: PROJECT_LIFECYCLE_STAGE
  "The current stage of the work, such as discovery, research, design,
  implementation, validation, or handoff."

[GL]: GOAL
  "The intended outcome of a section, subsection, task, request, theory object,
  validation pass, or handoff."

[GL01]: DEFINE_THE_DOCUMENT_OUTCOME
  "State the desired outcome of the populated document before expanding lower
  level content."

[GL02]: CAPTURE_PROJECT_IDENTITY
  "Capture the project identity so the document can stand on its own without
  hidden conversation context."

[GL03]: BUILD_CONTEXTUAL_BOOTSTRAP
  "Complete the who, what, where, why, and how fields so a fresh agent can
  ingest the document quickly."

[GL04]: MAP_SECTION_OBJECTIVES
  "Connect substantial sections to explicit goals, dependencies, and
  requirements."

[GL05]: DEFINE_BOUNDARIES
  "Make constraints, exclusions, and risk boundaries explicit enough for another
  agent to enforce."

[GL06]: PLAN_THE_WORKFLOW
  "Describe the ordered project phases from discovery through validation."

[GL07]: ATTACH_EVIDENCE_WITH_CONTEXT_WEIGHT
  "Connect real sources and evidence levels to claims without fabricating
  support."

[GL08]: TURN_CONTEXT_INTO_ACTIONABLE_OUTPUT
  "Convert contextual knowledge into design objects, tasks, acceptance criteria,
  and final handoff requirements."

[GL09]: CONVERT_TEMPLATE_TO_LINK_COMPLETE_DOCUMENT
  "Populate the template while preserving the reference-above and define-below
  pseudo-citation model."

[GL10]: VALIDATE_LINK_COMPLETENESS
  "Confirm that the completed document is internally linked, registry-complete,
  free of invalid placeholders, and lint-clean."

[DP]: DEPENDENCY
  "A prerequisite, support object, input, or component that must exist before a
  requirement can be satisfied."

[DP01]: CONTEXT_CITATION_REGISTRY
  "The bottom registry must exist because it stores the complete definitions for
  all active pseudo-citation markers."

[DP02]: PROJECT_CONTEXT_INPUTS
  "The project name, domain, owner role, outcome, stage, constraints, and
  non-goals needed to populate the document honestly."

[DP03]: USER_SUPPLIED_CONTEXT
  "The explicit context supplied by the user, repository, or source material
  before the agent expands the document."

[DP04]: TRACEABLE_METADATA
  "Section-level metadata is needed so goals, dependencies, and requirements can
  be traced from body sections to registry definitions."

[DP05]: KNOWN_LIMITS_AND_RISKS
  "Known constraints, prohibited behaviors, ignored directions, and failure modes
  that shape safe completion."

[DP06]: ORDERED_PHASES
  "The workflow depends on phase ownership so discovery, research, design,
  build, and validation do not collapse into one vague task."

[DP07]: REAL_SOURCE_MATERIAL
  "Research-source entries depend on actual sources rather than fabricated or
  future-intended citations."

[DP08]: DECISIONS_AND_ARTIFACTS
  "Design and handoff sections depend on concrete decisions, structures,
  criteria, and tasks."

[DP09]: AGENT_EDITING_DISCIPLINE
  "The agent must preserve registry-only definitions, usage-index coverage, and
  lint compatibility while editing."

[DP10]: MARKDOWN_REFERENCE_RULES
  "The format depends on Markdown reference-link behavior, including definition
  uniqueness and reference usage."

[RQ]: REQUIREMENT
  "A necessary condition, object, rule, output, or acceptance criterion required
  to satisfy the correlated goal."

[RQ01]: LINK_COMPLETE_STRUCTURE
  "The final document must define each active marker once, reference each active
  marker at least once, and keep definitions in the registry."

[RQ02]: REPLACE_TEMPLATE_TOKENS
  "Double-brace template tokens must be replaced with project-specific content
  before the document is treated as complete."

[RQ03]: WHO_WHAT_WHERE_WHY_HOW_COMPLETE
  "The contextual bootstrap must answer who, what, where, why, and how without
  relying on hidden conversation history."

[RQ04]: SECTION_METADATA_COMPLETE
  "Substantial sections should contain local goal, dependency, and requirement
  references instead of body-level definitions."

[RQ05]: GUARDRAILS_ARE_EXPLICIT
  "Constraints, do-not rules, ignore rules, risks, and acceptance thresholds must
  be stated directly."

[RQ06]: WORKFLOW_HAS_PHASE_OWNERSHIP
  "Each workflow phase should state what work it owns and what output it
  produces."

[RQ07]: DO_NOT_FABRICATE_SOURCES
  "A numbered research-source entry may be created only when a real source is
  available to describe."

[RQ08]: OUTPUT_IS_REVIEWABLE
  "The handoff must contain enough structure, criteria, and task detail for a
  reviewer or agent to judge completion."

[RQ09]: COMPLETE_WITHOUT_LINTER_SUPPRESSION
  "The file must be completed by repairing syntax and references, not by
  disabling markdownlint rules."

[RQ10]: LINT_CLEAN_FINAL_STATE
  "The final state should pass markdownlint without changing project
  configuration."

[CN]: CONSTRAINT
  "A rule, limit, boundary, or acceptance condition that must remain active
  during completion."

[CN01]: REQUIRED_CONSTRAINTS
  "Positive rules that must be preserved while the project document is edited or
  populated."

[CN02]: RISK_NOTES
  "Known failure modes, ambiguity risks, or technical hazards that should shape
  decisions."

[CN03]: ACCEPTANCE_THRESHOLD
  "The minimum state that counts as good enough for the current pass."

[DN]: DO_NOT
  "A negative rule that blocks drift, replication, unsafe assumptions, or
  unwanted behavior."

[DN01]: NO_FAKE_SOURCE_RECORDS
  "Do not create numbered research-source entries before a real source exists."

[IG]: IGNORE
  "A contextual exclusion marker used to remove a concept, source, style, or
  task from active relevance."

[IG01]: IGNORED_CONCEPT_SET
  "The populated set of concepts, sources, or directions that should be ignored
  for the current document."

[WL]: WORKFLOW
  "The ordered roadmap of phases used to move from intake through final
  validation."

[WL01]: DISCOVER
  "The phase that gathers context, unknowns, problem framing, and initial
  boundaries."

[WL02]: RESEARCH
  "The phase that verifies claims, attaches real sources, compares alternatives,
  and adjusts assumptions."

[WL03]: DESIGN
  "The phase that converts context and evidence into structure, architecture,
  requirements, or proposal objects."

[WL04]: BUILD
  "The phase that implements, authors, edits, or otherwise produces the target
  artifact."

[WL05]: VALIDATE
  "The phase that checks lint, completeness, consistency, acceptance criteria,
  and final handoff readiness."

[RS]: RESEARCH_SOURCE
  "A real external source entry such as a paper, documentation page, book,
  repository, article, dataset, or implementation note."

[EV]: EVIDENCE_LEVEL
  "A label that communicates how strongly a claim is supported."

[EV01]: ASSUMPTION
  "A working assumption that has not yet been verified."

[EV02]: INTERNAL_REASONING
  "A claim supported by internal logic, consistency, or derived reasoning but
  not yet external evidence."

[EV03]: EXTERNAL_SUPPORT
  "A claim supported by one or more real external sources."

[EV04]: VALIDATED_RESULT
  "A claim supported by implementation behavior, tests, measurements, review, or
  another concrete validation result."

[DS]: DESIGN_OBJECT
  "A design-level object such as architecture, interface, proposal, structure,
  artifact, or acceptance model."

[DS01]: DESIGN_SUMMARY
  "The concise design explanation that states what will be built or changed."

[DS02]: STRUCTURE_OR_INTERFACE
  "The document structure, module boundary, API, interface, or object layout
  used by the design."

[DS03]: ACCEPTANCE_CRITERIA
  "The conditions used to judge whether the design or output satisfies the
  stated requirements."

[TK]: TASK
  "A scoped work item that can be assigned, completed, validated, or deferred."

[TK01]: PRIMARY_TASK
  "The first major task needed to move the populated document forward."

[TK02]: SUPPORTING_TASK
  "A secondary task that supports the primary task or completes a required
  dependency."

[TK03]: VALIDATION_TASK
  "A task dedicated to linting, testing, review, or acceptance confirmation."

[OUT]: OUTPUT
  "The final artifact, response, handoff, or result expected from the agent."

[OUT01]: FINAL_ARTIFACT
  "The completed document, file, code, design, or deliverable produced by the
  work."

[OUT02]: VALIDATION_SUMMARY
  "A concise statement of lint status, link-completeness, major changes, and any
  unresolved limits."

[LN]: LINT_RULE
  "A validation rule used to keep the pseudo-citation system internally
  consistent and markdownlint-compatible."

[LN01]: REGISTRY_ONLY_DEFINITIONS
  "Live bracket-colon reference definitions should appear only in the bottom
  Context-Citations registry."

[LN02]: REMOVE_UNUSED_PHASES
  "If a template phase is not needed, remove or redefine it instead of leaving
  ornamental references."

[LN03]: CREATE_IDS_ONLY_FOR_REAL_CONCEPTS
  "Create a numbered ID only when the concept exists in the document and needs a
  reusable context handle."

[LN04]: DEFINE_EACH_ACTIVE_ID_ONCE
  "Each active marker must have exactly one registry definition."

[LN05]: REMOVE_UNUSED_DEFINITIONS
  "Unused registry entries should be referenced, repurposed, or removed instead
  of being kept as noise."

[LN06]: DO_NOT_SUPPRESS_MD053
  "Repair duplicate, unused, or accidental reference definitions rather than
  silencing markdownlint."

[MD053]: LINK_IMAGE_REFERENCE_DEFINITION_FAILURE
  "A markdownlint failure commonly caused by duplicate definitions, unused
  definitions, or body text accidentally shaped like a reference definition."
