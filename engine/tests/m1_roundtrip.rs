//! M1 harness — the first consumer of the public API (harness-first
//! decision, dev-spec §6.1). Proves the milestone exit: the reference
//! layouts settle through the public API to the same hashes as the
//! validated grammar path, submission is atomic, coordinate programs are
//! unrepresentable (VG01), and events arrive in deterministic order.

use nnui_solver::api::{Cause, Engine, Event, Extent, Lod, SubmitError};
use nnui_solver::api::SEED;
use nnui_solver::grammar;

const DASH: &str = include_str!("../layouts/dashboard.nnui");
const ARTICLE: &str = include_str!("../layouts/article.nnui");
const SETTINGS: &str = include_str!("../layouts/settings.nnui");

fn api_hash(src: &str) -> u64 {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(src).expect("valid program");
    e.settle().0
}

/// M1 exit: public-API hashes equal the validated grammar-path hashes.
#[test]
fn m1_api_matches_grammar_path_for_three_layouts() {
    for (name, src) in [
        ("dashboard", DASH),
        ("article", ARTICLE),
        ("settings", SETTINGS),
    ] {
        let layout = grammar::parse(src).unwrap();
        let mut w = nnui_solver::solver::World::from_layout(
            &layout, 1280.0, 800.0, SEED,
        );
        w.settle(4000);
        let reference = w.state_hash();
        let api = api_hash(src);
        assert_eq!(
            api, reference,
            "{name}: public API hash diverged from the layout-world path"
        );
    }
}

/// VG04 (partial): repeated full runs are bit-identical, and settling at
/// rest is idempotent.
#[test]
fn m1_hashes_reproduce_and_settle_is_idempotent() {
    assert_eq!(api_hash(DASH), api_hash(DASH));
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let h1 = e.settle();
    let h2 = e.settle();
    assert_eq!(h1, h2, "settling at rest must not move matter");
    let snap = e.snapshot(Lod::Extents);
    assert!(snap.settled);
    assert_eq!(snap.hash, Some(h1), "settled snapshot carries the hash");
    assert_eq!(snap.cells.len(), 4);
    assert_eq!(snap.cells[0].name, "main");
}

/// DS02: rejection is atomic — every error leaves prior state untouched.
#[test]
fn m1_rejection_is_atomic() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let before = e.settle();

    let cases: Vec<(&str, fn(&SubmitError) -> bool)> = vec![
        ("cell a {", |er| matches!(er, SubmitError::Parse(_))),
        ("cell a { S:0.5 } bind a b", |er| {
            matches!(er, SubmitError::UnknownRef(_))
        }),
        ("cell a { } contain { a } in a", |er| {
            matches!(er, SubmitError::ContainmentCycle(_))
        }),
        ("cell a { } cell b { } contain { a } in b contain { b } in a", |er| {
            matches!(er, SubmitError::ContainmentCycle(_))
        }),
        ("cell a { S:1.5 }", |er| matches!(er, SubmitError::OutOfRange(_))),
        ("cell a { } cell a { }", |er| matches!(er, SubmitError::Parse(_))),
        ("cell a { } bind a a", |er| {
            matches!(er, SubmitError::UnknownRef(_))
        }),
    ];
    for (src, is_expected) in cases {
        let err = e.submit(src).expect_err(src);
        assert!(is_expected(&err), "{src}: wrong error {err:?}");
        let snap = e.snapshot(Lod::Extents);
        assert_eq!(
            snap.hash,
            Some(before),
            "{src}: rejected submit disturbed engine state"
        );
    }
}

/// VG01: authored geometry is unrepresentable — every position-bearing
/// program fails at parse/validation, on a fresh engine.
#[test]
fn vg01_coordinate_programs_all_fail() {
    let corpus = [
        "cell a { x:10 y:20 }",
        "cell a { pos:3 }",
        "cell a { at:5 }",
        "at 10 20 cell a { }",
        "cell a { } place a 5 5",
        "rect 0 0 100 100",
        "cell a { S:0.4 left:0 top:0 }",
        "cell a { } move a 3 4",
    ];
    for src in corpus {
        let mut e = Engine::new(Extent::viewport(100.0, 100.0));
        assert!(
            e.submit(src).is_err(),
            "coordinate program was accepted: {src}"
        );
    }
}

/// Sharpened Invariant 4 (v0.2 §7): the settled state is exact and
/// flicker-free over a sustained observation window. Stepping a settled
/// world 600 frames must produce zero realized motion and an unchanged
/// hash — a settled medium never erupts.
#[test]
fn i4_settled_state_is_flicker_free() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let h = e.settle();
    for frame in 0..600 {
        let report = e.tick();
        assert_eq!(
            report.max_strain, 0.0,
            "flicker at observation frame {frame}"
        );
    }
    let snap = e.snapshot(Lod::Extents);
    assert_eq!(snap.hash, Some(h), "hash drifted during observation");
}

/// DS05: events arrive in deterministic order — resettling cause, then
/// per-cell settles in id order, then the global settle with the hash.
#[test]
fn m1_events_deterministic_order() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let h = e.settle();
    let ev = e.poll_events();
    assert_eq!(ev.first(), Some(&Event::Resettling(Cause::Injection)));
    assert_eq!(ev.last(), Some(&Event::GlobalSettled(h)));
    let cell_ids: Vec<u64> = ev
        .iter()
        .filter_map(|e| match e {
            Event::CellSettled(id) => Some(id.0),
            _ => None,
        })
        .collect();
    let mut sorted = cell_ids.clone();
    sorted.sort_unstable();
    assert_eq!(cell_ids, sorted, "cell events out of id order");
    assert_eq!(cell_ids.len(), 4, "all four cells should settle");
    assert!(e.poll_events().is_empty(), "poll drains the queue");
}
