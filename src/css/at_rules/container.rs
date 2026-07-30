//! `@container` at-rule rendering.

use std::fmt;

use super::RuleOrAtRule;

/// Render a `@container [name] (query) { ... }` block.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    name: Option<&str>,
    query: &str,
    rules: &[RuleOrAtRule],
) -> fmt::Result {
    let mut s = String::from("@container");
    if let Some(n) = name { s.push_str(&format!(" {}", n)); }
    s.push_str(&format!(" ({}) {{", query));
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
    fn display_at_rule_container_no_name_empty() {
        let at = AtRule::Container {
            name: None,
            query: Cow::Borrowed("min-width: 800px"),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@container (min-width: 800px) {}");
    }

    #[test]
    fn display_at_rule_container_with_name_empty() {
        let at = AtRule::Container {
            name: Some(Cow::Borrowed("sidebar")),
            query: Cow::Borrowed("min-width: 800px"),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@container sidebar (min-width: 800px) {}");
    }

    #[test]
    fn display_at_rule_container_with_rule() {
        let rule = RuleBuilder::new()
            .selector(Selector::class("box"))
            .property("color", Color::named("blue"))
            .build();
        let at = AtRule::Container {
            name: None,
            query: Cow::Borrowed("min-width: 800px"),
            rules: vec![RuleOrAtRule::Rule(rule)],
        };
        let s = at.to_string();
        assert!(s.contains("@container (min-width: 800px) {"));
        assert!(s.contains(".box"));
        assert!(s.contains("color: blue"));
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
        let result = std::fmt::write(
            &mut w,
            format_args!("{}", ContainerDisplay(&Cow::Borrowed("sidebar"), &Cow::Borrowed("min-width: 800px"), &[])),
        );
        result.unwrap();
        assert_eq!(w.0, "@container sidebar (min-width: 800px) {}");
    }

    struct ContainerDisplay<'a>(&'a Cow<'static, str>, &'a Cow<'static, str>, &'a [RuleOrAtRule]);
    impl<'a> fmt::Display for ContainerDisplay<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = Some(self.0.as_ref());
            render(f, name, self.1, self.2)
        }
    }
}
