# DESIGN: Workstream A API Contracts [DOC]

This is the Stage 2 design artifact [WL03] of the workstream A docspec
chain. It turns the dev-spec's design objects into worked-example contracts
before any implementation code exists, per the harness-first decision
recorded in [D02]. Marker numbering for design objects mirrors [D02] so the
same marker names the same object across the family. The file is maintained
link-complete [LC] under the lint discipline [LN].

## (0) Document role [GL01]

> Goal: [GL01] — DEFINE_DESIGN_ROLE
> Dependency: [DP01] — DISCOVERY_RECORD
> Requirement: [RQ01] — CONTRACTS_BEFORE_CODE

Three contracts are fixed here: the submission surface [DS02], the cell
wire format [DS04], and the read path [DS05]. Each closes with executable
conformance gates [VG] that the build stage inherits as acceptance criteria
[DS03]. Contract positions hold at internal-reasoning strength [EV02] until
their milestone validates them. Theory is not touched: every rule below is
a realization of the frozen v0.2 model in [D01].

## (1) Submission surface [DS02]

> Goal: [GL02] — FIX_SUBMISSION_SURFACE
> Dependency: [DP02] — VALIDATED_GRAMMAR
> Requirement: [RQ02] — NO_COORDINATE_PRODUCTION

The submission language is the validated R3 agent grammar, unchanged. The
API adds an engine object around it. The only geometry the API ever
accepts is the vessel extent — the container's boundary condition, which
is environment, not cell position [IV01].

```rust
pub struct Engine;            // opaque: solver world + trait registry
pub struct ProgramId(u64);
pub struct CellId(u64);       // stable across re-settles
pub struct StateHash(u64);    // FNV-1a over settled state

impl Engine {
    pub fn new(vessel: Extent) -> Engine;
    pub fn submit(&mut self, src: &str)
        -> Result<ProgramId, SubmitError>;       // atomic, see (2)
    pub fn settle(&mut self) -> StateHash;       // run to predicate
    pub fn tick(&mut self) -> TickReport;        // one solver frame
    pub fn snapshot(&self, lod: Lod) -> Snapshot;
    pub fn poll_events(&mut self) -> Vec<Event>;
    pub fn export_cell(&self, id: CellId) -> CellRecord;
    pub fn inject_record(&mut self, rec: &CellRecord)
        -> Result<CellId, SubmitError>;
}
```

Worked example — the R3 dashboard through the public surface:

```txt
cell main { S:0.3 n:0.8 xi:1 D:0.6  hold revenue.csv }
cell k1   { S:0.5 n:0.8 xi:1 D:0.4  hold kpi1 }
cell k2   { S:0.5 n:0.8 xi:1 D:0.4  hold kpi2 }
cell k3   { S:0.5 n:0.8 xi:1 D:0.4  hold kpi3 }
bind k1 k2 ; bind k2 k3
contain { main k1 k2 k3 } in viewport
```

```rust
let mut e = Engine::new(Extent::viewport(1280, 800));
let _p = e.submit(DASHBOARD_SRC)?;
let h1 = e.settle();
let h2 = e.settle();
assert_eq!(h1, h2);              // settling at rest is idempotent
let snap = e.snapshot(Lod::Extents);
assert!(snap.settled);
```

The M1 exit consumes this example: the hash produced here must equal the
hash the prototype binary produces for the same layout [CN01].

## (2) Error semantics [DS02]

> Goal: [GL03] — DEFINE_ERROR_SEMANTICS
> Dependency: [DP03] — FAILURE_ENUMERATION
> Requirement: [RQ03] — ATOMIC_SUBMISSION

Submission is atomic: a program either applies in full or is rejected in
full with the engine state untouched. There is no partial application and
therefore no partially-settled garbage state to reason about [RQ03].

| Code | Trigger | Effect |
| --- | --- | --- |
| `E-PARSE` | malformed program text | rejected, state untouched |
| `E-REF` | op names an undeclared cell | rejected, state untouched |
| `E-CYCLE` | containment cycle | rejected, state untouched |
| `E-ASSET` | asset-ref unresolvable | rejected, state untouched |
| `E-RANGE` | material dial outside its domain | rejected, state untouched |

Two deliberate non-errors:

- An asset of **unknown type** is not an error: it sequences to no known
  traits and the cell expresses germline, exactly as validated in R4. Only
  an unresolvable reference (`E-ASSET`) fails.
- There is **no coordinate error code**, because there is no way to commit
  the offense: the grammar has no coordinate production, so authored
  geometry is unrepresentable rather than forbidden-at-runtime [IV01].
  Gate [VG01] keeps it that way.

## (3) Cell wire format [DS04]

> Goal: [GL04] — DEFINE_WIRE_FORMAT
> Dependency: [DP04] — GRAMMAR_AS_CARRIER
> Requirement: [RQ04] — GEOMETRY_FREE_RECORD

Design decision: **the wire format is the agent grammar itself**, plus a
record header. One language authors cells and carries them; a cell export
is a valid program fragment that any NNUIC engine can inject.

```txt
cellrecord v1
origin-hash 0xdb5295e7d7bce689
cell main { S:0.31 n:0.8 xi:0.97 D:0.6
            hold revenue.csv#sha256:9f2c41d0 }
```

Normative rules:

1. **Field whitelist.** A record may carry exactly: identifier, `S`, `n`,
   `xi`, `D`, optional `skin`, and content as asset reference plus content
   hash. Nothing else. The whitelist is exact — validation is by equality
   with the allowed set, not by blacklisting position-like names [VG02].
2. **No geometry, structurally.** Position, extent, and settled shape are
   absent from the format. A cell re-settles under the destination
   vessel's local pressure; portability follows from what the record does
   not contain [IV01].
3. **Genome travels as provenance, not as cache.** The record carries the
   asset reference and content hash; traits are re-derived at the
   destination by the pure sequencing function, so expression stays
   deterministic end-to-end [IV07].
4. **Binds do not travel.** Gene transfer is opt-in per boundary; an
   auto-rebinding record would smuggle in proximity-triggered coupling.
   Export detaches binds; the injecting agent re-issues `bind` ops
   deliberately at the destination.
5. **Canonical numbers.** Material scalars serialize in shortest
   round-trip decimal form so export, inject, and re-settle reproduce
   bit-identical hashes [CN01], checked by [VG04].

## (4) Read path [DS05]

> Goal: [GL05] — DEFINE_READ_PATH
> Dependency: [DP05] — DISCOVERY_ANSWERS
> Requirement: [RQ05] — HONEST_INSTRUMENT

Direct realization of the discovery record in [D02] section 6.1:

```rust
pub struct Snapshot {
    pub tick: u64,
    pub settled: bool,
    pub hash: Option<StateHash>,   // Some only when settled
    pub cells: Vec<CellView>,
}
pub struct CellView {
    pub id: CellId,
    pub centroid: Vec2,
    pub extent: Extent,
    pub boundary: Vec<Vec2>,       // resolution set by requested Lod
    pub traits: Vec<TraitToken>,
    pub phenotype: PhenotypeId,
    pub settled: bool,
    pub strain: f64,               // realized post-arrest only
}
pub enum Event {
    CellSettled(CellId),
    GlobalSettled(StateHash),
    Resettling(Cause),  // content change, viewport change, injection
}
```

Rules:

- A snapshot is a detached copy; no handle it exposes can mutate solver
  state, so query order can never affect settlement [CN01].
- `strain` is the realized post-arrest displacement rate and nothing else
  — never a damped or zeroed velocity variable [IV04].
- `hash` exists only in settled snapshots; motion is never hashed.
- Mid-settle snapshots are legal; `settled` distinguishes transient from
  final geometry.
- Events are delivered in deterministic order (cell-id order within a
  tick), so event consumption cannot introduce nondeterminism [CN01].

## (5) Conformance gates [VG]

> Goal: [GL06] — DEFINE_CONFORMANCE_GATES
> Dependency: [DP06] — MILESTONE_EXITS
> Requirement: [RQ06] — GATES_ARE_EXECUTABLE

Each gate is an executable check the harness runs; together they are the
acceptance criteria [DS03] the build stage inherits.

- [VG01] **Coordinate gate.** The grammar and AST contain no coordinate
  production; a fuzz corpus of position-bearing programs must fail at
  parse, every one. Active from M1 onward [IV01].
- [VG02] **Schema gate.** The record field set equals the whitelist of
  rule 1 in section (3), by set equality. Active from M4.
- [VG03] **Instrument gate.** Whenever positions differ between
  consecutive snapshots, reported strain is nonzero; whenever `settled`
  is true, position deltas are exactly zero. A lying instrument fails the
  build. Active from M2 [IV04].
- [VG04] **Hash gate.** Submit, settle, export, inject, re-settle
  reproduces bit-identical hashes across repeated runs and process
  invocations. Partial from M1 (submit/settle), full at M4 [CN01].

## (Y) Link-complete usage index [LI]

This section is non-authoritative. It invokes active markers so the
registry below remains link-complete [LC], [LN].

- Document markers: [DOC], [D01], [D02], [LC], [LI], [LN], [WL], [WL03]
- Goal markers: [GL], [GL01], [GL02], [GL03], [GL04], [GL05], [GL06]
- Dependency markers: [DP], [DP01], [DP02], [DP03], [DP04], [DP05], [DP06]
- Requirement markers: [RQ], [RQ01], [RQ02], [RQ03], [RQ04], [RQ05],
  [RQ06]
- Design markers: [DS], [DS02], [DS03], [DS04], [DS05]
- Gate markers: [VG], [VG01], [VG02], [VG03], [VG04]
- Invariant and constraint markers: [IV], [IV01], [IV04], [IV07], [CN],
  [CN01]
- Evidence markers: [EV], [EV02]

## (Z) Context-Citations

[DOC]: DESIGN_API_CONTRACTS
  "The Stage 2 design document fixing the workstream A API contracts as
  worked examples before code."

[D01]: THEORY_SPECIFICATION
  "latex/v0.2/NNUI_CELL_PoC_v0.2.tex — the canonical frozen theory this
  design realizes."

[D02]: WORKSTREAM_A_DEVSPEC
  "docs/DEVSPEC_WORKSTREAM_A.md — the dev-spec whose design objects,
  discovery record, and milestones this document details."

[LC]: LINK_COMPLETE_STATE
  "A state where every active marker is defined once and referenced at
  least once."

[LI]: LINK_INDEX
  "The marker for the non-authoritative usage-index section."

[LN]: LINT_RULE
  "The pseudo-citation rule set: registry-only definitions, lint repaired
  in the document."

[WL]: WORKFLOW
  "The workstream A phase model, numbered consistently with the dev-spec."

[WL03]: DESIGN
  "The design phase: contracts as worked examples before implementation."

[GL]: GOAL
  "The intended outcome of a section."

[GL01]: DEFINE_DESIGN_ROLE
  "State what this document fixes and what strength its positions hold."

[GL02]: FIX_SUBMISSION_SURFACE
  "Define the engine object and the submission round-trip over the
  validated grammar."

[GL03]: DEFINE_ERROR_SEMANTICS
  "Enumerate every rejection and its effect on engine state."

[GL04]: DEFINE_WIRE_FORMAT
  "Fix the portable cell record and its normative rules."

[GL05]: DEFINE_READ_PATH
  "Fix the snapshot, view, and event surface an agent reads."

[GL06]: DEFINE_CONFORMANCE_GATES
  "Turn the contracts into executable acceptance checks."

[DP]: DEPENDENCY
  "A prerequisite object for a section."

[DP01]: DISCOVERY_RECORD
  "The Stage 1 answers recorded in the dev-spec that this design
  consumes."

[DP02]: VALIDATED_GRAMMAR
  "The R3 grammar, already lowered end-to-end into the R1 solver."

[DP03]: FAILURE_ENUMERATION
  "The identified ways a submitted program can be invalid."

[DP04]: GRAMMAR_AS_CARRIER
  "The decision that the authoring language is also the wire format."

[DP05]: DISCOVERY_ANSWERS
  "The six read-path answers of the discovery record."

[DP06]: MILESTONE_EXITS
  "The binary milestone exits the gates must serve."

[RQ]: REQUIREMENT
  "A necessary condition for a section's contract."

[RQ01]: CONTRACTS_BEFORE_CODE
  "All three contracts are fixed as worked examples before build starts."

[RQ02]: NO_COORDINATE_PRODUCTION
  "The submission language cannot represent authored geometry."

[RQ03]: ATOMIC_SUBMISSION
  "A program applies fully or not at all; rejection leaves state
  untouched."

[RQ04]: GEOMETRY_FREE_RECORD
  "The wire format structurally lacks position, extent, and settled
  shape."

[RQ05]: HONEST_INSTRUMENT
  "The read path exposes only realized post-arrest strain and hashes only
  settled states."

[RQ06]: GATES_ARE_EXECUTABLE
  "Every conformance gate runs as an automated check, not a review note."

[DS]: DESIGN_OBJECT
  "A stable design object of the workstream A architecture; numbering
  mirrors the dev-spec."

[DS02]: API_SURFACE
  "The submission contract: engine object, atomic submit, settle, and
  error semantics over the agent grammar."

[DS03]: ACCEPTANCE_CRITERIA
  "The executable gates plus binary milestone exits the build inherits."

[DS04]: CELL_SERIALIZATION_FORMAT
  "The grammar-carried, geometry-free, deterministic cell record."

[DS05]: READ_PATH_MODEL
  "The snapshot, cell view, and event surface with the honest strain
  readout."

[VG]: VALIDATION_GATE
  "An executable conformance check inherited by the build stage."

[VG01]: COORDINATE_GATE
  "Grammar and AST admit no coordinate production; position-bearing
  programs all fail at parse."

[VG02]: SCHEMA_GATE
  "The record field set equals the whitelist exactly, by set equality."

[VG03]: INSTRUMENT_GATE
  "Position deltas and reported strain must agree; a lying instrument
  fails the build."

[VG04]: HASH_GATE
  "The full round-trip reproduces bit-identical hashes across runs and
  processes."

[IV]: INVARIANT
  "The seven executable invariants of the v0.2 specification."

[IV01]: NO_AUTHORED_POSITIONS
  "No code path assigns a coordinate; every position is a solver output."

[IV04]: HONEST_THRESHOLD_SETTLING
  "Settling terminates by strain threshold measured on realized
  post-arrest displacement."

[IV07]: METAPHOR_REDUCES_TO_MECHANISM
  "Every biological operation is a defined deterministic mechanism; the
  genetic path contains no stochastic call."

[CN]: CONSTRAINT
  "A rule that must remain true throughout workstream A."

[CN01]: DETERMINISM_PRESERVED
  "Identical inputs produce bit-identical settled states through the
  public API."

[EV]: EVIDENCE_LEVEL
  "A label communicating how strongly a claim is supported."

[EV02]: INTERNAL_REASONING
  "A design position supported by the model's logic, awaiting milestone
  validation."
