//! Typed CSS values.
//!
//! Each value type in this module is the strongly-typed counterpart
//! to a CSS lexical form. They power [`crate::css::Value`]
//! (populated in Phase 3) and [`Color`] (Phase 1.14+).

mod numeric;

mod length;
pub use length::Length;

mod percentage;
pub use percentage::Percentage;

mod time;
pub use time::Time;

mod angle;
pub use angle::Angle;

mod frequency;
pub use frequency::Frequency;

mod resolution;
pub use resolution::Resolution;

mod number;
pub use number::{Number, Integer};

mod identifier;
pub use identifier::Ident;

mod url;
pub use url::Url;

mod string;
pub use string::CssString;

mod custom_property;
pub use custom_property::CustomProperty;

mod easing;
pub use easing::EasingFunction;

mod color;
#[allow(unused_imports)]
pub use color::{
    named_color_srgb, Color, ColorKind, ColorMix, ColorMixMethod, ColorMixSpace,
    ConversionError, ColorParseError, ColorSpace,
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