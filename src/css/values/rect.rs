//! `Rect` — typed CSS `rect()` function for the `clip` property.
//!
//! `rect()` defines a rectangle with four offsets from the edges of
//! the element. Each edge can be a `<length>` or the keyword `auto`.

use std::fmt;

use super::Length;

/// An edge of a `rect()`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RectEdge {
    /// The `auto` keyword.
    Auto,
    /// A length offset.
    Length(Length),
}

impl RectEdge {
    /// Construct an `auto` edge.
    pub fn auto() -> Self {
        RectEdge::Auto
    }

    /// Construct a length edge.
    pub fn length(l: Length) -> Self {
        RectEdge::Length(l)
    }
}

impl From<Length> for RectEdge {
    fn from(l: Length) -> Self {
        RectEdge::Length(l)
    }
}

impl fmt::Display for RectEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RectEdge::Auto => f.write_str("auto"),
            RectEdge::Length(l) => fmt::Display::fmt(l, f),
        }
    }
}

/// A CSS `rect()` value.
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    top: RectEdge,
    right: RectEdge,
    bottom: RectEdge,
    left: RectEdge,
}

impl Rect {
    /// Construct `rect(top, right, bottom, left)`.
    pub fn new(
        top: impl Into<RectEdge>,
        right: impl Into<RectEdge>,
        bottom: impl Into<RectEdge>,
        left: impl Into<RectEdge>,
    ) -> Self {
        Rect {
            top: top.into(),
            right: right.into(),
            bottom: bottom.into(),
            left: left.into(),
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rect({}, {}, {}, {})",
            self.top, self.right, self.bottom, self.left
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_rect_all_lengths() {
        let cases: [(Length, Length, Length, Length, &str); 3] = [
            (
                Length::px(0.0),
                Length::px(10.0),
                Length::px(20.0),
                Length::px(30.0),
                "rect(0px, 10px, 20px, 30px)",
            ),
            (
                Length::em(1.0),
                Length::em(2.0),
                Length::em(3.0),
                Length::em(4.0),
                "rect(1em, 2em, 3em, 4em)",
            ),
            (
                Length::pct(50.0),
                Length::pct(50.0),
                Length::pct(50.0),
                Length::pct(50.0),
                "rect(50%, 50%, 50%, 50%)",
            ),
        ];
        for (t, r, b, l, expected) in cases {
            assert_eq!(Rect::new(t, r, b, l).to_string(), expected);
        }
    }

    #[test]
    fn display_rect_mixed_edges() {
        let cases: [(RectEdge, RectEdge, RectEdge, RectEdge, &str); 3] = [
            (
                RectEdge::auto(),
                Length::px(10.0).into(),
                RectEdge::auto(),
                Length::px(20.0).into(),
                "rect(auto, 10px, auto, 20px)",
            ),
            (
                RectEdge::auto(),
                RectEdge::auto(),
                RectEdge::auto(),
                RectEdge::auto(),
                "rect(auto, auto, auto, auto)",
            ),
            (
                Length::px(0.0).into(),
                RectEdge::auto(),
                Length::px(100.0).into(),
                RectEdge::auto(),
                "rect(0px, auto, 100px, auto)",
            ),
        ];
        for (t, r, b, l, expected) in cases {
            assert_eq!(Rect::new(t, r, b, l).to_string(), expected);
        }
    }
}
