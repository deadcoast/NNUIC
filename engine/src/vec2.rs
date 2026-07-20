//! Deterministic 2D f64 vector math. No external dependencies; every
//! operation is a fixed sequence of IEEE-754 ops so results are reproducible
//! run-to-run (paper §5.4).
//!
//! Reconstructed for TK09 against the exact call surface `solver.rs` uses:
//! `new`, `zero`, `add`, `sub`, `scale`, `dot`, `len`, `norm_or`.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
    #[inline]
    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
    #[inline]
    pub fn add(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
    #[inline]
    pub fn sub(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
    #[inline]
    pub fn scale(self, k: f64) -> Vec2 {
        Vec2::new(self.x * k, self.y * k)
    }
    #[inline]
    pub fn dot(self, o: Vec2) -> f64 {
        self.x * o.x + self.y * o.y
    }
    #[inline]
    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    /// Unit vector, or `fallback` when the length is degenerate.
    #[inline]
    pub fn norm_or(self, fallback: Vec2) -> Vec2 {
        let l = self.len();
        if l > 1e-12 {
            self.scale(1.0 / l)
        } else {
            fallback
        }
    }
}
