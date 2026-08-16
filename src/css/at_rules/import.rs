//! `@import` at-rule rendering.

use std::fmt;

/// Render an `@import url [supports] [media];` statement.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    url: &str,
    supports: Option<&str>,
    media: Option<&str>,
) -> fmt::Result {
    let mut s = format!("@import \"{}\"", url);
    if let Some(sup) = supports {
        s.push_str(&format!(" supports({})", sup));
    }
    if let Some(m) = media {
        s.push_str(&format!(" {}", m));
    }
    s.push(';');
    f.write_str(&s)
}
