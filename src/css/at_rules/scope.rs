//! `@scope` at-rule rendering.

use std::fmt;

use super::RuleOrAtRule;

/// Render a `@scope [(root)] [to (limit)] { ... }` block.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    root: Option<&str>,
    limit: Option<&str>,
    rules: &[RuleOrAtRule],
) -> fmt::Result {
    let mut s = String::from("@scope");
    if let Some(r) = root { s.push_str(&format!(" ({})", r)); }
    if let Some(l) = limit { s.push_str(&format!(" to ({})", l)); }
    s.push_str(" {");
    for r in rules {
        s.push_str(&format!("\n  {}", r));
    }
    if !rules.is_empty() { s.push('\n'); }
    s.push('}');
    f.write_str(&s)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::css::at_rules::AtRule;
    use crate::css::rule::RuleBuilder;
    use crate::css::selector::Selector;
    use crate::css::values::Color;

    #[test]
    fn display_at_rule_scope_empty() {
        let at = AtRule::Scope {
            root: None,
            limit: None,
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@scope {}");
    }

    #[test]
    fn display_at_rule_scope_with_root() {
        let at = AtRule::Scope {
            root: Some(Cow::Borrowed(".light")),
            limit: None,
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@scope (.light) {}");
    }

    #[test]
    fn display_at_rule_scope_with_limit() {
        let at = AtRule::Scope {
            root: None,
            limit: Some(Cow::Borrowed(".dark")),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@scope to (.dark) {}");
    }

    #[test]
    fn display_at_rule_scope_with_root_and_limit() {
        let at = AtRule::Scope {
            root: Some(Cow::Borrowed(".light")),
            limit: Some(Cow::Borrowed(".dark")),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@scope (.light) to (.dark) {}");
    }

    #[test]
    fn display_at_rule_scope_with_rule() {
        let rule = RuleBuilder::new()
            .selector(Selector::class("box"))
            .property("color", Color::named("red"))
            .build();
        let at = AtRule::Scope {
            root: None,
            limit: None,
            rules: vec![RuleOrAtRule::Rule(rule)],
        };
        let s = at.to_string();
        assert!(s.contains("@scope {"));
        assert!(s.contains(".box"));
        assert!(s.contains("color: red"));
    }

    #[test]
    fn render_fn_directly() {
        struct W(String);
        impl fmt::Write for W {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.push_str(s);
                Ok(())
            }
        }
        let mut w = W(String::new());
        std::fmt::write(
            &mut w,
            format_args!(
                "{}",
                ScopeDisplay(Some(".light"), Some(".dark"), &[])
            ),
        )
        .unwrap();
        assert_eq!(w.0, "@scope (.light) to (.dark) {}");
    }

    struct ScopeDisplay<'a>(
        Option<&'a str>,
        Option<&'a str>,
        &'a [RuleOrAtRule],
    );
    impl<'a> fmt::Display for ScopeDisplay<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            render(f, self.0, self.1, self.2)
        }
    }
}
