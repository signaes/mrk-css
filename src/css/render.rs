//! Pretty-printer (renderer) for CSS.

use crate::css::at_rules::RuleOrAtRule;
use crate::css::StyleSheet;

/// Render a [`StyleSheet`] to its canonical pretty-printed CSS form.
pub(crate) fn render_sheet(sheet: &StyleSheet) -> String {
    let mut out = String::new();
    for (i, item) in sheet.items.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        match item {
            RuleOrAtRule::Rule(r) => out.push_str(&r.to_string()),
            RuleOrAtRule::AtRule(a) => out.push_str(&a.to_string()),
        }
    }
    out
}
