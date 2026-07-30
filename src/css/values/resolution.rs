//! `Resolution` — a CSS resolution value (dpi, dpcm, dppx, x).

use std::fmt;

use super::numeric::FloatConvert;

/// CSS resolution units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionUnit {
    /// Dots per inch (`dpi`).
    Dpi,
    /// Dots per centimeter (`dpcm`).
    Dpcm,
    /// Dots per CSS pixel (`dppx`).
    Dppx,
    /// Alias for `dppx` (`x`).
    X,
}

impl ResolutionUnit {
    /// The CSS source suffix for this unit.
    pub fn suffix(self) -> &'static str {
        match self {
            ResolutionUnit::Dpi => "dpi",
            ResolutionUnit::Dpcm => "dpcm",
            ResolutionUnit::Dppx => "dppx",
            ResolutionUnit::X => "x",
        }
    }
}

/// A CSS resolution value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resolution(pub(crate) f32, pub(crate) ResolutionUnit);

impl Resolution {
    /// Construct from dpi.
    pub fn dpi(value: f32) -> Self {
        Resolution(value, ResolutionUnit::Dpi)
    }

    /// Construct from dpcm.
    pub fn dpcm(value: f32) -> Self {
        Resolution(value, ResolutionUnit::Dpcm)
    }

    /// Construct from dppx.
    pub fn dppx(value: f32) -> Self {
        Resolution(value, ResolutionUnit::Dppx)
    }

    /// Construct from `x` (alias for `dppx`).
    pub fn x(value: f32) -> Self {
        Resolution(value, ResolutionUnit::X)
    }

    /// Return the magnitude.
    pub fn value(self) -> f32 {
        self.0
    }

    /// Return the unit.
    pub fn unit(self) -> ResolutionUnit {
        self.1
    }
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1.suffix())
    }
}

impl FloatConvert for Resolution {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dpi_constructor() {
        let r = Resolution::dpi(96.0);
        assert_eq!(r.value(), 96.0);
        assert_eq!(r.unit(), ResolutionUnit::Dpi);
    }

    #[test]
    fn dpcm_constructor() {
        let r = Resolution::dpcm(38.0);
        assert_eq!(r.unit(), ResolutionUnit::Dpcm);
    }

    #[test]
    fn dppx_constructor() {
        let r = Resolution::dppx(2.0);
        assert_eq!(r.unit(), ResolutionUnit::Dppx);
    }

    #[test]
    fn x_constructor() {
        let r = Resolution::x(1.0);
        assert_eq!(r.unit(), ResolutionUnit::X);
    }

    #[test]
    fn display_dpi() {
        assert_eq!(Resolution::dpi(96.0).to_string(), "96dpi");
    }

    #[test]
    fn display_dpcm() {
        assert_eq!(Resolution::dpcm(38.0).to_string(), "38dpcm");
    }

    #[test]
    fn display_dppx() {
        assert_eq!(Resolution::dppx(2.0).to_string(), "2dppx");
    }

    #[test]
    fn display_x() {
        assert_eq!(Resolution::x(1.0).to_string(), "1x");
    }

    #[test]
    fn display_zero() {
        assert_eq!(Resolution::dpi(0.0).to_string(), "0dpi");
    }

    #[test]
    fn to_f64_roundtrip() {
        assert_eq!(Resolution::dpi(96.0).to_f64(), 96.0);
        assert_eq!(Resolution::x(2.0).to_f64(), 2.0);
    }

    #[test]
    fn unit_suffix_strings() {
        assert_eq!(ResolutionUnit::Dpi.suffix(), "dpi");
        assert_eq!(ResolutionUnit::Dpcm.suffix(), "dpcm");
        assert_eq!(ResolutionUnit::Dppx.suffix(), "dppx");
        assert_eq!(ResolutionUnit::X.suffix(), "x");
    }

    #[test]
    fn equality() {
        assert_eq!(Resolution::dpi(96.0), Resolution::dpi(96.0));
        assert_ne!(Resolution::dpi(96.0), Resolution::dppx(96.0));
        assert_ne!(Resolution::x(1.0), Resolution::x(2.0));
    }

    #[test]
    fn clone_copy() {
        let r = Resolution::x(2.0);
        let r2 = r;
        assert_eq!(r, r2);
    }
}