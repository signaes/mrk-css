//! `Ratio` — a CSS `<ratio>` value.
//!
//! Represents a ratio as two non-negative `f32` values: a numerator and
//! an optional denominator. When the denominator is `1.0`, the value
//! renders as a single number; otherwise it renders as `<num>/<den>`.
//!
//! CSS examples: `16/9`, `1.5`, `4 / 3`.

use std::fmt;

use super::numeric::FloatConvert;

/// A CSS ratio value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ratio {
    numerator: f32,
    denominator: f32,
    explicit_denominator: bool,
}

impl Ratio {
    /// Construct a new `Ratio`.
    ///
    /// Negative values are clamped to `0.0`. A zero or negative
    /// denominator is replaced by `1.0` to avoid invalid CSS.
    pub fn new(numerator: f32, denominator: f32) -> Self {
        Ratio {
            numerator: numerator.max(0.0),
            denominator: if denominator > 0.0 { denominator } else { 1.0 },
            explicit_denominator: true,
        }
    }

    /// Construct a single-number ratio (denominator = 1).
    pub fn from_number(value: f32) -> Self {
        Ratio {
            numerator: value.max(0.0),
            denominator: 1.0,
            explicit_denominator: false,
        }
    }

    /// Return the numerator.
    pub fn numerator(self) -> f32 {
        self.numerator
    }

    /// Return the denominator.
    pub fn denominator(self) -> f32 {
        self.denominator
    }

    /// Return the ratio as a single floating-point value.
    pub fn value(self) -> f32 {
        self.numerator / self.denominator
    }
}

impl Default for Ratio {
    fn default() -> Self {
        Ratio::from_number(1.0)
    }
}

impl From<f32> for Ratio {
    fn from(v: f32) -> Self {
        Ratio::from_number(v)
    }
}

impl From<f64> for Ratio {
    fn from(v: f64) -> Self {
        Ratio::from_number(v as f32)
    }
}

impl From<i32> for Ratio {
    fn from(v: i32) -> Self {
        Ratio::from_number(v as f32)
    }
}

impl From<u8> for Ratio {
    fn from(v: u8) -> Self {
        Ratio::from_number(v as f32)
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1.0 && !self.explicit_denominator {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl FloatConvert for Ratio {
    fn to_f64(self) -> f64 {
        self.value() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_two_numbers() {
        assert_eq!(Ratio::new(16.0, 9.0).to_string(), "16/9");
    }

    #[test]
    fn display_single_number() {
        assert_eq!(Ratio::from_number(1.5).to_string(), "1.5");
    }

    #[test]
    fn clamps_negative_numerator() {
        assert_eq!(Ratio::new(-4.0, 3.0).numerator(), 0.0);
    }

    #[test]
    fn replaces_invalid_denominator() {
        assert_eq!(Ratio::new(4.0, 0.0).denominator(), 1.0);
        assert_eq!(Ratio::new(4.0, -1.0).denominator(), 1.0);
    }

    #[test]
    fn value_computes_division() {
        assert!((Ratio::new(16.0, 9.0).value() - 1.7777).abs() < 1e-4);
    }

    #[test]
    fn from_impls() {
        let _: Ratio = 1.5f32.into();
        let _: Ratio = 2.0f64.into();
        let _: Ratio = 3i32.into();
        let _: Ratio = 4u8.into();
    }
}
