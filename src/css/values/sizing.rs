//! `Sizing` — CSS intrinsic sizing keywords and `fit-content()`.
//!
//! Covers the values used by properties such as `width`, `height`,
//! `min-width`, `max-width`, `flex-basis`, and grid track sizing.

use std::fmt;

use super::{Length, Percentage};

/// A CSS sizing value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizing {
    /// `min-content`.
    MinContent,
    /// `max-content`.
    MaxContent,
    /// `fit-content(<length-percentage>)`.
    FitContent(FitContent),
    /// `stretch` (also historically `available` / `fill-available`).
    Stretch,
}

/// The argument accepted by `fit-content()`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FitContent {
    /// A length argument.
    Length(Length),
    /// A percentage argument.
    Percentage(Percentage),
}

impl Sizing {
    /// Construct `min-content`.
    pub fn min_content() -> Self {
        Sizing::MinContent
    }

    /// Construct `max-content`.
    pub fn max_content() -> Self {
        Sizing::MaxContent
    }

    /// Construct `fit-content(length)`.
    pub fn fit_content_length(l: Length) -> Self {
        Sizing::FitContent(FitContent::Length(l))
    }

    /// Construct `fit-content(percentage)`.
    pub fn fit_content_percentage(p: Percentage) -> Self {
        Sizing::FitContent(FitContent::Percentage(p))
    }

    /// Construct `stretch`.
    pub fn stretch() -> Self {
        Sizing::Stretch
    }
}

impl fmt::Display for FitContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FitContent::Length(l) => fmt::Display::fmt(l, f),
            FitContent::Percentage(p) => fmt::Display::fmt(p, f),
        }
    }
}

impl fmt::Display for Sizing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sizing::MinContent => f.write_str("min-content"),
            Sizing::MaxContent => f.write_str("max-content"),
            Sizing::FitContent(v) => write!(f, "fit-content({})", v),
            Sizing::Stretch => f.write_str("stretch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_sizing_keywords() {
        let cases: [(Sizing, &str); 3] = [
            (Sizing::min_content(), "min-content"),
            (Sizing::max_content(), "max-content"),
            (Sizing::stretch(), "stretch"),
        ];
        for (sizing, expected) in cases {
            assert_eq!(sizing.to_string(), expected);
        }
    }

    #[test]
    fn display_fit_content() {
        let cases: [(Sizing, &str); 2] = [
            (
                Sizing::fit_content_length(Length::px(200.0)),
                "fit-content(200px)",
            ),
            (
                Sizing::fit_content_percentage(Percentage::new(50.0)),
                "fit-content(50%)",
            ),
        ];
        for (sizing, expected) in cases {
            assert_eq!(sizing.to_string(), expected);
        }
    }
}
