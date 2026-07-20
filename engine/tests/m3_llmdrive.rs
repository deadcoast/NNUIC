//! M3 harness — LLM drive (dev-spec TK03, ledger TK12).
//!
//! Exit: a real LLM turns novel natural-language layout intents into valid
//! grammar programs that parse and settle with zero authored coordinates.
//! The LLM under test is Claude (Fable 5), acting as master developer; the
//! three intents below are novel (disjoint from the R3 reference set) and
//! the emitted programs are exactly what the model wrote from the intent —
//! material and containment, never geometry. Token counts are recorded
//! against the R3 baseline by `m3_tokens.py` (cl100k_base).

use nnui_solver::api::{Engine, Extent, Lod};

/// (intent the LLM was given, program the LLM emitted, expected cells)
const CASES: [(&str, &str, usize); 3] = [
    (
        "A kanban board: a slim header above three equally weighted \
         columns (todo, doing, done) that sit together as one group.",
        include_str!("../layouts/m3_kanban.nnui"),
        4,
    ),
    (
        "A media gallery: one large fluid featured player with a strip \
         of four cohesive thumbnails alongside it.",
        include_str!("../layouts/m3_gallery.nnui"),
        5,
    ),
    (
        "A chat app: a stiff sidebar of conversations that keeps to \
         itself, a soft fluid transcript pane, and a near-locked \
         composer bar.",
        include_str!("../layouts/m3_chat.nnui"),
        3,
    ),
];

/// Each emitted program is valid, settles deterministically through the
/// public API, and reports every cell settled.
#[test]
fn m3_emitted_programs_settle_deterministically() {
    for (intent, src, n_cells) in CASES {
        let run = |src: &str| {
            let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
            e.submit(src).expect(intent);
            let h = e.settle();
            (h, e.snapshot(Lod::Extents))
        };
        let (h1, snap) = run(src);
        let (h2, _) = run(src);
        assert_eq!(h1, h2, "nondeterministic settle for intent: {intent}");
        assert_eq!(snap.cells.len(), n_cells, "cell count for: {intent}");
        assert!(snap.settled, "did not settle for intent: {intent}");
        assert!(
            snap.cells.iter().all(|c| c.settled),
            "unsettled cell for intent: {intent}"
        );
    }
}

/// Zero authored coordinates, enforced structurally: the emitted programs
/// parse, and the same programs with a coordinate doctored in are
/// rejected — the grammar cannot represent the offense.
#[test]
fn m3_no_authored_coordinates() {
    for (intent, src, _) in CASES {
        let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
        e.submit(src).expect(intent);

        let doctored = src.replacen('{', "{ x:120 y:40", 1);
        let mut e2 = Engine::new(Extent::viewport(1280.0, 800.0));
        assert!(
            e2.submit(&doctored).is_err(),
            "doctored coordinates accepted for intent: {intent}"
        );
    }
}
