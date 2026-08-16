//! Typed CSS values.
//!
//! Each value type in this module is the strongly-typed counterpart
//! to a CSS lexical form. They power [`crate::css::Value`]
//! (populated in Phase 3) and [`Color`] (Phase 1.14+).

mod numeric;

mod length;
pub use length::{Length, LengthUnit};

mod percentage;
pub use percentage::Percentage;

mod position;
pub use position::{Position, PositionComponent, PositionKeyword};

mod ratio;
pub use ratio::Ratio;

mod time;
pub use time::Time;

mod angle;
pub use angle::Angle;

mod attr;
pub use attr::Attr;

mod frequency;
pub use frequency::Frequency;

mod resolution;
pub use resolution::Resolution;

mod rect;
pub use rect::{Rect, RectEdge};

mod sizing;
pub use sizing::{FitContent, Sizing};

mod calc;
pub use calc::{Calc, CalcExpr, CalcValue};

mod filter;
pub use filter::Filter;

mod global_keyword;
pub use global_keyword::GlobalKeyword;

mod transform;
pub use transform::TransformFunction;

mod shadow;
pub use shadow::Shadow;

mod number;
pub use number::{Integer, Number};

mod identifier;
pub use identifier::Ident;

mod url;
pub use url::Url;

mod string;
pub use string::CssString;

mod counter;
pub use counter::Counter;

mod custom_property;
pub use custom_property::CustomProperty;

mod easing;
pub use easing::{EasingFunction, JumpTerm, StepPosition};

mod env;
pub use env::{Env, EnvFallback};

mod color;
#[allow(unused_imports)]
pub use color::{
    Color, ColorKind, ColorMix, ColorMixMethod, ColorMixSpace, ColorParseError, ColorSpace,
    ConversionError, named_color_srgb,
};

/// Test-only approximate float equality, hand-rolled so the crate
/// stays zero-dependency (no `approx` dev-dependency — owner
/// decision, see the project code review §1.4). Compares with an
/// absolute epsilon (default `1e-4`).
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        assert!((($a) - ($b)).abs() <= 1e-4)
    };
    ($a:expr, $b:expr, $eps:expr) => {
        assert!((($a) - ($b)).abs() <= $eps)
    };
}

#[cfg(test)]
pub(crate) use assert_approx_eq;
