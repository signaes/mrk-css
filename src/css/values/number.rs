//! `Number` and `Integer` — CSS number values.

use std::fmt;

use super::numeric::FloatConvert;

/// A CSS number value (a real number with no units).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(pub(crate) f32);

impl Number {
    /// Construct from an `f32`.
    pub fn new(value: f32) -> Self {
        Number(value)
    }

    /// Return the value.
    pub fn value(self) -> f32 {
        self.0
    }
}

impl Default for Number {
    fn default() -> Self {
        Number(0.0)
    }
}

impl From<f32> for Number {
    fn from(v: f32) -> Self {
        Number(v)
    }
}

impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number(v as f32)
    }
}

impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Number(v as f32)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FloatConvert for Number {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

/// A CSS integer value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Integer(pub(crate) i32);

impl Integer {
    /// Construct from an `i32`.
    pub fn new(value: i32) -> Self {
        Integer(value)
    }

    /// Return the value.
    pub fn value(self) -> i32 {
        self.0
    }
}

impl From<i32> for Integer {
    fn from(v: i32) -> Self {
        Integer(v)
    }
}

impl From<u32> for Integer {
    fn from(v: u32) -> Self {
        Integer(v as i32)
    }
}

impl From<u8> for Integer {
    fn from(v: u8) -> Self {
        Integer(v as i32)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FloatConvert for Integer {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::numeric::FloatConvert;

    #[test]
    fn number_constructor() {
        assert_eq!(Number::new(1.5).value(), 1.5);
    }

    #[test]
    fn number_default_is_zero() {
        assert_eq!(Number::default().value(), 0.0);
    }

    #[test]
    fn number_from_f32() {
        assert_eq!(Number::from(2.5_f32).value(), 2.5);
    }

    #[test]
    fn number_from_f64() {
        assert_eq!(Number::from(2.5_f64).value(), 2.5);
    }

    #[test]
    fn number_from_i32() {
        assert_eq!(Number::from(7_i32).value(), 7.0);
    }

    #[test]
    fn number_display() {
        assert_eq!(Number::new(1.5).to_string(), "1.5");
    }

    #[test]
    fn number_display_zero() {
        assert_eq!(Number::new(0.0).to_string(), "0");
    }

    #[test]
    fn number_display_negative() {
        assert_eq!(Number::new(-2.25).to_string(), "-2.25");
    }

    #[test]
    fn number_to_f64() {
        // f32 → f64 promotion is exact (no rounding); we compare
        // against the f64 cast of the same f32 to avoid f32/f64
        // representation drift.
        let v = Number::new(3.5_f32).to_f64();
        assert_eq!(v, 3.5_f64);
    }

    #[test]
    fn number_equality() {
        assert_eq!(Number::new(1.5), Number::new(1.5));
        assert_ne!(Number::new(1.5), Number::new(2.5));
    }

    #[test]
    fn integer_constructor() {
        assert_eq!(Integer::new(42).value(), 42);
    }

    #[test]
    fn integer_default_is_zero() {
        assert_eq!(Integer::default().value(), 0);
    }

    #[test]
    fn integer_from_i32() {
        assert_eq!(Integer::from(-7_i32).value(), -7);
    }

    #[test]
    fn integer_from_u32() {
        assert_eq!(Integer::from(8_u32).value(), 8);
    }

    #[test]
    fn integer_from_u8() {
        assert_eq!(Integer::from(255_u8).value(), 255);
    }

    #[test]
    fn integer_display() {
        assert_eq!(Integer::new(42).to_string(), "42");
    }

    #[test]
    fn integer_display_zero() {
        assert_eq!(Integer::new(0).to_string(), "0");
    }

    #[test]
    fn integer_display_negative() {
        assert_eq!(Integer::new(-7).to_string(), "-7");
    }

    #[test]
    fn integer_to_f64() {
        assert_eq!(Integer::new(42).to_f64(), 42.0);
    }

    #[test]
    fn integer_equality() {
        assert_eq!(Integer::new(42), Integer::new(42));
        assert_ne!(Integer::new(42), Integer::new(43));
    }

    #[test]
    fn clone_copy_number() {
        let n = Number::new(1.5);
        let n2 = n;
        assert_eq!(n, n2);
    }

    #[test]
    fn clone_copy_integer() {
        let i = Integer::new(42);
        let i2 = i;
        assert_eq!(i, i2);
    }
}