//! `@supports` at-rule rendering.

use std::fmt;

use super::RuleOrAtRule;

/// Render a `@supports (condition) { ... }` block.
pub fn render(f: &mut fmt::Formatter<'_>, condition: &str, rules: &[RuleOrAtRule]) -> fmt::Result {
    let condition = condition.trim();
    // CSS requires the condition to be wrapped in parentheses. The
    // macro path already supplies them, but the builder API may not.
    let condition = if condition.starts_with('(') && condition.ends_with(')') {
        condition.to_string()
    } else {
        format!("({})", condition)
    };
    let mut s = format!("@supports {} {{", condition);
    for r in rules {
        s.push_str(&format!("\n  {}", r));
    }
    if !rules.is_empty() {
        s.push('\n');
    }
    s.push('}');
    f.write_str(&s)
}
