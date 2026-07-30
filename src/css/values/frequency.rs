//! `Frequency` — a CSS frequency value with a unit (Hz or kHz).

use std::fmt;

use super::numeric::FloatConvert;

/// CSS frequency units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyUnit {
    /// Hertz (`hz`).
    Hz,
    /// Kilohertz (`khz`).
    Khz,
}

impl FrequencyUnit {
    /// The CSS source suffix for this unit.
    pub fn suffix(self) -> &'static str {
        match self {
            FrequencyUnit::Hz => "hz",
            FrequencyUnit::Khz => "khz",
        }
    }
}

/// A CSS frequency value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frequency(pub(crate) f32, pub(crate) FrequencyUnit);

impl Frequency {
    /// Construct from hertz.
    pub fn hz(value: f32) -> Self {
        Frequency(value, FrequencyUnit::Hz)
    }

    /// Construct from kilohertz.
    pub fn khz(value: f32) -> Self {
        Frequency(value, FrequencyUnit::Khz)
    }

    /// Return the magnitude.
    pub fn value(self) -> f32 {
        self.0
    }

    /// Return the unit.
    pub fn unit(self) -> FrequencyUnit {
        self.1
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1.suffix())
    }
}

impl FloatConvert for Frequency {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hz_constructor() {
        let fr = Frequency::hz(440.0);
        assert_eq!(fr.value(), 440.0);
        assert_eq!(fr.unit(), FrequencyUnit::Hz);
    }

    #[test]
    fn khz_constructor() {
        let fr = Frequency::khz(20.0);
        assert_eq!(fr.value(), 20.0);
        assert_eq!(fr.unit(), FrequencyUnit::Khz);
    }

    #[test]
    fn display_hz() {
        assert_eq!(Frequency::hz(440.0).to_string(), "440hz");
    }

    #[test]
    fn display_khz() {
        assert_eq!(Frequency::khz(20.0).to_string(), "20khz");
    }

    #[test]
    fn display_zero() {
        assert_eq!(Frequency::hz(0.0).to_string(), "0hz");
    }

    #[test]
    fn to_f64_roundtrip() {
        assert_eq!(Frequency::hz(100.0).to_f64(), 100.0);
        assert_eq!(Frequency::khz(20.0).to_f64(), 20.0);
    }

    #[test]
    fn unit_suffix_strings() {
        assert_eq!(FrequencyUnit::Hz.suffix(), "hz");
        assert_eq!(FrequencyUnit::Khz.suffix(), "khz");
    }

    #[test]
    fn equality() {
        assert_eq!(Frequency::hz(100.0), Frequency::hz(100.0));
        assert_ne!(Frequency::hz(100.0), Frequency::khz(100.0));
        assert_ne!(Frequency::hz(100.0), Frequency::hz(200.0));
    }

    #[test]
    fn clone_copy() {
        let f = Frequency::hz(50.0);
        let f2 = f;
        assert_eq!(f, f2);
    }
}