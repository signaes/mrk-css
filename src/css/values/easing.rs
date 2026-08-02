//! `EasingFunction` — CSS transition / animation timing functions.

use std::fmt;

/// CSS easing function.
///
/// Includes the named keywords (`linear`, `ease`, `ease-in`,
/// `ease-out`, `ease-in-out`), the `cubic-bezier(...)` functional
/// notation, and the `steps(...)` functional notation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    /// `linear`.
    Linear,
    /// `ease`.
    Ease,
    /// `ease-in`.
    EaseIn,
    /// `ease-out`.
    EaseOut,
    /// `ease-in-out`.
    EaseInOut,
    /// `cubic-bezier(<x1>, <y1>, <x2>, <y2>)`.
    CubicBezier {
        /// X coordinate of the first control point.
        x1: f32,
        /// Y coordinate of the first control point.
        y1: f32,
        /// X coordinate of the second control point.
        x2: f32,
        /// Y coordinate of the second control point.
        y2: f32,
    },
    /// `steps(<count>, <jump-term>?, <position>?)`.
    Steps {
        /// Number of steps in the interval.
        count: u32,
        /// Where the jumps occur within the steps.
        jump_term: JumpTerm,
        /// Step position keyword (`start` / `end`).
        position: StepPosition,
    },
}

/// Jump term for the `steps()` easing function.
///
/// Variant names deliberately mirror the CSS spec keywords
/// (`jump-end`, `jump-start`, `jump-none`, `jump-both`) rather than
/// dropping the shared `Jump` prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum JumpTerm {
    /// Default `jump-end`.
    JumpEnd,
    /// `jump-start`.
    JumpStart,
    /// `jump-none`.
    JumpNone,
    /// `jump-both`.
    JumpBoth,
}

/// Position for the `steps()` easing function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepPosition {
    /// Default `end`.
    End,
    /// `start`.
    Start,
}

impl EasingFunction {
    /// Construct `linear`.
    pub fn linear() -> Self { EasingFunction::Linear }
    /// Construct `ease`.
    pub fn ease() -> Self { EasingFunction::Ease }
    /// Construct `ease-in`.
    pub fn ease_in() -> Self { EasingFunction::EaseIn }
    /// Construct `ease-out`.
    pub fn ease_out() -> Self { EasingFunction::EaseOut }
    /// Construct `ease-in-out`.
    pub fn ease_in_out() -> Self { EasingFunction::EaseInOut }

    /// Construct a `cubic-bezier(x1, y1, x2, y2)`.
    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        EasingFunction::CubicBezier { x1, y1, x2, y2 }
    }

    /// Construct `steps(count)` with default jump-term and position.
    pub fn steps(count: u32) -> Self {
        EasingFunction::Steps {
            count,
            jump_term: JumpTerm::JumpEnd,
            position: StepPosition::End,
        }
    }

    /// Construct `steps(count, jump_term, position)` with full
    /// control.
    pub fn steps_with(count: u32, jump_term: JumpTerm, position: StepPosition) -> Self {
        EasingFunction::Steps { count, jump_term, position }
    }
}

impl fmt::Display for EasingFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EasingFunction::Linear => f.write_str("linear"),
            EasingFunction::Ease => f.write_str("ease"),
            EasingFunction::EaseIn => f.write_str("ease-in"),
            EasingFunction::EaseOut => f.write_str("ease-out"),
            EasingFunction::EaseInOut => f.write_str("ease-in-out"),
            EasingFunction::CubicBezier { x1, y1, x2, y2 } => {
                write!(f, "cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            EasingFunction::Steps { count, jump_term, position } => {
                let mut s = format!("steps({}", count);
                match jump_term {
                    JumpTerm::JumpEnd => {}
                    JumpTerm::JumpStart => s.push_str(", jump-start"),
                    JumpTerm::JumpNone => s.push_str(", jump-none"),
                    JumpTerm::JumpBoth => s.push_str(", jump-both"),
                }
                match position {
                    StepPosition::End => {}
                    StepPosition::Start => {
                        if matches!(jump_term, JumpTerm::JumpEnd) {
                            s.push_str(", start");
                        }
                        // When jump_term is not JumpEnd, the jump-term
                        // already specifies the position (e.g.
                        // `jump-start` implies `start`), so we omit
                        // the redundant `start` keyword.
                    }
                }
                s.push(')');
                f.write_str(&s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_linear() {
        assert_eq!(EasingFunction::Linear.to_string(), "linear");
    }

    #[test]
    fn display_ease() {
        assert_eq!(EasingFunction::Ease.to_string(), "ease");
    }

    #[test]
    fn display_ease_in() {
        assert_eq!(EasingFunction::EaseIn.to_string(), "ease-in");
    }

    #[test]
    fn display_ease_out() {
        assert_eq!(EasingFunction::EaseOut.to_string(), "ease-out");
    }

    #[test]
    fn display_ease_in_out() {
        assert_eq!(EasingFunction::EaseInOut.to_string(), "ease-in-out");
    }

    #[test]
    fn display_cubic_bezier() {
        let cb = EasingFunction::cubic_bezier(0.25, 0.1, 0.25, 1.0);
        assert_eq!(cb.to_string(), "cubic-bezier(0.25, 0.1, 0.25, 1)");
    }

    #[test]
    fn display_steps_default() {
        let s = EasingFunction::steps(4);
        assert_eq!(s.to_string(), "steps(4)");
    }

    #[test]
    fn display_steps_jump_start() {
        let s = EasingFunction::steps_with(4, JumpTerm::JumpStart, StepPosition::End);
        assert_eq!(s.to_string(), "steps(4, jump-start)");
    }

    #[test]
    fn display_steps_jump_none() {
        let s = EasingFunction::steps_with(4, JumpTerm::JumpNone, StepPosition::End);
        assert_eq!(s.to_string(), "steps(4, jump-none)");
    }

    #[test]
    fn display_steps_jump_both() {
        let s = EasingFunction::steps_with(4, JumpTerm::JumpBoth, StepPosition::End);
        assert_eq!(s.to_string(), "steps(4, jump-both)");
    }

    #[test]
    fn display_steps_start_position_with_jump_end() {
        let s = EasingFunction::steps_with(4, JumpTerm::JumpEnd, StepPosition::Start);
        assert_eq!(s.to_string(), "steps(4, start)");
    }

    #[test]
    fn display_steps_start_position_with_jump_start() {
        let s = EasingFunction::steps_with(4, JumpTerm::JumpStart, StepPosition::Start);
        assert_eq!(s.to_string(), "steps(4, jump-start)");
    }

    #[test]
    fn equality_keyword() {
        assert_eq!(EasingFunction::Ease, EasingFunction::Ease);
        assert_ne!(EasingFunction::Ease, EasingFunction::Linear);
    }

    #[test]
    fn equality_cubic_bezier() {
        assert_eq!(
            EasingFunction::cubic_bezier(0.0, 0.0, 1.0, 1.0),
            EasingFunction::cubic_bezier(0.0, 0.0, 1.0, 1.0)
        );
        assert_ne!(
            EasingFunction::cubic_bezier(0.0, 0.0, 1.0, 1.0),
            EasingFunction::cubic_bezier(0.5, 0.0, 1.0, 1.0)
        );
    }

    #[test]
    fn equality_steps() {
        assert_eq!(EasingFunction::steps(4), EasingFunction::steps(4));
        assert_ne!(EasingFunction::steps(4), EasingFunction::steps(5));
    }

    #[test]
    fn constructor_linear() {
        assert_eq!(EasingFunction::linear(), EasingFunction::Linear);
    }

    #[test]
    fn constructor_ease() {
        assert_eq!(EasingFunction::ease(), EasingFunction::Ease);
    }

    #[test]
    fn constructor_ease_in() {
        assert_eq!(EasingFunction::ease_in(), EasingFunction::EaseIn);
    }

    #[test]
    fn constructor_ease_out() {
        assert_eq!(EasingFunction::ease_out(), EasingFunction::EaseOut);
    }

    #[test]
    fn constructor_ease_in_out() {
        assert_eq!(EasingFunction::ease_in_out(), EasingFunction::EaseInOut);
    }
}