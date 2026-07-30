//! `@supports` at-rule rendering.

use std::fmt;

use super::RuleOrAtRule;

/// Render a `@supports (condition) { ... }` block.
pub fn render(f: &mut fmt::Formatter<'_>, condition: &str, rules: &[RuleOrAtRule]) -> fmt::Result {
    let mut s = format!("@supports {} {{", condition);
    for r in rules {
        s.push_str(&format!("\n  {}", r));
    }
    if !rules.is_empty() { s.push('\n'); }
    s.push('}');
    f.write_str(&s)
}
