//! `@media` at-rule rendering.

use std::fmt;

use super::RuleOrAtRule;

/// Render a `@media (query) { ... }` block.
/// Render a `@media (query) { ... }` block.
pub fn render(f: &mut fmt::Formatter<'_>, query: &str, rules: &[RuleOrAtRule]) -> fmt::Result {
    let mut s = format!("@media {} {{", query);
    for r in rules {
        s.push_str(&format!("\n  {}", r));
    }
    if !rules.is_empty() { s.push('\n'); }
    s.push('}');
    f.write_str(&s)
}
