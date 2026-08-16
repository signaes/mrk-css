//! `Attr` — typed CSS `attr()` function.
//!
//! Used mainly in the `content` property to pull a value from an
//! element attribute. Supports the optional type and fallback
//! arguments.

use std::fmt;

use super::{CssString, Ident, Integer, Number};

/// A CSS `attr()` value.
#[derive(Debug, Clone, PartialEq)]
pub struct Attr {
    /// Attribute name.
    name: Ident,
    /// Optional type annotation (e.g. `string`, `color`, `url`).
    type_: Option<Ident>,
    /// Optional fallback value.
    fallback: Option<AttrFallback>,
}

/// A permitted `attr()` fallback value.
///
/// Kept as a local enum (rather than `Value`) so `Attr` can live in
/// `css::values` without creating a circular dependency with
/// `css::properties::Value`.
#[derive(Debug, Clone, PartialEq)]
pub enum AttrFallback {
    /// A quoted string fallback.
    String(CssString),
    /// A number fallback.
    Number(Number),
    /// An integer fallback.
    Integer(Integer),
}

impl Attr {
    /// Construct `attr(name)`.
    pub fn new(name: impl Into<Ident>) -> Self {
        Attr {
            name: name.into(),
            type_: None,
            fallback: None,
        }
    }

    /// Construct `attr(name type)`.
    pub fn with_type(name: impl Into<Ident>, type_: impl Into<Ident>) -> Self {
        Attr {
            name: name.into(),
            type_: Some(type_.into()),
            fallback: None,
        }
    }

    /// Construct `attr(name type, fallback)`.
    pub fn with_fallback(
        name: impl Into<Ident>,
        type_: impl Into<Ident>,
        fallback: impl Into<AttrFallback>,
    ) -> Self {
        Attr {
            name: name.into(),
            type_: Some(type_.into()),
            fallback: Some(fallback.into()),
        }
    }
}

impl From<CssString> for AttrFallback {
    fn from(v: CssString) -> Self {
        AttrFallback::String(v)
    }
}

impl From<&str> for AttrFallback {
    fn from(v: &str) -> Self {
        AttrFallback::String(CssString::new(v.to_string()))
    }
}

impl From<Number> for AttrFallback {
    fn from(v: Number) -> Self {
        AttrFallback::Number(v)
    }
}

impl From<Integer> for AttrFallback {
    fn from(v: Integer) -> Self {
        AttrFallback::Integer(v)
    }
}

impl fmt::Display for AttrFallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttrFallback::String(v) => fmt::Display::fmt(v, f),
            AttrFallback::Number(v) => fmt::Display::fmt(v, f),
            AttrFallback::Integer(v) => fmt::Display::fmt(v, f),
        }
    }
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attr({}", self.name)?;
        if let Some(t) = &self.type_ {
            write!(f, " {}", t)?;
            if let Some(fb) = &self.fallback {
                write!(f, ", {}", fb)?;
            }
        }
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_attr_name_only() {
        let cases: [(&str, &str); 2] = [("href", "attr(href)"), ("data-count", "attr(data-count)")];
        for (name, expected) in cases {
            assert_eq!(Attr::new(name).to_string(), expected);
        }
    }

    #[test]
    fn display_attr_with_type() {
        let cases: [((&str, &str), &str); 2] = [
            (("data-count", "integer"), "attr(data-count integer)"),
            (("data-url", "url"), "attr(data-url url)"),
        ];
        for ((name, type_), expected) in cases {
            assert_eq!(Attr::with_type(name, type_).to_string(), expected);
        }
    }

    #[test]
    fn display_attr_with_string_fallback() {
        let cases: [((&str, &str, &str), &str); 2] = [
            (
                ("data-label", "string", "missing"),
                "attr(data-label string, \"missing\")",
            ),
            (
                ("data-label", "string", ""),
                "attr(data-label string, \"\")",
            ),
        ];
        for ((name, type_, fallback), expected) in cases {
            assert_eq!(
                Attr::with_fallback(name, type_, fallback).to_string(),
                expected
            );
        }
    }

    #[test]
    fn display_attr_with_number_fallback() {
        let cases: [((&str, &str, AttrFallback), &str); 3] = [
            (
                ("data-count", "integer", Integer::new(0).into()),
                "attr(data-count integer, 0)",
            ),
            (
                ("data-ratio", "number", Number::new(1.5).into()),
                "attr(data-ratio number, 1.5)",
            ),
            (
                ("data-offset", "integer", Integer::new(-7).into()),
                "attr(data-offset integer, -7)",
            ),
        ];
        for ((name, type_, fallback), expected) in cases {
            assert_eq!(
                Attr::with_fallback(name, type_, fallback).to_string(),
                expected
            );
        }
    }
}
