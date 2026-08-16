//! `TransformFunction` — typed CSS 2D/3D transform functions.
//!
//! Covers the most common transform functions that use lengths and
//! angles. Percentage-based translations (e.g. `translate(50%, 50%)`)
//! are intentionally left as generic functions for now.

use std::fmt;

use super::{Angle, Length};

/// A CSS transform function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformFunction {
    /// `translateX(<length>)`.
    TranslateX(Length),
    /// `translateY(<length>)`.
    TranslateY(Length),
    /// `translateZ(<length>)`.
    TranslateZ(Length),
    /// `translate(<length>, <length>)`.
    Translate(Length, Length),
    /// `translate3d(<length>, <length>, <length>)`.
    Translate3d(Length, Length, Length),
    /// `scale(<number>[, <number>])`.
    Scale(f32, f32),
    /// `scaleX(<number>)`.
    ScaleX(f32),
    /// `scaleY(<number>)`.
    ScaleY(f32),
    /// `scaleZ(<number>)`.
    ScaleZ(f32),
    /// `scale3d(<number>, <number>, <number>)`.
    Scale3d(f32, f32, f32),
    /// `rotate(<angle>)`.
    Rotate(Angle),
    /// `rotateX(<angle>)`.
    RotateX(Angle),
    /// `rotateY(<angle>)`.
    RotateY(Angle),
    /// `rotateZ(<angle>)`.
    RotateZ(Angle),
    /// `skew(<angle>, <angle>)`.
    Skew(Angle, Angle),
    /// `skewX(<angle>)`.
    SkewX(Angle),
    /// `skewY(<angle>)`.
    SkewY(Angle),
}

impl TransformFunction {
    /// Construct `translateX(l)`.
    pub fn translate_x(l: Length) -> Self {
        TransformFunction::TranslateX(l)
    }

    /// Construct `translateY(l)`.
    pub fn translate_y(l: Length) -> Self {
        TransformFunction::TranslateY(l)
    }

    /// Construct `translateZ(l)`.
    pub fn translate_z(l: Length) -> Self {
        TransformFunction::TranslateZ(l)
    }

    /// Construct `translate(x, y)`.
    pub fn translate(x: Length, y: Length) -> Self {
        TransformFunction::Translate(x, y)
    }

    /// Construct `translate3d(x, y, z)`.
    pub fn translate_3d(x: Length, y: Length, z: Length) -> Self {
        TransformFunction::Translate3d(x, y, z)
    }

    /// Construct `scale(s)`.
    pub fn scale(s: impl Into<f32>) -> Self {
        let s = s.into();
        TransformFunction::Scale(s, s)
    }

    /// Construct `scale(x, y)`.
    pub fn scale_xy(x: impl Into<f32>, y: impl Into<f32>) -> Self {
        TransformFunction::Scale(x.into(), y.into())
    }

    /// Construct `scaleX(s)`.
    pub fn scale_x(s: impl Into<f32>) -> Self {
        TransformFunction::ScaleX(s.into())
    }

    /// Construct `scaleY(s)`.
    pub fn scale_y(s: impl Into<f32>) -> Self {
        TransformFunction::ScaleY(s.into())
    }

    /// Construct `scaleZ(s)`.
    pub fn scale_z(s: impl Into<f32>) -> Self {
        TransformFunction::ScaleZ(s.into())
    }

    /// Construct `scale3d(x, y, z)`.
    pub fn scale_3d(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
        TransformFunction::Scale3d(x.into(), y.into(), z.into())
    }

    /// Construct `rotate(a)`.
    pub fn rotate(a: Angle) -> Self {
        TransformFunction::Rotate(a)
    }

    /// Construct `rotateX(a)`.
    pub fn rotate_x(a: Angle) -> Self {
        TransformFunction::RotateX(a)
    }

    /// Construct `rotateY(a)`.
    pub fn rotate_y(a: Angle) -> Self {
        TransformFunction::RotateY(a)
    }

    /// Construct `rotateZ(a)`.
    pub fn rotate_z(a: Angle) -> Self {
        TransformFunction::RotateZ(a)
    }

    /// Construct `skew(x, y)`.
    pub fn skew(x: Angle, y: Angle) -> Self {
        TransformFunction::Skew(x, y)
    }

    /// Construct `skewX(a)`.
    pub fn skew_x(a: Angle) -> Self {
        TransformFunction::SkewX(a)
    }

    /// Construct `skewY(a)`.
    pub fn skew_y(a: Angle) -> Self {
        TransformFunction::SkewY(a)
    }
}

impl fmt::Display for TransformFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformFunction::TranslateX(v) => write!(f, "translateX({})", v),
            TransformFunction::TranslateY(v) => write!(f, "translateY({})", v),
            TransformFunction::TranslateZ(v) => write!(f, "translateZ({})", v),
            TransformFunction::Translate(x, y) => write!(f, "translate({}, {})", x, y),
            TransformFunction::Translate3d(x, y, z) => {
                write!(f, "translate3d({}, {}, {})", x, y, z)
            }
            TransformFunction::Scale(x, y) if x == y => write!(f, "scale({})", x),
            TransformFunction::Scale(x, y) => write!(f, "scale({}, {})", x, y),
            TransformFunction::ScaleX(v) => write!(f, "scaleX({})", v),
            TransformFunction::ScaleY(v) => write!(f, "scaleY({})", v),
            TransformFunction::ScaleZ(v) => write!(f, "scaleZ({})", v),
            TransformFunction::Scale3d(x, y, z) => write!(f, "scale3d({}, {}, {})", x, y, z),
            TransformFunction::Rotate(v) => write!(f, "rotate({})", v),
            TransformFunction::RotateX(v) => write!(f, "rotateX({})", v),
            TransformFunction::RotateY(v) => write!(f, "rotateY({})", v),
            TransformFunction::RotateZ(v) => write!(f, "rotateZ({})", v),
            TransformFunction::Skew(x, y) => write!(f, "skew({}, {})", x, y),
            TransformFunction::SkewX(v) => write!(f, "skewX({})", v),
            TransformFunction::SkewY(v) => write!(f, "skewY({})", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_translate_functions() {
        let cases: [(TransformFunction, &str); 5] = [
            (
                TransformFunction::translate_x(Length::px(10.0)),
                "translateX(10px)",
            ),
            (
                TransformFunction::translate_y(Length::px(20.0)),
                "translateY(20px)",
            ),
            (
                TransformFunction::translate_z(Length::px(30.0)),
                "translateZ(30px)",
            ),
            (
                TransformFunction::translate(Length::px(10.0), Length::px(20.0)),
                "translate(10px, 20px)",
            ),
            (
                TransformFunction::translate_3d(
                    Length::px(10.0),
                    Length::px(20.0),
                    Length::px(30.0),
                ),
                "translate3d(10px, 20px, 30px)",
            ),
        ];
        for (tf, expected) in cases {
            assert_eq!(tf.to_string(), expected);
        }
    }

    #[test]
    fn display_scale_functions() {
        let cases: [(TransformFunction, &str); 6] = [
            (TransformFunction::scale(1.5_f32), "scale(1.5)"),
            (
                TransformFunction::scale_xy(1.5_f32, 2.0_f32),
                "scale(1.5, 2)",
            ),
            (TransformFunction::scale_x(1.5_f32), "scaleX(1.5)"),
            (TransformFunction::scale_y(2.0_f32), "scaleY(2)"),
            (TransformFunction::scale_z(0.5_f32), "scaleZ(0.5)"),
            (
                TransformFunction::scale_3d(1.0_f32, 2.0_f32, 3.0_f32),
                "scale3d(1, 2, 3)",
            ),
        ];
        for (tf, expected) in cases {
            assert_eq!(tf.to_string(), expected);
        }
    }

    #[test]
    fn display_rotate_functions() {
        let cases: [(TransformFunction, &str); 5] = [
            (TransformFunction::rotate(Angle::deg(45.0)), "rotate(45deg)"),
            (
                TransformFunction::rotate_x(Angle::deg(45.0)),
                "rotateX(45deg)",
            ),
            (
                TransformFunction::rotate_y(Angle::deg(45.0)),
                "rotateY(45deg)",
            ),
            (
                TransformFunction::rotate_z(Angle::deg(45.0)),
                "rotateZ(45deg)",
            ),
            (
                TransformFunction::skew(Angle::deg(10.0), Angle::deg(20.0)),
                "skew(10deg, 20deg)",
            ),
        ];
        for (tf, expected) in cases {
            assert_eq!(tf.to_string(), expected);
        }
    }
}
