//! `Filter` — typed CSS filter functions.
//!
//! Covers the most common `<filter-function>` values used by the
//! `filter` and `backdrop-filter` properties.

use std::fmt;

use super::{Angle, Length, Number, Shadow};

/// A CSS filter function value.
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// `blur(<length>)`.
    Blur(Length),
    /// `brightness(<number>)`.
    Brightness(Number),
    /// `contrast(<number>)`.
    Contrast(Number),
    /// `grayscale(<number>)`.
    Grayscale(Number),
    /// `hue-rotate(<angle>)`.
    HueRotate(Angle),
    /// `invert(<number>)`.
    Invert(Number),
    /// `opacity(<number>)`.
    Opacity(Number),
    /// `saturate(<number>)`.
    Saturate(Number),
    /// `sepia(<number>)`.
    Sepia(Number),
    /// `drop-shadow(<shadow>)`.
    DropShadow(Shadow),
}

impl Filter {
    /// Construct `blur(length)`.
    pub fn blur(l: Length) -> Self {
        Filter::Blur(l)
    }

    /// Construct `brightness(n)`.
    pub fn brightness(n: impl Into<Number>) -> Self {
        Filter::Brightness(n.into())
    }

    /// Construct `contrast(n)`.
    pub fn contrast(n: impl Into<Number>) -> Self {
        Filter::Contrast(n.into())
    }

    /// Construct `grayscale(n)`.
    pub fn grayscale(n: impl Into<Number>) -> Self {
        Filter::Grayscale(n.into())
    }

    /// Construct `hue-rotate(angle)`.
    pub fn hue_rotate(a: Angle) -> Self {
        Filter::HueRotate(a)
    }

    /// Construct `invert(n)`.
    pub fn invert(n: impl Into<Number>) -> Self {
        Filter::Invert(n.into())
    }

    /// Construct `opacity(n)`.
    pub fn opacity(n: impl Into<Number>) -> Self {
        Filter::Opacity(n.into())
    }

    /// Construct `saturate(n)`.
    pub fn saturate(n: impl Into<Number>) -> Self {
        Filter::Saturate(n.into())
    }

    /// Construct `sepia(n)`.
    pub fn sepia(n: impl Into<Number>) -> Self {
        Filter::Sepia(n.into())
    }

    /// Construct `drop-shadow(shadow)`.
    pub fn drop_shadow(s: Shadow) -> Self {
        Filter::DropShadow(s)
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Filter::Blur(v) => write!(f, "blur({})", v),
            Filter::Brightness(v) => write!(f, "brightness({})", v),
            Filter::Contrast(v) => write!(f, "contrast({})", v),
            Filter::Grayscale(v) => write!(f, "grayscale({})", v),
            Filter::HueRotate(v) => write!(f, "hue-rotate({})", v),
            Filter::Invert(v) => write!(f, "invert({})", v),
            Filter::Opacity(v) => write!(f, "opacity({})", v),
            Filter::Saturate(v) => write!(f, "saturate({})", v),
            Filter::Sepia(v) => write!(f, "sepia({})", v),
            Filter::DropShadow(v) => write!(f, "drop-shadow({})", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_filter_functions() {
        let cases: [(Filter, &str); 10] = [
            (Filter::blur(Length::px(5.0)), "blur(5px)"),
            (Filter::brightness(1.5), "brightness(1.5)"),
            (Filter::contrast(0.8), "contrast(0.8)"),
            (Filter::grayscale(0.5), "grayscale(0.5)"),
            (Filter::hue_rotate(Angle::deg(90.0)), "hue-rotate(90deg)"),
            (Filter::invert(1.0), "invert(1)"),
            (Filter::opacity(0.5), "opacity(0.5)"),
            (Filter::saturate(2.0), "saturate(2)"),
            (Filter::sepia(0.3), "sepia(0.3)"),
            (
                Filter::drop_shadow(
                    Shadow::new(Length::px(2.0), Length::px(2.0)).blur(Length::px(4.0)),
                ),
                "drop-shadow(2px 2px 4px)",
            ),
        ];
        for (filter, expected) in cases {
            assert_eq!(filter.to_string(), expected);
        }
    }
}
