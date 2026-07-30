//! `Time` — a CSS time value with a unit (seconds or milliseconds).

use std::fmt;

use super::numeric::FloatConvert;

/// CSS time units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    /// Seconds (`s`).
    S,
    /// Milliseconds (`ms`).
    Ms,
}

impl TimeUnit {
    /// The CSS source suffix for this unit.
    pub fn suffix(self) -> &'static str {
        match self {
            TimeUnit::S => "s",
            TimeUnit::Ms => "ms",
        }
    }
}

/// A CSS time value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time(pub(crate) f32, pub(crate) TimeUnit);

impl Time {
    /// Construct from seconds.
    pub fn s(value: f32) -> Self {
        Time(value, TimeUnit::S)
    }

    /// Construct from milliseconds.
    pub fn ms(value: f32) -> Self {
        Time(value, TimeUnit::Ms)
    }

    /// Return the magnitude.
    pub fn value(self) -> f32 {
        self.0
    }

    /// Return the unit.
    pub fn unit(self) -> TimeUnit {
        self.1
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1.suffix())
    }
}

impl FloatConvert for Time {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_constructor() {
        let t = Time::s(1.5);
        assert_eq!(t.value(), 1.5);
        assert_eq!(t.unit(), TimeUnit::S);
    }

    #[test]
    fn ms_constructor() {
        let t = Time::ms(500.0);
        assert_eq!(t.value(), 500.0);
        assert_eq!(t.unit(), TimeUnit::Ms);
    }

    #[test]
    fn display_seconds() {
        assert_eq!(Time::s(1.5).to_string(), "1.5s");
    }

    #[test]
    fn display_seconds_zero() {
        assert_eq!(Time::s(0.0).to_string(), "0s");
    }

    #[test]
    fn display_milliseconds() {
        assert_eq!(Time::ms(500.0).to_string(), "500ms");
    }

    #[test]
    fn display_milliseconds_zero() {
        assert_eq!(Time::ms(0.0).to_string(), "0ms");
    }

    #[test]
    fn to_f64_roundtrip() {
        assert_eq!(Time::s(2.5).to_f64(), 2.5);
        assert_eq!(Time::ms(750.0).to_f64(), 750.0);
    }

    #[test]
    fn unit_suffix_strings() {
        assert_eq!(TimeUnit::S.suffix(), "s");
        assert_eq!(TimeUnit::Ms.suffix(), "ms");
    }

    #[test]
    fn equality() {
        assert_eq!(Time::s(1.0), Time::s(1.0));
        assert_ne!(Time::s(1.0), Time::ms(1.0));
        assert_ne!(Time::s(1.0), Time::s(2.0));
    }

    #[test]
    fn clone_copy() {
        let t = Time::s(3.0);
        let t2 = t; // Copy
        assert_eq!(t, t2);
    }
}