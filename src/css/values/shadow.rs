//! `Shadow` — typed CSS `box-shadow` / `text-shadow` value.
//!
//! Supports the standard shadow syntax:
//! `<offset-x> <offset-y> [<blur-radius>] [<spread-radius>] [<color>] [inset]`.

use std::fmt;

use super::{Color, Length};

/// A CSS shadow value.
#[derive(Debug, Clone, PartialEq)]
pub struct Shadow {
    /// Horizontal offset.
    x: Length,
    /// Vertical offset.
    y: Length,
    /// Blur radius.
    blur: Option<Length>,
    /// Spread radius (only for `box-shadow`).
    spread: Option<Length>,
    /// Shadow color.
    color: Option<Color>,
    /// `inset` flag (only for `box-shadow`).
    inset: bool,
}

impl Shadow {
    /// Construct a shadow with just the required offsets.
    pub fn new(x: Length, y: Length) -> Self {
        Shadow {
            x,
            y,
            blur: None,
            spread: None,
            color: None,
            inset: false,
        }
    }

    /// Add a blur radius.
    pub fn blur(mut self, l: Length) -> Self {
        self.blur = Some(l);
        self
    }

    /// Add a spread radius.
    pub fn spread(mut self, l: Length) -> Self {
        self.spread = Some(l);
        self
    }

    /// Add a color.
    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }

    /// Mark the shadow as `inset`.
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }
}

impl fmt::Display for Shadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inset {
            f.write_str("inset ")?;
        }
        write!(f, "{} {}", self.x, self.y)?;
        if let Some(b) = self.blur {
            write!(f, " {}", b)?;
        }
        if let Some(s) = self.spread {
            write!(f, " {}", s)?;
        }
        if let Some(c) = &self.color {
            write!(f, " {}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_shadow_variants() {
        let cases: [(Shadow, &str); 6] = [
            (Shadow::new(Length::px(2.0), Length::px(2.0)), "2px 2px"),
            (
                Shadow::new(Length::px(2.0), Length::px(2.0)).blur(Length::px(4.0)),
                "2px 2px 4px",
            ),
            (
                Shadow::new(Length::px(2.0), Length::px(2.0))
                    .blur(Length::px(4.0))
                    .spread(Length::px(1.0)),
                "2px 2px 4px 1px",
            ),
            (
                Shadow::new(Length::px(2.0), Length::px(2.0))
                    .blur(Length::px(4.0))
                    .color(Color::named("red")),
                "2px 2px 4px red",
            ),
            (
                Shadow::new(Length::px(2.0), Length::px(2.0))
                    .blur(Length::px(4.0))
                    .color(Color::named("red"))
                    .inset(),
                "inset 2px 2px 4px red",
            ),
            (
                Shadow::new(Length::px(0.0), Length::px(0.0))
                    .blur(Length::px(10.0))
                    .spread(Length::px(-2.0))
                    .color(Color::rgba(0, 0, 0, 0.5)),
                "0px 0px 10px -2px rgba(0, 0, 0, 0.5)",
            ),
        ];
        for (shadow, expected) in cases {
            assert_eq!(shadow.to_string(), expected);
        }
    }
}
