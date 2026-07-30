use crate::css::properties::Value;
use crate::css::selector::{Selector, PseudoSelector};
use crate::css::values::{Color, CssString, Length};
use crate::css::{AtRule, Declaration, StyleSheet};
use mrk::Renderable;

#[test]
fn empty_stylesheet_renders_to_empty_string() {
    let sheet = StyleSheet::new().build();
    assert_eq!(sheet.render(), "");
}

#[test]
fn empty_stylesheet_converts_to_node_raw() {
    let sheet = StyleSheet::new().build();
    let node: mrk::Node = sheet.into();
    let debug = format!("{:?}", node);
    assert!(debug.contains("Raw"));
}

#[test]
fn single_rule_with_one_declaration() {
    let sheet = StyleSheet::new()
        .rule(|r| r.selector(Selector::class("btn")).color(Color::named("red")))
        .build();
    let css = sheet.render();
    assert_eq!(css, ".btn {\n    color: red;\n  }");
}

#[test]
fn single_rule_with_multiple_declarations() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("card"))
                .background_color(Color::named("white"))
                .padding(Length::px(16.0))
                .border_radius(Length::px(8.0))
        })
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        ".card {\n    background-color: white;\n    padding: 16px;\n    border-radius: 8px;\n  }"
    );
}

#[test]
fn rule_with_multiple_selectors() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("btn"))
                .selector(Selector::class("btn-primary"))
                .color(Color::named("blue"))
        })
        .build();
    let css = sheet.render();
    assert_eq!(css, ".btn, .btn-primary {\n    color: blue;\n  }");
}

#[test]
fn multiple_rules() {
    let sheet = StyleSheet::new()
        .rule(|r| r.selector(Selector::type_("h1")).font_size(Length::px(32.0)))
        .rule(|r| r.selector(Selector::type_("h2")).font_size(Length::px(24.0)))
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "h1 {\n    font-size: 32px;\n  }\nh2 {\n    font-size: 24px;\n  }"
    );
}

#[test]
fn rule_with_nesting() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("card"))
                .color(Color::named("black"))
                .nest(|n| {
                    n.selector(Selector::Compound(vec![
                        Selector::NestingRef,
                        Selector::Pseudo(PseudoSelector::Class("hover".into())),
                    ]))
                    .color(Color::named("gray"))
                })
        })
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        ".card {\n    color: black;\n  \n    &:hover {\n      color: gray;\n    }\n}"
    );
}

#[test]
fn rule_with_nest_at_rule() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("responsive"))
                .nest_at_rule(AtRule::media("(max-width: 600px)").rule(|r| {
                    r.selector(Selector::class("&")).width(Length::pct(100.0))
                }).build())
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("@media (max-width: 600px)"));
    assert!(css.contains("width: 100%"));
}

#[test]
fn media_at_rule_with_rules() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::media("(min-width: 800px)")
                .rule(|r| r.selector(Selector::class("container")).width(Length::px(750.0)))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "@media (min-width: 800px) {\n  .container {\n    width: 750px;\n  }\n}"
    );
}

#[test]
fn font_face_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::font_face()
                .property("font-family", CssString::from("Open Sans"))
                .property("font-weight", Value::Integer(400.into()))
                .property("src", Value::Raw("url('open-sans.woff2')".into()))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert!(css.starts_with("@font-face {"));
    assert!(css.contains("font-family: \"Open Sans\""));
    assert!(css.contains("font-weight: 400"));
    assert!(css.contains("src: url('open-sans.woff2')"));
}

#[test]
fn keyframes_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::keyframes("fade-in")
                .rule(|r| r.selector(Selector::class("from")).opacity(Value::Number(0.0.into())))
                .rule(|r| r.selector(Selector::class("to")).opacity(Value::Number(1.0.into())))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "@keyframes fade-in {\n  .from {\n    opacity: 0;\n  }\n  .to {\n    opacity: 1;\n  }\n}"
    );
}

#[test]
fn supports_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::supports("(display: grid)")
                .rule(|r| r.selector(Selector::class("grid")).display("grid"))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "@supports (display: grid) {\n  .grid {\n    display: grid;\n  }\n}"
    );
}

#[test]
fn charset_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(AtRule::charset("UTF-8"))
        .build();
    assert_eq!(sheet.render(), "@charset \"UTF-8\";");
}

#[test]
fn namespace_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(AtRule::namespace("http://www.w3.org/2000/svg"))
        .build();
    assert_eq!(sheet.render(), "@namespace \"http://www.w3.org/2000/svg\";");
}

#[test]
fn import_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(AtRule::import("reset.css"))
        .build();
    assert_eq!(sheet.render(), "@import \"reset.css\";");
}

#[test]
fn mixed_rules_and_at_rules() {
    let sheet = StyleSheet::new()
        .rule(|r| r.selector(Selector::type_("body")).margin(Length::px(0.0)))
        .at_rule(
            AtRule::media("print")
                .rule(|r| r.selector(Selector::type_("body")).font_size(Length::pt(12.0)))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert!(css.contains("body {\n    margin: 0px;"));
    assert!(css.contains("@media print"));
}

#[test]
fn chain_rules_with_all_property_types() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("demo"))
                .color(Color::named("red"))
                .background_color(Color::named("blue"))
                .font_size(Length::px(16.0))
                .margin(Length::px(10.0))
                .padding(Length::px(20.0))
                .display("flex")
                .position("relative")
                .opacity(0.5f32)
                .z_index(Value::Integer(100.into()))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("color: red"));
    assert!(css.contains("font-size: 16px"));
    assert!(css.contains("opacity: 0.5"));
    assert!(css.contains("z-index: 100"));
}

#[test]
fn rule_with_shorthand_and_longhand() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("box"))
                .margin(Length::px(10.0))
                .margin_top(Length::px(0.0))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("margin: 10px;"));
    assert!(css.contains("margin-top: 0px;"));
}

#[test]
fn using_decl_directly() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("custom"))
                .decl(Declaration::new("--my-var", Value::Number(42.0.into())))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("--my-var: 42"));
}

#[test]
fn nested_blocks_preserve_declarations() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("parent"))
                .color(Color::named("black"))
                .nest(|n| n.selector(Selector::class("& .child")).color(Color::named("gray")))
                .font_size(Length::px(14.0))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("color: black;"));
    assert!(css.contains("font-size: 14px;"));
    assert!(css.contains("& .child"));
    assert!(css.contains("color: gray;"));
}

// ── `css!` macro integration tests (v2) ────────────────────────────────

#[test]
fn css_macro_basic_rule() {
    let sheet = crate::css! {
        .btn { color: red; }
    };
    let css = sheet.render();
    assert!(css.contains(".btn"));
    // Named colors are typed and render canonically.
    assert!(css.contains("color: rgb(255, 0, 0)"));
}

#[test]
fn css_macro_typed_value_length() {
    let sheet = crate::css! {
        .box { width: 8px; }
    };
    let css = sheet.render();
    assert!(css.contains("width: 8px"));
}

#[test]
fn css_macro_typed_value_hex_color() {
    let sheet = crate::css! {
        .hi { color: #fff; }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 255, 255)"));
}

#[test]
fn css_macro_typed_value_rgb() {
    let sheet = crate::css! {
        .hi { color: rgb(255, 0, 0); }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 0, 0)"));
}

#[test]
fn css_macro_typed_value_named_color() {
    let sheet = crate::css! {
        .hi { color: rebeccapurple; }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(102, 51, 153)"));
}

#[test]
fn css_macro_value_list() {
    let sheet = crate::css! {
        .m { margin: 8px 16px; }
    };
    let css = sheet.render();
    assert!(css.contains("margin: 8px 16px"));
}

#[test]
fn css_macro_important() {
    let sheet = crate::css! {
        .btn { color: red !important; }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 0, 0) !important;"));
}

#[test]
fn css_macro_url() {
    // `url(...)` is typed via the macro for general declarations.
    let sheet = crate::css! {
        .bg { background: url("img.png"); }
    };
    let css = sheet.render();
    assert!(css.contains("url("));
}

#[test]
fn css_macro_var_reference() {
    let sheet = crate::css! {
        .a { color: var(--brand); }
    };
    let css = sheet.render();
    assert!(css.contains("var(--brand)"));
}

#[test]
fn css_macro_nested_with_typed_value() {
    let sheet = crate::css! {
        .card {
            padding: 16px;
            & .text { font-weight: bold; }
        }
    };
    let css = sheet.render();
    assert!(css.contains("padding: 16px"));
    assert!(css.contains("& .text"));
    assert!(css.contains("font-weight: bold"));
}

#[test]
fn css_macro_at_media_with_typed_value() {
    let sheet = crate::css! {
        @media (max-width: 600px) {
            .btn { padding: 8px; }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@media"));
    assert!(css.contains("padding: 8px"));
}

#[test]
fn css_macro_quoted_value_passthrough() {
    let sheet = crate::css! {
        .m { background: "linear-gradient(red, blue)"; }
    };
    let css = sheet.render();
    assert!(css.contains("linear-gradient(red, blue)"));
}

#[test]
fn css_macro_glued_units() {
    // Glued number+unit tokens reach the macro intact (no lexer
    // error) and are typed via the runtime value parser.
    let sheet = crate::css! {
        .a { margin: 1.5rem; height: 100%; rotate: 45deg; }
    };
    let css = sheet.render();
    assert!(css.contains("margin: 1.5rem"));
    assert!(css.contains("height: 100%"));
    assert!(css.contains("rotate: 45deg"));
}

#[test]
fn css_macro_split_unit_single() {
    // Units starting with `e` after a decimal (`1.5em`) fail at lex
    // time (parsed as an exponent); the split form works.
    let sheet = crate::css! {
        .a { margin: 1.5 em; }
    };
    let css = sheet.render();
    assert!(css.contains("margin: 1.5em"));
}

#[test]
fn css_macro_mixed_list() {
    let sheet = crate::css! {
        .b { border: 1px solid red; }
    };
    let css = sheet.render();
    assert!(css.contains("border: 1px solid rgb(255, 0, 0)"));
}

// ── `css!` macro integration tests: combined v2 features ────────────

#[test]
fn css_macro_keyframes_with_important() {
    let sheet = crate::css! {
        @keyframes fade {
            from { opacity: 0; }
            to { opacity: 1 !important; }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@keyframes fade"));
    assert!(css.contains("opacity: 0;"));
    assert!(css.contains("opacity: 1 !important;"));
}

#[test]
fn css_macro_layer_with_nested_rules() {
    let sheet = crate::css! {
        @layer base {
            .btn {
                color: blue;
                &:hover { color: red; }
            }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@layer base {"));
    assert!(css.contains("color: rgb(0, 0, 255);"));
    assert!(css.contains("&:hover"));
}

#[test]
fn css_macro_custom_property_chain() {
    let sheet = crate::css! {
        :root { --brand: rebeccapurple; }
        .btn {
            color: var(--brand);
            background: var(--bg, blue);
        }
    };
    let css = sheet.render();
    assert!(css.contains("--brand: rgb(102, 51, 153);"));
    assert!(css.contains("color: var(--brand);"));
    assert!(css.contains("background: var(--bg, rgb(0, 0, 255));"));
}

#[test]
fn css_macro_media_with_custom_properties() {
    let sheet = crate::css! {
        @media (prefers-color-scheme: dark) {
            :root { --brand: white; }
            .btn { color: var(--brand); }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@media (prefers-color-scheme: dark)"));
    assert!(css.contains("--brand: rgb(255, 255, 255);"));
    assert!(css.contains("color: var(--brand);"));
}

#[test]
fn css_macro_container_scope_and_font_face() {
    let sheet = crate::css! {
        @font-face { font-family: "My Font"; src: url("font.woff2"); }
        @container sidebar (inline-size > 30ch) {
            .card { padding: 16px; }
        }
        @scope (.card) to (.content) {
            h1 { font-size: 1.5 rem; }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@font-face"));
    assert!(css.contains("src: url(\"font.woff2\");"));
    assert!(css.contains("@container sidebar (inline-size > 30ch)"));
    assert!(css.contains("@scope (.card) to (.content)"));
    assert!(css.contains("font-size: 1.5rem;"));
}

#[test]
fn css_macro_attribute_selectors() {
    let sheet = crate::css! {
        [disabled] { opacity: 0.5; }
        input[required] { border-color: red; }
        a[href^="https"] { color: green; }
        [data-x="Foo" i] { color: blue; }
    };
    let css = sheet.render();
    assert!(css.contains("[disabled]"));
    assert!(css.contains("input[required]"));
    assert!(css.contains("a[href^=\"https\"]"));
    assert!(css.contains("[data-x=\"Foo\" i]"));
}

#[test]
fn css_macro_calc_binary_operators() {
    let sheet = crate::css! {
        .a {
            width: calc(100% - 8px);
            height: calc(100% + 8px);
            margin-left: calc(1px - -2px);
        }
    };
    let css = sheet.render();
    assert!(css.contains("width: calc(100% - 8px);"));
    assert!(css.contains("height: calc(100% + 8px);"));
    assert!(css.contains("margin-left: calc(1px - -2px);"));
}

#[test]
fn css_macro_modern_color_and_transform() {
    let sheet = crate::css! {
        .a {
            color: hsl(120 50% 50%);
            transform: translate(-50%, -8px);
        }
    };
    let css = sheet.render();
    assert!(css.contains("color: hsl(120 50% 50%);"));
    assert!(css.contains("transform: translate(-50%, -8px);"));
}

#[test]
fn css_macro_transition_leading_dot() {
    let sheet = crate::css! {
        .a { transition: all .3s ease-in-out; }
    };
    let css = sheet.render();
    assert!(css.contains("transition: all 0.3s ease-in-out;"));
}

#[test]
fn css_macro_media_keyword_spacing() {
    let sheet = crate::css! {
        @media screen and (max-width: 600px) {
            .a { color: red; }
        }
        @supports not (display: grid) {
            .a { display: block; }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@media screen and (max-width: 600px)"));
    assert!(css.contains("@supports not (display: grid)"));
}

#[test]
fn css_macro_out_of_range_percentage() {
    let sheet = crate::css! {
        .a { width: 150%; top: -50%; }
    };
    let css = sheet.render();
    assert!(css.contains("width: 150%;"));
    assert!(css.contains("top: -50%;"));
}

// ── `css!` structural macro: compile-time structure (§18) ───────────

#[test]
fn css_macro_empty_stylesheet() {
    let sheet = crate::css! {};
    assert_eq!(sheet.render(), "");
}

#[test]
fn css_macro_empty_rule_body() {
    let sheet = crate::css! { .a { } };
    assert_eq!(sheet.render(), ".a {}");
}

#[test]
fn css_macro_stray_semicolons_are_skipped() {
    // Empty statements (`;`, `;;`) are dropped, like the runtime
    // chunker does.
    let sheet = crate::css! {
        ;
        .a { color: red;; }
        ;
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 0, 0);"));
}

#[test]
fn css_macro_statement_at_rules_top_level() {
    let sheet = crate::css! {
        @charset "utf-8";
        @import "reset.css";
        @namespace "http://www.w3.org/2000/svg";
        @layer base, utilities;
        @layer;
    };
    let css = sheet.render();
    assert!(css.contains("@charset \"utf-8\";"));
    assert!(css.contains("@import \"reset.css\";"));
    assert!(css.contains("@namespace \"http://www.w3.org/2000/svg\";"));
    assert!(css.contains("@layer base, utilities;"));
    assert!(css.contains("@layer;"));
}

#[test]
fn css_macro_nested_at_rule_in_rule() {
    let sheet = crate::css! {
        .a {
            color: red;
            @media (max-width: 600px) {
                & { color: blue; }
            }
        }
    };
    let css = sheet.render();
    assert!(css.contains("@media (max-width: 600px)"));
    assert!(css.contains("color: rgb(0, 0, 255);"));
}

#[test]
fn css_macro_important_split_form() {
    // `! important` written split lexes as two tokens; the runtime
    // grammar re-joins and recognizes it.
    let sheet = crate::css! {
        .a { color: red ! important; }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 0, 0) !important;"));
}

#[test]
#[should_panic(expected = "only allowed at the top level")]
fn css_macro_nested_statement_at_rule_panics() {
    let _ = crate::css! { .a { @import "x.css"; } };
}

// ── `css!` interpolation: `{ expr }` declaration values ─────────────

#[test]
fn css_macro_interpolate_str_value() {
    let color: &'static str = "red";
    let sheet = crate::css! {
        .a { color: { color }; }
    };
    let css = sheet.render();
    assert!(css.contains("color: red;"));
}

#[test]
fn css_macro_interpolate_string_value() {
    let bg = String::from("linear-gradient(red, blue)");
    let sheet = crate::css! {
        .a { background: { bg }; }
    };
    let css = sheet.render();
    assert!(css.contains("background: linear-gradient(red, blue);"));
}

#[test]
fn css_macro_interpolate_typed_values() {
    let sheet = crate::css! {
        .a {
            color: { Color::rgb(255, 0, 0) };
            width: { Length::px(8.0) };
            opacity: { 0.5f32 };
            z-index: { 10i32 };
        }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 0, 0);"));
    assert!(css.contains("width: 8px;"));
    assert!(css.contains("opacity: 0.5;"));
    assert!(css.contains("z-index: 10;"));
}

#[test]
fn css_macro_interpolate_css_value() {
    let v = Value::Percentage(crate::css::values::Percentage::new(50.0));
    let sheet = crate::css! {
        .a { width: { v }; }
    };
    let css = sheet.render();
    assert!(css.contains("width: 50%;"));
}

#[test]
fn css_macro_interpolate_dashed_name_and_custom_property() {
    let size = String::from("8px");
    let sheet = crate::css! {
        :root { --brand: { size }; }
        .a { background-color: { "blue" }; }
    };
    let css = sheet.render();
    assert!(css.contains("--brand: 8px;"));
    assert!(css.contains("background-color: blue;"));
}

#[test]
fn css_macro_interpolate_inside_nested_rule() {
    let hover: &'static str = "blue";
    let sheet = crate::css! {
        .a {
            color: red;
            &:hover { color: { hover }; }
        }
    };
    let css = sheet.render();
    assert!(css.contains("color: rgb(255, 0, 0);"));
    assert!(css.contains("color: blue;"));
}

#[test]
fn css_macro_interpolate_mixed_with_plain_declarations() {
    let gap = String::from("16px");
    let sheet = crate::css! {
        .a {
            margin: { gap };
            padding: 8px;
        }
    };
    let css = sheet.render();
    assert!(css.contains("margin: 16px;"));
    assert!(css.contains("padding: 8px;"));
}

#[test]
fn comments_inside_css_macro_are_stripped() {
    // Rust comments never reach the macro: the lexer strips them
    // before `css!` sees the token stream (project code review §3.8).
    let sheet = crate::css! {
        /* leading block comment */
        .btn {
            // line comment inside a rule body
            color: red; /* trailing block comment */
        }
        // line comment between rules
        .link { color: blue; }
    };
    let css_text = sheet.render();
    assert!(css_text.contains(".btn"));
    assert!(css_text.contains("color: rgb(255, 0, 0);"));
    assert!(css_text.contains(".link"));
    assert!(css_text.contains("color: rgb(0, 0, 255);"));
}
