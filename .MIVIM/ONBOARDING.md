# AGENT_ONBOARDING: How to utilize the Documents Proprietary Citation Format [AO], [A2]

This file is for a fresh local CLI agent that has no prior knowledge of the
user, repository, project history, or pseudo-citation format [CLI]. It explains
how the package works, how to populate the blank template, how to repair common
lint failures, how to read, preserve, extend, lint, and repair the current
pseudo-citation system used by the 4GINE and how to hand work back without
weakening the design/ SPINALTAP document family [A2],[PJ].

## (0) Fresh-agent bootstrap [A2]

> Goal: [GL01] — INGEST_THE_FORMAT_WITHOUT_PRIOR_CONTEXT
> Dependency: [DP01] — PACKAGE_FILE_ROLES
> Requirement: [RQ01] — PRESERVE_LINK_COMPLETE_FORMAT

The package has three files [PKG]:

- Blank template: [TEMPLATE_MIVIM.md](TEMPLATES/TEMPLATE_MIVIM.md#0-template-role-tpl01)
  [A1]
- Full onboarding guide: [ONBOARDING.md](ONBOARDING.md#0-fresh-agent-bootstrap-a2)
  [A2]
- Active-session reminder: [REFRESHER.md](REFRESHER.md#0-active-session-refresher-a3)
  [A3]

The agent should use the template when creating or rewriting a project document,
use this onboarding file when it needs full operating context, and use the
refresher when it only needs a compact steering reminder during an active
session [OP].

## (1) Who, what, where, why, how [WH]

> Goal: [GL02] — ESTABLISH_AGENT_CONTEXT
> Dependency: [DP02] — ZERO_PRIOR_MEMORY_ASSUMPTION
> Requirement: [RQ02] — EXPLAIN_THE_SYSTEM_EXPLICITLY

The expected reader is a fresh CLI agent with repository access but no project
memory. The agent may be asked to complete tags, add sections, normalize IDs,
repair lint errors, create related documents, or convert unfinished drafts into
link-complete form [CLI].

The project owner is designing a compact pseudo-sourcing format that borrows
from Markdown reference-link mechanics but repurposes them as contextual source
maps [FM]. The user is not asking for generic footnotes, generic academic
citations, or a new project config.

### (1.1) Who [WHO]

The intended reader is a local CLI agent, not a human browsing a normal
reference manual [CLI]. The agent may be asked to create a new document from
[TEMPLATE_MIVIM.md](TEMPLATES/TEMPLATE_MIVIM.md#0-template-role-tpl01), audit an
existing pseudo-cited document, repair MD053, expand tags, normalize IDs, or
complete the source-of-truth registry [A1].

The agent must assume it has no project memory. It should not rely on previous
conversation turns, previous drafts, or hidden context unless those materials
are present in the repository or explicitly supplied in the active prompt [DP02].

### (1.2) What [WHAT]

The pseudo-citation format is a Markdown-compatible contextual sourcing system
[FM]. It uses compact markers in the body, such as `[GL01]`, `[DP03]`, `[RQ04]`,
`[WL02]`, or `[RS]`, and defines each active marker once in a bottom registry
named `(Z) Context-Citations` [RG].

The markers act like contextual knowledge handles. They are not decorative tags,
ordinary hashtags, or loose labels [OP01]. When the body invokes a marker, the
registry provides the full meaning, source-of-truth description, and intended
context weight [OP02].

The format is a structured Markdown pattern where compact markers such as goal,
dependency, requirement, workflow, theory, style, evidence, and module IDs are
referenced throughout the body and defined once in a bottom registry [NS].

A marker is a compact context handle:

```md
The theory must remain technically plausible [TP] and avoid source replication
[DN01].
```

The registry entry is the full definition:

```md
[TP]: TECHNICALLY_PLAUSIBLE
  "A creative theory standard that permits novelty while rejecting incoherent,
  unbounded, or non-actionable speculation."
```

### (1.3) Where [WHERE]

References may appear throughout the upper body: headings, paragraphs,
blockquote metadata, tables, lists, and usage indexes [BD]. Definitions belong
only in the bottom registry [RG].

The agent must not create live Markdown reference definitions above the registry
[VAL01]. A line shaped like `[ID]: DESTINATION "title"` is a definition, not a
harmless note. If it appears in body metadata, markdownlint can treat it as a
duplicate, unused, or accidental reference definition [MD053].

Markers appear anywhere above the registry as references: headings, paragraphs,
lists, local metadata, design notes, usage indexes, or workflow summaries [BD].

Definitions appear only in the bottom section named `(Z) Context-Citations`
[RG]. Do not create live reference-definition lines above that registry [LN01].

### (1.4) Why [WHY]

The format compresses repeated context while preserving traceability [FM]. The
upper document stays readable and compact; the bottom registry carries the full
definitions, requirements, dependencies, source notes, and lint rules [RG].

The design also lets the user preserve a pseudo-academic source/citation style
without changing project markdownlint configuration [MD]. The correct repair for
format errors is to fix the document structure, not to silence the linter
[VAL08].

The format compresses repeated context. It lets the upper document stay small
while the bottom registry carries the full contextual weight, intent, and
source-of-truth definitions [CW].

The design also keeps the document compatible with markdownlint without changing
project lint configuration [MD]. It intentionally uses Markdown reference-link
syntax, so the system must respect Markdown reference-definition rules [DP01].

### (1.5) How [HOW]

The operating method is:

```txt
reference above; define once below; usage-index all active markers; lint clean
```

Correct body metadata references definitions:

```md
> Goal: [GL01] — INGEST_THE_FORMAT_WITHOUT_PRIOR_CONTEXT
> Dependency: [DP01] — PACKAGE_FILE_ROLES
> Requirement: [RQ01] — PRESERVE_LINK_COMPLETE_FORMAT
```

Correct registry definitions create definitions:

```md
[GL01]: INGEST_THE_FORMAT_WITHOUT_PRIOR_CONTEXT
  "Give a fresh local CLI agent enough context to understand the package
  without prior project memory."
```

Do not write draft body metadata as definitions:

```md
> [GLXX]: TODO ""
> [DPXX]: TODO ""
> [RQXX]: TODO ""
```

That draft shape violates the current link-complete format [FIX01].

## (2) File role model [PKG]

> Goal: [GL03] — DISTINGUISH_PACKAGE_FILES
> Dependency: [DP03] — FILE_ROLE_SEPARATION
> Requirement: [RQ03] — USE_THE_RIGHT_FILE_FOR_THE_RIGHT_TASK

### (2.1) TEMPLATE_MIVIM.md [A1]

`TEMPLATE_MIVIM.md` is the reusable blank specimen [A1]. It contains the
body pattern, local metadata pattern, source-record pattern, workflow scaffold,
usage index, and bottom registry. It is designed to be copied or populated for
an unrelated project without carrying domain-specific theory from earlier work.

Use it when the task is to create a new pseudo-cited document, convert a rough
note into the format, or provide a blank structure for another agent [POP01].

### (2.2) ONBOARDING.md [A2]

`ONBOARDING.md` is the complete ingestion guide [A2]. It explains the format,
file roles, CLI workflow, MD053 behavior, link-complete rules, and repair
patterns. Use it when an agent is new, confused, repairing structural errors, or
about to perform a broad completion pass [ING].

### (2.3) REFRESHER.md [A3]

`REFRESHER.md` is a short active-session reminder [A3]. It does not replace the
full onboarding file. Use it when the agent already understands the system but
needs a compact steering note that links back to the key headers in this file
and the template [REF].

## (3) Core format model [FM]

> Goal: [GL04] — EXPLAIN_FORMAT_MECHANICS
> Dependency: [DP04] — MARKDOWN_REFERENCE_SYNTAX
> Requirement: [RQ04] — KEEP_BODY_AND_REGISTRY_SEPARATE

### (3.1) Body layer [BD]

The body layer is the upper document [BD]. It carries readable project content
and invokes compact context markers. It may contain section metadata, task
lists, technical prose, tables, source plans, and design decisions.

A correct body invocation is simple:

```md
The project must preserve link-complete pseudo-citations [LC] and repair lint
failures without changing config [VAL08].
```

### (3.2) Registry layer [RG]

The registry layer is the bottom source-of-truth section named `(Z)
Context-Citations` [RG]. It defines every active marker once.

Preferred registry shape:

```md
[ID]: CATEGORY_LABEL
  "Complete contextual definition."
```

`ID` is the marker, `CATEGORY_LABEL` is the compact destination label, and the
quoted title is the complete contextual definition [OP02].

### (3.3) Usage index [UI]

The usage index is a non-authoritative section placed before the registry [UI].
It invokes every active marker at least once so markdownlint does not treat
registry entries as unused [VAL04].

The usage index is not the source of truth. It must not define markers. It only
references them [VAL01].

### (3.4) Link-complete state [LC]

A document is link-complete when every active marker used in the body resolves
to exactly one registry entry, every active registry entry is referenced at
least once, no active zero-number placeholder IDs remain, definitions appear
only in the registry, and markdownlint passes under the existing configuration
[LC].

## (4) Operating mechanics [OP]

> Goal: [GL05] — EXECUTE_THE_FORMAT_CORRECTLY
> Dependency: [DP05] — AGENT_DISCIPLINE
> Requirement: [RQ05] — DO_NOT_INVENT_FRIVOLOUS_STRUCTURE

The agent should preserve the current design rather than inventing unnecessary
new syntax [OP]. Enhancements are acceptable only when they improve traceability,
reduce ambiguity, repair lint behavior, or make agent ingestion more reliable
[OP05].

Use stable namespaces [OP03]. Examples include goals `[GL]`, dependencies
`[DP]`, requirements `[RQ]`, workflow `[WL]`, research sources `[RS]`, evidence
levels `[EV]`, design objects `[DS]`, tasks `[TK]`, do-not rules `[DN]`, ignore
rules `[IG]`, lint rules `[LN]`, and package files `[A1]`, `[A2]`, `[A3]`.

Do not add a namespace just because a word could be abbreviated. Add one only
when the document needs a reusable context family that existing namespaces do
not represent [OP04].

## (5) Populating TEMPLATE_MIVIM.md [POP]

> Goal: [GL06] — POPULATE_TEMPLATE_WITH_REAL_CONTEXT
> Dependency: [DP06] — TEMPLATE_TOKEN_REPLACEMENT
> Requirement: [RQ06] — NO_FAKE_COMPLETION

When using [TEMPLATE_MIVIM.md](TEMPLATES/TEMPLATE_MIVIM.md#0-template-role-tpl01),
perform this sequence [POP]:

1. Replace identity tokens such as `{{PROJECT_NAME}}`, `{{PRIMARY_OUTCOME}}`,
   and `{{KEY_CONSTRAINTS}}` before expanding detailed content [POP01].
2. Complete the WHO / WHAT / WHERE / WHY / HOW fields so the document can stand
   alone [POP02].
3. Keep local metadata in reference form, not definition form [POP03].
4. Create numbered IDs only for concepts that really exist in the populated
   document [POP04].
5. Add real research-source entries only when actual sources exist [POP05].
6. Update the usage index after adding or removing registry entries [UI].
7. Run markdownlint and repair the document instead of changing config [VAL08].

The template intentionally uses double-brace tokens for blank fields [POP01].
Those tokens are not pseudo-citation markers. They should be replaced with real
text during population [RQ06].

## (6) CLI validation workflow [VAL]

> Goal: [GL07] — VALIDATE_BEFORE_HANDOFF
> Dependency: [DP07] — SHELL_SCAN_AND_LINT_TOOLS
> Requirement: [RQ07] — REPORT_VALIDATION_STATUS

Use repository-local shell inspection before final handoff [CLI]:

```sh
# Find live reference definitions.
grep -nE '^\[[A-Za-z0-9]+\]:' *.md

# Find illegal blockquote definitions.
grep -nE '^>\s*\[[A-Za-z0-9]+\]:' *.md

# Find active zero-number placeholder IDs.
grep -nE '\[[A-Z]{1,6}00\]' *.md

# Run markdownlint without disabling MD053.
npx --yes markdownlint-cli2 TEMPLATE_MIVIM.md ONBOARDING.md REFRESHER.md
```

For deeper audits, build a map of definitions and body references outside
fenced code. Check for missing definitions, unused definitions, duplicate labels
after case folding, and references that should be inline links instead of
pseudo-citation markers [VAL02], [VAL03], [VAL04].

## (7) MD053 and markdownlint repair [MD053]

> Goal: [GL08] — REPAIR_REFERENCE_DEFINITION_FAILURES
> Dependency: [DP08] — MARKDOWNLINT_BEHAVIOR
> Requirement: [RQ08] — DO_NOT_SUPPRESS_MD053

MD053 usually appears in this format for one of three reasons [MD053]:

- A registry definition is unused because no body reference or usage-index entry
  invokes it [VAL04].
- A label is defined more than once, often because body metadata accidentally
  used `[ID]:` syntax before the registry defined the same ID [VAL05].
- A label was changed only by case, such as `[GL01]` versus `[gl01]`, but
  Markdown treats reference labels as normalized rather than truly separate
  names [VAL06].

Repair sequence:

1. Search for duplicate definitions with `grep -nE '^\[[A-Za-z0-9]+\]:'`.
2. Search for illegal body definitions with `grep -nE '^>\s*\[[A-Za-z0-9]+\]:'`.
3. Convert illegal body definitions into readable metadata references [FIX01].
4. Remove or use unused registry entries [FIX02].
5. Normalize renamed draft tags into explicit aliases only when historical
   continuity matters [FIX03].
6. Run markdownlint again [VAL08].

## (8) Common repair patterns [FIX]

> Goal: [GL09] — APPLY_STANDARD_REPAIRS
> Dependency: [DP09] — KNOWN_FAILURE_MODES
> Requirement: [RQ09] — REPAIR_WITH_MINIMAL_STRUCTURAL_DRIFT

### (8.1) Body definition to metadata reference [FIX01]

Draft:

```md
> [GLXX]: TODO ""
> [DPXX]: TODO ""
> [RQXX]: TODO ""
```

Link-complete form:

```md
> Goal: [GL04] — MAP_SECTION_OBJECTIVES
> Dependency: [DP04] — TRACEABLE_METADATA
> Requirement: [RQ04] — SECTION_METADATA_COMPLETE
```

### (8.2) Unused registry entry [FIX02]

If the entry is real, reference it in the body or usage index. If the entry is
not real, remove it. Do not keep ornamental definitions just to preserve a draft
idea [FIX02].

### (8.3) Renamed draft tag [FIX03]

If an old tag must remain for history, define it as a deprecated alias. If it is
not needed, replace it everywhere with the active tag and delete the old
registry entry [FIX03].

### (8.4) Fake research source [FIX04]

Do not create `[RS01]` merely because a source might be found later. Keep source
needs as plain prose or fenced templates until a real source exists [FIX04].

### (8.5) Excessive enhancement [FIX05]

Do not add a new namespace, section family, or citation layer unless it improves
traceability, validation, or agent ingestion. The user explicitly permits useful
enhancements, not decorative complexity [FIX05].

### (8.6) Unsafe examples [FIX06]

Invalid or draft syntax may appear only inside fenced code blocks. Outside a
fenced block, Markdown can parse the syntax as live definitions or references
[FIX06].

## (9) Agent output contract [OUT]

> Goal: [GL10] — RETURN_USABLE_ARTIFACTS
> Dependency: [DP10] — COMPLETION_AUDIT
> Requirement: [RQ10] — HANDOFF_IS_EXPLICIT

When returning work to the user, provide the generated or modified file links,
state whether markdownlint passed, state whether link-completeness was checked,
and summarize structural repairs such as placeholder removal, registry
completion, alias handling, and usage-index updates [OUT].

Do not claim external factual support unless real source records were added
[FIX04]. Do not ask for confirmation before resolving obvious placeholder IDs
when the document context clearly implies the correct completion [OP].

## (0) Agent mission [GL01]

Your mission is to preserve a link-complete Markdown pseudo-source system while
editing or extending project documents [FM]. Treat the document as two connected
layers:

1. The upper body is a compact contextual knowledge map [BD].
2. The bottom registry is the source-of-truth definition layer [Z].

Never treat bracketed markers as decorative tags. Each active marker is a
context pointer that must resolve to one registry definition and must be used in
the body or the usage index [UI].

## (2) Core model [FM]

### (2.1) Body layer [BD]

The body layer is the readable working document. It uses markers as compact
references. The body may contain plain explanatory prose, technical theory,
metadata blocks, lists, code blocks, style lists, and workflow sections.

The body must not create active definitions. It must not use the live pattern:

```md
[ID]: DESTINATION "title"
```

except inside fenced examples or inside the bottom registry [LN01].

### (2.2) Registry layer [RG]

The registry layer is the final authoritative source of marker definitions. It
is normally the final major section of the document:

```md
## (Z) Context-Citations
```

Each active marker has one definition. The preferred registry shape is:

```md
[ID]: CATEGORY_LABEL
  "Complete contextual definition."
```

`ID` is the bracket marker, `CATEGORY_LABEL` is the compact destination, and
the quoted text is the source-of-truth explanation [CE].

### (2.3) Usage index [UI]

The usage index is a non-authoritative bridge section placed before the
registry. It references every active marker at least once. This keeps reference
definitions from being treated as unused by markdownlint [LN04].

The usage index is not the source of truth. It must not redefine markers. It
only invokes them.

### (2.4) Link-complete state [LC]

A file is link-complete when all of these are true:

- Every active marker used in the body resolves to exactly one registry entry.
- Every active registry entry is referenced at least once.
- The registry is the only live definition location.
- No zero-number placeholder IDs remain in active text.
- Deprecated draft tags are either removed or declared as explicit aliases.
- The file passes markdownlint under the existing project configuration.

## (3) Required syntax [SYX]

### (3.1) Marker reference syntax [MR]

Use marker references in the body:

```md
[GL01]
[DP04]
[RQ07]
[SA]
[SA01]
[TH03]
[SY14]
```

Shortcut references are acceptable because the registry supplies the matching
reference definitions [DP01].

### (3.2) Local metadata syntax [LM]

Use readable local metadata. Do not use bracket-colon syntax in metadata.

Correct:

```md
> Goal: [GL05] — THEORY_GENERATION_BATCH
> Dependency: [DP05] — CREATIVE_MATHEMATICAL_SPACE
> Requirement: [RQ06] — TECHNICAL_PLAUSIBILITY_FILTER
```

Incorrect:

```md
> [GL05]: TODO ""
> [DP00]: TODO ""
> [RQ00]: TODO ""
```

The correct form references existing definitions. The incorrect form creates
new reference definitions and may duplicate later registry entries [MD053].

### (3.3) Registry syntax [RSX]

Only the registry uses active reference-definition syntax:

```md
[DP05]: CREATIVE_MATHEMATICAL_SPACE
  "The brainstorm depends on technical imagination, loose early constraints,
  and strong anti-replication control."
```

Keep the category label short, stable, and mostly uppercase snake case [LN07].
Use the quoted definition for the full description.

### (3.4) Code examples [EX]

Code examples may show invalid or draft syntax only inside fenced code blocks.
Fenced blocks prevent markdownlint from interpreting those lines as live
reference definitions [LN08].

## (4) Namespace model [NS]

Use namespaces to keep marker families stable. Do not invent a new namespace
when an existing one already represents the concept.

| Namespace | Meaning | Typical use |
| --- | --- | --- |
| `[Z]` | Context-citation registry | Bottom source-of-truth section |
| `[FM]` | Format model | Rules for this pseudo-source system |
| `[GL]` | Goal | Desired outcome of a section or task |
| `[DP]` | Dependency | Prerequisite or support object |
| `[RQ]` | Requirement | Required condition or acceptance rule |
| `[WL]` | Workflow | Ordered project phase |
| `[TH]` | Theory candidate | Brainstormed or refined engine theory |
| `[SA]` | SPINALTAP algorithm | Core engine organization model |
| `[AS]` | Algorithmic stem | Modular formula ownership model |
| `[CW]` | Context weight | Allowed influence of source material |
| `[DN]` | Do-not rule | Negative contextual constraint |
| `[SY]` | Style | Formal visual/rendering style marker |
| `[IG]` | Ignore rule | Excluded design direction or input |
| `[RS]` | Research source | External source record pattern |
| `[EV]` | Evidence level | Support strength for a claim |
| `[DS]` | Design object | Proposal or docspec architecture object |
| `[MO]` | Module object | Sourcecode-facing module marker |
| `[LN]` | Lint rule | Pseudo-citation validation rule |
| `[LI]` | Link index | Non-authoritative usage anchor |

Numbered IDs extend the family. For example, `[GL]` defines the goal family,
while `[GL01]`, `[GL02]`, and `[GL03]` define individual goals [ID].

## (5) Project context the agent must know [PJ]

The current project family is 4GINE: SPINALTAP multi-modular algorithmic engine
[SA]. It is a novel three-dimensional game-engine theory and development path.
The core model uses a nervous-system-inspired algorithm chain:

```txt
[SA] = [SA01] -> [SA02] -> [SA03]
```

The conceptual meanings are:

- `[SA01]` nervous_system: broad but bounded algorithm families.
- `[SA02]` vertebrae: local solvers, transform units, and scoped formulas.
- `[SA03]` spinal_column: aggregation, routing, and reconciliation layer.

The biological terms are not decorative. They describe scope, ownership, data
routing, solver boundaries, and aggregation responsibility [SA].

The algorithmic stem model [AS] means smaller algorithmic stems solve bounded
jobs and pass results into parent stems. The project is intentionally novel, but
novelty must remain technically plausible [TP] and research-adjustable [SP].

## (6) Editing protocol for CLI agents [CLI]

### (6.1) Before editing [ED01]

1. Locate the bottom registry heading.
2. Build a registry map of every definition label.
3. Build a body-reference map of every marker reference outside fenced code.
4. Identify unresolved references, unused definitions, duplicates, aliases, and
   zero-number placeholders.
5. Do not start broad rewrites until the map is understood.

Useful shell scan patterns:

```sh
# Live reference definitions.
grep -nE '^\[[A-Za-z0-9]+\]:' file.md

# Illegal blockquote definitions above the registry.
grep -nE '^>\s*\[[A-Za-z0-9]+\]:' file.md

# Draft zero-number IDs.
grep -nE '\[[A-Z]{1,6}00\]' file.md

# Markdownlint validation.
markdownlint-cli2 file.md
```

### (6.2) When adding a new section [ED02]

Every substantial workflow or design section should have a goal, dependency,
and requirement metadata block [LN09]. Use existing IDs when the section is
already covered. Create new IDs only when the section introduces genuinely new
context.

Correct pattern:

```md
## (E) PROPOSAL: [WL03]

> Goal: [GL08] — PROPOSAL_FORMATION
> Dependency: [DP08] — PROPOSAL_INPUTS
> Requirement: [RQ09] — PROPOSAL_READINESS
```

Then define any new IDs once in the registry and add them to the usage index.

### (6.3) When resolving placeholders [ED03]

Do not preserve bracketed zero IDs in a link-complete pass [LN05]. Resolve them
by doing one of the following:

1. Infer the real concept and assign the next available numbered ID.
2. Reuse an existing ID if the existing definition already covers the concept.
3. Delete the placeholder if the metadata is not needed.
4. Convert unresolved intent into plain prose without brackets until it is real.

Do not write active placeholders such as:

```md
[DP00]
[RQ00]
[RS00]
[GL00]
```

A numbered ID is allowed only when the definition exists or is created in the
same pass [RQ03].

### (6.4) When renaming or correcting tags [ED04]

If a draft tag was misspelled or renamed, prefer the corrected active tag. Keep
the old tag only when historical continuity matters [LN06]. Old tags must be
explicit aliases, not silent duplicates.

Correct alias pattern:

```md
[PW01]: DEPRECATED_ALIAS
  "Former artifact tag for THEORIZE_BRAINSTORM; preserve only as historical
  draft context and use WL01 as the active workflow ID."
```

Do not keep two active IDs with the same meaning unless one is clearly marked
as deprecated or legacy.

### (6.5) When adding external research [ED05]

Do not fabricate research-source IDs. Create a numbered research-source entry
only after an actual source exists [RS]. Until then, describe the research need
in prose or task form without a bracketed source marker.

Research sources should carry context weight [CW]. External source influence
must not overpower the internal SPINALTAP structure [DN01].

Recommended source-record shape:

```md
[RSnn]: RESEARCH_SOURCE
  "TITLE: ...
  SOURCE: ...
  YEAR: ...
  LINK_OR_DOI: ...
  USED_FOR: ...
  CONTEXT_WEIGHT: LOW | MEDIUM | HIGH
  LINKED_CONTEXT: [SA], [THnn], [RQnn], [DPnn]"
```

## (7) Lint behavior and MD053 [MD]

Markdown treats this shape as a reference definition:

```md
[label]: destination "optional title"
```

The pseudo-citation format intentionally uses that mechanism in the registry.
Because of that, the system must obey Markdown reference-definition rules
[DP01].

MD053 can appear when a reference definition is unused, duplicated, or parsed in
an unintended location [MD053]. The most common cause in this format is writing
metadata as a definition above the registry:

```md
> [GL01]: TODO ""
```

That line is not harmless metadata. It is a reference definition inside a
blockquote. If the registry later defines `[GL01]` again, the linter may report
a duplicate definition. If the marker is not referenced, the linter may report
an unused definition. If casing differs, the labels can still collide because
Markdown reference labels are case-normalized [LN03].

Do not silence the linter. Repair the format.

## (8) Completion rules [VAL]

A completion pass must satisfy these rules:

1. Definitions live only in `(Z) Context-Citations` [LN01].
2. Body metadata references IDs but does not define IDs [LN02].
3. Marker labels are unique after case folding [LN03].
4. Every active registry entry is referenced at least once [LN04].
5. No active zero-number placeholder IDs remain [LN05].
6. Deprecated tags are explicit aliases [LN06].
7. Category labels remain stable and readable [LN07].
8. Invalid examples are fenced in code blocks [LN08].
9. Workflow sections carry goal, dependency, and requirement metadata [LN09].
10. External factual claims route through research-source records when they
    become design support [LN10].
11. Nested double quotes inside definition titles are avoided or rewritten
    [LN11].
12. `markdownlint-cli2` passes without changing project configuration [LN12].

## (9) Agent decision table [DT]

- Marker used in body but missing from registry: add a real registry entry
  or replace the marker with a valid existing one.
- Registry entry unused: reference it in the body, add it to the usage index,
  or remove it.
- Zero-number placeholder found: resolve it to a real numbered ID or remove
  the bracketed marker.
- Duplicate definition found: keep one source-of-truth entry and convert extras
  into references.
- Misspelled old tag found: replace it with the corrected tag or define the old
  tag as a deprecated alias.
- New research needed but no source exists: write a plain prose task note; do
  not create a numbered source ID.
- New concept introduced: choose an existing namespace or create a justified new
  namespace entry.
- MD053 appears: search for duplicate or unused reference definitions before
  editing config.
- User asks for format completion: complete IDs, definitions, usages, aliases,
  and the usage index first.
- User asks for project theory expansion: preserve [SA], [AS], [CW], [DN01],
  [TP], and [SP] semantics.

## (10) Anti-patterns [FAIL]

Do not do these:

```md
> [GL01]: TODO ""
> [DP00]: TODO ""
> [RQ00]: TODO ""
```

```md
[AS]: ALGORITHMIC_STEM "Uses smaller batch algorithm "stems" ..."
```

```md
[gl01]: SOME_OTHER_GOAL
  "Case changed to avoid duplicate."
```

```md
[RS00]: RESEARCH_SOURCE
  "TODO find paper later."
```

The first block creates illegal body definitions. The second block risks
breaking the quoted title with nested double quotes. The third block is still a
duplicate after case normalization. The fourth block creates a fake numbered
source before a source exists [FAIL].

## (11) Corrective patterns [FIX]

### Convert body definitions to references [FIX01]

Draft:

```md
> [GL04]: TODO ""
> [DP00]: TODO ""
> [RQ00]: TODO ""
```

Link-complete form:

```md
> Goal: [GL05] — THEORY_GENERATION_BATCH
> Dependency: [DP05] — CREATIVE_MATHEMATICAL_SPACE
> Requirement: [RQ06] — TECHNICAL_PLAUSIBILITY_FILTER
```

### Convert TODO IDs to real IDs [FIX02]

Draft:

```md
[DP01]: DEPENDENCY_ONE "[TODO]"
```

Link-complete form:

```md
[DP01]: CONTEXT_CITATION_REGISTRY
  "The registry must contain the full description for every active reusable
  marker used in the document."
```

### Convert old artifact names to aliases [FIX03]

Draft artifact:

```md
[PW01]: THEORIZE_BRAINSTORM
  "Old brainstorm marker."
```

Link-complete alias:

```md
[PW01]: DEPRECATED_ALIAS
  "Former artifact tag for THEORIZE_BRAINSTORM; preserve only as historical
  draft context and use WL01 as the active workflow ID."
```

### Preserve examples without creating definitions [FIX04]

Invalid syntax demonstrations must be fenced:

````md
```md
> [GL01]: TODO ""
```
````

Outside a fenced code block, the same line becomes active syntax and may trip
lint [EX].

## (12) New-document template [TPL]

Use this shape when creating a new project document that uses the format:

```md
# TITLE: DOCUMENT_NAME [DOC]

Intro text with compact references [FM], [PJ].

## (A) SECTION_NAME: [WL01]

> Goal: [GL01] — GOAL_NAME
> Dependency: [DP01] — DEPENDENCY_NAME
> Requirement: [RQ01] — REQUIREMENT_NAME

Body text using references [SA], [AS], [TP].

## (Y) LINK-COMPLETE USAGE INDEX: [LI]

- Core markers: [DOC], [FM], [PJ], [LI]
- Goal markers: [GL], [GL01]
- Dependency markers: [DP], [DP01]
- Requirement markers: [RQ], [RQ01]

## (Z) Context-Citations

[DOC]: DOCUMENT_NAME
  "Definition."

[FM]: FORMAT_MODEL
  "Definition."

[GL]: GOAL
  "Definition."

[GL01]: GOAL_NAME
  "Definition."
```

The template is illustrative. Do not copy the example IDs blindly into a real
file if those IDs already exist with different meanings [ID].

## (13) Validation workflow [VW]

Run validation in this order:

```sh
# 1. Check for illegal body definitions.
grep -nE '^>\s*\[[A-Za-z0-9]+\]:' file.md

# 2. Check for zero placeholders.
grep -nE '\[[A-Z]{1,6}00\]' file.md

# 3. Check all reference definitions.
grep -nE '^\[[A-Za-z0-9]+\]:' file.md

# 4. Run markdownlint without disabling rules.
markdownlint-cli2 file.md
```

If lint fails, fix the document, not the config [RQ04].

## (14) Agent output expectations [OUT]

When returning work to the user:

- Provide the completed or edited Markdown file.
- State whether markdownlint passed.
- State major repairs made, especially placeholder removal, alias creation,
  registry completion, and usage-index updates.
- Do not claim external factual support unless source records were actually
  added.
- Do not ask the user to confirm obvious placeholder completions when the
  document context gives enough information to infer a real ID [VAL].

## (15) Link-complete usage index [LI]

This section is intentionally non-authoritative. It references every active
onboarding marker so the registry definitions below are needed by the Markdown
reference system [UI].

- Package markers: [A1], [A2], [A3], [PKG], [REF], [ING], [CLI]
- Context markers: [WH], [WHO], [WHAT], [WHERE], [WHY], [HOW]
- Format markers: [FM], [BD], [RG], [UI], [LI], [LC], [OP], [OP01], [OP02],
  [OP03], [OP04], [OP05]
- Population markers: [POP], [POP01], [POP02], [POP03], [POP04], [POP05]
- Extended namespace markers: [WL], [WL02], [RS], [RS01], [EV], [DS], [TK],
  [DN], [IG], [LN]
- Goal markers: [GL], [GL01], [GL02], [GL03], [GL04], [GL05], [GL06], [GL07],
  [GL08], [GL09], [GL10]
- Dependency markers: [DP], [DP01], [DP02], [DP03], [DP04], [DP05], [DP06],
  [DP07], [DP08], [DP09], [DP10]
- Requirement markers: [RQ], [RQ01], [RQ02], [RQ03], [RQ04], [RQ05], [RQ06],
  [RQ07], [RQ08], [RQ09], [RQ10]
- Validation and repair markers: [VAL], [VAL01], [VAL02], [VAL03], [VAL04],
  [VAL05], [VAL06], [VAL07], [VAL08], [MD], [MD053], [FIX], [FIX01], [FIX02],
  [FIX03], [FIX04], [FIX05], [FIX06], [OUT]
- Onboarding and context markers: [AO], [PJ], [CLI], [WH], [WH01], [WH02],
  [WH03], [WH04], [WH05]
- Format markers: [FM], [BD], [RG], [Z], [UI], [LI], [LC], [CE], [MR],
  [LM], [RSX], [EX], [SYX], [NS], [ID]
- Goal, dependency, and requirement markers: [GL], [GL01], [DP], [DP01],
  [RQ], [RQ03], [RQ04]
- Project markers: [SA], [SA01], [SA02], [SA03], [AS], [CW], [DN], [DN01],
  [TP], [SP], [WL], [TH], [RS], [EV], [SY], [IG], [IG01], [DS], [MO]
- Editing markers: [ED01], [ED02], [ED03], [ED04], [ED05], [VAL], [DT],
  [FAIL], [FIX], [FIX01], [FIX02], [FIX03], [FIX04], [TPL], [VW], [OUT]
- Lint markers: [MD], [MD053], [LN], [LN01], [LN02], [LN03], [LN04],
  [LN05], [LN06], [LN07], [LN08], [LN09], [LN10], [LN11], [LN12]

## (Z) Context-Citations

[A1]: TEMPLATE_MIVIM
  "The reusable blank template for creating a new context-dense,
  pseudo-citation Markdown document."

[A2]: AGENT_ONBOARDING
  "The complete fresh-agent ingestion guide for understanding, preserving, and
  repairing the pseudo-citation package."

[A3]: ACTIVE_SESSION_REFRESHER
  "The compact active-session reminder that links back to the full onboarding
  guide and blank template."

[PKG]: PACKAGE_MODEL
  "The three-file support package made of the template, onboarding guide, and
  refresher."

[REF]: REFRESHER_ROLE
  "The role of the short reminder file used to steer an already-active agent
  back to the core rules."

[ING]: INGESTION_DOCUMENT
  "A document written for agent ingestion rather than human narrative reading."

[CLI]: LOCAL_CLI_AGENT
  "A local shell-capable agent such as Codex, Claude Code, or another assistant
  operating directly on repository files."

[WH]: WHO_WHAT_WHERE_WHY_HOW
  "The contextual explanation set that tells an agent who the file is for, what
  it is, where it applies, why it exists, and how to operate it."

[WHO]: WHO_CONTEXT
  "The explanation of the intended agent reader and its lack of prior project
  memory."

[WHAT]: WHAT_CONTEXT
  "The explanation of the pseudo-citation format as compact references connected
  to bottom-registry definitions."

[WHERE]: WHERE_CONTEXT
  "The placement rule that body references can appear throughout the document
  while definitions belong only in the bottom registry."

[WHY]: WHY_CONTEXT
  "The purpose of the format: compact traceability, contextual weight, and
  markdownlint-compatible sourcing behavior."

[HOW]: HOW_CONTEXT
  "The operating method: reference above, define once below, index active
  markers, and lint clean."

[FM]: FORMAT_MODEL
  "The Markdown-compatible pseudo-citation model that uses compact body markers
  and complete registry definitions."

[BD]: BODY_LAYER
  "The upper document layer that contains readable working content and marker
  references."

[RG]: REGISTRY_LAYER
  "The bottom source-of-truth layer named Context-Citations where active markers
  are defined exactly once."

[UI]: USAGE_INDEX
  "A non-authoritative section that invokes active markers so their registry
  definitions remain needed."

[LI]: LINK_INDEX
  "The usage-index marker used to anchor active references without becoming the
  source of truth."

[LC]: LINK_COMPLETE_STATE
  "A finished state where all active markers are defined once, referenced at
  least once, placeholder-free, and lint-clean."

[OP]: OPERATING_MECHANICS
  "The behavior rules an agent follows while preserving the pseudo-citation
  design."

[OP01]: MARKERS_ARE_CONTEXT_HANDLES
  "Pseudo-citation markers are contextual knowledge handles rather than
  decorative labels."

[OP02]: REGISTRY_CARRIES_CONTEXT_WEIGHT
  "The registry holds the full meaning and source-of-truth context behind each
  compact marker."

[OP03]: STABLE_NAMESPACE_MODEL
  "Namespace families should remain stable so markers are predictable and easy
  to audit."

[OP04]: NO_FRIVOLOUS_NAMESPACES
  "Do not create new marker families unless they solve a real traceability,
  validation, or ingestion problem."

[OP05]: USEFUL_ENHANCEMENT_ONLY
  "Enhancements are allowed only when they improve the document's impact,
  design, or operational clarity."

[POP]: TEMPLATE_POPULATION
  "The process of converting the blank template into a project-specific,
  link-complete document."

[POP01]: REPLACE_IDENTITY_TOKENS_FIRST
  "Populate project identity fields before expanding detailed content."

[POP02]: COMPLETE_CONTEXT_BOOTSTRAP
  "Complete who, what, where, why, and how so the document stands without hidden
  context."

[POP03]: LOCAL_METADATA_REFERENCES_ONLY
  "Local section metadata should invoke existing IDs rather than define new
  ones."

[POP04]: CREATE_IDS_FOR_REAL_CONCEPTS_ONLY
  "Create numbered IDs only when the concept exists and needs a reusable marker."

[POP05]: REAL_SOURCES_ONLY
  "Create numbered research-source entries only when actual source material is
  available."

[WL]: WORKFLOW
  "A marker family for ordered phases, roadmap steps, or repeated process states
  inside a populated pseudo-citation document."

[WL02]: RESEARCH_WORKFLOW
  "An example workflow marker used to describe a research phase that should be
  defined only when it exists as active document context."

[RS]: RESEARCH_SOURCE
  "A marker family for real external or internal source records such as papers,
  documentation, datasets, standards, or verified project notes."

[RS01]: REAL_SOURCE_ENTRY
  "An example numbered source record; it must not be created in a populated file
  until actual source material exists."

[EV]: EVIDENCE_LEVEL
  "A marker family for describing how strongly a claim is supported and what
  limits apply to that support."

[DS]: DESIGN_OBJECT
  "A marker family for design-level architecture objects, proposal components,
  or docspec elements."

[TK]: TASK_OBJECT
  "A marker family for actionable tasks, task-list entries, implementation
  steps, or validation work items."

[DN]: DO_NOT
  "A marker family for negative constraints that exclude, limit, or release an
  item from active relevance."

[IG]: IGNORE_RULE
  "A marker family for explicit ignore rules that remove a style, source,
  method, or direction from active consideration."

[LN]: LINT_RULE
  "A marker family for pseudo-citation validation rules that protect link
  completeness, registry discipline, and markdownlint compatibility."

[GL]: GOAL
  "The intended outcome of a section, subsection, task, validation pass, or
  handoff."

[GL01]: INGEST_THE_FORMAT_WITHOUT_PRIOR_CONTEXT
  "Give a fresh local CLI agent enough context to understand the package without
  prior project memory."

[GL02]: ESTABLISH_AGENT_CONTEXT
  "Explain who, what, where, why, and how so the agent can orient itself."

[GL03]: DISTINGUISH_PACKAGE_FILES
  "Make the three package file roles explicit so the agent uses the correct file
  for the current task."

[GL04]: EXPLAIN_FORMAT_MECHANICS
  "Describe the body layer, registry layer, usage index, and link-complete
  state."

[GL05]: EXECUTE_THE_FORMAT_CORRECTLY
  "Guide the agent's editing behavior so it preserves the design and avoids
  frivolous structure."

[GL06]: POPULATE_TEMPLATE_WITH_REAL_CONTEXT
  "Convert the blank template into a specific document with honest, complete
  project context."

[GL07]: VALIDATE_BEFORE_HANDOFF
  "Run scans and lint checks before returning generated or modified files."

[GL08]: REPAIR_REFERENCE_DEFINITION_FAILURES
  "Identify and repair MD053 causes without silencing markdownlint."

[GL09]: APPLY_STANDARD_REPAIRS
  "Use established repair patterns for common pseudo-citation failures."

[GL10]: RETURN_USABLE_ARTIFACTS
  "Return files, validation status, and a clear summary of structural changes."

[DP]: DEPENDENCY
  "A prerequisite, support object, input, or condition needed to satisfy the
  related requirement."

[DP01]: PACKAGE_FILE_ROLES
  "The agent must know the distinct roles of the template, onboarding guide, and
  refresher."

[DP02]: ZERO_PRIOR_MEMORY_ASSUMPTION
  "The agent must assume no hidden project knowledge unless the current files or
  prompt provide it."

[DP03]: FILE_ROLE_SEPARATION
  "The package depends on clear separation between reusable template, full
  onboarding, and short refresher."

[DP04]: MARKDOWN_REFERENCE_SYNTAX
  "The format depends on Markdown reference-definition behavior and therefore
  must respect its parsing rules."

[DP05]: AGENT_DISCIPLINE
  "The agent must avoid adding ornamental structure and preserve the established
  syntax rules."

[DP06]: TEMPLATE_TOKEN_REPLACEMENT
  "Blank template tokens must be replaced with real project context during
  population."

[DP07]: SHELL_SCAN_AND_LINT_TOOLS
  "Validation depends on grep-style scans, optional custom audits, and
  markdownlint."

[DP08]: MARKDOWNLINT_BEHAVIOR
  "MD053 repair depends on understanding duplicate, unused, and accidentally
  parsed reference definitions."

[DP09]: KNOWN_FAILURE_MODES
  "Common repair patterns depend on recognizing recurring malformed syntax."

[DP10]: COMPLETION_AUDIT
  "Final handoff depends on checking lint status, link completeness, and major
  structural repairs."

[RQ]: REQUIREMENT
  "A necessary condition, output, rule, or acceptance criterion required to
  achieve the correlated goal."

[RQ01]: PRESERVE_LINK_COMPLETE_FORMAT
  "The agent must preserve one definition per active marker, registry-only
  definitions, usage coverage, and lint cleanliness."

[RQ02]: EXPLAIN_THE_SYSTEM_EXPLICITLY
  "The onboarding must describe the system without assuming prior project
  context."

[RQ03]: USE_THE_RIGHT_FILE_FOR_THE_RIGHT_TASK
  "The agent should choose template, onboarding, or refresher according to the
  task's actual need."

[RQ04]: KEEP_BODY_AND_REGISTRY_SEPARATE
  "Body text should reference markers; the registry should define markers."

[RQ05]: DO_NOT_INVENT_FRIVOLOUS_STRUCTURE
  "New structures should be added only when they improve traceability,
  validation, or ingestion."

[RQ06]: NO_FAKE_COMPLETION
  "The agent must not pretend that template tokens, missing sources, or untested
  claims are complete."

[RQ07]: REPORT_VALIDATION_STATUS
  "The final handoff should state whether markdownlint and link-completeness
  checks passed."

[RQ08]: DO_NOT_SUPPRESS_MD053
  "Repair reference-definition failures instead of disabling markdownlint rules."

[RQ09]: REPAIR_WITH_MINIMAL_STRUCTURAL_DRIFT
  "Fix broken syntax while preserving the user's intended pseudo-citation
  design."

[RQ10]: HANDOFF_IS_EXPLICIT
  "Return usable artifacts with clear notes about validation, repairs, and
  unresolved limitations."

[VAL]: VALIDATION_WORKFLOW
  "The CLI-oriented scan and lint process used before final handoff."

[VAL01]: DEFINITIONS_ONLY_IN_REGISTRY
  "Live bracket-colon definitions should appear only in the bottom registry."

[VAL02]: MISSING_DEFINITION_CHECK
  "Every body marker should resolve to one registry definition."

[VAL03]: UNUSED_DEFINITION_CHECK
  "Every active registry definition should be invoked in the body or usage
  index."

[VAL04]: USAGE_INDEX_COVERAGE
  "The usage index should cover active markers that do not naturally appear
  elsewhere."

[VAL05]: DUPLICATE_DEFINITION_CHECK
  "A marker label should not be defined more than once after normalization."

[VAL06]: CASE_NORMALIZATION_CHECK
  "Case changes should not be used to create supposedly distinct marker labels."

[VAL07]: ZERO_PLACEHOLDER_CHECK
  "Completed documents should not contain active zero-number placeholder IDs."

[VAL08]: LINT_WITHOUT_CONFIG_CHANGES
  "Run markdownlint and repair document structure without changing project lint
  configuration."

[MD]: MARKDOWNLINT_COMPATIBILITY
  "The requirement that the pseudo-citation format work with current
  markdownlint behavior."

[MD053]: LINK_IMAGE_REFERENCE_DEFINITION_FAILURE
  "A markdownlint failure commonly caused by unused definitions, duplicate
  definitions, or accidental body definitions."

[FIX]: REPAIR_PATTERN
  "A standard correction for a recurring pseudo-citation format problem."

[FIX01]: BODY_DEFINITION_TO_METADATA_REFERENCE
  "Convert draft body definitions into readable local metadata that references
  existing IDs."

[FIX02]: UNUSED_ENTRY_REPAIR
  "Reference a real unused registry entry or remove an unreal unused entry."

[FIX03]: RENAMED_TAG_REPAIR
  "Replace old tags with active tags or preserve old tags as explicit deprecated
  aliases."

[FIX04]: FAKE_SOURCE_REPAIR
  "Do not create numbered research sources unless actual source material exists."

[FIX05]: EXCESSIVE_ENHANCEMENT_REPAIR
  "Avoid new structure unless it improves traceability, validation, or agent
  ingestion."

[FIX06]: FENCED_EXAMPLE_REPAIR
  "Place invalid or illustrative syntax inside fenced code blocks so Markdown
  does not parse it as live syntax."

[OUT]: AGENT_OUTPUT_CONTRACT
  "The expected final response behavior after creating or editing the package
  files."

[AO]: AGENT_ONBOARDING
  "A machine-oriented onboarding document for local CLI agents first seeing
  the current pseudo-citation format."

[PJ]: PROJECT_CONTEXT
  "The 4GINE / SPINALTAP project family: a novel three-dimensional game engine
  theory organized through the SPINALTAP_ALGORITHM and related modular
  mathematical design patterns."

[WH01]: WHO_CONTEXT
  "The agent audience, user role, and expectation that no hidden project
  context is assumed."

[WH02]: WHAT_CONTEXT
  "The definition of the pseudo-citation format as compact marker references
  connected to complete bottom-registry entries."

[WH03]: WHERE_CONTEXT
  "The placement rule: references live throughout the body, while definitions
  live in the bottom Context-Citations registry."

[WH04]: WHY_CONTEXT
  "The purpose of the format: compact traceability, reusable context, and
  lint-compatible pseudo-sourcing without project config changes."

[WH05]: HOW_CONTEXT
  "The operating method: reference above, define once below, index active
  markers, and keep markdownlint clean."

[Z]: CONTEXT_CITATIONS
  "The bottom registry that houses the complete source-of-truth definitions for
  all active pseudo-citation markers used in the document."

[CE]: CITATION_ENTRY
  "A single pseudo-citation object consisting of a marker label, compact
  category destination, and complete quoted context definition."

[MR]: MARKER_REFERENCE
  "A body-level invocation of a pseudo-citation marker, usually written as a
  shortcut reference such as [GL01] or [SA]."

[LM]: LOCAL_METADATA
  "Section-level metadata that references goal, dependency, requirement, or
  workflow markers without using bracket-colon definition syntax."

[RSX]: REGISTRY_SYNTAX
  "The live reference-definition syntax reserved for Context-Citations entries
  in the bottom registry."

[EX]: FENCED_EXAMPLE
  "A fenced code block used to show draft, invalid, or illustrative syntax
  without causing Markdown to parse it as live references or definitions."

[SYX]: REQUIRED_SYNTAX
  "The required syntax model for marker references, local metadata, registry
  definitions, and code examples."

[NS]: NAMESPACE_MODEL
  "The rule set for assigning stable marker families such as goals,
  dependencies, requirements, theories, styles, and modules."

[ID]: NUMBERED_IDENTIFIER
  "A stable marker label in a namespace family, where the family marker defines
  the class and numbered IDs define individual concepts."

[SA]: SPINALTAP_ALGORITHM
  "A modular algorithm family organized through a nervous-system-inspired
  framework for a mathematical three-dimensional game engine."

[SA01]: nervous_system
  "Broad but bounded algorithm families that each own a scoped area of the
  engine ecosystem."

[SA02]: vertebrae
  "Smaller local solvers, transform units, or scoped formulations operating on
  defined data targets."

[SA03]: spinal_column
  "The aggregation, routing, and reconciliation layer that orders module
  relationships and produces higher-order engine state."

[AS]: ALGORITHMIC_STEM
  "A modular operation model where scoped algorithmic stems solve assigned jobs
  and pass bounded results to parent stems."

[CW]: CONTEXT_WEIGHT
  "The allowed influence weight of discovered data, source material, research,
  or prior theory on the current output."

[DN01]: DATA_REPLICATION
  "Do not over-weight established theories so heavily that the result becomes a
  lightly modified replica of the original."

[TP]: TECHNICALLY_PLAUSIBLE
  "A creative theory standard that permits novelty while rejecting incoherent,
  unbounded, or non-actionable speculation."

[SP]: SOFT_PROVEN
  "A research-supported plausibility standard based on coherent mathematics,
  explicit assumptions, comparison, and critical adjustment."

[TH]: THEORY_CANDIDATE
  "A brainstormed or research-adjusted theoretical object that may become part
  of the SPINALTAP_ALGORITHM engine model."

[SY]: STYLE
  "Formal video-game design or rendering style marker framed around platform
  capability, rendering behavior, or presentation constraint."

[IG01]: STYLE_EXCLUSION_PIXEL_ART
  "Ignore traditional pixel_art as an engine-level or core-design solution for
  this document family unless the ignore rule is explicitly revised."

[MO]: MODULE_OBJECT
  "Implementation-facing module marker that maps the conceptual algorithm model
  onto sourcecode and documentation structure."

[ED01]: PRE_EDIT_SCAN
  "Before editing, locate the registry, build maps of definitions and marker
  references, identify duplicates, and inspect unresolved placeholders."

[ED02]: ADD_SECTION_PROTOCOL
  "When adding a substantial section, include goal, dependency, and requirement
  metadata that references existing or newly defined IDs."

[ED03]: PLACEHOLDER_RESOLUTION_PROTOCOL
  "When resolving draft placeholders, replace them with real IDs, reuse valid
  existing IDs, remove them, or convert unresolved intent to plain prose."

[ED04]: TAG_RENAME_PROTOCOL
  "When correcting renamed or misspelled tags, use the active corrected tag and
  preserve old tags only as explicit deprecated aliases when needed."

[ED05]: RESEARCH_ENTRY_PROTOCOL
  "When adding external research, create numbered source entries only after real
  sources exist and assign explicit context weight."

[DT]: AGENT_DECISION_TABLE
  "A compact mapping from common document states to the required agent repair
  action."

[FAIL]: ANTI_PATTERNS
  "Common malformed patterns that break the pseudo-citation system or trigger
  markdownlint errors."

[TPL]: NEW_DOCUMENT_TEMPLATE
  "A starter shape for creating a new Markdown file that follows the current
  pseudo-citation format."

[VW]: VALIDATION_WORKFLOW
  "The shell-oriented scan and markdownlint sequence used to confirm the file
  is link-complete."

[LN01]: REGISTRY_ONLY_DEFINITIONS
  "Only the bottom Context-Citations registry should use live bracket-colon
  reference-definition syntax for active markers."

[LN02]: BODY_REFERENCES_ONLY
  "The upper document may reference markers, but it must not define those
  markers."

[LN03]: CASE_NORMALIZED_UNIQUENESS
  "Marker labels must be unique after case folding because Markdown reference
  labels are matched by normalized form."

[LN04]: USAGE_INDEX_REQUIRED_WHEN_NEEDED
  "Use a non-authoritative usage index when active registry definitions need a
  visible reference to remain lint-clean."

[LN05]: ZERO_PLACEHOLDER_BAN
  "Do not leave active zero-number placeholder IDs in completed document
  passes."

[LN06]: EXPLICIT_DEPRECATED_ALIAS
  "Deprecated or renamed IDs must be declared as aliases instead of silently
  reusing the old meaning."

[LN07]: STABLE_CATEGORY_LABELS
  "Category destination labels should remain short, stable, readable, and
  mostly uppercase snake case."

[LN08]: FENCE_INVALID_SYNTAX
  "Invalid syntax demonstrations must be fenced so Markdown does not parse them
  as real references or definitions."

[LN09]: WORKFLOW_METADATA_COMPLETENESS
  "Substantial workflow sections should include goal, dependency, and
  requirement metadata."

[LN10]: EXTERNAL_CLAIM_LINKAGE
  "External factual or research claims should connect to a research-source
  record before they become design support."

[LN11]: SAFE_QUOTATION_RULE
  "Registry definitions should avoid unsafe nested double quotes and prefer
  apostrophes or rewrites inside title text."

[LN12]: MARKDOWNLINT_PASS_REQUIRED
  "The final file should pass markdownlint without suppressing MD053 or
  changing project configuration."
