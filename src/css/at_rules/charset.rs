//! `@charset` at-rule rendering.

use std::fmt;

/// Render an `@charset "encoding";` statement.
pub fn render(f: &mut fmt::Formatter<'_>, encoding: &str) -> fmt::Result {
    write!(f, "@charset \"{}\";", encoding)
}
