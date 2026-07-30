//! `@font-face` at-rule rendering.

use std::fmt;

use crate::css::declaration::Declaration;

/// Render a `@font-face { ... }` block.
pub fn render(f: &mut fmt::Formatter<'_>, declarations: &[Declaration]) -> fmt::Result {
    let mut s = String::from("@font-face {");
    for d in declarations {
        s.push_str(&format!("\n  {}", d));
    }
    if !declarations.is_empty() { s.push('\n'); }
    s.push('}');
    f.write_str(&s)
}
