//! `@page` at-rule rendering.

use std::fmt;

use crate::css::at_rules::PageMarginBox;
use crate::css::declaration::Declaration;

/// Render a `@page [pseudo] { ... }` block, including any page-margin
/// boxes (`@top-left`, `@bottom-center`, …) declared inside it.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    pseudo: Option<&str>,
    declarations: &[Declaration],
    margin_boxes: &[PageMarginBox],
) -> fmt::Result {
    let mut s = String::from("@page");
    if let Some(p) = pseudo { s.push_str(&format!(" {}", p)); }
    s.push_str(" {");
    for d in declarations {
        s.push_str(&format!("\n  {}", d));
    }
    for mb in margin_boxes {
        if mb.declarations.is_empty() {
            s.push_str(&format!("\n  {} {{}}", mb.area));
        } else {
            s.push_str(&format!("\n  {} {{", mb.area));
            for d in &mb.declarations {
                s.push_str(&format!("\n    {}", d));
            }
            s.push('\n');
            s.push_str("  }");
        }
    }
    if !declarations.is_empty() || !margin_boxes.is_empty() { s.push('\n'); }
    s.push('}');
    f.write_str(&s)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::css::at_rules::AtRule;
    use crate::css::declaration::Declaration;
    use crate::css::properties::Value;

    #[test]
    fn display_at_rule_page_no_pseudo_empty() {
        let at = AtRule::Page {
            pseudo: None,
            declarations: vec![],
            margin_boxes: vec![],
        };
        assert_eq!(at.to_string(), "@page {}");
    }

    #[test]
    fn display_at_rule_page_with_pseudo_empty() {
        let at = AtRule::Page {
            pseudo: Some(Cow::Borrowed(":first")),
            declarations: vec![],
            margin_boxes: vec![],
        };
        assert_eq!(at.to_string(), "@page :first {}");
    }

    #[test]
    fn display_at_rule_page_with_decl() {
        let at = AtRule::Page {
            pseudo: None,
            declarations: vec![Declaration::new("margin", Value::Number(1.0.into()))],
            margin_boxes: vec![],
        };
        let s = at.to_string();
        assert!(s.contains("@page {"));
        assert!(s.contains("margin: 1"));
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
        std::fmt::write(&mut w, format_args!("{}", PageDisplay(Some(":first"), &[], &[]))).unwrap();
        assert_eq!(w.0, "@page :first {}");
    }

    struct PageDisplay<'a>(Option<&'a str>, &'a [Declaration], &'a [crate::css::at_rules::PageMarginBox]);
    impl<'a> fmt::Display for PageDisplay<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            render(f, self.0, self.1, self.2)
        }
    }

    // ── Page-margin boxes ────────────────────────────────────────

    #[test]
    fn display_at_rule_page_with_margin_box() {
        let mb = PageMarginBox {
            area: Cow::Borrowed("@top-left"),
            declarations: vec![Declaration::new("content", Value::String("Header".into()))],
        };
        let at = AtRule::Page {
            pseudo: None,
            declarations: vec![],
            margin_boxes: vec![mb],
        };
        assert_eq!(
            at.to_string(),
            "@page {\n  @top-left {\n    content: \"Header\";\n  }\n}"
        );
    }

    #[test]
    fn display_at_rule_page_with_pseudo_and_margin_boxes() {
        let at = AtRule::Page {
            pseudo: Some(Cow::Borrowed(":first")),
            declarations: vec![Declaration::new("margin", Value::Number(1.0.into()))],
            margin_boxes: vec![
                PageMarginBox {
                    area: Cow::Borrowed("@top-left"),
                    declarations: vec![Declaration::new("content", Value::String("H".into()))],
                },
                PageMarginBox {
                    area: Cow::Borrowed("@bottom-center"),
                    declarations: vec![],
                },
            ],
        };
        let s = at.to_string();
        assert!(s.starts_with("@page :first {"));
        assert!(s.contains("margin: 1"));
        assert!(s.contains("@top-left {"));
        assert!(s.contains("content: \"H\""));
        assert!(s.contains("@bottom-center {}"));
    }

    #[test]
    fn page_margin_box_empty_renders_with_braces() {
        let at = AtRule::Page {
            pseudo: None,
            declarations: vec![],
            margin_boxes: vec![PageMarginBox {
                area: Cow::Borrowed("@top-left"),
                declarations: vec![],
            }],
        };
        assert_eq!(at.to_string(), "@page {\n  @top-left {}\n}");
    }
}
