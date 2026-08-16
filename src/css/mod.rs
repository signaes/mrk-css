//! Type-safe CSS authoring.
//!
//! `mrk_css::css` provides a fluent, type-safe API for building CSS
//! stylesheets that mirrors the data model used by `mrk`'s `html` and
//! `svg` modules. Compose [`StyleSheet`]s from [`Rule`](rule::Rule)s and at-rules,
//! attach declarations with strongly-typed property setters, and
//! render to a canonical pretty-printed CSS string.
//!
//! The crate depends on [`mrk`](https://github.com/signaes/mrk) only
//! for the [`Renderable`] trait and the
//! [`Node`](mrk::Node) conversion; CSS conversion math and the Color 4
//! parser are pure functions of the typed values.
//!
//! ## Quick start
//!
//! ```
//! use mrk_css::css::*;
//! use mrk_css::css::values::{Color, Length};
//! use mrk_css::Renderable;
//!
//! let sheet = StyleSheet::new()
//!     .rule(|s| s.selector(Selector::class("btn")).block(|r| {
//!         r.color(Color::named("rebeccapurple"))
//!          .padding(Value::List(vec![Length::px(8.0).into(), Length::px(16.0).into()]))
//!          .background_color(Color::named("white"))
//!     }))
//!     .at_rule(
//!         AtRule::media("(min-width: 800px)")
//!             .rule(|s| s.selector(Selector::class("btn")).block(|r| {
//!                 r.font_size(Length::px(18.0))
//!             }))
//!             .build(),
//!     )
//!     .build();
//!
//! let css = sheet.render();
//! assert!(css.contains(".btn"));
//! assert!(css.contains("@media (min-width: 800px)"));
//! ```
//!
//! The [`css!`](crate::css!) macro compiles CSS-like syntax into the
//! same [`StyleSheet`] structure, with typed value parsing, nesting,
//! and every at-rule:
//!
//! ```
//! use mrk_css::{css, Renderable};
//!
//! let sheet = css! {
//!     :root { --brand: rebeccapurple; }
//!     .btn {
//!         color: var(--brand);
//!         padding: 8px 16px;
//!         &:hover { color: blue; }
//!     }
//!     @media (min-width: 800px) {
//!         .btn { padding: 16px 32px; }
//!     }
//! };
//!
//! let css = sheet.render();
//! assert!(css.contains("--brand: rgb(102, 51, 153);"));
//! assert!(css.contains("padding: 8px 16px;"));
//! assert!(css.contains("@media (min-width: 800px)"));
//! ```
//!
//! # Architecture
//!
//! - [`StyleSheet`] — top-level container for rules and at-rules.
//! - [`Rule`](rule::Rule) — a selector list + declarations + (optional) nested
//!   rules. CSS nesting via `&` is supported natively.
//! - [`AtRule`] — every standard at-rule (`@media`, `@supports`,
//!   `@container`, `@scope`, `@layer`, `@keyframes`, `@font-face`,
//!   `@page`, `@import`, `@charset`, `@namespace`).
//! - [`Declaration`] — `name: value;` (with optional `!important`).
//! - [`Selector`] — type / class / id / attribute / pseudo / compound
//!   / combinators.
//! - [`Value`] — the property-value AST; all entries are typed.
//! - Typed values in `values`: `Color`, `Length`, `Percentage`,
//!   `Time`, `Angle`, `Number`, `Integer`, `CssString`, `Url`,
//!   `Ident`, `CustomProperty`, `EasingFunction`, `Frequency`,
//!   `Resolution`.
//!
//! # Color conversions
//!
//! `Color` carries `ColorKind`. Every `Color` can be converted to
//! any of `sRGB`, `HSL`, `OKLab`, `OKLCH`, or hex via `into_rgb`,
//! `into_hsl`, `into_oklab`, `into_oklch`, `into_hex`. All
//! conversions are `Result`-wrapped; out-of-gamut values are reduced
//! via binary-search chroma reduction in OKLCH. See
//! `ConversionError`.
//!
//! Conversions are pure functions with no internal caching. For
//! workloads that convert the same set of colors repeatedly, compute
//! once at the right boundary or wrap calls in your own `HashMap`.

use crate::css::at_rules::RuleOrAtRule;
use mrk::Renderable;

pub mod values;

/// A collection of CSS rules and at-rules.
///
/// See the [module-level documentation](self) for an overview of how
/// to build and render stylesheets.
pub struct StyleSheet {
    items: Vec<crate::css::at_rules::RuleOrAtRule>,
}

/// Builder for a [`StyleSheet`].
///
/// Constructed via [`StyleSheet::new`]. Chain `.rule(...)` /
/// `.at_rule(...)` calls to populate, then `.build()` to seal.
pub struct StyleSheetBuilder {
    items: Vec<crate::css::at_rules::RuleOrAtRule>,
}

impl StyleSheet {
    /// Construct an empty stylesheet builder.
    ///
    /// Named `new` for discoverability even though it returns the
    /// builder rather than a `StyleSheet` directly.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> StyleSheetBuilder {
        StyleSheetBuilder { items: Vec::new() }
    }

    /// Construct a stylesheet directly from its items.
    ///
    /// Used by the runtime parser behind the [`css!`](crate::css!)
    /// macro.
    pub(crate) fn from_items(items: Vec<crate::css::at_rules::RuleOrAtRule>) -> StyleSheet {
        StyleSheet { items }
    }

    /// The rules and at-rules of this stylesheet, in source order.
    ///
    /// Enables downstream consumers (e.g. `mrk-pdf`'s layout engine) to
    /// perform selector matching and cascade resolution over a parsed
    /// stylesheet.
    pub fn items(&self) -> &[crate::css::at_rules::RuleOrAtRule] {
        &self.items
    }
}

impl StyleSheetBuilder {
    /// Add a CSS rule via a builder closure.
    pub fn rule(
        mut self,
        f: impl FnOnce(crate::css::rule::RuleBuilder) -> crate::css::rule::RuleBuilder,
    ) -> Self {
        let rule = f(crate::css::rule::RuleBuilder::new()).build();
        self.items.push(RuleOrAtRule::Rule(rule));
        self
    }

    /// Add a pre-built at-rule.
    pub fn at_rule(mut self, at_rule: crate::css::at_rules::AtRule) -> Self {
        self.items.push(RuleOrAtRule::AtRule(at_rule));
        self
    }

    /// Build a `StyleSheet` from the current state.
    pub fn build(self) -> StyleSheet {
        StyleSheet { items: self.items }
    }
}

impl Renderable for StyleSheet {
    fn render(&self) -> String {
        crate::css::render::render_sheet(self)
    }
}

impl From<StyleSheet> for mrk::Node {
    fn from(sheet: StyleSheet) -> Self {
        mrk::Node::Raw(std::borrow::Cow::Owned(sheet.render()))
    }
}

pub use crate::css::at_rules::AtRule;
pub use crate::css::selector::Selector;
pub use declaration::Declaration;
pub use properties::Value;

pub mod at_rules;
pub(crate) mod declaration;
pub(crate) mod macros;
#[doc(hidden)]
pub mod parse;
pub(crate) mod properties;
pub mod render;
pub mod rule;
pub mod selector;

#[cfg(test)]
mod tests;
