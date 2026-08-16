//! `Length` — a CSS length value with a unit.

use std::fmt;

use super::numeric::FloatConvert;

/// CSS length units.
///
/// Includes absolute units (`px`, `cm`, `mm`, `in`, `pt`, `pc`, `q`),
/// font-relative units (`em`, `rem`, `ex`, `ch`, `cap`, `rcap`, `lh`,
/// `rlh`), viewport-relative units (`vw`, `vh`, `vmin`, `vmax`, `vi`,
/// `vb`), small/large/dynamic viewport units, container query length
/// units, percentage, and the grid fractional unit (`fr`).
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
    /// Cap (height of a capital letter).
    Cap,
    /// Rcap (root cap).
    Rcap,
    /// Lh (line height of the element).
    Lh,
    /// Rlh (root line height).
    Rlh,
    /// 1% of viewport width.
    Vw,
    /// 1% of viewport height.
    Vh,
    /// 1% of the smaller of vw/vh.
    Vmin,
    /// 1% of the larger of vw/vh.
    Vmax,
    /// 1% of the viewport's inline axis.
    Vi,
    /// 1% of the viewport's block axis.
    Vb,
    /// 1% of the small viewport width.
    Svw,
    /// 1% of the small viewport height.
    Svh,
    /// 1% of the smaller of svw/svh.
    Svmin,
    /// 1% of the larger of svw/svh.
    Svmax,
    /// 1% of the small viewport inline axis.
    Svi,
    /// 1% of the small viewport block axis.
    Svb,
    /// 1% of the large viewport width.
    Lvw,
    /// 1% of the large viewport height.
    Lvh,
    /// 1% of the smaller of lvw/lvh.
    Lvmin,
    /// 1% of the larger of lvw/lvh.
    Lvmax,
    /// 1% of the large viewport inline axis.
    Lvi,
    /// 1% of the large viewport block axis.
    Lvb,
    /// 1% of the dynamic viewport width.
    Dvw,
    /// 1% of the dynamic viewport height.
    Dvh,
    /// 1% of the smaller of dvw/dvh.
    Dvmin,
    /// 1% of the larger of dvw/dvh.
    Dvmax,
    /// 1% of the dynamic viewport inline axis.
    Dvi,
    /// 1% of the dynamic viewport block axis.
    Dvb,
    /// 1% of the nearest container's width.
    Cqw,
    /// 1% of the nearest container's height.
    Cqh,
    /// 1% of the nearest container's inline size.
    Cqi,
    /// 1% of the nearest container's block size.
    Cqb,
    /// 1% of the smaller of cqi/cqb.
    Cqmin,
    /// 1% of the larger of cqi/cqb.
    Cqmax,
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
            LengthUnit::Cap => "cap",
            LengthUnit::Rcap => "rcap",
            LengthUnit::Lh => "lh",
            LengthUnit::Rlh => "rlh",
            LengthUnit::Vw => "vw",
            LengthUnit::Vh => "vh",
            LengthUnit::Vmin => "vmin",
            LengthUnit::Vmax => "vmax",
            LengthUnit::Vi => "vi",
            LengthUnit::Vb => "vb",
            LengthUnit::Svw => "svw",
            LengthUnit::Svh => "svh",
            LengthUnit::Svmin => "svmin",
            LengthUnit::Svmax => "svmax",
            LengthUnit::Svi => "svi",
            LengthUnit::Svb => "svb",
            LengthUnit::Lvw => "lvw",
            LengthUnit::Lvh => "lvh",
            LengthUnit::Lvmin => "lvmin",
            LengthUnit::Lvmax => "lvmax",
            LengthUnit::Lvi => "lvi",
            LengthUnit::Lvb => "lvb",
            LengthUnit::Dvw => "dvw",
            LengthUnit::Dvh => "dvh",
            LengthUnit::Dvmin => "dvmin",
            LengthUnit::Dvmax => "dvmax",
            LengthUnit::Dvi => "dvi",
            LengthUnit::Dvb => "dvb",
            LengthUnit::Cqw => "cqw",
            LengthUnit::Cqh => "cqh",
            LengthUnit::Cqi => "cqi",
            LengthUnit::Cqb => "cqb",
            LengthUnit::Cqmin => "cqmin",
            LengthUnit::Cqmax => "cqmax",
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

    /// Construct from cap.
    pub fn cap(value: f32) -> Self {
        Length(value, LengthUnit::Cap)
    }

    /// Construct from rcap.
    pub fn rcap(value: f32) -> Self {
        Length(value, LengthUnit::Rcap)
    }

    /// Construct from lh.
    pub fn lh(value: f32) -> Self {
        Length(value, LengthUnit::Lh)
    }

    /// Construct from rlh.
    pub fn rlh(value: f32) -> Self {
        Length(value, LengthUnit::Rlh)
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

    /// Construct from vi.
    pub fn vi(value: f32) -> Self {
        Length(value, LengthUnit::Vi)
    }

    /// Construct from vb.
    pub fn vb(value: f32) -> Self {
        Length(value, LengthUnit::Vb)
    }

    /// Construct from svw.
    pub fn svw(value: f32) -> Self {
        Length(value, LengthUnit::Svw)
    }

    /// Construct from svh.
    pub fn svh(value: f32) -> Self {
        Length(value, LengthUnit::Svh)
    }

    /// Construct from svmin.
    pub fn svmin(value: f32) -> Self {
        Length(value, LengthUnit::Svmin)
    }

    /// Construct from svmax.
    pub fn svmax(value: f32) -> Self {
        Length(value, LengthUnit::Svmax)
    }

    /// Construct from svi.
    pub fn svi(value: f32) -> Self {
        Length(value, LengthUnit::Svi)
    }

    /// Construct from svb.
    pub fn svb(value: f32) -> Self {
        Length(value, LengthUnit::Svb)
    }

    /// Construct from lvw.
    pub fn lvw(value: f32) -> Self {
        Length(value, LengthUnit::Lvw)
    }

    /// Construct from lvh.
    pub fn lvh(value: f32) -> Self {
        Length(value, LengthUnit::Lvh)
    }

    /// Construct from lvmin.
    pub fn lvmin(value: f32) -> Self {
        Length(value, LengthUnit::Lvmin)
    }

    /// Construct from lvmax.
    pub fn lvmax(value: f32) -> Self {
        Length(value, LengthUnit::Lvmax)
    }

    /// Construct from lvi.
    pub fn lvi(value: f32) -> Self {
        Length(value, LengthUnit::Lvi)
    }

    /// Construct from lvb.
    pub fn lvb(value: f32) -> Self {
        Length(value, LengthUnit::Lvb)
    }

    /// Construct from dvw.
    pub fn dvw(value: f32) -> Self {
        Length(value, LengthUnit::Dvw)
    }

    /// Construct from dvh.
    pub fn dvh(value: f32) -> Self {
        Length(value, LengthUnit::Dvh)
    }

    /// Construct from dvmin.
    pub fn dvmin(value: f32) -> Self {
        Length(value, LengthUnit::Dvmin)
    }

    /// Construct from dvmax.
    pub fn dvmax(value: f32) -> Self {
        Length(value, LengthUnit::Dvmax)
    }

    /// Construct from dvi.
    pub fn dvi(value: f32) -> Self {
        Length(value, LengthUnit::Dvi)
    }

    /// Construct from dvb.
    pub fn dvb(value: f32) -> Self {
        Length(value, LengthUnit::Dvb)
    }

    /// Construct from cqw.
    pub fn cqw(value: f32) -> Self {
        Length(value, LengthUnit::Cqw)
    }

    /// Construct from cqh.
    pub fn cqh(value: f32) -> Self {
        Length(value, LengthUnit::Cqh)
    }

    /// Construct from cqi.
    pub fn cqi(value: f32) -> Self {
        Length(value, LengthUnit::Cqi)
    }

    /// Construct from cqb.
    pub fn cqb(value: f32) -> Self {
        Length(value, LengthUnit::Cqb)
    }

    /// Construct from cqmin.
    pub fn cqmin(value: f32) -> Self {
        Length(value, LengthUnit::Cqmin)
    }

    /// Construct from cqmax.
    pub fn cqmax(value: f32) -> Self {
        Length(value, LengthUnit::Cqmax)
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

    type UnitCase = (fn(f32) -> Length, &'static str, f32, &'static str);

    #[test]
    fn all_unit_constructors_and_display_table() {
        let cases: [UnitCase; 39] = [
            (Length::px, "px", 4.0, "4px"),
            (Length::em, "em", 1.5, "1.5em"),
            (Length::rem, "rem", 2.0, "2rem"),
            (Length::ex, "ex", 3.0, "3ex"),
            (Length::ch, "ch", 1.0, "1ch"),
            (Length::cap, "cap", 2.0, "2cap"),
            (Length::rcap, "rcap", 2.0, "2rcap"),
            (Length::lh, "lh", 1.5, "1.5lh"),
            (Length::rlh, "rlh", 1.5, "1.5rlh"),
            (Length::vw, "vw", 50.0, "50vw"),
            (Length::vh, "vh", 100.0, "100vh"),
            (Length::vmin, "vmin", 10.0, "10vmin"),
            (Length::vmax, "vmax", 10.0, "10vmax"),
            (Length::vi, "vi", 10.0, "10vi"),
            (Length::vb, "vb", 10.0, "10vb"),
            (Length::svw, "svw", 10.0, "10svw"),
            (Length::svh, "svh", 10.0, "10svh"),
            (Length::svmin, "svmin", 10.0, "10svmin"),
            (Length::svmax, "svmax", 10.0, "10svmax"),
            (Length::svi, "svi", 10.0, "10svi"),
            (Length::svb, "svb", 10.0, "10svb"),
            (Length::lvw, "lvw", 10.0, "10lvw"),
            (Length::lvh, "lvh", 10.0, "10lvh"),
            (Length::lvmin, "lvmin", 10.0, "10lvmin"),
            (Length::lvmax, "lvmax", 10.0, "10lvmax"),
            (Length::lvi, "lvi", 10.0, "10lvi"),
            (Length::lvb, "lvb", 10.0, "10lvb"),
            (Length::dvw, "dvw", 10.0, "10dvw"),
            (Length::dvh, "dvh", 10.0, "10dvh"),
            (Length::dvmin, "dvmin", 10.0, "10dvmin"),
            (Length::dvmax, "dvmax", 10.0, "10dvmax"),
            (Length::dvi, "dvi", 10.0, "10dvi"),
            (Length::dvb, "dvb", 10.0, "10dvb"),
            (Length::cqw, "cqw", 10.0, "10cqw"),
            (Length::cqh, "cqh", 10.0, "10cqh"),
            (Length::cqi, "cqi", 10.0, "10cqi"),
            (Length::cqb, "cqb", 10.0, "10cqb"),
            (Length::cqmin, "cqmin", 10.0, "10cqmin"),
            (Length::cqmax, "cqmax", 10.0, "10cqmax"),
        ];
        for (ctor, unit, value, expected) in cases {
            let l = ctor(value);
            assert_eq!(l.unit().suffix(), unit);
            assert_eq!(l.to_string(), expected, "unit {unit}");
        }
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
        assert_eq!(Length::cap(2.0).to_string(), "2cap");
        assert_eq!(Length::rcap(2.0).to_string(), "2rcap");
        assert_eq!(Length::lh(1.5).to_string(), "1.5lh");
        assert_eq!(Length::rlh(1.5).to_string(), "1.5rlh");
        assert_eq!(Length::vw(50.0).to_string(), "50vw");
        assert_eq!(Length::vh(100.0).to_string(), "100vh");
        assert_eq!(Length::vmin(10.0).to_string(), "10vmin");
        assert_eq!(Length::vmax(10.0).to_string(), "10vmax");
        assert_eq!(Length::vi(10.0).to_string(), "10vi");
        assert_eq!(Length::vb(10.0).to_string(), "10vb");
        assert_eq!(Length::svw(10.0).to_string(), "10svw");
        assert_eq!(Length::lvh(10.0).to_string(), "10lvh");
        assert_eq!(Length::dvmin(10.0).to_string(), "10dvmin");
        assert_eq!(Length::cqi(10.0).to_string(), "10cqi");
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
        assert_eq!(LengthUnit::Cap.suffix(), "cap");
        assert_eq!(LengthUnit::Rcap.suffix(), "rcap");
        assert_eq!(LengthUnit::Lh.suffix(), "lh");
        assert_eq!(LengthUnit::Rlh.suffix(), "rlh");
        assert_eq!(LengthUnit::Vw.suffix(), "vw");
        assert_eq!(LengthUnit::Vh.suffix(), "vh");
        assert_eq!(LengthUnit::Vmin.suffix(), "vmin");
        assert_eq!(LengthUnit::Vmax.suffix(), "vmax");
        assert_eq!(LengthUnit::Vi.suffix(), "vi");
        assert_eq!(LengthUnit::Vb.suffix(), "vb");
        assert_eq!(LengthUnit::Svw.suffix(), "svw");
        assert_eq!(LengthUnit::Lvh.suffix(), "lvh");
        assert_eq!(LengthUnit::Dvmin.suffix(), "dvmin");
        assert_eq!(LengthUnit::Cqi.suffix(), "cqi");
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
