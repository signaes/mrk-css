//! `Env` — typed CSS `env()` function.
//!
//! `env()` exposes user-agent-defined environment variables such as
//! `safe-area-inset-top`. It accepts an optional fallback value that
//! is used when the environment variable is not defined.

use std::fmt;

use super::{CssString, Ident, Integer, Length, Number};

/// A CSS `env()` value.
#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    /// Environment variable name.
    name: Ident,
    /// Optional fallback value.
    fallback: Option<EnvFallback>,
}

/// A permitted `env()` fallback value.
///
/// Kept local to `env` so the type can live in `css::values` without
/// depending on `css::properties::Value`.
#[derive(Debug, Clone, PartialEq)]
pub enum EnvFallback {
    /// A quoted string fallback.
    String(CssString),
    /// A number fallback.
    Number(Number),
    /// An integer fallback.
    Integer(Integer),
    /// A length fallback.
    Length(Length),
}

impl Env {
    /// Construct `env(name)`.
    pub fn new(name: impl Into<Ident>) -> Self {
        Env {
            name: name.into(),
            fallback: None,
        }
    }

    /// Construct `env(name, fallback)`.
    pub fn with_fallback(name: impl Into<Ident>, fallback: impl Into<EnvFallback>) -> Self {
        Env {
            name: name.into(),
            fallback: Some(fallback.into()),
        }
    }
}

impl From<CssString> for EnvFallback {
    fn from(v: CssString) -> Self {
        EnvFallback::String(v)
    }
}

impl From<&str> for EnvFallback {
    fn from(v: &str) -> Self {
        EnvFallback::String(CssString::new(v.to_string()))
    }
}

impl From<Number> for EnvFallback {
    fn from(v: Number) -> Self {
        EnvFallback::Number(v)
    }
}

impl From<Integer> for EnvFallback {
    fn from(v: Integer) -> Self {
        EnvFallback::Integer(v)
    }
}

impl From<Length> for EnvFallback {
    fn from(v: Length) -> Self {
        EnvFallback::Length(v)
    }
}

impl fmt::Display for EnvFallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvFallback::String(v) => fmt::Display::fmt(v, f),
            EnvFallback::Number(v) => fmt::Display::fmt(v, f),
            EnvFallback::Integer(v) => fmt::Display::fmt(v, f),
            EnvFallback::Length(v) => fmt::Display::fmt(v, f),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "env({}", self.name)?;
        if let Some(fb) = &self.fallback {
            write!(f, ", {}", fb)?;
        }
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_env_name_only() {
        let cases: [(&str, &str); 2] = [
            ("safe-area-inset-top", "env(safe-area-inset-top)"),
            ("safe-area-inset-bottom", "env(safe-area-inset-bottom)"),
        ];
        for (name, expected) in cases {
            assert_eq!(Env::new(name).to_string(), expected);
        }
    }

    #[test]
    fn display_env_with_string_fallback() {
        let cases: [((&str, &str), &str); 2] = [
            (("my-var", "fallback"), "env(my-var, \"fallback\")"),
            (("my-var", ""), "env(my-var, \"\")"),
        ];
        for ((name, fallback), expected) in cases {
            assert_eq!(Env::with_fallback(name, fallback).to_string(), expected);
        }
    }

    #[test]
    fn display_env_with_numeric_fallback() {
        let cases: [((&str, EnvFallback), &str); 3] = [
            (("my-var", Integer::new(0).into()), "env(my-var, 0)"),
            (("my-var", Number::new(1.5).into()), "env(my-var, 1.5)"),
            (("my-var", Number::new(-2.0).into()), "env(my-var, -2)"),
        ];
        for ((name, fallback), expected) in cases {
            assert_eq!(Env::with_fallback(name, fallback).to_string(), expected);
        }
    }

    #[test]
    fn display_env_with_length_fallback() {
        let cases: [((&str, Length), &str); 3] = [
            (
                ("safe-area-inset-top", Length::px(0.0)),
                "env(safe-area-inset-top, 0px)",
            ),
            (
                ("safe-area-inset-top", Length::em(1.5)),
                "env(safe-area-inset-top, 1.5em)",
            ),
            (
                ("safe-area-inset-top", Length::rem(1.0)),
                "env(safe-area-inset-top, 1rem)",
            ),
        ];
        for ((name, length), expected) in cases {
            assert_eq!(Env::with_fallback(name, length).to_string(), expected);
        }
    }
}
