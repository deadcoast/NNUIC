# REFRESHER: Active Session Pseudo-Citation Steering Note [A3]

## (0) Active-session refresher [A3]

Use this file only as a compact steering note during an active session [REF].
When structural uncertainty appears, return to the full bootstrap in
[ONBOARDING.md](ONBOARDING.md#0-fresh-agent-bootstrap-a2) [A2], the operating
mechanics in [ONBOARDING.md](ONBOARDING.md#4-operating-mechanics-op), and the
MD053 repair section in
[ONBOARDING.md](ONBOARDING.md#7-md053-and-markdownlint-repair-md053) [MD053].
When creating a new project document, populate the blank specimen at
[TEMPLATE_MIVIM.md](TEMPLATES/TEMPLATE_MIVIM.md#0-template-role-tpl01) [A1] and
follow its agent population protocol at
[TEMPLATE_MIVIM.md](TEMPLATES/TEMPLATE_MIVIM.md#8-population-protocol-for-agents-tpl02)
[CLI].

Keep the core rule active: reference markers above, define each active marker
once in `(Z) Context-Citations`, keep a usage index for active definitions, and
never leave `[00]` placeholder IDs in a completed pass [FM], [RG], [UI], [LC].
Do not silence markdownlint; repair body definitions, duplicate labels, unused
definitions, fake source records, and unfenced invalid examples in the document
itself [LN].

## (Y) Link-complete usage index [LI]

This section is non-authoritative. It invokes every active refresher marker so
the registry definitions below remain needed [UI].

- Package markers: [A1], [A2], [A3], [REF], [CLI]
- Format markers: [FM], [RG], [UI], [LI], [LC]
- Validation markers: [LN], [MD053]

## (Z) Context-Citations

[A1]: TEMPLATE_MIVIM
  "The blank pseudo-citation template used to create a new context-dense,
  link-complete Markdown document."

[A2]: AGENT_ONBOARDING
  "The full fresh-agent onboarding file that explains the package, format,
  validation workflow, and repair patterns."

[A3]: ACTIVE_SESSION_REFRESHER
  "The short active-session reminder used to steer an agent back to the core
  pseudo-citation rules."

[REF]: REFRESHER_ROLE
  "A compact reminder role for agents that already have enough context but need
  quick alignment during ongoing work."

[CLI]: LOCAL_CLI_AGENT
  "A local shell-capable editing agent such as Codex, Claude Code, or another
  assistant operating on repository files."

[FM]: FORMAT_MODEL
  "The pseudo-citation model where body markers point to bottom-registry
  definitions."

[RG]: REGISTRY_LAYER
  "The bottom Context-Citations section where active markers are defined exactly
  once."

[UI]: USAGE_INDEX
  "A non-authoritative section that references active markers so their registry
  definitions remain needed."

[LI]: LINK_INDEX
  "The marker for the usage-index section that anchors active definitions."

[LC]: LINK_COMPLETE_STATE
  "A state where active markers are defined once, referenced at least once, free
  of zero placeholders, and lint-clean."

[LN]: LINT_RULE
  "The instruction to repair pseudo-citation syntax and link behavior in the
  document rather than suppressing markdownlint."

[MD053]: LINK_IMAGE_REFERENCE_DEFINITION_FAILURE
  "A markdownlint reference-definition failure usually caused by duplicate,
  unused, or accidentally parsed reference definitions."
