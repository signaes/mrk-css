//! `@namespace` at-rule rendering.

use std::fmt;

/// Render an `@namespace [prefix] url;` statement.
pub fn render(f: &mut fmt::Formatter<'_>, prefix: Option<&str>, url: &str) -> fmt::Result {
    let mut s = String::from("@namespace");
    if let Some(p) = prefix {
        s.push_str(&format!(" {}", p));
    }
    s.push_str(&format!(" \"{}\";", url));
    f.write_str(&s)
}
