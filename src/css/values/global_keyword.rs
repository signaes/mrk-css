//! `GlobalKeyword` — CSS-wide global keywords.
//!
//! These keywords are accepted by every CSS property.

use std::fmt;

/// A CSS global keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalKeyword {
    /// `initial`.
    Initial,
    /// `inherit`.
    Inherit,
    /// `unset`.
    Unset,
    /// `revert`.
    Revert,
}

impl GlobalKeyword {
    /// Construct `initial`.
    pub fn initial() -> Self {
        GlobalKeyword::Initial
    }

    /// Construct `inherit`.
    pub fn inherit() -> Self {
        GlobalKeyword::Inherit
    }

    /// Construct `unset`.
    pub fn unset() -> Self {
        GlobalKeyword::Unset
    }

    /// Construct `revert`.
    pub fn revert() -> Self {
        GlobalKeyword::Revert
    }
}

impl fmt::Display for GlobalKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalKeyword::Initial => f.write_str("initial"),
            GlobalKeyword::Inherit => f.write_str("inherit"),
            GlobalKeyword::Unset => f.write_str("unset"),
            GlobalKeyword::Revert => f.write_str("revert"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_global_keywords() {
        let cases: [(GlobalKeyword, &str); 4] = [
            (GlobalKeyword::initial(), "initial"),
            (GlobalKeyword::inherit(), "inherit"),
            (GlobalKeyword::unset(), "unset"),
            (GlobalKeyword::revert(), "revert"),
        ];
        for (kw, expected) in cases {
            assert_eq!(kw.to_string(), expected);
        }
    }
}
