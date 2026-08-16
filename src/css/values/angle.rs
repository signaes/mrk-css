//! `Angle` — a CSS angle value with a unit (deg, rad, grad, turn).

use std::fmt;

use super::numeric::FloatConvert;

/// CSS angle units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    /// Degrees (`deg`).
    Deg,
    /// Radians (`rad`).
    Rad,
    /// Gradians (`grad`).
    Grad,
    /// Turns (`turn`).
    Turn,
}

impl AngleUnit {
    /// The CSS source suffix for this unit.
    pub fn suffix(self) -> &'static str {
        match self {
            AngleUnit::Deg => "deg",
            AngleUnit::Rad => "rad",
            AngleUnit::Grad => "grad",
            AngleUnit::Turn => "turn",
        }
    }
}

/// A CSS angle value. Stored in the unit supplied at construction;
/// printed with the corresponding suffix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(pub(crate) f32, pub(crate) AngleUnit);

impl Angle {
    /// Construct from degrees.
    pub fn deg(value: f32) -> Self {
        Angle(value, AngleUnit::Deg)
    }

    /// Construct from radians.
    pub fn rad(value: f32) -> Self {
        Angle(value, AngleUnit::Rad)
    }

    /// Construct from gradians.
    pub fn grad(value: f32) -> Self {
        Angle(value, AngleUnit::Grad)
    }

    /// Construct from turns.
    pub fn turn(value: f32) -> Self {
        Angle(value, AngleUnit::Turn)
    }

    /// Return the magnitude.
    pub fn value(self) -> f32 {
        self.0
    }

    /// Return the unit.
    pub fn unit(self) -> AngleUnit {
        self.1
    }

    /// Convert the angle to degrees regardless of the source unit.
    pub fn to_degrees(self) -> f32 {
        match self.1 {
            AngleUnit::Deg => self.0,
            AngleUnit::Rad => self.0.to_degrees(),
            AngleUnit::Grad => self.0 * (360.0 / 400.0),
            AngleUnit::Turn => self.0 * 360.0,
        }
    }

    /// Convert the angle to radians regardless of the source unit.
    pub fn to_radians(self) -> f32 {
        match self.1 {
            AngleUnit::Deg => self.0.to_radians(),
            AngleUnit::Rad => self.0,
            AngleUnit::Grad => self.0 * (std::f32::consts::PI / 200.0),
            AngleUnit::Turn => self.0 * 2.0 * std::f32::consts::PI,
        }
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1.suffix())
    }
}

impl FloatConvert for Angle {
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::values::assert_approx_eq;

    #[test]
    fn deg_constructor() {
        let a = Angle::deg(45.0);
        assert_eq!(a.value(), 45.0);
        assert_eq!(a.unit(), AngleUnit::Deg);
    }

    #[test]
    fn rad_constructor() {
        let a = Angle::rad(1.5);
        assert_eq!(a.value(), 1.5);
        assert_eq!(a.unit(), AngleUnit::Rad);
    }

    #[test]
    fn grad_constructor() {
        let a = Angle::grad(100.0);
        assert_eq!(a.unit(), AngleUnit::Grad);
    }

    #[test]
    fn turn_constructor() {
        let a = Angle::turn(0.25);
        assert_eq!(a.unit(), AngleUnit::Turn);
    }

    #[test]
    fn display_deg() {
        assert_eq!(Angle::deg(45.0).to_string(), "45deg");
    }

    #[test]
    fn display_rad() {
        assert_eq!(Angle::rad(1.1234).to_string(), "1.1234rad");
    }

    #[test]
    fn display_grad() {
        assert_eq!(Angle::grad(100.0).to_string(), "100grad");
    }

    #[test]
    fn display_turn() {
        assert_eq!(Angle::turn(0.25).to_string(), "0.25turn");
    }

    #[test]
    fn display_zero() {
        assert_eq!(Angle::deg(0.0).to_string(), "0deg");
    }

    #[test]
    fn to_f64_roundtrip() {
        assert_eq!(Angle::deg(45.0).to_f64(), 45.0);
        assert_eq!(Angle::turn(0.5).to_f64(), 0.5);
    }

    #[test]
    fn unit_suffix_strings() {
        assert_eq!(AngleUnit::Deg.suffix(), "deg");
        assert_eq!(AngleUnit::Rad.suffix(), "rad");
        assert_eq!(AngleUnit::Grad.suffix(), "grad");
        assert_eq!(AngleUnit::Turn.suffix(), "turn");
    }

    #[test]
    fn to_degrees_from_each_unit() {
        assert_approx_eq!(Angle::deg(45.0).to_degrees(), 45.0);
        assert_approx_eq!(Angle::rad(std::f32::consts::PI).to_degrees(), 180.0);
        assert_approx_eq!(Angle::grad(100.0).to_degrees(), 90.0);
        assert_approx_eq!(Angle::turn(0.5).to_degrees(), 180.0);
    }

    #[test]
    fn to_radians_from_each_unit() {
        assert_approx_eq!(Angle::deg(180.0).to_radians(), std::f32::consts::PI, 1e-5);
        assert_approx_eq!(Angle::rad(2.0).to_radians(), 2.0);
        assert_approx_eq!(Angle::grad(200.0).to_radians(), std::f32::consts::PI, 1e-5);
        assert_approx_eq!(Angle::turn(0.5).to_radians(), std::f32::consts::PI, 1e-5);
    }

    #[test]
    fn equality() {
        assert_eq!(Angle::deg(45.0), Angle::deg(45.0));
        assert_ne!(Angle::deg(45.0), Angle::rad(45.0));
        assert_ne!(Angle::deg(45.0), Angle::deg(90.0));
    }

    #[test]
    fn clone_copy() {
        let a = Angle::deg(180.0);
        let a2 = a;
        assert_eq!(a, a2);
    }
}
