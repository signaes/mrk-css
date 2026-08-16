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
    c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '\u{0080}'..='\u{10FFFF}')
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
    fn new_valid_names() {
        let cases: [&str; 6] = ["--my-var", "--my_var", "--x1", "--1", "--2x", "--x"];
        for name in cases {
            let cp = CustomProperty::new(name).expect(name);
            assert_eq!(cp.name(), name);
        }
    }

    #[test]
    fn new_invalid_names() {
        let cases: [&str; 8] = [
            "my-var",
            "-my-var",
            "--",
            "--my var",
            "--my!var",
            "--my/var",
            "--my\u{007F}var",
            "",
        ];
        for name in cases {
            assert!(
                CustomProperty::new(name).is_none(),
                "{name} should be invalid"
            );
        }
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

    #[test]
    fn new_starts_with_digit_is_valid() {
        // Custom property names are a special <ident> token; a digit
        // immediately after `--` is allowed.
        assert!(CustomProperty::new("--1").is_some());
        assert!(CustomProperty::new("--2x").is_some());
    }

    #[test]
    fn new_rejects_empty_name_and_control_chars() {
        assert!(CustomProperty::new("--").is_none());
        assert!(CustomProperty::new("--my\u{007F}var").is_none());
    }
}
