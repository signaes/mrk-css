//! At-rule types: [`AtRule`], [`RuleOrAtRule`], [`Keyframe`].
//!
//! The variants are defined in this file; helper constructors and
//! per-variant rendering live in per-at-rule submodules:
//! [`media`], [`supports`], [`container`], [`scope`], [`layer`],
//! [`keyframes`], [`font_face`], [`page`], [`import`], [`charset`],
//! [`namespace`].

use std::borrow::Cow;
use std::fmt;

use crate::css::declaration::Declaration;

pub mod charset;
pub mod container;
pub mod font_face;
pub mod import;
pub mod keyframes;
pub mod layer;
pub mod media;
pub mod namespace;
pub mod page;
pub mod scope;
pub mod supports;

/// A CSS at-rule.
///
/// Covers every standard at-rule: `@media`, `@supports`,
/// `@container`, `@scope`, `@layer`, `@keyframes`, `@font-face`,
/// `@page`, `@import`, `@charset`, `@namespace`.
///
/// Each variant's fields are self-documenting; individual field
/// docs are omitted to keep the AST concise.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum AtRule {
    /// `@media (query) { rules }`
    Media {
        query: Cow<'static, str>,
        rules: Vec<RuleOrAtRule>,
    },
    /// `@supports (condition) { rules }`
    Supports {
        condition: Cow<'static, str>,
        rules: Vec<RuleOrAtRule>,
    },
    /// `@container [name] (query) { rules }`
    Container {
        name: Option<Cow<'static, str>>,
        query: Cow<'static, str>,
        rules: Vec<RuleOrAtRule>,
    },
    /// `@scope [(root)] [to (limit)] { rules }`
    Scope {
        root: Option<Cow<'static, str>>,
        limit: Option<Cow<'static, str>>,
        rules: Vec<RuleOrAtRule>,
    },
    /// `@layer [name] { rules }` or just `@layer name;`
    Layer {
        name: Option<Cow<'static, str>>,
        rules: Vec<RuleOrAtRule>,
    },
    /// `@keyframes name { ... }`
    Keyframes {
        name: Cow<'static, str>,
        keyframes: Vec<Keyframe>,
    },
    /// `@font-face { ... }`
    FontFace {
        declarations: Vec<Declaration>,
    },
    /// `@page [pseudo] { ... }`
    Page {
        pseudo: Option<Cow<'static, str>>,
        declarations: Vec<Declaration>,
    },
    /// `@import url [supports] [media];`
    Import {
        url: Cow<'static, str>,
        supports: Option<Cow<'static, str>>,
        media: Option<Cow<'static, str>>,
    },
    /// `@charset "encoding";`
    Charset {
        encoding: Cow<'static, str>,
    },
    /// `@namespace [prefix] url;`
    Namespace {
        prefix: Option<Cow<'static, str>>,
        url: Cow<'static, str>,
    },
}

/// A single keyframe block in a `@keyframes` rule.
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Keyframe selectors (e.g. `"from"`, `"to"`, `"50%"`).
    pub selectors: Vec<Cow<'static, str>>,
    /// Declarations at this keyframe stop.
    pub declarations: Vec<Declaration>,
}

/// Top-level item in a [`crate::css::StyleSheet`]: either a
/// [`crate::css::rule::Rule`] or an [`AtRule`].
#[derive(Debug, Clone)]
pub enum RuleOrAtRule {
    /// A CSS rule with selectors + declarations (+ optional nesting).
    Rule(crate::css::rule::Rule),
    /// A CSS at-rule.
    AtRule(AtRule),
}

// ── Helper constructors ─────────────────────────────────────────────

impl AtRule {
    /// Create an `@media` at-rule.
    pub fn media(query: impl Into<Cow<'static, str>>) -> AtRuleBuilder {
        AtRuleBuilder {
            at_rule: AtRule::Media {
                query: query.into(),
                rules: Vec::new(),
            },
        }
    }

    /// Create a `@supports` at-rule.
    pub fn supports(condition: impl Into<Cow<'static, str>>) -> AtRuleBuilder {
        AtRuleBuilder {
            at_rule: AtRule::Supports {
                condition: condition.into(),
                rules: Vec::new(),
            },
        }
    }

    /// Create a `@keyframes` at-rule.
    pub fn keyframes(name: impl Into<Cow<'static, str>>) -> AtRuleBuilder {
        AtRuleBuilder {
            at_rule: AtRule::Keyframes {
                name: name.into(),
                keyframes: Vec::new(),
            },
        }
    }

    /// Create a `@font-face` at-rule.
    pub fn font_face() -> AtRuleBuilder {
        AtRuleBuilder {
            at_rule: AtRule::FontFace {
                declarations: Vec::new(),
            },
        }
    }

    /// Create an `@import` at-rule.
    pub fn import(url: impl Into<Cow<'static, str>>) -> AtRule {
        AtRule::Import {
            url: url.into(),
            supports: None,
            media: None,
        }
    }

    /// Create a `@charset` at-rule.
    pub fn charset(encoding: impl Into<Cow<'static, str>>) -> AtRule {
        AtRule::Charset {
            encoding: encoding.into(),
        }
    }

    /// Create a `@namespace` at-rule.
    pub fn namespace(url: impl Into<Cow<'static, str>>) -> AtRule {
        AtRule::Namespace {
            prefix: None,
            url: url.into(),
        }
    }
}

// ── Display ─────────────────────────────────────────────────────────

impl fmt::Display for AtRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtRule::Media { query, rules } => media::render(f, query, rules),
            AtRule::Supports { condition, rules } => supports::render(f, condition, rules),
            AtRule::Container { name, query, rules } => container::render(f, name.as_deref(), query, rules),
            AtRule::Scope { root, limit, rules } => scope::render(f, root.as_deref(), limit.as_deref(), rules),
            AtRule::Layer { name, rules } => layer::render(f, name.as_deref(), rules),
            AtRule::Keyframes { name, keyframes } => keyframes::render(f, name, keyframes),
            AtRule::FontFace { declarations } => font_face::render(f, declarations),
            AtRule::Page { pseudo, declarations } => page::render(f, pseudo.as_deref(), declarations),
            AtRule::Import { url, supports, media } => import::render(f, url, supports.as_deref(), media.as_deref()),
            AtRule::Charset { encoding } => charset::render(f, encoding),
            AtRule::Namespace { prefix, url } => namespace::render(f, prefix.as_deref(), url),
        }
    }
}

impl fmt::Display for RuleOrAtRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleOrAtRule::Rule(r) => fmt::Display::fmt(r, f),
            RuleOrAtRule::AtRule(a) => fmt::Display::fmt(a, f),
        }
    }
}

impl fmt::Display for Keyframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        keyframes::render_keyframe(f, self)
    }
}

// ── AtRuleBuilder ───────────────────────────────────────────────────

/// Fluent builder for at-rules that can contain rules or declarations.
#[derive(Debug, Clone)]
pub struct AtRuleBuilder {
    at_rule: AtRule,
}

impl AtRuleBuilder {
    /// Add a rule via a closure (for `@media`, `@supports`, etc.).
    pub fn rule(mut self, f: impl FnOnce(crate::css::rule::RuleBuilder) -> crate::css::rule::RuleBuilder) -> Self {
        let rule = f(crate::css::rule::RuleBuilder::new()).build();
        self.add_rule(rule);
        self
    }

    fn add_rule(&mut self, rule: crate::css::rule::Rule) {
        match &mut self.at_rule {
            AtRule::Media { rules, .. }
            | AtRule::Supports { rules, .. }
            | AtRule::Layer { rules, .. } => {
                rules.push(RuleOrAtRule::Rule(rule));
            }
            AtRule::Keyframes { keyframes, .. } => {
                // Flatten: wrap each selector as a Keyframe
                // This is a simplification; for now push a keyframe
                // with the rule's selectors and declarations.
                let selectors: Vec<Cow<'static, str>> = rule.selectors.iter()
                    .map(|s| Cow::Owned(s.to_string()))
                    .collect();
                keyframes.push(Keyframe {
                    selectors,
                    declarations: rule.declarations,
                });
            }
            AtRule::FontFace { declarations, .. } => {
                declarations.extend(rule.declarations);
            }
            // Container, Scope, Page, Import, Charset, Namespace
            // have no rule-builder methods, so no rules can be added
            // through this path.
            _ => {}
        }
    }

    /// Add a declaration (for `@font-face`, `@page`, `@keyframes`).
    pub fn decl(mut self, decl: Declaration) -> Self {
        match &mut self.at_rule {
            AtRule::FontFace { declarations, .. }
            | AtRule::Page { declarations, .. } => {
                declarations.push(decl);
            }
            AtRule::Keyframes { keyframes, .. } => {
                if let Some(last) = keyframes.last_mut() {
                    last.declarations.push(decl);
                }
            }
            _ => {}
        }
        self
    }

    /// Add a property via name/value (for `@font-face`, `@page`).
    pub fn property(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<crate::css::properties::Value>) -> Self {
        self = self.decl(Declaration::new(name.into(), value.into()));
        self
    }

    /// Build the `AtRule`.
    pub fn build(self) -> AtRule {
        self.at_rule
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::selector::Selector;
    use crate::css::values::Color;

    // ── Helper constructors ──────────────────────────────────────

    #[test]
    fn media_constructor() {
        let at = AtRule::media("(min-width: 800px)").build();
        assert!(at.to_string().contains("@media"));
    }

    #[test]
    fn supports_constructor() {
        let at = AtRule::supports("display: flex").build();
        assert!(at.to_string().contains("@supports"));
    }

    #[test]
    fn keyframes_constructor() {
        let at = AtRule::keyframes("fade").build();
        assert!(at.to_string().contains("@keyframes"));
    }

    #[test]
    fn font_face_constructor() {
        let at = AtRule::font_face().build();
        assert!(at.to_string().contains("@font-face"));
    }

    #[test]
    fn import_constructor() {
        let at = AtRule::import("style.css");
        assert!(at.to_string().contains("@import"));
    }

    #[test]
    fn charset_constructor() {
        let at = AtRule::charset("UTF-8");
        assert!(
            matches!(at, AtRule::Charset { ref encoding } if encoding == "UTF-8")
        );
    }

    #[test]
    fn namespace_constructor() {
        let at = AtRule::namespace("http://www.w3.org/2000/svg");
        assert!(at.to_string().contains("@namespace"));
    }

    // ── Display ─────────────────────────────────────────────────

    #[test]
    fn display_media_empty() {
        let at = AtRule::media("screen").build();
        let s = at.to_string();
        assert_eq!(s, "@media screen {}");
    }

    #[test]
    fn display_media_with_rule() {
        let at = AtRule::media("(min-width: 800px)")
            .rule(|r| r.selector(Selector::class("btn")).property("color", Color::named("red")))
            .build();
        let s = at.to_string();
        assert!(s.contains("@media (min-width: 800px) {"));
        assert!(s.contains(".btn"));
        assert!(s.contains("color: red"));
    }

    #[test]
    fn display_supports() {
        let at = AtRule::supports("display: grid").build();
        let s = at.to_string();
        assert_eq!(s, "@supports display: grid {}");
    }

    #[test]
    fn display_keyframes() {
        let at = AtRule::keyframes("fade").build();
        let s = at.to_string();
        assert_eq!(s, "@keyframes fade {}");
    }

    #[test]
    fn display_font_face() {
        let at = AtRule::font_face().build();
        let s = at.to_string();
        assert_eq!(s, "@font-face {}");
    }

    #[test]
    fn display_import() {
        let at = AtRule::import("style.css");
        let s = at.to_string();
        assert_eq!(s, "@import \"style.css\";");
    }

    #[test]
    fn display_charset() {
        let at = AtRule::charset("UTF-8");
        let s = at.to_string();
        assert_eq!(s, "@charset \"UTF-8\";");
    }

    #[test]
    fn display_namespace() {
        let at = AtRule::namespace("http://www.w3.org/2000/svg");
        let s = at.to_string();
        assert_eq!(s, "@namespace \"http://www.w3.org/2000/svg\";");
    }

    // ── RuleOrAtRule ─────────────────────────────────────────────

    #[test]
    fn rule_or_at_rule_rule() {
        use crate::css::rule::RuleBuilder;
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .property("color", Color::named("red"))
            .build();
        let item = RuleOrAtRule::Rule(r);
        assert!(item.to_string().contains(".btn"));
    }

    #[test]
    fn rule_or_at_rule_at_rule() {
        let item = RuleOrAtRule::AtRule(AtRule::import("style.css"));
        assert!(item.to_string().contains("@import"));
    }

    // ── Keyframe ─────────────────────────────────────────────────

    #[test]
    fn keyframe_construction() {
        let kf = Keyframe {
            selectors: vec!["from".into(), "to".into()],
            declarations: vec![
                Declaration::new("opacity", crate::css::properties::Value::Number(0.0.into())),
            ],
        };
        assert_eq!(kf.selectors.len(), 2);
    }

    #[test]
    fn keyframe_display() {
        let kf = Keyframe {
            selectors: vec!["from".into()],
            declarations: vec![
                Declaration::new("opacity", crate::css::properties::Value::Number(0.0.into())),
            ],
        };
        let s = kf.to_string();
        assert!(s.contains("from"));
        assert!(s.contains("opacity"));
    }

    // ── AtRuleBuilder ────────────────────────────────────────────

    #[test]
    fn builder_media_with_rules() {
        let at = AtRule::media("screen")
            .rule(|r| r.selector(Selector::class("btn")).property("color", Color::named("red")))
            .rule(|r| r.selector(Selector::class("link")).property("color", Color::named("blue")))
            .build();
        assert!(matches!(&at, AtRule::Media { rules, .. } if rules.len() == 2));
    }

    #[test]
    fn builder_font_face_with_decls() {
        let at = AtRule::font_face()
            .property("font-family", "\"My Font\"")
            .property("src", "url('font.woff2')")
            .build();
        assert!(matches!(&at, AtRule::FontFace { declarations, .. } if declarations.len() == 2));
    }

    #[test]
    fn builder_keyframes_with_rules() {
        let at = AtRule::keyframes("fade")
            .rule(|r| r.selector(Selector::Universal).property("opacity", crate::css::values::Number::from(0.0)))
            .build();
        assert!(matches!(&at, AtRule::Keyframes { keyframes, .. } if keyframes.len() == 1));
    }

    #[test]
    fn at_rule_clone() {
        let a = AtRule::import("style.css");
        let b = a.clone();
        assert_eq!(a.to_string(), b.to_string());
    }

    #[test]
    fn at_rule_debug() {
        let at = AtRule::charset("UTF-8");
        let debug = format!("{:?}", at);
        assert!(debug.contains("Charset"));
    }

    // ── Coverage: exercise fallback branches ─────────────────────

    #[test]
    fn builder_decl_noop_for_media() {
        // decl() called on a Media builder is a no-op (Media has no decls field).
        let at = AtRule::media("screen")
            .decl(Declaration::new("color", crate::css::properties::Value::Number(1.0.into())))
            .build();
        assert!(matches!(&at, AtRule::Media { rules, .. } if rules.is_empty()));
    }

    #[test]
    fn builder_decl_noop_for_supports() {
        let at = AtRule::supports("display: flex")
            .property("color", "red")
            .build();
        assert!(matches!(&at, AtRule::Supports { rules, .. } if rules.is_empty()));
    }

    #[test]
    fn builder_decl_noop_for_keyframes_empty() {
        // decl() on Keyframes with no existing keyframe is a no-op.
        let at = AtRule::keyframes("fade")
            .decl(Declaration::new("opacity", crate::css::properties::Value::Number(0.0.into())))
            .build();
        assert!(matches!(&at, AtRule::Keyframes { keyframes, .. } if keyframes.is_empty()));
    }

    #[test]
    fn builder_rule_for_keyframes_creates_keyframe() {
        // add_rule on Keyframes creates a Keyframe from the rule's selectors.
        let at = AtRule::keyframes("fade")
            .rule(|r| r.selector(Selector::pseudo_class("from")).property("opacity", crate::css::values::Number::from(0.0)))
            .build();
        assert!(matches!(&at, AtRule::Keyframes { keyframes, .. } if keyframes.len() == 1 && keyframes[0].selectors.len() == 1));
    }

    #[test]
    fn rule_or_at_rule_display_at_rule_variant() {
        // Exercise the AtRule branch of RuleOrAtRule::Display.
        let rule = RuleOrAtRule::AtRule(AtRule::import("a.css"));
        let s = format!("{}", rule);
        assert!(s.contains("@import"));
    }

    #[test]
    fn rule_or_at_rule_display_rule_variant() {
        let r = crate::css::rule::RuleBuilder::new()
            .selector(Selector::class("x"))
            .property("color", Color::named("red"))
            .build();
        let item = RuleOrAtRule::Rule(r);
        let s = format!("{}", item);
        assert!(s.contains(".x"));
    }

    #[test]
    fn keyframe_display_via_at_rule() {
        // Exercise Keyframe::Display via the keyframes::render_keyframe path.
        let kf = Keyframe {
            selectors: vec![Cow::Borrowed("from"), Cow::Borrowed("to")],
            declarations: vec![Declaration::new(
                "opacity",
                crate::css::properties::Value::Number(1.0.into()),
            )],
        };
        let s = kf.to_string();
        assert!(s.contains("from"));
        assert!(s.contains("to"));
        assert!(s.contains("opacity"));
    }

    // ── Coverage: AtRule variants with optional fields ────────

    #[test]
    fn display_at_rule_import_with_supports() {
        let at = AtRule::Import {
            url: Cow::Borrowed("style.css"),
            supports: Some(Cow::Borrowed("display: flex")),
            media: None,
        };
        let s = at.to_string();
        assert!(s.contains("supports(display: flex)"));
    }

    #[test]
    fn display_at_rule_import_with_media() {
        let at = AtRule::Import {
            url: Cow::Borrowed("style.css"),
            supports: None,
            media: Some(Cow::Borrowed("screen")),
        };
        let s = at.to_string();
        assert!(s.contains("screen"));
    }

    #[test]
    fn display_at_rule_import_with_both() {
        let at = AtRule::Import {
            url: Cow::Borrowed("style.css"),
            supports: Some(Cow::Borrowed("display: flex")),
            media: Some(Cow::Borrowed("screen")),
        };
        let s = at.to_string();
        assert!(s.contains("supports"));
        assert!(s.contains("screen"));
    }

    #[test]
    fn display_at_rule_namespace_with_prefix() {
        let at = AtRule::Namespace {
            prefix: Some(Cow::Borrowed("svg")),
            url: Cow::Borrowed("http://www.w3.org/2000/svg"),
        };
        let s = at.to_string();
        assert!(s.contains("svg"));
        assert!(s.contains("http://www.w3.org/2000/svg"));
    }

    #[test]
    fn builder_page_with_pseudo() {
        let at = AtRule::Page {
            pseudo: Some(Cow::Borrowed(":first")),
            declarations: vec![],
        };
        assert_eq!(at.to_string(), "@page :first {}");
    }

    #[test]
    fn builder_page_with_declaration() {
        let at = AtRule::Page {
            pseudo: None,
            declarations: vec![Declaration::new(
                "margin",
                crate::css::properties::Value::Number(1.0.into()),
            )],
        };
        let s = at.to_string();
        assert!(s.contains("margin"));
    }

    #[test]
    fn builder_keyframes_with_multiple_selectors() {
        let at = AtRule::keyframes("fade")
            .rule(|r| r.selector(Selector::Class(Cow::Borrowed("from"))).property("opacity", crate::css::values::Number::from(0.0)))
            .rule(|r| r.selector(Selector::Class(Cow::Borrowed("to"))).property("opacity", crate::css::values::Number::from(1.0)))
            .build();
        assert!(matches!(&at, AtRule::Keyframes { keyframes, .. }
            if keyframes.len() == 2 && keyframes[0].selectors.len() == 1 && keyframes[1].selectors.len() == 1));
    }

    #[test]
    fn builder_font_face_with_property() {
        let at = AtRule::font_face()
            .property("font-family", "\"Test Font\"")
            .build();
        assert!(matches!(&at, AtRule::FontFace { declarations, .. } if declarations.len() == 1));
    }

    // ── Coverage: render function branches ───────────────────────

    #[test]
    fn display_at_rule_charset() {
        let at = AtRule::charset("UTF-8");
        assert_eq!(at.to_string(), "@charset \"UTF-8\";");
    }

    #[test]
    fn display_at_rule_font_face_empty() {
        let at = AtRule::FontFace { declarations: vec![] };
        assert_eq!(at.to_string(), "@font-face {}");
    }

    #[test]
    fn display_at_rule_media_empty() {
        let at = AtRule::Media {
            query: Cow::Borrowed("screen"),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@media screen {}");
    }

    #[test]
    fn display_at_rule_supports_empty() {
        let at = AtRule::Supports {
            condition: Cow::Borrowed("display: flex"),
            rules: vec![],
        };
        assert_eq!(at.to_string(), "@supports display: flex {}");
    }

    #[test]
    fn display_at_rule_keyframes_empty() {
        let at = AtRule::Keyframes {
            name: Cow::Borrowed("fade"),
            keyframes: vec![],
        };
        assert_eq!(at.to_string(), "@keyframes fade {}");
    }

    #[test]
    fn display_at_rule_namespace_empty_prefix() {
        let at = AtRule::Namespace {
            prefix: None,
            url: Cow::Borrowed("http://www.w3.org/2000/svg"),
        };
        assert_eq!(
            at.to_string(),
            "@namespace \"http://www.w3.org/2000/svg\";"
        );
    }

    #[test]
    fn display_at_rule_import_no_options() {
        let at = AtRule::Import {
            url: Cow::Borrowed("style.css"),
            supports: None,
            media: None,
        };
        assert_eq!(at.to_string(), "@import \"style.css\";");
    }

    #[test]
    fn add_rule_to_font_face_via_builder() {
        // Calling .rule() on a FontFace builder extends declarations.
        let at = AtRule::font_face()
            .rule(|r| {
                r.selector(Selector::Universal)
                    .property("src", "url(font.woff2)")
            })
            .build();
        assert!(at.to_string().contains("@font-face"));
        assert!(at.to_string().len() > "@font-face {}".len());
    }

    #[test]
    fn add_rule_to_page_via_builder() {
        // Page arm of add_rule is unreachable from any builder, but
        // we can construct a Page directly and exercise the
        // rendering path.
        let at = AtRule::Page {
            pseudo: None,
            declarations: vec![Declaration::new(
                "margin",
                crate::css::properties::Value::Number(1.0.into()),
            )],
        };
        assert!(at.to_string().contains("margin"));
    }

    #[test]
    fn builder_rule_noop_for_container() {
        // The `_ => {}` wildcard arm in add_rule for types without
        // rule-builder support (Container, Scope, Page, Import, etc.).
        use std::borrow::Cow;
        let at = AtRuleBuilder {
            at_rule: AtRule::Container {
                name: Some(Cow::Borrowed("sidebar")),
                query: Cow::Borrowed("inline-size"),
                rules: vec![],
            },
        }
            .rule(|r| r.selector(Selector::class("x")).property("color", Color::named("red")))
            .build();
        assert!(at.to_string().contains("@container"));
    }

    #[test]
    fn builder_decl_for_keyframes_with_keyframe() {
        // decl() on Keyframes with an existing keyframe pushes to
        // its declarations, hitting the `Some(last)` branch.
        let at = AtRule::keyframes("fade")
            .rule(|r| r.selector(Selector::pseudo_class("from")).property("opacity", crate::css::values::Number::from(0.0)))
            .decl(Declaration::new("animation-timing-function", "ease".into()))
            .build();
        assert!(matches!(&at, AtRule::Keyframes { keyframes, .. }
            if keyframes.len() == 1 && !keyframes[0].declarations.is_empty()));
    }

    #[test]
    fn add_rule_to_layer_via_builder() {
        let at = AtRuleBuilder {
            at_rule: AtRule::Layer {
                name: Some(Cow::Borrowed("utilities")),
                rules: vec![],
            },
        }
            .rule(|r| r.selector(Selector::class("x")).property("color", Color::named("red")))
            .build();
        let s = at.to_string();
        assert!(s.contains("utilities"));
        assert!(s.contains(".x"));
    }

    #[test]
    fn builder_decl_for_page() {
        let at = AtRuleBuilder {
            at_rule: AtRule::Page {
                pseudo: None,
                declarations: vec![],
            },
        }
            .decl(Declaration::new("margin", crate::css::properties::Value::Number(1.0.into())))
            .build();
        let s = at.to_string();
        assert!(s.contains("margin"));
    }
}
