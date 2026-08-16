//! The [`css!`] macro: CSS-like syntax compiled to the typed
//! [`StyleSheet`](crate::css::StyleSheet) API.
//!
//! The macro recognizes *structure* at compile time — rules vs
//! at-rules, blocks vs statements, declarations vs nested rules —
//! using two token-munching helper macros ([`__mrk_css_sheet!`] and
//! [`__mrk_css_body!`]). *Grammar* (selector syntax, at-rule
//! preludes, declaration values) is delegated to the runtime helpers
//! in [`parse`](crate::css::parse), so malformed structure is a rustc
//! error at the call site while malformed values panic with a message
//! pointing at the offending fragment.

/// Build a [`StyleSheet`](crate::css::StyleSheet) from CSS-like syntax.
///
/// Exported at the
/// crate root (and importable as `use mrk_css::css;`, which brings both the
/// `css` module and the `css!` macro into scope).
///
/// Values are parsed into typed [`Value`](crate::css::Value)s:
/// dimensions (`8px`, `1.5rem`), percentages (`100%`), angles
/// (`45deg`), times, hex colors (`#fff`), color functions
/// (`rgb(255, 0, 0)`), the 148 named colors (`rebeccapurple`),
/// `url(…)`, `var(--name[, fallback])`, plain numbers, and
/// space-separated lists (`margin: 8px 16px`). Quoted strings pass
/// through as [`Value::String`](crate::css::Value). `!important` is
/// recognized (both the glued `!important` and the split
/// `! important` lexed forms).
///
/// The macro recognizes structure at compile time: unclosed blocks,
/// stray top-level declarations, and missing `;` terminators are
/// rustc errors at the call site. Value-grammar errors (unknown
/// at-rule, bad selector, malformed value) panic at expansion time
/// with a message pointing at the offending fragment.
///
/// # Example
///
/// ```
/// use mrk_css::{css, Renderable};
///
/// let sheet = css! {
///     .btn {
///         background-color: rgba(0,0,0);
///         color: blue;
///         width: 8px;
///         &:hover { color: red; }
///         & .text { font-weight: bold; }
///         "&.primary" { color: green; }
///         @media "(min-width: 800px)" {
///             & { width: 100%; }
///         }
///     }
///     @media (prefers-color-scheme: dark) {
///         .btn { color: white; }
///     }
/// };
///
/// let css_text = sheet.render();
/// assert!(css_text.contains(".btn"));
/// assert!(css_text.contains("background-color: rgb(0, 0, 0);"));
/// assert!(css_text.contains("color: rgb(0, 0, 255);"));
/// assert!(css_text.contains("width: 8px;"));
/// assert!(css_text.contains("&:hover"));
/// assert!(css_text.contains("& .text"));
/// assert!(css_text.contains("@media (min-width: 800px)"));
/// ```
///
/// # Interpolation
///
/// A declaration value written as a single `{ expr }` group splices a
/// Rust value into the stylesheet — the same pattern the `html!` and
/// `svg!` macros in `mrk` use. The
/// expression must implement `Into<Value>`: `&'static str` and
/// `String` become raw (verbatim) values, `f32`/`f64`/`i32` become
/// numbers, and the typed value wrappers (`Color`, `Length`,
/// `Percentage`, …) convert to their typed
/// [`Value`](crate::css::Value) variant.
///
/// ```
/// use mrk_css::{css, Renderable};
///
/// let brand = String::from("rebeccapurple");
/// let gap: &'static str = "8px";
/// let sheet = css! {
///     .btn {
///         background: { brand };
///         margin: { gap };
///         opacity: { 0.9f32 };
///         z-index: { 10i32 };
///     }
/// };
/// let css_text = sheet.render();
/// assert!(css_text.contains("background: rebeccapurple;"));
/// assert!(css_text.contains("margin: 8px;"));
/// assert!(css_text.contains("opacity: 0.9;"));
/// assert!(css_text.contains("z-index: 10;"));
/// ```
///
/// The interpolated expression must be the *whole* value
/// (`color: { c };` works; `border: 1px solid { c };` does not), and
/// interpolation is available in rule bodies (at any nesting depth)
/// but not inside at-rule bodies, which are parsed by the runtime
/// grammar. Interpolation in selector or at-rule-prelude position is
/// not supported.
///
/// # Supported syntax
///
/// - Style rules with selectors: `.class`, `#id`, `type`, `*`, `&`,
///   attribute selectors (`[foo]`, `[foo="bar"]`, `[href^="https" i]`),
///   `:pseudo-class`, `::pseudo-element`, functional pseudo-classes
///   (`:nth-child(2n + 1)`), glued compounds (`.btn.primary`, `a.btn`),
///   descendant (juxtaposition), child `>`, sibling `+` / `~`, and
///   comma-separated lists (`.a, .b { }`).
/// - Nested rules at any depth, with or without `&` (CSS nesting).
/// - Every at-rule the [`AtRule`](crate::css::AtRule) AST supports:
///   `@media`, `@supports`, `@container`, `@scope`, `@layer` (block
///   and statement forms), `@keyframes`, `@font-face`, `@page`,
///   `@import`, `@charset`, and `@namespace`. At-rule preludes may be
///   bare tokens or a quoted string.
/// - Custom properties: `--brand: rebeccapurple;` declares and
///   `var(--brand)` / `var(--brand, blue)` references them.
/// - `calc()` and other functional values, including binary
///   operators (`calc(100% - 8px)`), space-separated arguments
///   (`hsl(120 50% 50%)`), and negative or >100% percentages
///   (`translate(-50%, -8px)`).
/// - Declarations as `name: value;` (see the value forms above).
/// - Rust comments (`//` and `/* */`) anywhere — the lexer strips
///   them before the macro sees the tokens.
///
/// # Limitations (inherent to `macro_rules!`)
///
/// - **Recursion limit.** Structure is recognized by token-munching
///   macros, which cost one recursion frame per token. This crate
///   sets `#[recursion_limit = "256"]`; downstream crates compiling
///   very large stylesheets may need to raise their own limit.
/// - **`1.5em` and `1.5ex` fail at lex time.** Rust's lexer reads the
///   `e` as a float exponent. Write `1.5 em` (split) or quote the
///   value instead. Other glued dimensions (`8px`, `1.5rem`, `100%`,
///   `45deg`, `0.5s`) work as-is.
/// - **`#id` glued after an identifier fails at lex time.** Rust
///   reads `.a#b` / `div#main` as an identifier with a `#` prefix
///   suffix and rejects it. `#main` on its own (or after a
///   combinator) works as-is; glue cases need a quoted selector:
///   `"div#main" { }` — it is rendered verbatim (see
///   [`selector()`](crate::css::selector::selector)).
#[macro_export]
macro_rules! css {
    ($($body:tt)*) => {{
        #[allow(unused_mut)] // empty sheets emit no assignments
        let mut __sheet = $crate::css::StyleSheet::new();
        $crate::__mrk_css_sheet!(@scan __sheet [] [$($body)*]);
        __sheet.build()
    }};
}

/// Implementation detail of [`css!`](crate::css!). Munches the
/// top-level token stream one token at a time, classifying each item
/// at its terminator: `;` ends a statement at-rule, `{ … }` ends a
/// style rule or block at-rule. Not public API — it is `#[macro_export]`ed only so the expansion of the user-facing macro can reach it. Calling it directly is unsupported: its grammar and generated code may change in any release.
///
/// The accumulator/rest are kept in separate bracketed groups so no
/// matcher is ever ambiguous (a `$($t:tt)*` repetition directly
/// followed by `;` or `{` is rejected by rustc as a local
/// ambiguity). Selector text, at-rule preludes, and at-rule bodies
/// are `stringify!`'d and handed to the runtime grammar in
/// [`parse`](crate::css::parse).
#[doc(hidden)]
#[macro_export]
macro_rules! __mrk_css_sheet {
    // Done: empty accumulator, empty input.
    (@scan $sheet:ident [] []) => {};
    // Trailing tokens with no `;` or `{ … }` terminator.
    (@scan $sheet:ident [$($acc:tt)+] []) => {
        ::std::compile_error!("css!: expected `;` or `{ … }` after these tokens");
    };
    // Stray `;` (empty statement) — skipped, like the runtime chunker.
    (@scan $sheet:ident [] [; $($rest:tt)*]) => {
        $crate::__mrk_css_sheet!(@scan $sheet [] [$($rest)*]);
    };
    // Statement at-rule: `@charset …;`, `@import …;`, `@layer …;`, …
    (@scan $sheet:ident [@ $($at:tt)*] [; $($rest:tt)*]) => {{
        $sheet = $sheet.at_rule($crate::css::parse::parse_at_rule(
            ::std::stringify!(@ $($at)*),
            ::std::option::Option::None,
        ));
        $crate::__mrk_css_sheet!(@scan $sheet [] [$($rest)*]);
    }};
    // A statement that does not start with `@` is a declaration —
    // those need a rule around them.
    (@scan $sheet:ident [$($acc:tt)+] [; $($rest:tt)*]) => {
        ::std::compile_error!(
            "css!: declarations need a rule around them — wrap them in a `selector { … }` block"
        );
    };
    // Block at-rule: `@media … { … }`, `@font-face { … }`, …
    (@scan $sheet:ident [@ $($at:tt)*] [{ $($body:tt)* } $($rest:tt)*]) => {{
        $sheet = $sheet.at_rule($crate::css::parse::parse_at_rule(
            ::std::stringify!(@ $($at)*),
            ::std::option::Option::Some(::std::stringify!($($body)*)),
        ));
        $crate::__mrk_css_sheet!(@scan $sheet [] [$($rest)*]);
    }};
    // Style rule: `selector { body }`.
    (@scan $sheet:ident [$($sel:tt)+] [{ $($body:tt)* } $($rest:tt)*]) => {{
        $sheet = $sheet.rule(|__r| {
            let __r = $crate::css::parse::parse_selector_list(::std::stringify!($($sel)+))
                .into_iter()
                .fold(__r, |__r, __s| __r.selector(__s));
            $crate::__mrk_css_body!(@body __r $($body)*)
        });
        $crate::__mrk_css_sheet!(@scan $sheet [] [$($rest)*]);
    }};
    // Shift one token from the input into the accumulator.
    (@scan $sheet:ident [$($acc:tt)*] [$t:tt $($rest:tt)*]) => {
        $crate::__mrk_css_sheet!(@scan $sheet [$($acc)* $t] [$($rest)*]);
    };
}

/// Implementation detail of [`css!`](crate::css!). Munches a rule
/// body, evaluating to the populated builder (`RuleBuilder` or
/// `NestedBuilder`). Not public API — it is `#[macro_export]`ed only so the expansion of the user-facing macro can reach it. Calling it directly is unsupported: its grammar and generated code may change in any release.
///
/// Interpolated declarations (`name: { expr };` and
/// `--name: { expr };`) are matched at entry, before the generic
/// scanner, because a brace group would otherwise be read as a
/// nested-rule block. The scanner then classifies each item at its
/// terminator: `;` ends a declaration, `{ … }` ends a nested rule or
/// nested block at-rule.
#[doc(hidden)]
#[macro_export]
macro_rules! __mrk_css_body {
    // Empty body: the builder passes through unchanged.
    (@body $builder:ident) => { $builder };
    // Interpolated declaration: `name: { expr };` (dashed names ok).
    (@body $builder:ident $name:ident $(- $npart:ident)* : { $e:expr } ; $($rest:tt)*) => {{
        let $builder = $builder.decl($crate::css::Declaration::new(
            ::std::concat!(::std::stringify!($name) $(, "-", ::std::stringify!($npart))*),
            ::std::convert::Into::<$crate::css::Value>::into($e),
        ));
        $crate::__mrk_css_body!(@body $builder $($rest)*)
    }};
    // Interpolated custom property: `--name: { expr };` (`--` lexes
    // as two `-` puncts).
    (@body $builder:ident - - $name:ident $(- $npart:ident)* : { $e:expr } ; $($rest:tt)*) => {{
        let $builder = $builder.decl($crate::css::Declaration::new(
            ::std::concat!("--", ::std::stringify!($name) $(, "-", ::std::stringify!($npart))*),
            ::std::convert::Into::<$crate::css::Value>::into($e),
        ));
        $crate::__mrk_css_body!(@body $builder $($rest)*)
    }};
    // Anything else: scan one token at a time.
    (@body $builder:ident $($tokens:tt)*) => {
        $crate::__mrk_css_body!(@scan $builder [] [$($tokens)*])
    };
    // Done: empty accumulator, empty input.
    (@scan $builder:ident [] []) => { $builder };
    // Trailing tokens with no `;` or `{ … }` terminator.
    (@scan $builder:ident [$($acc:tt)+] []) => {
        ::std::compile_error!("css!: expected `;` or `{ … }` after these tokens in this block");
    };
    // Stray `;` (empty statement) — skipped, like the runtime chunker.
    (@scan $builder:ident [] [; $($rest:tt)*]) => {
        $crate::__mrk_css_body!(@scan $builder [] [$($rest)*])
    };
    // Declaration: `name: value;` — the whole statement is
    // `stringify!`'d and parsed by the runtime grammar (which also
    // rejects nested statement at-rules with a dedicated message).
    (@scan $builder:ident [$($acc:tt)+] [; $($rest:tt)*]) => {{
        let $builder = $builder.decl($crate::css::parse::parse_declaration(
            ::std::stringify!($($acc)+),
        ));
        $crate::__mrk_css_body!(@scan $builder [] [$($rest)*])
    }};
    // Nested block at-rule: `@media … { … }` inside a rule.
    (@scan $builder:ident [@ $($at:tt)*] [{ $($inner:tt)* } $($rest:tt)*]) => {{
        let $builder = $builder.nest_at_rule($crate::css::parse::parse_at_rule(
            ::std::stringify!(@ $($at)*),
            ::std::option::Option::Some(::std::stringify!($($inner)*)),
        ));
        $crate::__mrk_css_body!(@scan $builder [] [$($rest)*])
    }};
    // Nested rule: `selector { body }` (CSS nesting, with or
    // without `&`).
    (@scan $builder:ident [$($sel:tt)+] [{ $($inner:tt)* } $($rest:tt)*]) => {{
        let $builder = $builder.nest(|__n| {
            let __n = $crate::css::parse::parse_selector_list(::std::stringify!($($sel)+))
                .into_iter()
                .fold(__n, |__n, __s| __n.selector(__s));
            $crate::__mrk_css_body!(@body __n $($inner)*)
        });
        $crate::__mrk_css_body!(@scan $builder [] [$($rest)*])
    }};
    // Shift one token from the input into the accumulator.
    (@scan $builder:ident [$($acc:tt)*] [$t:tt $($rest:tt)*]) => {
        $crate::__mrk_css_body!(@scan $builder [$($acc)* $t] [$($rest)*])
    };
}
