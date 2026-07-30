//! `CssString` — a CSS quoted string with escape handling.

use std::borrow::Cow;
use std::fmt;

/// A CSS string literal. Always printed as `"..."` with the
/// standard CSS escape sequences (`\\`, `\"`, `\n`, `\r`, `\t`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssString(pub(crate) Cow<'static, str>);

impl CssString {
    /// Construct from any string-like value.
    pub fn new(s: impl Into<Cow<'static, str>>) -> Self {
        CssString(s.into())
    }

    /// Return the raw inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for CssString {
    fn from(s: &'static str) -> Self {
        CssString(Cow::Borrowed(s))
    }
}

impl From<String> for CssString {
    fn from(s: String) -> Self {
        CssString(Cow::Owned(s))
    }
}

impl fmt::Display for CssString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("\"");
        for ch in self.0.chars() {
            match ch {
                '\\' => {
                    let _ = f.write_str("\\\\");
                }
                '"' => {
                    let _ = f.write_str("\\\"");
                }
                '\n' => {
                    let _ = f.write_str("\\n");
                }
                '\r' => {
                    let _ = f.write_str("\\r");
                }
                '\t' => {
                    let _ = f.write_str("\\t");
                }
                _ => {
                    use std::fmt::Write;
                    let _ = f.write_char(ch);
                }
            }
        }
        f.write_str("\"")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_str() {
        assert_eq!(CssString::new("hello").as_str(), "hello");
    }

    #[test]
    fn new_from_string() {
        let s: String = String::from("world");
        assert_eq!(CssString::new(s).as_str(), "world");
    }

    #[test]
    fn from_static_str() {
        let s = CssString::from("hi");
        assert_eq!(s.as_str(), "hi");
    }

    #[test]
    fn from_owned_string() {
        let s = CssString::from(String::from("dyn"));
        assert_eq!(s.as_str(), "dyn");
    }

    #[test]
    fn display_simple() {
        assert_eq!(CssString::new("hello").to_string(), "\"hello\"");
    }

    #[test]
    fn display_empty() {
        assert_eq!(CssString::new("").to_string(), "\"\"");
    }

    #[test]
    fn display_escapes_backslash() {
        assert_eq!(CssString::new("a\\b").to_string(), "\"a\\\\b\"");
    }

    #[test]
    fn display_escapes_quote() {
        assert_eq!(CssString::new("she said \"hi\"").to_string(), "\"she said \\\"hi\\\"\"");
    }

    #[test]
    fn display_escapes_newline() {
        assert_eq!(CssString::new("a\nb").to_string(), "\"a\\nb\"");
    }

    #[test]
    fn display_escapes_carriage_return() {
        assert_eq!(CssString::new("a\rb").to_string(), "\"a\\rb\"");
    }

    #[test]
    fn display_escapes_tab() {
        assert_eq!(CssString::new("a\tb").to_string(), "\"a\\tb\"");
    }

    #[test]
    fn equality() {
        assert_eq!(CssString::new("a"), CssString::new("a"));
        assert_ne!(CssString::new("a"), CssString::new("b"));
    }
}