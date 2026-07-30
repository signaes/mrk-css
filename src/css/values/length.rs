//! `Length` — a CSS length value with a unit.

use std::fmt;

use super::numeric::FloatConvert;

/// CSS length units.
///
/// Includes absolute units (`px`, `cm`, `mm`, `in`, `pt`, `pc`, `q`),
/// font-relative units (`em`, `rem`, `ex`, `ch`), viewport-relative
/// units (`vw`, `vh`, `vmin`, `vmax`), percentage, and the grid
/// fractional unit (`fr`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    /// Pixels.
    Px,
    /// Em (font size of the element).
    Em,
    /// Rem (root em).
    Rem,
    /// Ex (x-height of the font).
    Ex,
    /// Ch (advance of "0" glyph).
    Ch,
    /// 1% of viewport width.
    Vw,
    /// 1% of viewport height.
    Vh,
    /// 1% of the smaller of vw/vh.
    Vmin,
    /// 1% of the larger of vw/vh.
    Vmax,
    /// Percentage.
    Percentage,
    /// Centimeters.
    Cm,
    /// Millimeters.
    Mm,
    /// Inches.
    In,
    /// Points (1/72 inch).
    Pt,
    /// Picas (12 points).
    Pc,
    /// Quarter-millimeters.
    Q,
    /// Grid fractional unit.
    Fr,
}

impl LengthUnit {
    /// The CSS source suffix for this unit.
    pub fn suffix(self) -> &'static str {
        match self {
            LengthUnit::Px => "px",
            LengthUnit::Em => "em",
            LengthUnit::Rem => "rem",
            LengthUnit::Ex => "ex",
            LengthUnit::Ch => "ch",
            LengthUnit::Vw => "vw",
            LengthUnit::Vh => "vh",
            LengthUnit::Vmin => "vmin",
            LengthUnit::Vmax => "vmax",
            LengthUnit::Percentage => "%",
            LengthUnit::Cm => "cm",
            LengthUnit::Mm => "mm",
            LengthUnit::In => "in",
            LengthUnit::Pt => "pt",
            LengthUnit::Pc => "pc",
            LengthUnit::Q => "q",
            LengthUnit::Fr => "fr",
        }
    }
}

/// A CSS length value. Stored in the unit supplied at construction;
/// printed with the corresponding suffix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length(pub(crate) f32, pub(crate) LengthUnit);

impl Length {
    /// Construct from pixels.
    pub fn px(value: f32) -> Self {
        Length(value, LengthUnit::Px)
    }

    /// Construct from ems.
    pub fn em(value: f32) -> Self {
        Length(value, LengthUnit::Em)
    }

    /// Construct from rems.
    pub fn rem(value: f32) -> Self {
        Length(value, LengthUnit::Rem)
    }

    /// Construct from exes.
    pub fn ex(value: f32) -> Self {
        Length(value, LengthUnit::Ex)
    }

    /// Construct from ch.
    pub fn ch(value: f32) -> Self {
        Length(value, LengthUnit::Ch)
    }

    /// Construct from vw (1% of viewport width).
    pub fn vw(value: f32) -> Self {
        Length(value, LengthUnit::Vw)
    }

    /// Construct from vh (1% of viewport height).
    pub fn vh(value: f32) -> Self {
        Length(value, LengthUnit::Vh)
    }

    /// Construct from vmin.
    pub fn vmin(value: f32) -> Self {
        Length(value, LengthUnit::Vmin)
    }

    /// Construct from vmax.
    pub fn vmax(value: f32) -> Self {
        Length(value, LengthUnit::Vmax)
    }

    /// Construct from percent.
    pub fn pct(value: f32) -> Self {
        Length(value, LengthUnit::Percentage)
    }

    /// Construct from centimeters.
    pub fn cm(value: f32) -> Self {
        Length(value, LengthUnit::Cm)
    }

    /// Construct from millimeters.
    pub fn mm(value: f32) -> Self {
        Length(value, LengthUnit::Mm)
    }

    /// Construct from inches.
    pub fn inches(value: f32) -> Self {
        Length(value, LengthUnit::In)
    }

    /// Construct from points.
    pub fn pt(value: f32) -> Self {
        Length(value, LengthUnit::Pt)
    }

    /// Construct from picas.
    pub fn pc(value: f32) -> Self {
        Length(value, LengthUnit::Pc)
    }

    /// Construct from quarter-millimeters.
    pub fn q(value: f32) -> Self {
        Length(value, LengthUnit::Q)
    }

    /// Construct from grid fractional units.
    pub fn fr(value: f32) -> Self {
        Length(value, LengthUnit::Fr)
    }

    /// Return the magnitude.
    pub fn value(self) -> f32 {
        self.0
    }

    /// Return the unit.
    pub fn unit(self) -> LengthUnit {
        self.1
    }
}

impl Default for Length {
    fn default() -> Self {
        Length(0.0, LengthUnit::Px)
    }
}

impl From<f32> for Length {
    /// Assumes pixels.
    fn from(v: f32) -> Self {
        Length::px(v)
    }
}

impl From<f64> for Length {
    /// Assumes pixels.
    fn from(v: f64) -> Self {
        Length::px(v as f32)
    }
}

impl From<i32> for Length {
    /// Assumes pixels.
    fn from(v: i32) -> Self {
        Length::px(v as f32)
    }
}

impl From<u32> for Length {
    /// Assumes pixels.
    fn from(v: u32) -> Self {
        Length::px(v as f32)
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1.suffix())
    }
}

impl FloatConvert for Length {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn px_constructor() {
        let l = Length::px(4.0);
        assert_eq!(l.value(), 4.0);
        assert_eq!(l.unit(), LengthUnit::Px);
    }

    #[test]
    fn em_constructor() {
        let l = Length::em(1.5);
        assert_eq!(l.unit(), LengthUnit::Em);
    }

    #[test]
    fn rem_constructor() {
        let l = Length::rem(2.0);
        assert_eq!(l.unit(), LengthUnit::Rem);
    }

    #[test]
    fn ex_constructor() {
        let l = Length::ex(3.0);
        assert_eq!(l.unit(), LengthUnit::Ex);
    }

    #[test]
    fn ch_constructor() {
        let l = Length::ch(1.0);
        assert_eq!(l.unit(), LengthUnit::Ch);
    }

    #[test]
    fn vw_constructor() {
        let l = Length::vw(50.0);
        assert_eq!(l.unit(), LengthUnit::Vw);
    }

    #[test]
    fn vh_constructor() {
        let l = Length::vh(100.0);
        assert_eq!(l.unit(), LengthUnit::Vh);
    }

    #[test]
    fn vmin_constructor() {
        let l = Length::vmin(10.0);
        assert_eq!(l.unit(), LengthUnit::Vmin);
    }

    #[test]
    fn vmax_constructor() {
        let l = Length::vmax(10.0);
        assert_eq!(l.unit(), LengthUnit::Vmax);
    }

    #[test]
    fn pct_constructor() {
        let l = Length::pct(50.0);
        assert_eq!(l.unit(), LengthUnit::Percentage);
    }

    #[test]
    fn cm_constructor() {
        let l = Length::cm(1.0);
        assert_eq!(l.unit(), LengthUnit::Cm);
    }

    #[test]
    fn mm_constructor() {
        let l = Length::mm(10.0);
        assert_eq!(l.unit(), LengthUnit::Mm);
    }

    #[test]
    fn inches_constructor() {
        let l = Length::inches(1.0);
        assert_eq!(l.unit(), LengthUnit::In);
    }

    #[test]
    fn pt_constructor() {
        let l = Length::pt(12.0);
        assert_eq!(l.unit(), LengthUnit::Pt);
    }

    #[test]
    fn pc_constructor() {
        let l = Length::pc(6.0);
        assert_eq!(l.unit(), LengthUnit::Pc);
    }

    #[test]
    fn q_constructor() {
        let l = Length::q(40.0);
        assert_eq!(l.unit(), LengthUnit::Q);
    }

    #[test]
    fn fr_constructor() {
        let l = Length::fr(2.0);
        assert_eq!(l.unit(), LengthUnit::Fr);
    }

    #[test]
    fn default_is_zero_px() {
        assert_eq!(Length::default(), Length::px(0.0));
    }

    #[test]
    fn from_f32() {
        assert_eq!(Length::from(4.5_f32), Length::px(4.5));
    }

    #[test]
    fn from_f64() {
        assert_eq!(Length::from(4.5_f64), Length::px(4.5));
    }

    #[test]
    fn from_i32() {
        assert_eq!(Length::from(8_i32), Length::px(8.0));
    }

    #[test]
    fn from_u32() {
        assert_eq!(Length::from(8_u32), Length::px(8.0));
    }

    #[test]
    fn display_each_unit() {
        assert_eq!(Length::px(4.0).to_string(), "4px");
        assert_eq!(Length::em(1.5).to_string(), "1.5em");
        assert_eq!(Length::rem(2.0).to_string(), "2rem");
        assert_eq!(Length::ex(3.0).to_string(), "3ex");
        assert_eq!(Length::ch(1.0).to_string(), "1ch");
        assert_eq!(Length::vw(50.0).to_string(), "50vw");
        assert_eq!(Length::vh(100.0).to_string(), "100vh");
        assert_eq!(Length::vmin(10.0).to_string(), "10vmin");
        assert_eq!(Length::vmax(10.0).to_string(), "10vmax");
        assert_eq!(Length::pct(50.0).to_string(), "50%");
        assert_eq!(Length::cm(1.0).to_string(), "1cm");
        assert_eq!(Length::mm(10.0).to_string(), "10mm");
        assert_eq!(Length::inches(1.0).to_string(), "1in");
        assert_eq!(Length::pt(12.0).to_string(), "12pt");
        assert_eq!(Length::pc(6.0).to_string(), "6pc");
        assert_eq!(Length::q(40.0).to_string(), "40q");
        assert_eq!(Length::fr(2.0).to_string(), "2fr");
    }

    #[test]
    fn display_zero_px() {
        assert_eq!(Length::px(0.0).to_string(), "0px");
    }

    #[test]
    fn to_f64_roundtrip() {
        assert_eq!(Length::px(4.0).to_f64(), 4.0);
        assert_eq!(Length::em(1.5).to_f64(), 1.5);
    }

    #[test]
    fn unit_suffix_strings() {
        assert_eq!(LengthUnit::Px.suffix(), "px");
        assert_eq!(LengthUnit::Em.suffix(), "em");
        assert_eq!(LengthUnit::Rem.suffix(), "rem");
        assert_eq!(LengthUnit::Ex.suffix(), "ex");
        assert_eq!(LengthUnit::Ch.suffix(), "ch");
        assert_eq!(LengthUnit::Vw.suffix(), "vw");
        assert_eq!(LengthUnit::Vh.suffix(), "vh");
        assert_eq!(LengthUnit::Vmin.suffix(), "vmin");
        assert_eq!(LengthUnit::Vmax.suffix(), "vmax");
        assert_eq!(LengthUnit::Percentage.suffix(), "%");
        assert_eq!(LengthUnit::Cm.suffix(), "cm");
        assert_eq!(LengthUnit::Mm.suffix(), "mm");
        assert_eq!(LengthUnit::In.suffix(), "in");
        assert_eq!(LengthUnit::Pt.suffix(), "pt");
        assert_eq!(LengthUnit::Pc.suffix(), "pc");
        assert_eq!(LengthUnit::Q.suffix(), "q");
        assert_eq!(LengthUnit::Fr.suffix(), "fr");
    }

    #[test]
    fn equality() {
        assert_eq!(Length::px(4.0), Length::px(4.0));
        assert_ne!(Length::px(4.0), Length::em(4.0));
        assert_ne!(Length::px(4.0), Length::px(5.0));
    }

    #[test]
    fn clone_copy() {
        let l = Length::px(4.0);
        let l2 = l;
        assert_eq!(l, l2);
    }
}