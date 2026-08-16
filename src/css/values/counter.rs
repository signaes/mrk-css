//! `Counter` — typed CSS `counter()` and `counters()` functions.
//!
//! These are typically used inside the `content` property for generated
//! content and list numbering.

use std::fmt;

use super::{CssString, Ident};

/// A CSS `counter()` or `counters()` value.
#[derive(Debug, Clone, PartialEq)]
pub enum Counter {
    /// `counter(name)` or `counter(name, style)`.
    Counter {
        /// Counter name.
        name: Ident,
        /// Optional list-style type (e.g. `decimal`, `lower-roman`).
        style: Option<Ident>,
    },
    /// `counters(name, separator)` or `counters(name, separator, style)`.
    Counters {
        /// Counter name.
        name: Ident,
        /// Separator string.
        separator: CssString,
        /// Optional list-style type.
        style: Option<Ident>,
    },
}

impl Counter {
    /// Construct `counter(name)`.
    pub fn single(name: impl Into<Ident>) -> Self {
        Counter::Counter {
            name: name.into(),
            style: None,
        }
    }

    /// Construct `counter(name, style)`.
    pub fn single_with_style(name: impl Into<Ident>, style: impl Into<Ident>) -> Self {
        Counter::Counter {
            name: name.into(),
            style: Some(style.into()),
        }
    }

    /// Construct `counters(name, separator)`.
    pub fn counters(name: impl Into<Ident>, separator: impl Into<CssString>) -> Self {
        Counter::Counters {
            name: name.into(),
            separator: separator.into(),
            style: None,
        }
    }

    /// Construct `counters(name, separator, style)`.
    pub fn counters_with_style(
        name: impl Into<Ident>,
        separator: impl Into<CssString>,
        style: impl Into<Ident>,
    ) -> Self {
        Counter::Counters {
            name: name.into(),
            separator: separator.into(),
            style: Some(style.into()),
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Counter::Counter { name, style: None } => {
                write!(f, "counter({})", name)
            }
            Counter::Counter {
                name,
                style: Some(style),
            } => {
                write!(f, "counter({}, {})", name, style)
            }
            Counter::Counters {
                name,
                separator,
                style: None,
            } => {
                write!(f, "counters({}, {})", name, separator)
            }
            Counter::Counters {
                name,
                separator,
                style: Some(style),
            } => {
                write!(f, "counters({}, {}, {})", name, separator, style)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_counter() {
        assert_eq!(Counter::single("item").to_string(), "counter(item)");
        assert_eq!(
            Counter::single_with_style("item", "lower-roman").to_string(),
            "counter(item, lower-roman)"
        );
    }

    #[test]
    fn display_counters() {
        assert_eq!(
            Counter::counters("section", ".").to_string(),
            "counters(section, \".\")"
        );
        assert_eq!(
            Counter::counters_with_style("section", ".", "decimal").to_string(),
            "counters(section, \".\", decimal)"
        );
    }
}
