//! CSS rules: [`Rule`], [`RuleBuilder`], [`NestedBlock`], [`NestedBuilder`].
//!
//! A [`Rule`] bundles selectors, declarations, and optional nested
//! blocks. The [`RuleBuilder`] provides a fluent API for constructing
//! rules inline.

use std::borrow::Cow;
use std::fmt;

use crate::css::at_rules::AtRule;
use crate::css::declaration::{Declaration, DeclarationBlock};
use crate::css::properties::Value;
use crate::css::selector::Selector;

// Bring the property-setter macro into scope.
use crate::define_property;

/// A CSS rule: selectors + declarations + optional nested blocks.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Selector list.
    pub selectors: Vec<Selector>,
    /// Declarations.
    pub declarations: Vec<Declaration>,
    /// Nested rule / at-rule blocks.
    pub nested: Vec<NestedBlock>,
}

/// A nested block inside a rule (either a sub-rule or an at-rule).
#[derive(Debug, Clone)]
pub enum NestedBlock {
    /// Nested rule (implicit `&` parent reference).
    Rule(Rule),
    /// Nested at-rule.
    AtRule(AtRule),
}

/// Fluent builder for a [`Rule`].
#[derive(Debug, Clone, Default)]
pub struct RuleBuilder {
    selectors: Vec<Selector>,
    block: DeclarationBlock,
    nested: Vec<NestedBlock>,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (i, sel) in self.selectors.iter().enumerate() {
            if i > 0 { s.push_str(", "); }
            s.push_str(&sel.to_string());
        }
        s.push_str(" {");
        for d in &self.declarations {
            s.push_str(&format!("\n    {}", d));
        }
        if !self.declarations.is_empty() { s.push_str("\n  "); }
        if !self.nested.is_empty() {
            for n in &self.nested {
                match n {
                    NestedBlock::Rule(r) => {
                        s.push_str("\n  ");
                        let r_str = r.to_string();
                        for line in r_str.lines() {
                            s.push_str(&format!("  {}", line));
                            s.push('\n');
                        }
                    }
                    NestedBlock::AtRule(a) => {
                        let a_str = a.to_string();
                        for line in a_str.lines() {
                            s.push_str(&format!("  {}", line));
                            s.push('\n');
                        }
                    }
                }
            }
        }
        s.push('}');
        f.write_str(&s)
    }
}

impl RuleBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        RuleBuilder {
            selectors: Vec::new(),
            block: DeclarationBlock::new(),
            nested: Vec::new(),
        }
    }

    /// Add a selector to the selector list.
    pub fn selector(mut self, sel: Selector) -> Self {
        self.selectors.push(sel);
        self
    }

    /// Set the selector list.
    pub fn selectors(mut self, sels: Vec<Selector>) -> Self {
        self.selectors = sels;
        self
    }

    /// Add a declaration.
    pub fn decl(mut self, d: Declaration) -> Self {
        self.block = self.block.decl(d);
        self
    }

    /// Add a declaration via a `name`, `value` pair.
    pub fn property(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<Value>) -> Self {
        self.block = self.block.decl(Declaration::new(name.into(), value.into()));
        self
    }

    /// Execute a closure that returns a populated `Self` (for inline
    /// block construction).
    pub fn block(mut self, f: impl FnOnce(Self) -> Self) -> Self {
        self = f(self);
        self
    }

    /// Add a nested rule (nesting with `&`).
    pub fn nest(mut self, f: impl FnOnce(NestedBuilder) -> NestedBuilder) -> Self {
        let builder = NestedBuilder::new();
        let nested = f(builder).build();
        self.nested.push(NestedBlock::Rule(nested));
        self
    }

    /// Add a nested at-rule.
    pub fn nest_at_rule(mut self, at_rule: AtRule) -> Self {
        self.nested.push(NestedBlock::AtRule(at_rule));
        self
    }

    /// Consume the builder and produce a [`Rule`].
    pub fn build(self) -> Rule {
        Rule {
            selectors: self.selectors,
            declarations: self.block.into_declarations(),
            nested: self.nested,
        }
    }

    // ── Color & Background ───────────────────────────────────────
    define_property!(RuleBuilder, "color" => color, "Set the foreground color.");
    define_property!(RuleBuilder, "background-color" => background_color, "Set the background color.");
    define_property!(RuleBuilder, "background" => background, "Set the background shorthand.", shorthand);
    define_property!(RuleBuilder, "opacity" => opacity, "Set the opacity.");
    define_property!(RuleBuilder, "fill" => fill, "Set the fill color (SVG).");
    define_property!(RuleBuilder, "stroke" => stroke, "Set the stroke color (SVG).");
    define_property!(RuleBuilder, "caret-color" => caret_color, "Set the caret color.");
    define_property!(RuleBuilder, "text-decoration-color" => text_decoration_color, "Set the text decoration color.");
    define_property!(RuleBuilder, "column-rule-color" => column_rule_color, "Set the column rule color.");

    // ── Font ─────────────────────────────────────────────────────
    define_property!(RuleBuilder, "font-size" => font_size, "Set the font size.");
    define_property!(RuleBuilder, "font-weight" => font_weight, "Set the font weight.");
    define_property!(RuleBuilder, "font-family" => font_family, "Set the font family.");
    define_property!(RuleBuilder, "font-style" => font_style, "Set the font style.");
    define_property!(RuleBuilder, "font" => font, "Set the font shorthand.", shorthand);

    // ── Text ─────────────────────────────────────────────────────
    define_property!(RuleBuilder, "text-align" => text_align, "Set the text alignment.");
    define_property!(RuleBuilder, "text-decoration" => text_decoration, "Set the text decoration.");
    define_property!(RuleBuilder, "line-height" => line_height, "Set the line height.");
    define_property!(RuleBuilder, "letter-spacing" => letter_spacing, "Set the letter spacing.");
    define_property!(RuleBuilder, "text-transform" => text_transform, "Set the text transform.");
    define_property!(RuleBuilder, "white-space" => white_space, "Set the white-space mode.");

    // ── Box Model ────────────────────────────────────────────────
    define_property!(RuleBuilder, "margin" => margin, "Set the margin shorthand.", shorthand);
    define_property!(RuleBuilder, "margin-top" => margin_top, "Set the top margin.");
    define_property!(RuleBuilder, "margin-right" => margin_right, "Set the right margin.");
    define_property!(RuleBuilder, "margin-bottom" => margin_bottom, "Set the bottom margin.");
    define_property!(RuleBuilder, "margin-left" => margin_left, "Set the left margin.");
    define_property!(RuleBuilder, "padding" => padding, "Set the padding shorthand.", shorthand);
    define_property!(RuleBuilder, "padding-top" => padding_top, "Set the top padding.");
    define_property!(RuleBuilder, "padding-right" => padding_right, "Set the right padding.");
    define_property!(RuleBuilder, "padding-bottom" => padding_bottom, "Set the bottom padding.");
    define_property!(RuleBuilder, "padding-left" => padding_left, "Set the left padding.");
    define_property!(RuleBuilder, "width" => width, "Set the element width.");
    define_property!(RuleBuilder, "height" => height, "Set the element height.");
    define_property!(RuleBuilder, "min-width" => min_width, "Set the minimum width.");
    define_property!(RuleBuilder, "min-height" => min_height, "Set the minimum height.");
    define_property!(RuleBuilder, "max-width" => max_width, "Set the maximum width.");
    define_property!(RuleBuilder, "max-height" => max_height, "Set the maximum height.");
    define_property!(RuleBuilder, "border" => border, "Set the border shorthand.", shorthand);
    define_property!(RuleBuilder, "border-top" => border_top, "Set the top border.", shorthand);
    define_property!(RuleBuilder, "border-right" => border_right, "Set the right border.", shorthand);
    define_property!(RuleBuilder, "border-bottom" => border_bottom, "Set the bottom border.", shorthand);
    define_property!(RuleBuilder, "border-left" => border_left, "Set the left border.", shorthand);
    define_property!(RuleBuilder, "border-color" => border_color, "Set the border color.");
    define_property!(RuleBuilder, "border-width" => border_width, "Set the border width.");
    define_property!(RuleBuilder, "border-style" => border_style, "Set the border style.");
    define_property!(RuleBuilder, "border-radius" => border_radius, "Set the border radius.");
    define_property!(RuleBuilder, "border-top-left-radius" => border_top_left_radius, "Set the top-left border radius.");
    define_property!(RuleBuilder, "border-top-right-radius" => border_top_right_radius, "Set the top-right border radius.");
    define_property!(RuleBuilder, "border-bottom-left-radius" => border_bottom_left_radius, "Set the bottom-left border radius.");
    define_property!(RuleBuilder, "border-bottom-right-radius" => border_bottom_right_radius, "Set the bottom-right border radius.");
    define_property!(RuleBuilder, "box-sizing" => box_sizing, "Set the box sizing mode.");
    define_property!(RuleBuilder, "aspect-ratio" => aspect_ratio, "Set the aspect ratio.");

    // ── Flex & Grid ──────────────────────────────────────────────
    define_property!(RuleBuilder, "display" => display, "Set the display mode.");
    define_property!(RuleBuilder, "flex" => flex, "Set the flex shorthand.", shorthand);
    define_property!(RuleBuilder, "flex-direction" => flex_direction, "Set the flex direction.");
    define_property!(RuleBuilder, "flex-wrap" => flex_wrap, "Set the flex wrap mode.");
    define_property!(RuleBuilder, "flex-grow" => flex_grow, "Set the flex grow factor.");
    define_property!(RuleBuilder, "flex-shrink" => flex_shrink, "Set the flex shrink factor.");
    define_property!(RuleBuilder, "flex-basis" => flex_basis, "Set the flex basis.");
    define_property!(RuleBuilder, "justify-content" => justify_content, "Set the justify-content mode.");
    define_property!(RuleBuilder, "align-items" => align_items, "Set the align-items mode.");
    define_property!(RuleBuilder, "align-self" => align_self, "Set the align-self mode.");
    define_property!(RuleBuilder, "gap" => gap, "Set the gap size.");
    define_property!(RuleBuilder, "row-gap" => row_gap, "Set the row gap size.");
    define_property!(RuleBuilder, "column-gap" => column_gap, "Set the column gap size.");
    define_property!(RuleBuilder, "grid" => grid, "Set the grid shorthand.", shorthand);
    define_property!(RuleBuilder, "grid-template-columns" => grid_template_columns, "Set the grid template columns.");
    define_property!(RuleBuilder, "grid-template-rows" => grid_template_rows, "Set the grid template rows.");
    define_property!(RuleBuilder, "grid-column" => grid_column, "Set the grid column.");
    define_property!(RuleBuilder, "grid-row" => grid_row, "Set the grid row.");

    // ── Positioning ──────────────────────────────────────────────
    define_property!(RuleBuilder, "position" => position, "Set the position mode.");
    define_property!(RuleBuilder, "top" => top, "Set the top offset.");
    define_property!(RuleBuilder, "right" => right, "Set the right offset.");
    define_property!(RuleBuilder, "bottom" => bottom, "Set the bottom offset.");
    define_property!(RuleBuilder, "left" => left, "Set the left offset.");
    define_property!(RuleBuilder, "z-index" => z_index, "Set the z-index.");
    define_property!(RuleBuilder, "inset" => inset, "Set the inset shorthand.", shorthand);

    // ── Animation & Transition ───────────────────────────────────
    define_property!(RuleBuilder, "animation" => animation, "Set the animation shorthand.", shorthand);
    define_property!(RuleBuilder, "animation-name" => animation_name, "Set the animation name.");
    define_property!(RuleBuilder, "animation-duration" => animation_duration, "Set the animation duration.");
    define_property!(RuleBuilder, "animation-timing-function" => animation_timing_function, "Set the animation timing function.");
    define_property!(RuleBuilder, "animation-delay" => animation_delay, "Set the animation delay.");
    define_property!(RuleBuilder, "animation-iteration-count" => animation_iteration_count, "Set the animation iteration count.");
    define_property!(RuleBuilder, "animation-direction" => animation_direction, "Set the animation direction.");
    define_property!(RuleBuilder, "animation-fill-mode" => animation_fill_mode, "Set the animation fill mode.");
    define_property!(RuleBuilder, "transition" => transition, "Set the transition shorthand.", shorthand);
    define_property!(RuleBuilder, "transition-property" => transition_property, "Set the transition property.");
    define_property!(RuleBuilder, "transition-duration" => transition_duration, "Set the transition duration.");
    define_property!(RuleBuilder, "transition-timing-function" => transition_timing_function, "Set the transition timing function.");
    define_property!(RuleBuilder, "transition-delay" => transition_delay, "Set the transition delay.");
    define_property!(RuleBuilder, "transform" => transform, "Set the transform function.");
    define_property!(RuleBuilder, "transform-origin" => transform_origin, "Set the transform origin.");
    define_property!(RuleBuilder, "transform-style" => transform_style, "Set the transform style.");
    define_property!(RuleBuilder, "perspective" => perspective, "Set the perspective.");
    define_property!(RuleBuilder, "perspective-origin" => perspective_origin, "Set the perspective origin.");
    define_property!(RuleBuilder, "rotate" => rotate, "Set the rotate transform.");
    define_property!(RuleBuilder, "scale" => scale, "Set the scale transform.");
    define_property!(RuleBuilder, "translate" => translate, "Set the translate transform.");
    define_property!(RuleBuilder, "backface-visibility" => backface_visibility, "Set the backface visibility.");

    // ── Overflow & Visibility ────────────────────────────────────
    define_property!(RuleBuilder, "overflow" => overflow, "Set the overflow mode.");
    define_property!(RuleBuilder, "overflow-x" => overflow_x, "Set the horizontal overflow mode.");
    define_property!(RuleBuilder, "overflow-y" => overflow_y, "Set the vertical overflow mode.");
    define_property!(RuleBuilder, "visibility" => visibility, "Set the visibility mode.");
    define_property!(RuleBuilder, "clip" => clip, "Set the clip rectangle.");
    define_property!(RuleBuilder, "clip-path" => clip_path, "Set the clip path.");

    // ── Outline ──────────────────────────────────────────────────
    define_property!(RuleBuilder, "outline" => outline, "Set the outline shorthand.", shorthand);
    define_property!(RuleBuilder, "outline-color" => outline_color, "Set the outline color.");
    define_property!(RuleBuilder, "outline-width" => outline_width, "Set the outline width.");
    define_property!(RuleBuilder, "outline-style" => outline_style, "Set the outline style.");
    define_property!(RuleBuilder, "outline-offset" => outline_offset, "Set the outline offset.");

    // ── Table ────────────────────────────────────────────────────
    define_property!(RuleBuilder, "border-collapse" => border_collapse, "Set the border collapse mode.");
    define_property!(RuleBuilder, "border-spacing" => border_spacing, "Set the border spacing.");
    define_property!(RuleBuilder, "table-layout" => table_layout, "Set the table layout algorithm.");
    define_property!(RuleBuilder, "caption-side" => caption_side, "Set the caption side.");
    define_property!(RuleBuilder, "empty-cells" => empty_cells, "Set the empty cells mode.");

    // ── List & Columns ───────────────────────────────────────────
    define_property!(RuleBuilder, "list-style" => list_style, "Set the list style shorthand.", shorthand);
    define_property!(RuleBuilder, "list-style-type" => list_style_type, "Set the list style type.");
    define_property!(RuleBuilder, "list-style-position" => list_style_position, "Set the list style position.");
    define_property!(RuleBuilder, "columns" => columns, "Set the columns shorthand.", shorthand);
    define_property!(RuleBuilder, "column-count" => column_count, "Set the column count.");
    define_property!(RuleBuilder, "column-width" => column_width, "Set the column width.");
    define_property!(RuleBuilder, "column-rule" => column_rule, "Set the column rule.", shorthand);
    define_property!(RuleBuilder, "column-span" => column_span, "Set the column span.");

    // ── Misc ─────────────────────────────────────────────────────
    define_property!(RuleBuilder, "cursor" => cursor, "Set the cursor style.");
    define_property!(RuleBuilder, "box-shadow" => box_shadow, "Set the box shadow.");
    define_property!(RuleBuilder, "filter" => filter, "Set the filter function.");
    define_property!(RuleBuilder, "backdrop-filter" => backdrop_filter, "Set the backdrop filter function.");
    define_property!(RuleBuilder, "mask" => mask, "Set the mask shorthand.", shorthand);
    define_property!(RuleBuilder, "mask-image" => mask_image, "Set the mask image.");
    define_property!(RuleBuilder, "mask-mode" => mask_mode, "Set the mask mode.");
    define_property!(RuleBuilder, "mask-repeat" => mask_repeat, "Set the mask repeat mode.");
    define_property!(RuleBuilder, "mask-position" => mask_position, "Set the mask position.");
    define_property!(RuleBuilder, "mask-clip" => mask_clip, "Set the mask clip.");
    define_property!(RuleBuilder, "mask-origin" => mask_origin, "Set the mask origin.");
    define_property!(RuleBuilder, "mask-size" => mask_size, "Set the mask size.");
    define_property!(RuleBuilder, "mask-composite" => mask_composite, "Set the mask composite mode.");
    define_property!(RuleBuilder, "content" => content, "Set the content property.");
    define_property!(RuleBuilder, "pointer-events" => pointer_events, "Set pointer-events mode.");
    define_property!(RuleBuilder, "user-select" => user_select, "Set user-select mode.");
    define_property!(RuleBuilder, "appearance" => appearance, "Set the appearance mode.");
}

/// Builder for a nested rule that automatically prepends the
/// parent-relative selector.
#[derive(Debug, Clone, Default)]
pub struct NestedBuilder {
    inner: RuleBuilder,
}

impl NestedBuilder {
    /// Create an empty nested builder.
    pub fn new() -> Self {
        NestedBuilder {
            inner: RuleBuilder::new(),
        }
    }

    /// Add a selector (the `&` is prepended at render time).
    pub fn selector(self, sel: Selector) -> Self {
        NestedBuilder {
            inner: self.inner.selector(sel),
        }
    }

    /// Add a declaration.
    pub fn decl(self, d: Declaration) -> Self {
        NestedBuilder {
            inner: self.inner.decl(d),
        }
    }

    /// Add a declaration via `name`, `value`.
    pub fn property(self, name: impl Into<Cow<'static, str>>, value: impl Into<Value>) -> Self {
        NestedBuilder {
            inner: self.inner.property(name, value),
        }
    }

    /// Execute a closure that returns a populated builder.
    pub fn block(self, f: impl FnOnce(Self) -> Self) -> Self {
        f(self)
    }

    /// Deep-nest another rule.
    pub fn nest(self, f: impl FnOnce(NestedBuilder) -> NestedBuilder) -> Self {
        NestedBuilder {
            inner: self.inner.nest(f),
        }
    }

    /// Add a nested at-rule.
    pub fn nest_at_rule(self, at_rule: AtRule) -> Self {
        NestedBuilder {
            inner: self.inner.nest_at_rule(at_rule),
        }
    }

    /// Build the nested rule.
    pub fn build(self) -> Rule {
        self.inner.build()
    }

    // ── Property setters (delegate to inner RuleBuilder) ─────────
    define_property!(NestedBuilder, "color" => color, "Set the foreground color.");
    define_property!(NestedBuilder, "background-color" => background_color, "Set the background color.");
    define_property!(NestedBuilder, "background" => background, "Set the background shorthand.", shorthand);
    define_property!(NestedBuilder, "opacity" => opacity, "Set the opacity.");
    define_property!(NestedBuilder, "fill" => fill, "Set the fill color (SVG).");
    define_property!(NestedBuilder, "stroke" => stroke, "Set the stroke color (SVG).");
    define_property!(NestedBuilder, "caret-color" => caret_color, "Set the caret color.");
    define_property!(NestedBuilder, "text-decoration-color" => text_decoration_color, "Set the text decoration color.");
    define_property!(NestedBuilder, "column-rule-color" => column_rule_color, "Set the column rule color.");
    define_property!(NestedBuilder, "font-size" => font_size, "Set the font size.");
    define_property!(NestedBuilder, "font-weight" => font_weight, "Set the font weight.");
    define_property!(NestedBuilder, "font-family" => font_family, "Set the font family.");
    define_property!(NestedBuilder, "font-style" => font_style, "Set the font style.");
    define_property!(NestedBuilder, "text-align" => text_align, "Set the text alignment.");
    define_property!(NestedBuilder, "text-decoration" => text_decoration, "Set the text decoration.");
    define_property!(NestedBuilder, "line-height" => line_height, "Set the line height.");
    define_property!(NestedBuilder, "letter-spacing" => letter_spacing, "Set the letter spacing.");
    define_property!(NestedBuilder, "text-transform" => text_transform, "Set the text transform.");
    define_property!(NestedBuilder, "white-space" => white_space, "Set the white-space mode.");
    define_property!(NestedBuilder, "margin" => margin, "Set the margin shorthand.", shorthand);
    define_property!(NestedBuilder, "margin-top" => margin_top, "Set the top margin.");
    define_property!(NestedBuilder, "margin-right" => margin_right, "Set the right margin.");
    define_property!(NestedBuilder, "margin-bottom" => margin_bottom, "Set the bottom margin.");
    define_property!(NestedBuilder, "margin-left" => margin_left, "Set the left margin.");
    define_property!(NestedBuilder, "padding" => padding, "Set the padding shorthand.", shorthand);
    define_property!(NestedBuilder, "padding-top" => padding_top, "Set the top padding.");
    define_property!(NestedBuilder, "padding-right" => padding_right, "Set the right padding.");
    define_property!(NestedBuilder, "padding-bottom" => padding_bottom, "Set the bottom padding.");
    define_property!(NestedBuilder, "padding-left" => padding_left, "Set the left padding.");
    define_property!(NestedBuilder, "width" => width, "Set the element width.");
    define_property!(NestedBuilder, "height" => height, "Set the element height.");
    define_property!(NestedBuilder, "min-width" => min_width, "Set the minimum width.");
    define_property!(NestedBuilder, "min-height" => min_height, "Set the minimum height.");
    define_property!(NestedBuilder, "max-width" => max_width, "Set the maximum width.");
    define_property!(NestedBuilder, "max-height" => max_height, "Set the maximum height.");
    define_property!(NestedBuilder, "border" => border, "Set the border shorthand.", shorthand);
    define_property!(NestedBuilder, "border-top" => border_top, "Set the top border.", shorthand);
    define_property!(NestedBuilder, "border-right" => border_right, "Set the right border.", shorthand);
    define_property!(NestedBuilder, "border-bottom" => border_bottom, "Set the bottom border.", shorthand);
    define_property!(NestedBuilder, "border-left" => border_left, "Set the left border.", shorthand);
    define_property!(NestedBuilder, "border-color" => border_color, "Set the border color.");
    define_property!(NestedBuilder, "border-width" => border_width, "Set the border width.");
    define_property!(NestedBuilder, "border-style" => border_style, "Set the border style.");
    define_property!(NestedBuilder, "border-radius" => border_radius, "Set the border radius.");
    define_property!(NestedBuilder, "border-top-left-radius" => border_top_left_radius, "Set the top-left border radius.");
    define_property!(NestedBuilder, "border-top-right-radius" => border_top_right_radius, "Set the top-right border radius.");
    define_property!(NestedBuilder, "border-bottom-left-radius" => border_bottom_left_radius, "Set the bottom-left border radius.");
    define_property!(NestedBuilder, "border-bottom-right-radius" => border_bottom_right_radius, "Set the bottom-right border radius.");
    define_property!(NestedBuilder, "box-sizing" => box_sizing, "Set the box sizing mode.");
    define_property!(NestedBuilder, "aspect-ratio" => aspect_ratio, "Set the aspect ratio.");
    define_property!(NestedBuilder, "display" => display, "Set the display mode.");
    define_property!(NestedBuilder, "flex" => flex, "Set the flex shorthand.", shorthand);
    define_property!(NestedBuilder, "flex-direction" => flex_direction, "Set the flex direction.");
    define_property!(NestedBuilder, "flex-wrap" => flex_wrap, "Set the flex wrap mode.");
    define_property!(NestedBuilder, "flex-grow" => flex_grow, "Set the flex grow factor.");
    define_property!(NestedBuilder, "flex-shrink" => flex_shrink, "Set the flex shrink factor.");
    define_property!(NestedBuilder, "flex-basis" => flex_basis, "Set the flex basis.");
    define_property!(NestedBuilder, "justify-content" => justify_content, "Set justify-content.");
    define_property!(NestedBuilder, "align-items" => align_items, "Set align-items.");
    define_property!(NestedBuilder, "align-self" => align_self, "Set align-self.");
    define_property!(NestedBuilder, "gap" => gap, "Set the gap size.");
    define_property!(NestedBuilder, "row-gap" => row_gap, "Set the row gap.");
    define_property!(NestedBuilder, "column-gap" => column_gap, "Set the column gap.");
    define_property!(NestedBuilder, "grid" => grid, "Set the grid shorthand.", shorthand);
    define_property!(NestedBuilder, "grid-template-columns" => grid_template_columns, "Set grid template columns.");
    define_property!(NestedBuilder, "grid-template-rows" => grid_template_rows, "Set grid template rows.");
    define_property!(NestedBuilder, "position" => position, "Set the position mode.");
    define_property!(NestedBuilder, "top" => top, "Set the top offset.");
    define_property!(NestedBuilder, "right" => right, "Set the right offset.");
    define_property!(NestedBuilder, "bottom" => bottom, "Set the bottom offset.");
    define_property!(NestedBuilder, "left" => left, "Set the left offset.");
    define_property!(NestedBuilder, "z-index" => z_index, "Set the z-index.");
    define_property!(NestedBuilder, "animation" => animation, "Set the animation shorthand.", shorthand);
    define_property!(NestedBuilder, "animation-name" => animation_name, "Set the animation name.");
    define_property!(NestedBuilder, "animation-duration" => animation_duration, "Set the animation duration.");
    define_property!(NestedBuilder, "animation-timing-function" => animation_timing_function, "Set the animation timing function.");
    define_property!(NestedBuilder, "animation-delay" => animation_delay, "Set the animation delay.");
    define_property!(NestedBuilder, "transition" => transition, "Set the transition shorthand.", shorthand);
    define_property!(NestedBuilder, "transition-duration" => transition_duration, "Set the transition duration.");
    define_property!(NestedBuilder, "transition-timing-function" => transition_timing_function, "Set the transition timing function.");
    define_property!(NestedBuilder, "transform" => transform, "Set the transform function.");
    define_property!(NestedBuilder, "transform-origin" => transform_origin, "Set the transform origin.");
    define_property!(NestedBuilder, "transform-style" => transform_style, "Set the transform style.");
    define_property!(NestedBuilder, "perspective" => perspective, "Set the perspective.");
    define_property!(NestedBuilder, "perspective-origin" => perspective_origin, "Set the perspective origin.");
    define_property!(NestedBuilder, "rotate" => rotate, "Set the rotate transform.");
    define_property!(NestedBuilder, "scale" => scale, "Set the scale transform.");
    define_property!(NestedBuilder, "translate" => translate, "Set the translate transform.");
    define_property!(NestedBuilder, "backface-visibility" => backface_visibility, "Set the backface visibility.");
    define_property!(NestedBuilder, "overflow" => overflow, "Set the overflow mode.");
    define_property!(NestedBuilder, "visibility" => visibility, "Set the visibility mode.");
    define_property!(NestedBuilder, "cursor" => cursor, "Set the cursor style.");
    define_property!(NestedBuilder, "box-shadow" => box_shadow, "Set the box shadow.");
    define_property!(NestedBuilder, "filter" => filter, "Set the filter function.");
    define_property!(NestedBuilder, "backdrop-filter" => backdrop_filter, "Set the backdrop filter function.");
    define_property!(NestedBuilder, "mask" => mask, "Set the mask shorthand.", shorthand);
    define_property!(NestedBuilder, "mask-image" => mask_image, "Set the mask image.");
    define_property!(NestedBuilder, "mask-mode" => mask_mode, "Set the mask mode.");
    define_property!(NestedBuilder, "mask-repeat" => mask_repeat, "Set the mask repeat mode.");
    define_property!(NestedBuilder, "mask-position" => mask_position, "Set the mask position.");
    define_property!(NestedBuilder, "mask-clip" => mask_clip, "Set the mask clip.");
    define_property!(NestedBuilder, "mask-origin" => mask_origin, "Set the mask origin.");
    define_property!(NestedBuilder, "mask-size" => mask_size, "Set the mask size.");
    define_property!(NestedBuilder, "mask-composite" => mask_composite, "Set the mask composite mode.");
    define_property!(NestedBuilder, "content" => content, "Set the content property.");
    define_property!(NestedBuilder, "pointer-events" => pointer_events, "Set pointer-events.");
    define_property!(NestedBuilder, "user-select" => user_select, "Set user-select.");
    define_property!(NestedBuilder, "appearance" => appearance, "Set the appearance mode.");
}

impl fmt::Display for NestedBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NestedBlock::Rule(rule) => fmt::Display::fmt(rule, f),
            NestedBlock::AtRule(at_rule) => fmt::Display::fmt(at_rule, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::at_rules::RuleOrAtRule;
    use crate::css::properties::Value;
    use crate::css::selector::Selector;
    use crate::css::values::Color;

    #[test]
    fn rule_builder_selector() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .decl(Declaration::new("color", Value::Color(Color::named("red"))))
            .build();
        assert_eq!(r.selectors.len(), 1);
        assert_eq!(r.declarations.len(), 1);
        assert!(r.nested.is_empty());
    }

    #[test]
    fn rule_builder_property() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .property("color", Color::named("red"))
            .build();
        assert_eq!(r.declarations[0].name, "color");
    }

    #[test]
    fn rule_builder_block() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .block(|b| {
                b.property("color", Color::named("red"))
                    .property("padding", crate::css::values::Length::px(8.0))
            })
            .build();
        assert_eq!(r.declarations.len(), 2);
    }

    #[test]
    fn rule_builder_nest() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .property("color", Color::named("red"))
            .nest(|n| {
                n.selector(Selector::pseudo_class("hover"))
                    .property("color", Color::named("blue"))
            })
            .build();
        assert_eq!(r.nested.len(), 1);
    }

    #[test]
    fn rule_builder_nest_at_rule() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .nest_at_rule(AtRule::Media {
                query: Cow::Borrowed("(min-width: 800px)"),
                rules: vec![],
            })
            .build();
        assert_eq!(r.nested.len(), 1);
    }

    #[test]
    fn rule_builder_multiple_selectors() {
        let r = RuleBuilder::new()
            .selectors(vec![
                Selector::class("btn"),
                Selector::class("button"),
            ])
            .property("color", Color::named("red"))
            .build();
        assert_eq!(r.selectors.len(), 2);
    }

    #[test]
    fn rule_builder_default() {
        let r = RuleBuilder::new().build();
        assert!(r.selectors.is_empty());
        assert!(r.declarations.is_empty());
        assert!(r.nested.is_empty());
    }

    #[test]
    fn nested_builder_preserves_decls() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .decl(Declaration::new("color", Value::Color(Color::named("blue"))))
            .build();
        assert_eq!(n.declarations.len(), 1);
        assert_eq!(n.selectors.len(), 1);
    }

    #[test]
    fn nested_builder_block() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("focus"))
            .block(|b| b.property("outline", "none"))
            .build();
        assert_eq!(n.declarations.len(), 1);
    }

    #[test]
    fn rule_via_rule_or_at_rule() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .property("color", Color::named("red"))
            .build();
        let item = RuleOrAtRule::Rule(r);
        let s = item.to_string();
        assert!(s.contains(".btn"));
    }

    #[test]
    fn nested_block_rule_variant() {
        let r = RuleBuilder::new()
            .selector(Selector::class("btn"))
            .property("color", Color::named("red"))
            .build();
        let nb = NestedBlock::Rule(r);
        let s = nb.to_string();
        assert!(s.contains(".btn"));
    }

    #[test]
    fn nested_block_at_rule_variant() {
        let nb = NestedBlock::AtRule(AtRule::Media {
            query: Cow::Borrowed("screen"),
            rules: vec![],
        });
        let s = nb.to_string();
        assert!(s.contains("@media"));
    }

    // ── New properties (Phase 4 completion) ──────────────────────

    #[test]
    fn rule_builder_color_extensions() {
        let r = RuleBuilder::new()
            .selector(Selector::class("svg"))
            .fill(Color::named("red"))
            .stroke(Color::named("blue"))
            .caret_color(Color::named("green"))
            .text_decoration_color(Color::named("yellow"))
            .column_rule_color(Color::named("orange"))
            .build();
        assert_eq!(r.declarations.len(), 5);
        assert_eq!(r.declarations[0].name, "fill");
        assert_eq!(r.declarations[1].name, "stroke");
        assert_eq!(r.declarations[2].name, "caret-color");
        assert_eq!(r.declarations[3].name, "text-decoration-color");
        assert_eq!(r.declarations[4].name, "column-rule-color");
    }

    #[test]
    fn rule_builder_border_extensions() {
        let r = RuleBuilder::new()
            .selector(Selector::class("box"))
            .border_top("1px solid red")
            .border_right("1px solid blue")
            .border_bottom("1px solid green")
            .border_left("1px solid yellow")
            .border_top_left_radius("4px")
            .border_top_right_radius("4px")
            .border_bottom_left_radius("4px")
            .border_bottom_right_radius("4px")
            .aspect_ratio("16 / 9")
            .build();
        assert_eq!(r.declarations.len(), 9);
        assert_eq!(r.declarations[0].name, "border-top");
        assert_eq!(r.declarations[4].name, "border-top-left-radius");
        assert_eq!(r.declarations[8].name, "aspect-ratio");
    }

    #[test]
    fn rule_builder_transform_extensions() {
        let r = RuleBuilder::new()
            .selector(Selector::class("t"))
            .transform_origin("50% 50%")
            .transform_style("preserve-3d")
            .perspective("800px")
            .perspective_origin("50% 50%")
            .rotate("45deg")
            .scale("1.5")
            .translate("10px")
            .backface_visibility("hidden")
            .build();
        assert_eq!(r.declarations.len(), 8);
        assert_eq!(r.declarations[0].name, "transform-origin");
        assert_eq!(r.declarations[7].name, "backface-visibility");
    }

    #[test]
    fn rule_builder_filter_and_mask() {
        let r = RuleBuilder::new()
            .selector(Selector::class("fx"))
            .backdrop_filter("blur(8px)")
            .mask("url(#m)")
            .mask_image("url(#m)")
            .mask_mode("alpha")
            .mask_repeat("no-repeat")
            .mask_position("center")
            .mask_clip("border-box")
            .mask_origin("border-box")
            .mask_size("cover")
            .mask_composite("add")
            .build();
        assert_eq!(r.declarations.len(), 10);
        assert_eq!(r.declarations[0].name, "backdrop-filter");
        assert_eq!(r.declarations[1].name, "mask");
        assert_eq!(r.declarations[9].name, "mask-composite");
    }

    #[test]
    fn nested_builder_color_extensions() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .fill(Color::named("red"))
            .stroke(Color::named("blue"))
            .caret_color(Color::named("green"))
            .text_decoration_color(Color::named("yellow"))
            .column_rule_color(Color::named("orange"))
            .build();
        assert_eq!(n.declarations.len(), 5);
    }

    #[test]
    fn nested_builder_border_extensions() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .border_top("1px solid red")
            .border_top_left_radius("4px")
            .aspect_ratio("1 / 1")
            .build();
        assert_eq!(n.declarations.len(), 3);
    }

    #[test]
    fn nested_builder_transform_extensions() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .transform_origin("50% 50%")
            .rotate("45deg")
            .scale("1.5")
            .build();
        assert_eq!(n.declarations.len(), 3);
    }

    #[test]
    fn nested_builder_filter_and_mask() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .backdrop_filter("blur(8px)")
            .mask("url(#m)")
            .build();
        assert_eq!(n.declarations.len(), 2);
    }

    // ── Coverage: Display for NestedBlock and nest method ──────

    #[test]
    fn nested_block_display_rule() {
        let r = RuleBuilder::new()
            .selector(Selector::class("x"))
            .property("color", Color::named("red"))
            .build();
        let nb = NestedBlock::Rule(r);
        let s = format!("{}", nb);
        assert!(s.contains(".x"));
        assert!(s.contains("color: red"));
    }

    #[test]
    fn nested_block_display_at_rule() {
        let nb = NestedBlock::AtRule(AtRule::import("foo.css"));
        let s = format!("{}", nb);
        assert!(s.contains("@import"));
    }

    #[test]
    fn nested_builder_nest_method() {
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .nest(|n| n.property("color", Color::named("blue")))
            .build();
        assert_eq!(n.declarations.len(), 0);
        assert_eq!(n.nested.len(), 1);
    }

    #[test]
    fn nested_builder_nest_at_rule() {
        // Iterate over both nested blocks (an AtRule and a Rule) so the
        // matches! assertion at L793 is exercised with both true and false.
        let n = NestedBuilder::new()
            .selector(Selector::pseudo_class("hover"))
            .nest_at_rule(AtRule::Media {
                query: Cow::Borrowed("screen"),
                rules: vec![],
            })
            .nest(|b| b.selector(Selector::class("inner")).property("color", Color::named("red")))
            .build();
        assert_eq!(n.nested.len(), 2);
        let mut expected_is_at_rule = [true, false];
        for nested in &n.nested {
            let is_at_rule = matches!(nested, NestedBlock::AtRule(_));
            assert_eq!(is_at_rule, expected_is_at_rule[0]);
            expected_is_at_rule = [expected_is_at_rule[1], expected_is_at_rule[0]];
        }
    }

    #[test]
    fn rule_via_rule_or_at_rule_debug() {
        let r = RuleBuilder::new()
            .selector(Selector::class("x"))
            .property("color", Color::named("red"))
            .build();
        let item = RuleOrAtRule::Rule(r);
        let _ = format!("{:?}", item);
    }

    #[test]
    fn nested_block_debug() {
        let r = RuleBuilder::new()
            .selector(Selector::class("x"))
            .property("color", Color::named("red"))
            .build();
        let nb = NestedBlock::Rule(r);
        let _ = format!("{:?}", nb);
    }
}
