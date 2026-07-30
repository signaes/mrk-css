//! `CustomProperty` — a CSS custom property name (e.g. `--my-var`).
//!
//! Custom properties are used by `var(--name)` references and by
//! declarations like `--my-var: 16px`.

use std::borrow::Cow;
use std::fmt;

/// A CSS custom property name, including the leading `--`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomProperty(pub(crate) Cow<'static, str>);

impl CustomProperty {
    /// Construct from a name. Validates that the input starts with
    /// `--` and contains only valid identifier characters.
    ///
    /// Returns `None` if the input is not a valid custom property
    /// name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Option<Self> {
        let name = name.into();
        if !name.starts_with("--") || name.len() == 2 {
            return None;
        }
        if !name[2..].chars().all(is_custom_prop_char) {
            return None;
        }
        Some(CustomProperty(name))
    }

    /// Return the full name including the leading `--`.
    pub fn name(&self) -> &str {
        &self.0
    }
}

fn is_custom_prop_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(c, '-' | '_' | '\u{0080}'..='\u{10FFFF}')
}

impl fmt::Display for CustomProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        let cp = CustomProperty::new("--my-var").unwrap();
        assert_eq!(cp.name(), "--my-var");
    }

    #[test]
    fn new_with_underscore() {
        let cp = CustomProperty::new("--my_var").unwrap();
        assert_eq!(cp.name(), "--my_var");
    }

    #[test]
    fn new_with_digits() {
        assert!(CustomProperty::new("--x1").is_some());
    }

    #[test]
    fn new_missing_dashes() {
        assert!(CustomProperty::new("my-var").is_none());
        assert!(CustomProperty::new("-my-var").is_none());
    }

    #[test]
    fn new_just_dashes() {
        assert!(CustomProperty::new("--").is_none());
    }

    #[test]
    fn new_with_space_fails() {
        assert!(CustomProperty::new("--my var").is_none());
    }

    #[test]
    fn new_with_special_char_fails() {
        assert!(CustomProperty::new("--my!var").is_none());
        assert!(CustomProperty::new("--my/var").is_none());
    }

    #[test]
    fn display_with_dashes() {
        assert_eq!(CustomProperty::new("--x").unwrap().to_string(), "--x");
    }

    #[test]
    fn equality() {
        assert_eq!(
            CustomProperty::new("--x").unwrap(),
            CustomProperty::new("--x").unwrap()
        );
    }

    #[test]
    fn new_with_unicode() {
        assert!(CustomProperty::new("--\u{0080}var").is_some());
        assert!(CustomProperty::new("--emoji-\u{1F600}").is_some());
    }
}