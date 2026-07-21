//! M2 harness — the read path (dev-spec TK02, design D09 §4).
//!
//! Exit: agents obtain settled geometry and settlement events without
//! polling internals; the exposed strain is the realized post-arrest
//! value; a lying instrument (observable geometry moves while strain
//! reads zero, or geometry frozen while strain reads motion) fails the
//! build (VG03).

use nnui_solver::api::{Cause, Engine, Event, Extent, Lod, Snapshot};

const DASH: &str = include_str!("../layouts/dashboard.nnui");

fn observable_geometry(s: &Snapshot) -> Vec<(f64, f64)> {
    let mut g = Vec::new();
    for c in &s.cells {
        g.push((c.centroid.x, c.centroid.y));
        for p in &c.boundary {
            g.push((p.x, p.y));
        }
    }
    g
}

/// VG03 — the instrument gate, both directions, every frame of a full
/// settle run: observable geometry changed ⇔ reported strain nonzero.
#[test]
fn vg03_instrument_gate() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let mut before = e.snapshot(Lod::Hull);
    let mut settled_frames = 0;
    for frame in 0..6000 {
        let report = e.tick();
        let after = e.snapshot(Lod::Hull);
        let moved =
            observable_geometry(&before) != observable_geometry(&after);
        if moved {
            assert!(
                report.max_strain > 0.0,
                "frame {frame}: geometry moved but strain reads zero \
                 (lying instrument)"
            );
        } else {
            assert_eq!(
                report.max_strain, 0.0,
                "frame {frame}: geometry frozen but strain reads motion \
                 (lying instrument)"
            );
        }
        if after.settled {
            assert!(!moved, "frame {frame}: settled snapshot but geometry moved");
            settled_frames += 1;
            if settled_frames > 60 {
                return; // settled and stable for a sustained window
            }
        }
        before = after;
    }
    panic!("never settled within 2000 frames");
}

/// M2 exit: settlement is detected by consuming events from a tick loop —
/// no internal polling. Cell events precede the global event; nothing is
/// emitted twice; a second settle emits nothing new.
#[test]
fn m2_events_without_polling() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let mut seen: Vec<Event> = Vec::new();
    let mut global_at: Option<usize> = None;
    for frame in 0..6000 {
        e.tick();
        for ev in e.poll_events() {
            if matches!(ev, Event::GlobalSettled(_)) && global_at.is_none() {
                global_at = Some(frame);
            }
            seen.push(ev);
        }
        if let Some(g) = global_at {
            if frame > g + 120 {
                break; // long quiet tail after global settle
            }
        }
    }
    let global_count = seen
        .iter()
        .filter(|e| matches!(e, Event::GlobalSettled(_)))
        .count();
    let cell_ids: Vec<u64> = seen
        .iter()
        .filter_map(|e| match e {
            Event::CellSettled(id) => Some(id.0),
            _ => None,
        })
        .collect();
    assert_eq!(global_count, 1, "global settle must fire exactly once");
    let mut unique = cell_ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 4, "all four cells settle");
    assert_eq!(
        cell_ids.len(),
        unique.len(),
        "no duplicate cell-settled events"
    );
    assert_eq!(seen.first(), Some(&Event::Resettling(Cause::Injection)));
    assert!(
        matches!(seen.last(), Some(Event::GlobalSettled(_))),
        "global settle is the final event"
    );
    // settle() on the already-settled world emits nothing new
    e.settle();
    assert!(e.poll_events().is_empty(), "re-settle emitted stale events");
}

/// Boundary polylines: non-empty convex hulls at Lod::Hull, empty at
/// Lod::Extents, deterministic across repeated snapshots.
#[test]
fn m2_boundary_hulls_are_valid_and_deterministic() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    e.settle();
    let a = e.snapshot(Lod::Hull);
    let b = e.snapshot(Lod::Hull);
    for (ca, cb) in a.cells.iter().zip(&b.cells) {
        assert!(ca.boundary.len() >= 3, "hull degenerate for {}", ca.name);
        assert_eq!(
            ca.boundary, cb.boundary,
            "hull not deterministic for {}",
            ca.name
        );
        // convexity: consistent turn direction all the way around
        let n = ca.boundary.len();
        for i in 0..n {
            let o = ca.boundary[i];
            let p = ca.boundary[(i + 1) % n];
            let q = ca.boundary[(i + 2) % n];
            let cross =
                (p.x - o.x) * (q.y - o.y) - (p.y - o.y) * (q.x - o.x);
            assert!(cross > 0.0, "hull not convex/ccw for {}", ca.name);
        }
    }
    let ext = e.snapshot(Lod::Extents);
    assert!(ext.cells.iter().all(|c| c.boundary.is_empty()));
}

/// Vessel mapping: every observable maps inside the vessel, and the map
/// is pure view (applying it never touches solver state or the hash).
#[test]
fn m2_vessel_map_contains_all_geometry() {
    let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
    e.submit(DASH).unwrap();
    let h = e.settle();
    let s = e.snapshot(Lod::Hull);
    for c in &s.cells {
        for &p in std::iter::once(&c.centroid).chain(&c.boundary) {
            let v = s.map.apply(p);
            assert!(
                v.x >= -1e-9 && v.x <= s.vessel.w + 1e-9,
                "{}: x out of vessel",
                c.name
            );
            assert!(
                v.y >= -1e-9 && v.y <= s.vessel.h + 1e-9,
                "{}: y out of vessel",
                c.name
            );
        }
    }
    assert_eq!(e.snapshot(Lod::Hull).hash, Some(h), "view mapping is pure");
}
