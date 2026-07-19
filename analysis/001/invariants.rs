//! Executable invariants (paper §8) + R1/R2 exit criteria as tests.

use nnui_solver::rheology::{self, Material};
use nnui_solver::solver::World;

/// R2: identical inputs → bit-identical settled state, every run.
#[test]
fn r2_bit_identical_determinism() {
    let mk = || {
        let mut w = World::new_packed(50, 5, 60.0, 52.0, 0xABCDEF);
        w.settle(4000);
        w.state_hash()
    };
    let base = mk();
    for _ in 0..20 {
        assert_eq!(mk(), base, "settled state diverged between runs");
    }
}

/// I4: settling is threshold-terminated — it actually reaches rest, and the
/// final max speed is below the configured epsilon.
#[test]
fn i4_settling_reaches_threshold() {
    let mut w = World::new_packed(50, 5, 60.0, 52.0, 0xABCDEF);
    let frames = w.settle(4000);
    assert!(frames < 4000, "did not settle within budget (frames={frames})");
    assert!(w.max_speed() < w.eps_settle, "max_speed {} !< eps", w.max_speed());
}

/// I3: locking is the S→1 limit. A near-locked, isolated region does not drift.
#[test]
fn i3_locked_region_holds_still() {
    // one region, near-locked, alone in a large container, starting at rest.
    let mut w = World::new_packed(1, 5, 60.0, 52.0, 0xABCDEF);
    // force region 0 near-locked and record its centroid
    w.regions[0].mat = Material::fluidish(0.9999);
    let before: Vec<_> = w.pos.clone();
    for _ in 0..120 {
        w.step();
    }
    let mut max_disp = 0.0f64;
    for i in 0..w.pos.len() {
        let d = w.pos[i].sub(before[i]).len();
        if d > max_disp {
            max_disp = d;
        }
    }
    assert!(max_disp < 1e-3, "locked region drifted by {max_disp}");
}

/// I2: the constitutive law is real — flow index n changes apparent viscosity,
/// and solidity monotonically stiffens (smaller compliance).
#[test]
fn i2_constitutive_law_is_real() {
    let g = 4.0;
    let mut thin = Material::fluidish(0.3);
    thin.n = 0.6;
    let mut thick = Material::fluidish(0.3);
    thick.n = 1.4;
    assert!((rheology::eta_app(&thin, g) - rheology::eta_app(&thick, g)).abs() > 1e-6);

    // monotone: higher S ⇒ smaller compliance ⇒ stiffer
    let mut last = f64::INFINITY;
    for &s in &[0.1, 0.3, 0.6, 0.9, 0.99] {
        let a = rheology::compliance(&Material::fluidish(s), 0.05, 1.0);
        assert!(a < last, "compliance not decreasing at s={s} (a={a}, last={last})");
        last = a;
    }
}

/// Stays inside the container (confinement holds through settling).
#[test]
fn confinement_holds() {
    let mut w = World::new_packed(50, 5, 60.0, 52.0, 0xABCDEF);
    w.settle(4000);
    for i in 0..w.pos.len() {
        let r = w.radius[i];
        assert!(w.pos[i].x >= -1e-6 && w.pos[i].x <= w.w + 1e-6, "x out of bounds");
        assert!(w.pos[i].y >= -1e-6 && w.pos[i].y <= w.h + 1e-6, "y out of bounds");
        let _ = r;
    }
}
