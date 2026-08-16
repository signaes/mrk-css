//! `Percentage` — a CSS percentage value.
//!
//! Represented as an `f32` magnitude in `[0.0, 100.0]`. Constructors
//! clamp out-of-range values to the closest valid bound.

use std::fmt;

use super::numeric::FloatConvert;

/// A percentage value. Internally `f32`; CSS prints as a number
/// followed by `%`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(pub(crate) f32);

impl Percentage {
    /// Construct a new `Percentage`, clamping to `[0.0, 100.0]`.
    pub fn new(value: f32) -> Self {
        Percentage(value.clamp(0.0, 100.0))
    }

    /// Construct a `Percentage` without clamping.
    ///
    /// Useful for values inside `calc()` where percentages outside the
    /// `[0, 100]` range are valid.
    pub fn from_raw(value: f32) -> Self {
        Percentage(value)
    }

    /// Return the inner magnitude.
    pub fn value(self) -> f32 {
        self.0
    }
}

impl Default for Percentage {
    fn default() -> Self {
        Percentage(0.0)
    }
}

impl From<f32> for Percentage {
    fn from(v: f32) -> Self {
        Percentage::new(v)
    }
}

impl From<f64> for Percentage {
    fn from(v: f64) -> Self {
        Percentage::new(v as f32)
    }
}

impl From<i32> for Percentage {
    fn from(v: i32) -> Self {
        Percentage::new(v as f32)
    }
}

impl From<u8> for Percentage {
    fn from(v: u8) -> Self {
        Percentage::new(v as f32)
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl FloatConvert for Percentage {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamps_above_100() {
        assert_eq!(Percentage::new(150.0).value(), 100.0);
    }

    #[test]
    fn new_clamps_below_0() {
        assert_eq!(Percentage::new(-5.0).value(), 0.0);
    }

    #[test]
    fn new_keeps_in_range() {
        assert_eq!(Percentage::new(50.0).value(), 50.0);
    }

    #[test]
    fn default_is_zero() {
        assert_eq!(Percentage::default().value(), 0.0);
    }

    #[test]
    fn from_f32() {
        assert_eq!(Percentage::from(33.5_f32).value(), 33.5);
    }

    #[test]
    fn from_f64() {
        assert_eq!(Percentage::from(33.5_f64).value(), 33.5);
    }

    #[test]
    fn from_i32() {
        assert_eq!(Percentage::from(75_i32).value(), 75.0);
    }

    #[test]
    fn from_u8() {
        assert_eq!(Percentage::from(100_u8).value(), 100.0);
    }

    #[test]
    fn from_i32_clamps() {
        assert_eq!(Percentage::from(200_i32).value(), 100.0);
        assert_eq!(Percentage::from(-10_i32).value(), 0.0);
    }

    #[test]
    fn display_integer() {
        assert_eq!(Percentage::new(50.0).to_string(), "50%");
    }

    #[test]
    fn display_decimal() {
        assert_eq!(Percentage::new(33.333).to_string(), "33.333%");
    }

    #[test]
    fn display_zero() {
        assert_eq!(Percentage::new(0.0).to_string(), "0%");
    }

    #[test]
    fn display_full() {
        assert_eq!(Percentage::new(100.0).to_string(), "100%");
    }

    #[test]
    fn to_f64_roundtrip() {
        let p = Percentage::new(75.0);
        assert_eq!(p.to_f64(), 75.0);
    }

    #[test]
    fn equality() {
        assert_eq!(Percentage::new(50.0), Percentage::new(50.0));
        assert_ne!(Percentage::new(50.0), Percentage::new(51.0));
    }

    #[test]
    fn clone_copy() {
        let p = Percentage::new(25.0);
        let p2 = p; // Copy
        assert_eq!(p, p2);
    }
}
