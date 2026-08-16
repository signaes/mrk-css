//! `Position` — a CSS `<position>` value.
//!
//! Represents the position value used by properties such as
//! `background-position`, `transform-origin`, `object-position`, etc.
//! Each axis is either a keyword (`top`, `bottom`, `left`, `right`,
//! `center`) or a `Length` value (which already covers percentages).

use std::fmt;

use super::Length;

/// A position keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionKeyword {
    /// `top`
    Top,
    /// `bottom`
    Bottom,
    /// `left`
    Left,
    /// `right`
    Right,
    /// `center`
    Center,
}

impl PositionKeyword {
    /// Render the keyword to its CSS name.
    fn as_str(&self) -> &'static str {
        match self {
            PositionKeyword::Top => "top",
            PositionKeyword::Bottom => "bottom",
            PositionKeyword::Left => "left",
            PositionKeyword::Right => "right",
            PositionKeyword::Center => "center",
        }
    }
}

impl fmt::Display for PositionKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A single component of a `<position>` value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionComponent {
    /// A keyword such as `top`, `left`, or `center`.
    Keyword(PositionKeyword),
    /// A length or percentage value.
    Length(Length),
}

impl From<PositionKeyword> for PositionComponent {
    fn from(k: PositionKeyword) -> Self {
        PositionComponent::Keyword(k)
    }
}

impl From<Length> for PositionComponent {
    fn from(l: Length) -> Self {
        PositionComponent::Length(l)
    }
}

impl fmt::Display for PositionComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionComponent::Keyword(k) => fmt::Display::fmt(k, f),
            PositionComponent::Length(l) => fmt::Display::fmt(l, f),
        }
    }
}

/// A CSS `<position>` value.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    x: PositionComponent,
    y: Option<PositionComponent>,
}

impl Position {
    /// Construct a one-axis position (e.g. `center`).
    pub fn new(component: impl Into<PositionComponent>) -> Self {
        Position {
            x: component.into(),
            y: None,
        }
    }

    /// Construct a two-axis position (e.g. `top left`).
    pub fn new2(x: impl Into<PositionComponent>, y: impl Into<PositionComponent>) -> Self {
        Position {
            x: x.into(),
            y: Some(y.into()),
        }
    }

    /// `center`.
    pub fn center() -> Self {
        Position::new(PositionKeyword::Center)
    }

    /// `top`.
    pub fn top() -> Self {
        Position::new(PositionKeyword::Top)
    }

    /// `bottom`.
    pub fn bottom() -> Self {
        Position::new(PositionKeyword::Bottom)
    }

    /// `left`.
    pub fn left() -> Self {
        Position::new(PositionKeyword::Left)
    }

    /// `right`.
    pub fn right() -> Self {
        Position::new(PositionKeyword::Right)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.x)?;
        if let Some(y) = &self.y {
            write!(f, " {y}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_single_keyword() {
        assert_eq!(Position::center().to_string(), "center");
        assert_eq!(Position::top().to_string(), "top");
    }

    #[test]
    fn display_two_keywords() {
        assert_eq!(
            Position::new2(PositionKeyword::Top, PositionKeyword::Left).to_string(),
            "top left"
        );
    }

    #[test]
    fn display_mixed() {
        assert_eq!(
            Position::new2(Length::px(10.0), Length::pct(50.0)).to_string(),
            "10px 50%"
        );
    }
}
