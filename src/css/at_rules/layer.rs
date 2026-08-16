//! `@layer` at-rule rendering.

use std::fmt;

use super::RuleOrAtRule;

/// Render a `@layer [name] { ... }` block or `@layer name;`.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    name: Option<&str>,
    rules: &[RuleOrAtRule],
) -> fmt::Result {
    let mut s = String::from("@layer");
    if let Some(n) = name {
        s.push_str(&format!(" {}", n));
    }
    if rules.is_empty() {
        s.push(';');
    } else {
        s.push_str(" {");
        for r in rules {
            s.push_str(&format!("\n  {}", r));
        }
        s.push_str("\n}");
    }
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
    fn display_at_rule_layer_with_name_empty() {
        let at = AtRule::Layer {
            name: Some(Cow::Borrowed("reset")),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@layer reset;");
    }

    #[test]
    fn display_at_rule_layer_no_name_empty() {
        let at = AtRule::Layer {
            name: None,
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@layer;");
    }

    #[test]
    fn display_at_rule_layer_with_rule() {
        let rule = RuleBuilder::new()
            .selector(Selector::class("box"))
            .property("color", Color::named("red"))
            .build();
        let at = AtRule::Layer {
            name: Some(Cow::Borrowed("base")),
            rules: vec![RuleOrAtRule::Rule(rule)],
        };
        let s = at.to_string();
        assert_eq!(s, "@layer base {\n  .box {\n    color: red;\n  }\n}");
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
        std::fmt::write(&mut w, format_args!("{}", LayerDisplay(Some("reset"), &[]))).unwrap();
        assert_eq!(w.0, "@layer reset;");
    }

    struct LayerDisplay<'a>(Option<&'a str>, &'a [RuleOrAtRule]);
    impl<'a> fmt::Display for LayerDisplay<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            render(f, self.0, self.1)
        }
    }
}
