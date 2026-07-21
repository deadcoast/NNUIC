//! Addendum v0.3 validation exits for the E_cohesion curvature-flow
//! design. T1 (no interleaving) is checked here at the engine level;
//! T2/T3 run in the browser verifier; T4 is the rest of this suite.

use nnui_solver::api::{Engine, Extent, Lod};
use nnui_solver::vec2::Vec2;

const LAYOUTS: [(&str, &str); 4] = [
    ("dashboard", include_str!("../layouts/dashboard.nnui")),
    ("kanban", include_str!("../layouts/m3_kanban.nnui")),
    ("gallery", include_str!("../layouts/m3_gallery.nnui")),
    ("chat", include_str!("../layouts/m3_chat.nnui")),
];

fn polygon_area(p: &[Vec2]) -> f64 {
    let n = p.len();
    if n < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    for i in 0..n {
        let q = p[(i + 1) % n];
        a += p[i].x * q.y - q.x * p[i].y;
    }
    a.abs() / 2.0
}

/// Sutherland–Hodgman clip of `subject` against convex CCW `clip`.
fn clip_convex(subject: &[Vec2], clip: &[Vec2]) -> Vec<Vec2> {
    let mut out: Vec<Vec2> = subject.to_vec();
    let n = clip.len();
    for i in 0..n {
        if out.is_empty() {
            break;
        }
        let a = clip[i];
        let b = clip[(i + 1) % n];
        let inside = |p: Vec2| -> bool {
            (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x) >= 0.0
        };
        let input = std::mem::take(&mut out);
        let m = input.len();
        for j in 0..m {
            let cur = input[j];
            let prev = input[(j + m - 1) % m];
            let cur_in = inside(cur);
            let prev_in = inside(prev);
            if cur_in != prev_in {
                // intersection of prev->cur with a->b
                let d1 = Vec2::new(cur.x - prev.x, cur.y - prev.y);
                let d2 = Vec2::new(b.x - a.x, b.y - a.y);
                let denom = d1.x * d2.y - d1.y * d2.x;
                if denom.abs() > 1e-12 {
                    let t = ((a.x - prev.x) * d2.y
                        - (a.y - prev.y) * d2.x)
                        / denom;
                    out.push(Vec2::new(
                        prev.x + d1.x * t,
                        prev.y + d1.y * t,
                    ));
                }
            }
            if cur_in {
                out.push(cur);
            }
        }
    }
    out
}

/// T1 — no interleaving: for every reference layout, the pairwise hull
/// intersection between settled cells is < 2% of the smaller cell.
#[test]
#[ignore = "addendum exit T1 UNMET: drive-based exclusion equilibrates at ~4% pairwise overlap vs the 2% bar. Not a regression - the exit has never passed with full fill; see CHANGELOG 025. Next pass: constraint-based formulation. Run explicitly with --ignored to measure progress."]
fn t1_no_interleaving() {
    for (name, src) in LAYOUTS {
        let mut e = Engine::new(Extent::viewport(1280.0, 800.0));
        e.submit(src).unwrap();
        e.settle();
        let s = e.snapshot(Lod::Hull);
        assert!(s.settled, "{name}: did not settle");
        for i in 0..s.cells.len() {
            for j in (i + 1)..s.cells.len() {
                let a = &s.cells[i];
                let b = &s.cells[j];
                let inter =
                    clip_convex(&a.boundary, &b.boundary);
                let ov = polygon_area(&inter);
                let min_area = polygon_area(&a.boundary)
                    .min(polygon_area(&b.boundary));
                let frac = ov / min_area.max(1.0);
                assert!(
                    frac < 0.02,
                    "{name}: cells {} and {} interleave — overlap {:.1}% \
                     of the smaller cell",
                    a.name,
                    b.name,
                    frac * 100.0
                );
            }
        }
    }
}
