//! The constitutive law from the NNUI_CELL PoC, §4-§5.
//!
//! This module is the whole physical heart. Every number a region "feels" comes
//! from here: how stiff it is, how it yields, how it rebuilds structure at rest,
//! and — crucially — how its material state becomes an XPBD constraint compliance.
//!
//! Invariant I2 (paper §8): this is a *real* constitutive law evaluated per frame.
//! Changing `n` changes behaviour; changing `s` spans liquid→locked. There is no
//! easing curve or spring preset anywhere.

/// Per-region material parameters — the "dials" (paper appendix).
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// Solidity S ∈ [0,1). Liquid at 0, locked at →1. Drives yield & consistency.
    pub s: f64,
    /// Flow index n. n<1 shear-thinning (post-yield), n>1 thickening.
    pub n: f64,
    /// Thixotropic structure ξ ∈ [0,1]. Erodes under shear, rebuilds at rest.
    pub xi: f64,
    /// Rebuild timescale T_r (seconds). Smaller = oozes/settles faster.
    pub t_r: f64,
    /// Structure breakdown coefficient a.
    pub a: f64,
}

impl Material {
    /// A sensible mid-range material.
    pub fn fluidish(s: f64) -> Self {
        Material {
            s,
            n: 0.8,     // shear-thinning post-yield
            xi: 1.0,    // starts fully structured
            t_r: 0.25,  // quarter-second rebuild
            a: 6.0,
        }
    }
}

// ---- constitutive constants (paper §4). Fixed for the prototype. ----
const TAU_Y0: f64 = 1.0; // base yield scale
const KAPPA: f64 = 1.5; // yield shaping exponent κ
const K0: f64 = 1.0; // base consistency
const BETA: f64 = 2.0; // consistency gain with solidity
const GAMMA_C: f64 = 8.0; // critical shear rate γ_c (impact hardening onset)
const Q: f64 = 2.0; // impact-hardening exponent q
const PAPANASTASIOU_M: f64 = 50.0; // yield regularization m

/// Solidity-modulated yield scale τ_y(S) = τ_y0 · (S/(1-S))^κ  (paper eq. 2).
/// The pole at S→1 is deliberate: yield diverges ⇒ region locks.
#[inline]
pub fn tau_y_of_s(s: f64) -> f64 {
    // clamp just below 1 to keep the pole finite-but-huge in f64.
    let s = s.clamp(0.0, 1.0 - 1e-9);
    let ratio = s / (1.0 - s);
    TAU_Y0 * ratio.powf(KAPPA)
}

/// Consistency K(S) = K0 (1 + β S)  (paper §4.1).
#[inline]
pub fn k_of_s(s: f64) -> f64 {
    K0 * (1.0 + BETA * s)
}

/// Rate-dependent effective yield (paper eq. 5, the "dual-rate response"):
/// τ_y_eff = τ_y(S) · ξ · (1 + (γ̇/γ_c)^q).
/// Slow input sees ~τ_y(S)ξ and flows; a fast flick spikes the yield and is rejected.
#[inline]
pub fn tau_y_eff(mat: &Material, gamma_dot: f64) -> f64 {
    let base = tau_y_of_s(mat.s) * mat.xi;
    let hardening = 1.0 + (gamma_dot.max(0.0) / GAMMA_C).powf(Q);
    base * hardening
}

/// Papanastasiou-regularized apparent viscosity (paper eq. 4):
/// η_app = τ_y_eff·(1 - e^{-m γ̇})/γ̇ + K(S)·γ̇^{n-1}.
/// Returns a large-but-finite viscosity as γ̇→0 (rigid), which is what we want.
#[inline]
pub fn eta_app(mat: &Material, gamma_dot: f64) -> f64 {
    let g = gamma_dot.max(1e-9);
    let ty = tau_y_eff(mat, g);
    let yield_term = ty * (1.0 - (-PAPANASTASIOU_M * g).exp()) / g;
    let flow_term = k_of_s(mat.s) * g.powf(mat.n - 1.0);
    yield_term + flow_term
}

/// Map apparent viscosity → XPBD compliance α (paper eq. 12): α = 1/(k0·g(η_app)).
/// We use g(η)=1+η so stiffness rises with viscosity and α→0 as η→∞ (locked).
/// `alpha_base` sets the softest (most compliant) end of the scale.
#[inline]
pub fn compliance(mat: &Material, gamma_dot: f64, alpha_base: f64) -> f64 {
    let eta = eta_app(mat, gamma_dot);
    // stiffness g(η); alpha = alpha_base / g(η).
    let g = 1.0 + eta;
    alpha_base / g
}

/// Advance thixotropic structure one step (paper eq. 7):
/// dξ/dt = (1/T_r)(1-ξ) - a·ξ·γ̇, explicit Euler, clamped to [0,1].
/// At rest ξ→1 (stiffens, drives settling); under shear ξ falls (softens, flows).
#[inline]
pub fn step_structure(mat: &Material, gamma_dot: f64, dt: f64) -> f64 {
    let rebuild = (1.0 - mat.xi) / mat.t_r;
    let breakdown = mat.a * mat.xi * gamma_dot.max(0.0);
    let next = mat.xi + dt * (rebuild - breakdown);
    next.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yield_diverges_as_solidity_rises() {
        assert!(tau_y_of_s(0.99) > tau_y_of_s(0.5));
        assert!(tau_y_of_s(0.9999) > 1e3);
    }

    #[test]
    fn compliance_falls_toward_zero_as_locked() {
        let liquid = Material::fluidish(0.1);
        let solid = Material::fluidish(0.999);
        let a_liq = compliance(&liquid, 0.01, 1.0);
        let a_sol = compliance(&solid, 0.01, 1.0);
        // locked region is far stiffer (much smaller compliance)
        assert!(a_sol < a_liq * 1e-3, "a_sol={a_sol} a_liq={a_liq}");
    }

    #[test]
    fn flow_index_changes_behaviour() {
        // Invariant I2: n actually matters.
        let mut thin = Material::fluidish(0.3);
        thin.n = 0.6;
        let mut thick = Material::fluidish(0.3);
        thick.n = 1.4;
        let g = 5.0;
        assert!((eta_app(&thin, g) - eta_app(&thick, g)).abs() > 1e-6);
    }

    #[test]
    fn structure_rebuilds_at_rest() {
        let mut m = Material::fluidish(0.3);
        m.xi = 0.2;
        let next = step_structure(&m, 0.0, 0.05);
        assert!(next > m.xi, "structure should climb at rest");
    }
}
