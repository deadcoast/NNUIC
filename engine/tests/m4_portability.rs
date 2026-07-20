//! M4 harness — serialization and portability (dev-spec TK04, ledger
//! TK13). Exit: a cell serialized from layout A and injected into layout
//! B re-expresses and re-settles bit-identically across repeated runs;
//! the wire format carries no geometry, checked structurally (VG02).

use nnui_solver::api::{CellId, CellRecord, Engine, Extent, Lod, SubmitError};
use nnui_solver::grammar;

const DASH: &str = include_str!("../layouts/dashboard.nnui");
const SETTINGS: &str = include_str!("../layouts/settings.nnui");

fn settled_engine(src: &str) -> Engine {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(src).unwrap();
    e.settle();
    e
}

/// The M4 exit: export from settled A, inject into settled B, re-settle.
/// The full procedure repeated from scratch is bit-identical.
#[test]
fn m4_cross_layout_injection_is_bit_identical() {
    let run = || {
        let a = settled_engine(DASH);
        let rec = a.export_cell(CellId(0)); // "main", with evolved material
        let wire = rec.to_text();
        let rec2 = CellRecord::from_text(&wire).unwrap();
        assert_eq!(rec, rec2, "wire round-trip must be lossless");

        let mut b = settled_engine(SETTINGS);
        let id = b.inject_record(&rec2).unwrap();
        let h = b.settle();
        (id, h, b.snapshot(Lod::Extents))
    };
    let (id1, h1, snap) = run();
    let (id2, h2, _) = run();
    assert_eq!(h1, h2, "cross-layout injection not deterministic");
    assert_eq!(id1, id2);
    assert_eq!(snap.cells.len(), 4, "settings(3) + injected main");
    let injected = &snap.cells[id1.0 as usize];
    assert_eq!(injected.name, "main");
    assert!(injected.settled, "injected cell re-settled in new vessel");
    assert!(snap.settled);
}

/// VG02 — the schema gate, by set equality: an exported record's field
/// set is exactly {identifier, S, n, xi, D, content}. Nothing else can
/// exist in the format, and geometry-bearing records are rejected.
#[test]
fn vg02_schema_gate() {
    let a = settled_engine(DASH);
    for i in 0..4u64 {
        let rec = a.export_cell(CellId(i));
        // re-parse the fragment and check the exact field set
        let l = grammar::parse(&rec.source).unwrap();
        assert_eq!(l.cells.len(), 1);
        assert!(l.ops.is_empty());
        // The grammar's Cell struct IS the whitelist: id, s, n, xi, d,
        // content. Structural: the record text must contain each dial
        // exactly once and no other key:value pair.
        let src = &rec.source;
        for key in ["S:", "n:", "xi:", "D:"] {
            assert_eq!(
                src.matches(key).count(),
                1,
                "dial {key} must appear exactly once"
            );
        }
        let colon_pairs = src.matches(':').count();
        assert_eq!(colon_pairs, 4, "exactly the four whitelisted dials");
    }
    // geometry-bearing records are unrepresentable: rejected at parse
    let mut b = settled_engine(SETTINGS);
    for bad in [
        "cell w { S:0.5 n:0.8 xi:1 D:0.5 x:120 germline }",
        "cell w { S:0.5 pos:3 germline }",
        "cell w { left:0 top:0 germline }",
    ] {
        let rec = CellRecord { version: 1, origin: None, source: bad.into() };
        assert!(
            b.inject_record(&rec).is_err(),
            "geometry-bearing record accepted: {bad}"
        );
    }
}

/// Binds do not travel: exported records from a bound group carry no ops,
/// and a record smuggling a bind op is rejected.
#[test]
fn m4_binds_do_not_travel() {
    let a = settled_engine(DASH); // k1-k2, k2-k3 are bound in the program
    for i in 1..4u64 {
        let rec = a.export_cell(CellId(i));
        assert!(
            !rec.source.contains("bind"),
            "bind leaked into record: {}",
            rec.source
        );
    }
    let mut b = settled_engine(SETTINGS);
    let smuggle = CellRecord {
        version: 1,
        origin: None,
        source: "cell w { germline } bind w nav".into(),
    };
    let err = b.inject_record(&smuggle).unwrap_err();
    assert!(matches!(err, SubmitError::Parse(_)));
}

/// Canonical numbers: export → inject → export is a fixpoint (the second
/// record's material text equals the first, modulo material evolution
/// during the settle — so compare un-settled worlds).
#[test]
fn m4_canonical_form_is_a_fixpoint() {
    let a = settled_engine(DASH);
    let rec1 = a.export_cell(CellId(0));
    let mut b = settled_engine(SETTINGS);
    let id = b.inject_record(&rec1).unwrap();
    // export immediately (no settle): material state as injected
    let rec2 = b.export_cell(id);
    assert_eq!(rec1.source, rec2.source, "canonical form must be stable");
}

/// Position cannot leak: two cells with identical material and dials but
/// different settled positions export identical records (modulo name).
#[test]
fn m4_position_cannot_leak_into_records() {
    let a = settled_engine(DASH);
    // k1 and k2 are declared identically (same dials, different assets —
    // so compare after normalizing name and asset)
    let norm = |s: &str, name: &str, asset: &str| {
        s.replace(name, "CELL").replace(asset, "ASSET")
    };
    let r1 = norm(&a.export_cell(CellId(1)).source, "k1", "kpi1");
    let r2 = norm(&a.export_cell(CellId(2)).source, "k2", "kpi2");
    assert_eq!(
        r1, r2,
        "records differ for identically-declared cells — geometry or \
         other hidden state is leaking into the wire format"
    );
}

/// Duplicate identity is rejected atomically: injecting a record whose id
/// already exists leaves the engine untouched.
#[test]
fn m4_duplicate_injection_is_atomic() {
    let a = settled_engine(DASH);
    let rec = a.export_cell(CellId(0)); // "main"
    let mut b = settled_engine(DASH); // already has "main"
    let before = b.snapshot(Lod::Extents).hash;
    let err = b.inject_record(&rec).unwrap_err();
    assert!(matches!(err, SubmitError::Parse(_)), "duplicate id: {err:?}");
    assert_eq!(b.snapshot(Lod::Extents).hash, before, "state disturbed");
}
