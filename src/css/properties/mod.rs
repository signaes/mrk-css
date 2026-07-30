//! Property-value AST: [`Value`] enum and the [`define_property!`] macro.
//!
//! [`Value`] is the central enum that wraps every typed CSS value.
//! It provides `Display`, `From` impls for every value type, and
//! `pub(crate)` helpers for the renderer.

use std::borrow::Cow;
use std::fmt;

use crate::css::values::{
    named_color_srgb, Angle, Color, CssString, CustomProperty, EasingFunction, Frequency, Ident,
    Integer, Length, Number, Percentage, Resolution, Time, Url,
};

/// A typed CSS property value.
///
/// Every variant wraps a strongly-typed `values` entry.
/// `Value::Raw` is `pub(crate)` and used for unknown / custom
/// property values that bypass the type system.
#[derive(Debug, Clone)]
pub enum Value {
    /// Wraps a `Color` value.
    Color(Color),
    /// Wraps a `Length` value.
    Length(Length),
    /// Wraps a `Percentage` value.
    Percentage(Percentage),
    /// Wraps a `Time` value.
    Time(Time),
    /// Wraps an `Angle` value.
    Angle(Angle),
    /// Wraps a `Frequency` value.
    Frequency(Frequency),
    /// Wraps a `Resolution` value.
    Resolution(Resolution),
    /// Wraps a `Number` value.
    Number(Number),
    /// Wraps an `Integer` value.
    Integer(Integer),
    /// Wraps a `CssString` value.
    String(CssString),
    /// Wraps a `Url` value.
    Url(Url),
    /// Wraps an `Ident` value.
    Identifier(Ident),
    /// Wraps a `CustomProperty` value.
    CustomProperty(CustomProperty),
    /// Wraps an `EasingFunction` value.
    EasingFunction(EasingFunction),
    /// A functional notation: `name(args...)`.
    Function {
        /// Function name.
        name: Cow<'static, str>,
        /// Function arguments.
        args: Vec<Value>,
    },
    /// A space-separated value list.
    List(Vec<Value>),
    /// A raw CSS string (crate-internal escape hatch).
    Raw(Cow<'static, str>),
}

// ── From impls ──────────────────────────────────────────────────────

impl From<Color> for Value {
    fn from(v: Color) -> Self { Value::Color(v) }
}
impl From<Length> for Value {
    fn from(v: Length) -> Self { Value::Length(v) }
}
impl From<Percentage> for Value {
    fn from(v: Percentage) -> Self { Value::Percentage(v) }
}
impl From<Time> for Value {
    fn from(v: Time) -> Self { Value::Time(v) }
}
impl From<Angle> for Value {
    fn from(v: Angle) -> Self { Value::Angle(v) }
}
impl From<Frequency> for Value {
    fn from(v: Frequency) -> Self { Value::Frequency(v) }
}
impl From<Resolution> for Value {
    fn from(v: Resolution) -> Self { Value::Resolution(v) }
}
impl From<Number> for Value {
    fn from(v: Number) -> Self { Value::Number(v) }
}
impl From<Integer> for Value {
    fn from(v: Integer) -> Self { Value::Integer(v) }
}
impl From<CssString> for Value {
    fn from(v: CssString) -> Self { Value::String(v) }
}
impl From<Url> for Value {
    fn from(v: Url) -> Self { Value::Url(v) }
}
impl From<Ident> for Value {
    fn from(v: Ident) -> Self { Value::Identifier(v) }
}
impl From<CustomProperty> for Value {
    fn from(v: CustomProperty) -> Self { Value::CustomProperty(v) }
}
impl From<EasingFunction> for Value {
    fn from(v: EasingFunction) -> Self { Value::EasingFunction(v) }
}

impl From<&'static str> for Value {
    fn from(s: &'static str) -> Self {
        Value::Raw(Cow::Borrowed(s))
    }
}
impl From<f32> for Value {
    fn from(v: f32) -> Self { Value::Number(v.into()) }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self { Value::Number(v.into()) }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self { Value::Integer(v.into()) }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Raw(Cow::Owned(s))
    }
}

// ── Display ─────────────────────────────────────────────────────────

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Color(v) => fmt::Display::fmt(v, f),
            Value::Length(v) => fmt::Display::fmt(v, f),
            Value::Percentage(v) => fmt::Display::fmt(v, f),
            Value::Time(v) => fmt::Display::fmt(v, f),
            Value::Angle(v) => fmt::Display::fmt(v, f),
            Value::Frequency(v) => fmt::Display::fmt(v, f),
            Value::Resolution(v) => fmt::Display::fmt(v, f),
            Value::Number(v) => fmt::Display::fmt(v, f),
            Value::Integer(v) => fmt::Display::fmt(v, f),
            Value::String(v) => fmt::Display::fmt(v, f),
            Value::Url(v) => fmt::Display::fmt(v, f),
            Value::Identifier(v) => fmt::Display::fmt(v, f),
            Value::CustomProperty(v) => fmt::Display::fmt(v, f),
            Value::EasingFunction(v) => fmt::Display::fmt(v, f),
            Value::Function { name, args } => {
                let mut s = String::from(name.as_ref());
                s.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { s.push_str(", "); }
                    s.push_str(&arg.to_string());
                }
                s.push(')');
                f.write_str(&s)
            }
            Value::List(items) => {
                let mut s = String::new();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { s.push(' '); }
                    s.push_str(&item.to_string());
                }
                f.write_str(&s)
            }
            Value::Raw(s) => f.write_str(s),
        }
    }
}

impl Value {
    /// Render this value to a CSS string. Called by the pretty-printer.
    #[allow(dead_code)]
    pub(crate) fn into_string(self) -> String {
        self.to_string()
    }
}

// ── Value parser (used by the `css!` macro) ────────────────────────────────

/// Parse a single-valued CSS string into a typed [`Value`].
///
/// Recognizes (in order):
/// 1. Hex colors: `#fff`, `#ff0000`
/// 2. Function notation: `rgb(255, 0, 0)`, `url(img.png)`, etc.
/// 3. Number + unit: `8px`, `1.5em`, `100%`, `90deg`, `0.3s`, …
/// 4. Bare integers: `0`, `42`
/// 5. Bare floats: `1.5`, `-1.5`
/// 6. Named colors: `red`, `blue`, `rebeccapurple`, … (148 entries)
/// 7. Anything else: wrapped as `Value::Identifier`
///
/// Whitespace around the input is trimmed. Empty / whitespace-only
/// input becomes `Value::Raw("")`.
pub(crate) fn parse_value(s: &str) -> Value {
    let s = s.trim();
    if s.is_empty() {
        return Value::Raw(Cow::Borrowed(""));
    }

    // Function call: name(args) — try before hex/literal so "rgb(255,0,0)"
    // is recognized as a color fn, not as raw "rgb ( 255 , 0 , 0 )".
    if let Some((name, args_str)) = split_function_call(s) {
        return parse_function_value(name, args_str);
    }

    // Hex color: #fff or #ff0000.
    if let Some(c) = s.strip_prefix('#').and_then(|hex| Color::hex(&format!("#{hex}"))) {
        return Value::Color(c);
    }

    // Number + unit (glued like "8px" or split like "8 px").
    if let Some(v) = parse_number_with_unit(s) {
        return v;
    }

    // Bare signed integer.
    if let Ok(n) = s.parse::<i32>() {
        return Value::Integer(Integer::from(n));
    }

    // Bare signed float (catches -1.5, 1.5, etc.).
    if let Ok(n) = s.parse::<f32>() {
        return Value::Number(Number::from(n));
    }

    // Named CSS color (148 entries).
    if let Some((r, g, b)) = named_color_srgb(s) {
        return Value::Color(Color::rgb(r, g, b));
    }

    // Fallback: identifier (covers keywords like `auto`, `none`,
    // `inherit`, `initial`, `unset`, `transparent`, `currentcolor`,
    // `solid`, `bold`, custom idents, etc.).
    Value::Identifier(Ident::from(s.to_string()))
}

/// Parse a space-separated list of values into a `Vec<Value>`.
///
/// Each whitespace-separated token is parsed independently with
/// [`parse_value`]. Returns a `Value::List` if more than one token
/// survives, otherwise returns the single value (or
/// `Value::Raw("")` for an empty input).
pub(crate) fn parse_value_list(s: &str) -> Value {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.is_empty() {
        return Value::Raw(Cow::Borrowed(""));
    }
    if parts.len() == 1 {
        return parse_value(parts[0]);
    }
    let mut values: Vec<Value> = Vec::new();
    for part in parts {
        let (head, glued) = split_glued_dot(part);
        values.push(parse_value(head));
        if let Some(tail) = glued {
            values.push(parse_value(tail));
        }
    }
    Value::List(values)
}

/// Split a token like `all.3s` back into `all` and `.3s`.
///
/// `stringify!` drops the space between an identifier and a
/// leading-dot number (`all .3s` → `all.3s`), gluing the two tokens
/// into one. Split them again so the value renders as valid CSS.
/// Only splits when the token starts with an identifier character,
/// the `.` follows an identifier character, and a digit comes right
/// after it — `1.5em`, `-.5` and `url(image.jpg)` are left alone.
fn split_glued_dot(token: &str) -> (&str, Option<&str>) {
    let bytes = token.as_bytes();
    if !matches!(bytes.first(), Some(b) if b.is_ascii_alphabetic() || *b == b'_') {
        return (token, None);
    }
    for i in 1..bytes.len() {
        if bytes[i] == b'.'
            && matches!(bytes[i - 1], b if b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            && matches!(bytes.get(i + 1), Some(b) if b.is_ascii_digit())
        {
            return (&token[..i], Some(&token[i..]));
        }
    }
    (token, None)
}

/// Parse a string like `"8px"`, `"1.5em"`, `"100%"`, `"90deg"` into
/// a typed [`Value`]. Returns `None` if the string isn't a recognized
/// number-with-unit.
///
/// If the string has no unit (e.g. `"8"`, `"1.5"`, `"-1"`), returns
/// `None` — the caller falls back to integer/float parsing.
fn parse_number_with_unit(s: &str) -> Option<Value> {
    let s = s.trim();
    let bytes = s.as_bytes();
    let mut i = 0;
    let has_sign = bytes.first() == Some(&b'-') || bytes.first() == Some(&b'+');
    if has_sign {
        i = 1;
    }
    let num_start = 0;
    let mut has_dot = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            i += 1;
        } else if b == b'.' && !has_dot && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
            has_dot = true;
            i += 1;
        } else {
            break;
        }
    }
    if i == num_start || (has_sign && i == num_start + 1) {
        return None;
    }
    let unit = s[i..].trim();
    if unit.is_empty() {
        return None;
    }
    // The scanner above only admits valid `f32` syntax (optional sign,
    // digits, one dot followed by a digit), so the parse cannot fail.
    let n: f32 = s[..i].parse().unwrap_or(0.0);
    Some(unit_to_value(n, unit))
}

/// Map a numeric value + unit string to a typed [`Value`].
fn unit_to_value(n: f32, unit: &str) -> Value {
    match unit {
        // Length
        "px" => Value::Length(Length::px(n)),
        "em" => Value::Length(Length::em(n)),
        "rem" => Value::Length(Length::rem(n)),
        "ex" => Value::Length(Length::ex(n)),
        "ch" => Value::Length(Length::ch(n)),
        "vw" => Value::Length(Length::vw(n)),
        "vh" => Value::Length(Length::vh(n)),
        "vmin" => Value::Length(Length::vmin(n)),
        "vmax" => Value::Length(Length::vmax(n)),
        "cm" => Value::Length(Length::cm(n)),
        "mm" => Value::Length(Length::mm(n)),
        "in" => Value::Length(Length::inches(n)),
        "pt" => Value::Length(Length::pt(n)),
        "pc" => Value::Length(Length::pc(n)),
        "fr" => Value::Length(Length::fr(n)),
        // Percentage — the typed value clamps to [0, 100] by design;
        // out-of-range percentages (valid in CSS, e.g. `translate(-50%)`
        // or `width: 150%`) are kept verbatim as raw text instead.
        "%" if (0.0..=100.0).contains(&n) => Value::Percentage(Percentage::new(n)),
        "%" => Value::Raw(Cow::Owned(format!("{n}%"))),
        // Angle
        "deg" => Value::Angle(Angle::deg(n)),
        "rad" => Value::Angle(Angle::rad(n)),
        "grad" => Value::Angle(Angle::grad(n)),
        "turn" => Value::Angle(Angle::turn(n)),
        // Time
        "s" => Value::Time(Time::s(n)),
        "ms" => Value::Time(Time::ms(n)),
        // Frequency
        "hz" => Value::Frequency(Frequency::hz(n)),
        "khz" => Value::Frequency(Frequency::khz(n)),
        // Resolution
        "dpi" => Value::Resolution(Resolution::dpi(n)),
        "dpcm" => Value::Resolution(Resolution::dpcm(n)),
        "x" => Value::Resolution(Resolution::x(n)),
        // Unknown unit — fall back to raw (caller will handle).
        _ => Value::Raw(Cow::Owned(format!("{}{}", n, unit))),
    }
}

/// Split `name(args)` into the function name and the argument string.
/// Returns `None` if the string isn't a function call.
fn split_function_call(s: &str) -> Option<(&str, &str)> {
    let open = s.find('(')?;
    if !s.ends_with(')') {
        return None;
    }
    let name = s[..open].trim();
    let args = &s[open + 1..s.len() - 1];
    if name.is_empty() || !is_ident(name) {
        return None;
    }
    Some((name, args))
}

/// Build a [`Value`] from a parsed function call.
fn parse_function_value(name: &str, args_str: &str) -> Value {
    // Function arguments are comma-separated (CSS spec). Each
    // argument may itself be a space-separated list, e.g. modern
    // color notation `hsl(120 50% 50%)`.
    let arg_vec: Vec<Value> = if args_str.trim().is_empty() {
        Vec::new()
    } else {
        args_str.split(',').map(|a| parse_value_list(a.trim())).collect()
    };

    if let Some(color) = try_color_factory(name, &arg_vec) {
        return Value::Color(color);
    }

    // url(s) — strip quotes.
    if name == "url" {
        let s = args_str.trim();
        let unquoted = s
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| s.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(s);
        return Value::Url(Url::local(unquoted.to_string()));
    }

    // var(--name) or var(--name, fallback).
    if name == "var" {
        return parse_var_value(args_str);
    }

    // Generic functional notation.
    Value::Function {
        name: Cow::Owned(name.to_string()),
        args: arg_vec,
    }
}

/// Try to construct a [`Color`] from a known Color factory name and
/// already-parsed argument values.
fn try_color_factory(name: &str, args: &[Value]) -> Option<Color> {
    let floats: Vec<f32> = args
        .iter()
        .filter_map(|v| match v {
            Value::Number(n) => Some(n.value()),
            Value::Integer(i) => Some(i.value() as f32),
            Value::Percentage(p) => Some(p.value()),
            _ => None,
        })
        .collect();

    match (name, floats.as_slice()) {
        ("rgb", [r, g, b]) => Some(Color::rgb(r.clamp(0.0, 255.0) as u8, g.clamp(0.0, 255.0) as u8, b.clamp(0.0, 255.0) as u8)),
        ("rgba", [r, g, b, a]) => Some(Color::rgba(r.clamp(0.0, 255.0) as u8, g.clamp(0.0, 255.0) as u8, b.clamp(0.0, 255.0) as u8, *a)),
        ("hsl", [h, s, l]) => Some(Color::hsl(*h, *s, *l)),
        ("hsla", [h, s, l, a]) => Some(Color::hsla(*h, *s, *l, *a)),
        _ => None,
    }
}

/// Parse a `var(--name)` or `var(--name, fallback)` argument string.
fn parse_var_value(args_str: &str) -> Value {
    let mut parts = args_str.splitn(2, ',');
    let name_part = parts.next().unwrap_or("").trim().to_string();
    let fallback_str = parts.next().unwrap_or("").trim();
    let cp = CustomProperty::new(name_part.clone()).or_else(|| {
        // Allow shorthand without the leading `--` in the macro.
        if name_part.starts_with("--") {
            CustomProperty::new(name_part.clone())
        } else {
            CustomProperty::new(format!("--{}", name_part))
        }
    });
    if let Some(cp) = cp {
        if fallback_str.is_empty() {
            Value::Function {
                name: Cow::Borrowed("var"),
                args: vec![Value::CustomProperty(cp)],
            }
        } else {
            let fallback = parse_decl_value(fallback_str);
            Value::Function {
                name: Cow::Borrowed("var"),
                args: vec![Value::CustomProperty(cp), fallback],
            }
        }
    } else {
        Value::Raw(Cow::Owned(format!("var({})", args_str)))
    }
}

/// Cheap identifier check: alphanumerics, `-`, `_` only.
fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Split a value string at the last `!important` marker. Returns
/// `(value_without_important, important)`.
pub(crate) fn split_important(s: &str) -> (&str, bool) {
    // Find the last `!important` token (case-insensitive).
    let lower = s.to_lowercase();
    if let Some(idx) = lower.rfind("!important") {
        // Make sure the `!` is preceded by whitespace (not part of
        // a value like `!important_value`).
        let prefix_ok = idx == 0
            || s.as_bytes()
                .get(idx.wrapping_sub(1))
                .map(|b| b.is_ascii_whitespace())
                .unwrap_or(false);
        let suffix_ok = idx + "!important".len() == s.len()
            || s.as_bytes()
                .get(idx + "!important".len())
                .map(|b| b.is_ascii_whitespace())
                .unwrap_or(false);
        if prefix_ok && suffix_ok {
            let value = s[..idx].trim_end();
            return (value, true);
        }
    }
    (s, false)
}

/// Parse a declaration value string into a typed [`Value`].
///
/// First tries [`parse_value`] on the whole string (single values,
/// including function calls with interior whitespace like
/// `rgb(255, 0, 0)`). If that yields a fallback (`Raw`, or an
/// `Identifier` containing whitespace), retries with
/// [`parse_value_list`] so multi-token values like `8px 16px` and
/// `1px solid red` become `Value::List`.
pub(crate) fn parse_decl_value(s: &str) -> Value {
    let v = parse_value(s);
    let multi = s.split_whitespace().count() > 1;
    if multi && matches!(v, Value::Raw(_) | Value::Identifier(_)) {
        parse_value_list(s)
    } else {
        v
    }
}

/// Define a property setter on a builder type.
///
/// # Syntax
///
/// ```ignore
/// define_property!(RuleBuilder, "color" => color, "Set the foreground color.");
/// define_property!(RuleBuilder, "background" => background, "Set the background.", shorthand);
/// ```
///
/// The macro generates:
/// ```ignore
/// impl RuleBuilder {
///     #[doc = "Set the foreground color."]
///     pub fn color(self, value: impl Into<Value>) -> RuleBuilder {
///         self.decl(Declaration::new(Cow::Borrowed("color"), value.into()))
///     }
/// }
/// ```
///
/// When `shorthand` is present, the generated method accepts
/// `Into<Value>` but is marked with a doc-comment note that it
/// accepts a shorthand value. The expansion is otherwise identical.
#[macro_export]
macro_rules! define_property {
    ($builder:ident, $name:literal => $method:ident, $doc:literal) => {
        define_property!(@inner $builder, $name, $method, $doc, false);
    };
    ($builder:ident, $name:literal => $method:ident, $doc:literal, shorthand) => {
        define_property!(@inner $builder, $name, $method, $doc, true);
    };
    (@inner $builder:ident, $name:expr, $method:ident, $doc:expr, $_shorthand:expr) => {
        #[doc = $doc]
        pub fn $method(self, value: impl Into<$crate::css::Value>) -> $builder {
            self.decl($crate::css::Declaration::new(
                std::borrow::Cow::Borrowed($name),
                value.into(),
            ))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_from_color() {
        let v: Value = Color::rgb(255, 0, 0).into();
        assert!(format!("{:?}", v).contains("Color("));
    }

    #[test]
    fn value_from_length() {
        let v: Value = Length::px(16.0).into();
        assert!(format!("{:?}", v).contains("Length("));
    }

    #[test]
    fn value_from_percentage() {
        let v: Value = Percentage::new(50.0).into();
        assert!(format!("{:?}", v).contains("Percentage("));
    }

    #[test]
    fn value_from_time() {
        let v: Value = Time::s(1.5).into();
        assert!(format!("{:?}", v).contains("Time("));
    }

    #[test]
    fn value_from_angle() {
        let v: Value = Angle::deg(45.0).into();
        assert!(format!("{:?}", v).contains("Angle("));
    }

    #[test]
    fn value_from_frequency() {
        let v: Value = Frequency::hz(100.0).into();
        assert!(format!("{:?}", v).contains("Frequency("));
    }

    #[test]
    fn value_from_resolution() {
        let v: Value = Resolution::dpi(96.0).into();
        assert!(format!("{:?}", v).contains("Resolution("));
    }

    #[test]
    fn value_from_number() {
        let v: Value = Number::new(1.5).into();
        assert!(format!("{:?}", v).contains("Number("));
    }

    #[test]
    fn value_from_integer() {
        let v: Value = Integer::new(42).into();
        assert!(format!("{:?}", v).contains("Integer("));
    }

    #[test]
    fn value_from_css_string() {
        let v: Value = CssString::new("hello").into();
        assert!(format!("{:?}", v).contains("String("));
    }

    #[test]
    fn value_from_url() {
        let v: Value = Url::local("style.css").into();
        assert!(format!("{:?}", v).contains("Url("));
    }

    #[test]
    fn value_from_ident() {
        let v: Value = Ident::from("auto").into();
        assert!(format!("{:?}", v).contains("Identifier("));
    }

    #[test]
    fn value_from_custom_property() {
        let v: Value = CustomProperty::new("--my-var").unwrap().into();
        assert!(format!("{:?}", v).contains("CustomProperty("));
    }

    #[test]
    fn value_from_easing() {
        let v: Value = EasingFunction::Ease.into();
        assert!(format!("{:?}", v).contains("EasingFunction("));
    }

    #[test]
    fn value_from_static_str() {
        let v: Value = Value::from("raw-value");
        assert!(format!("{:?}", v).contains("Raw("));
    }

    #[test]
    fn value_display_color() {
        assert_eq!(Value::Color(Color::named("red")).to_string(), "red");
    }

    #[test]
    fn value_display_length() {
        assert_eq!(Value::Length(Length::px(16.0)).to_string(), "16px");
    }

    #[test]
    fn value_display_percentage() {
        assert_eq!(Value::Percentage(Percentage::new(50.0)).to_string(), "50%");
    }

    #[test]
    fn value_display_function() {
        let v = Value::Function {
            name: Cow::Borrowed("var"),
            args: vec![Value::Identifier(Ident::from("--my-var"))],
        };
        assert_eq!(v.to_string(), "var(--my-var)");
    }

    #[test]
    fn value_display_function_multi_args() {
        let v = Value::Function {
            name: Cow::Borrowed("rgb"),
            args: vec![
                Value::Number(Number::new(255.0)),
                Value::Number(Number::new(0.0)),
                Value::Number(Number::new(0.0)),
            ],
        };
        assert_eq!(v.to_string(), "rgb(255, 0, 0)");
    }

    #[test]
    fn value_display_list() {
        let v = Value::List(vec![
            Value::Length(Length::px(8.0)),
            Value::Length(Length::px(16.0)),
        ]);
        assert_eq!(v.to_string(), "8px 16px");
    }

    #[test]
    fn value_display_raw() {
        let v = Value::Raw(Cow::Borrowed("some-raw-value"));
        assert_eq!(v.to_string(), "some-raw-value");
    }

    #[test]
    fn value_into_string() {
        let v = Value::Color(Color::named("red"));
        assert_eq!(v.into_string(), "red");
    }

    #[test]
    fn value_from_f64() {
        let v: Value = 1.5f64.into();
        assert!(format!("{:?}", v).contains("Number("));
    }

    #[test]
    fn value_from_i32() {
        // Iterate over a mix of inputs so the same matches! line is
        // hit with both true (Integer from i32) and false (non-Integer
        // from str).
        let cases: [(Box<dyn Fn() -> Value>, bool); 4] = [
            (Box::new(|| 0i32.into()), true),
            (Box::new(|| 42i32.into()), true),
            (Box::new(|| "hello".into()), false),
            (Box::new(|| "world".into()), false),
        ];
        for (make, expected_int) in &cases {
            let v = make();
            let is_int = matches!(v, Value::Integer(_));
            assert_eq!(is_int, *expected_int);
        }
        let v: Value = 0i32.into();
        assert_eq!(v.to_string(), "0");
    }

    #[test]
    fn value_from_string() {
        let v: Value = String::from("hi").into();
        assert!(format!("{:?}", v).contains("Raw("));
    }

    #[test]
    fn value_display_time() {
        let v = Value::Time(Time::s(1.5));
        assert_eq!(v.to_string(), "1.5s");
    }

    #[test]
    fn value_display_angle() {
        let v = Value::Angle(Angle::deg(45.0));
        assert_eq!(v.to_string(), "45deg");
    }

    #[test]
    fn value_display_frequency() {
        let v = Value::Frequency(Frequency::hz(100.0));
        assert_eq!(v.to_string(), "100hz");
    }

    #[test]
    fn value_display_resolution() {
        let v = Value::Resolution(Resolution::dpi(96.0));
        assert_eq!(v.to_string(), "96dpi");
    }

    #[test]
    fn value_display_url() {
        let v = Value::Url(Url::local("foo.css"));
        assert_eq!(v.to_string(), "url(\"foo.css\")");
    }

    #[test]
    fn value_display_custom_property() {
        let v = Value::CustomProperty(CustomProperty::new("--my-var").unwrap());
        assert_eq!(v.to_string(), "--my-var");
    }

    #[test]
    fn value_display_easing() {
        let v = Value::EasingFunction(EasingFunction::linear());
        assert_eq!(v.to_string(), "linear");
    }

    // ── parse_value + parse_value_list ─────────────────────────────

    #[test]
    fn parse_value_hex_color_3() {
        let v = parse_value("#fff");
        // Exercise both arms of the matches! macro (true: color,
        // false: not) so region coverage sees both outcomes.
        for (input, want) in [("#fff", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        // Color::hex parses to Rgb, so display is `rgb(255, 255, 255)`.
        assert_eq!(v.to_string(), "rgb(255, 255, 255)");
    }

    #[test]
    fn parse_value_hex_color_6() {
        let v = parse_value("#ff0000");
        for (input, want) in [("#ff0000", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        assert_eq!(v.to_string(), "rgb(255, 0, 0)");
    }

    #[test]
    fn parse_value_named_color() {
        let v = parse_value("red");
        for (input, want) in [("red", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        // Color::named resolves to Rgb via the lookup table.
        assert_eq!(v.to_string(), "rgb(255, 0, 0)");
    }

    #[test]
    fn parse_value_rebeccapurple() {
        let v = parse_value("rebeccapurple");
        for (input, want) in [("rebeccapurple", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        assert_eq!(v.to_string(), "rgb(102, 51, 153)");
    }

    #[test]
    fn parse_value_integer() {
        let v = parse_value("42");
        for (input, want) in [("42", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Integer(_)), want);
        }
        assert_eq!(v.to_string(), "42");
    }

    #[test]
    fn parse_value_float() {
        let v = parse_value("1.5");
        for (input, want) in [("1.5", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Number(_)), want);
        }
        assert_eq!(v.to_string(), "1.5");
    }

    #[test]
    fn parse_value_negative_float() {
        let v = parse_value("-1.5");
        for (input, want) in [("-1.5", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Number(_)), want);
        }
        assert_eq!(v.to_string(), "-1.5");
    }

    #[test]
    fn parse_value_length_px() {
        let v = parse_value("8px");
        for (input, want) in [("8px", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Length(_)), want);
        }
        assert_eq!(v.to_string(), "8px");
    }

    #[test]
    fn parse_value_length_px_split() {
        let v = parse_value("8 px");
        for (input, want) in [("8 px", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Length(_)), want);
        }
        assert_eq!(v.to_string(), "8px");
    }

    #[test]
    fn parse_value_length_frac_em() {
        let v = parse_value("1.5em");
        for (input, want) in [("1.5em", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Length(_)), want);
        }
        assert_eq!(v.to_string(), "1.5em");
    }

    #[test]
    fn parse_value_percentage() {
        let v = parse_value("100%");
        for (input, want) in [("100%", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Percentage(_)), want);
        }
        assert_eq!(v.to_string(), "100%");
    }

    #[test]
    fn parse_value_out_of_range_percentage_is_raw() {
        // Percentages outside [0, 100] are valid CSS but the typed
        // `Percentage` clamps them — keep them verbatim instead.
        assert_eq!(parse_value("-50%").to_string(), "-50%");
        assert_eq!(parse_value("150%").to_string(), "150%");
        assert_eq!(parse_value("translate(-50%, -8px)").to_string(), "translate(-50%, -8px)");
    }

    #[test]
    fn parse_value_angle() {
        let v = parse_value("90deg");
        for (input, want) in [("90deg", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Angle(_)), want);
        }
        assert_eq!(v.to_string(), "90deg");
    }

    #[test]
    fn parse_value_time() {
        let v = parse_value("0.3s");
        for (input, want) in [("0.3s", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Time(_)), want);
        }
        assert_eq!(v.to_string(), "0.3s");
    }

    #[test]
    fn parse_value_frequency() {
        let v = parse_value("16khz");
        for (input, want) in [("16khz", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Frequency(_)), want);
        }
        assert_eq!(v.to_string(), "16khz");
    }

    #[test]
    fn parse_value_resolution() {
        let v = parse_value("96dpi");
        for (input, want) in [("96dpi", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Resolution(_)), want);
        }
        assert_eq!(v.to_string(), "96dpi");
    }

    #[test]
    fn parse_value_rgb_function() {
        let v = parse_value("rgb(255, 0, 0)");
        for (input, want) in [("rgb(255, 0, 0)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        assert_eq!(v.to_string(), "rgb(255, 0, 0)");
    }

    #[test]
    fn parse_value_rgba_function() {
        let v = parse_value("rgba(0, 0, 0, 0.5)");
        for (input, want) in [("rgba(0, 0, 0, 0.5)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        assert_eq!(v.to_string(), "rgba(0, 0, 0, 0.5)");
    }

    #[test]
    fn parse_value_hsl_function() {
        let v = parse_value("hsl(0, 100%, 50%)");
        for (input, want) in [("hsl(0, 100%, 50%)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        assert_eq!(v.to_string(), "hsl(0, 100%, 50%)");
    }

    #[test]
    fn parse_value_hsla_function() {
        // Exercise both arms of the matches! macro (true: color,
        // false: not) so region coverage sees both outcomes.
        for (input, want) in [("hsla(120, 50%, 50%, 0.5)", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Color(_)), want);
        }
        assert_eq!(
            parse_value("hsla(120, 50%, 50%, 0.5)").to_string(),
            "hsla(120, 50%, 50%, 0.5)"
        );
    }

    #[test]
    fn parse_value_more_units() {
        for (input, want) in [
            ("1ex", "1ex"),
            ("1ch", "1ch"),
            ("2vw", "2vw"),
            ("2vh", "2vh"),
            ("3vmin", "3vmin"),
            ("3vmax", "3vmax"),
            ("1cm", "1cm"),
            ("1mm", "1mm"),
            ("1in", "1in"),
            ("1pt", "1pt"),
            ("1pc", "1pc"),
            ("1fr", "1fr"),
            ("1rad", "1rad"),
            ("200grad", "200grad"),
            ("1turn", "1turn"),
            ("500ms", "500ms"),
            ("60hz", "60hz"),
            ("300dpcm", "300dpcm"),
            ("2x", "2x"),
        ] {
            assert_eq!(parse_value(input).to_string(), want, "input: {input}");
        }
    }

    #[test]
    fn is_ident_outcomes() {
        for (input, want) in [
            ("rgb", true),
            ("my-fn", true),
            ("my_fn", true),
            ("", false),
            ("a b", false),
            ("a.b", false),
        ] {
            assert_eq!(is_ident(input), want, "input: {input}");
        }
    }

    #[test]
    fn parse_value_url() {
        let v = parse_value("url(img.png)");
        for (input, want) in [("url(img.png)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Url(_)), want);
        }
        assert_eq!(v.to_string(), "url(\"img.png\")");
    }

    #[test]
    fn parse_value_url_quoted() {
        let v = parse_value("url(\"img.png\")");
        for (input, want) in [("url(\"img.png\")", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Url(_)), want);
        }
        assert_eq!(v.to_string(), "url(\"img.png\")");
    }

    #[test]
    fn parse_value_url_single_quoted() {
        for (input, want) in [("url('img.png')", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Url(_)), want);
        }
        assert_eq!(parse_value("url('img.png')").to_string(), "url(\"img.png\")");
    }

    #[test]
    fn parse_value_var() {
        let v = parse_value("var(--my-var)");
        for (input, want) in [("var(--my-var)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Function { .. }), want);
        }
        assert_eq!(v.to_string(), "var(--my-var)");
    }

    #[test]
    fn parse_value_var_with_fallback() {
        let v = parse_value("var(--my-var, auto)");
        for (input, want) in [("var(--my-var, auto)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Function { .. }), want);
        }
        assert_eq!(v.to_string(), "var(--my-var, auto)");
    }

    #[test]
    fn parse_value_var_with_fallback_color() {
        // Fallback is a color — resolves to a Color, but renders as
        // the input CSS.
        let v = parse_value("var(--my-var, blue)");
        for (input, want) in [("var(--my-var, blue)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Function { .. }), want);
        }
        assert_eq!(v.to_string(), "var(--my-var, rgb(0, 0, 255))");
    }

    #[test]
    fn parse_value_ident_keyword() {
        let v = parse_value("auto");
        for (input, want) in [("auto", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Identifier(_)), want);
        }
        assert_eq!(v.to_string(), "auto");
    }

    #[test]
    fn parse_value_ident_solid() {
        let v = parse_value("solid");
        for (input, want) in [("solid", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Identifier(_)), want);
        }
        assert_eq!(v.to_string(), "solid");
    }

    #[test]
    fn parse_value_empty_string() {
        for (input, want) in [("", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Raw(_)), want);
        }
    }

    #[test]
    fn parse_value_list_single() {
        for (input, want) in [("8px", true), ("auto", false)] {
            assert_eq!(matches!(parse_value_list(input), Value::Length(_)), want);
        }
    }

    #[test]
    fn parse_value_list_two() {
        let v = parse_value_list("8px 16px");
        assert!(matches!(v, Value::List(items) if items.len() == 2));
    }

    #[test]
    fn parse_value_list_three() {
        let v = parse_value_list("8px 0 16px");
        assert!(matches!(v, Value::List(items) if items.len() == 3));
    }

    #[test]
    fn parse_value_list_empty() {
        for (input, want) in [("", true), ("8px", false)] {
            assert_eq!(matches!(parse_value_list(input), Value::Raw(_)), want);
        }
    }

    #[test]
    fn parse_value_list_glued_leading_dot() {
        // `stringify!(all .3s ease-in-out)` → `all.3s ease-in-out`;
        // the glued token is split back into `all` and `.3s`.
        assert_eq!(
            parse_value_list("all.3s ease-in-out").to_string(),
            "all 0.3s ease-in-out"
        );
        // `-` and `_` are identifier characters too.
        assert_eq!(parse_value_list("a-.5 c").to_string(), "a- 0.5 c");
        assert_eq!(parse_value_list("a_.5 c").to_string(), "a_ 0.5 c");
        // Decimal numbers and signed leading-dot numbers stay glued.
        assert_eq!(parse_value_list("1.5em solid").to_string(), "1.5em solid");
        assert_eq!(parse_value_list("-.5 solid").to_string(), "-0.5 solid");
        // A dot not followed by a digit is not split.
        assert_eq!(
            parse_value_list("url(image.jpg) no-repeat").to_string(),
            "url(\"image.jpg\") no-repeat"
        );
    }

    #[test]
    fn parse_value_generic_function() {
        let v = parse_value("translateX(10px)");
        for (input, want) in [("translateX(10px)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Function { .. }), want);
        }
        assert_eq!(v.to_string(), "translateX(10px)");
    }

    #[test]
    fn parse_value_function_unclosed() {
        for (input, want) in [("rgb(255, 0, 0", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Identifier(_)), want);
        }
    }

    #[test]
    fn parse_value_function_empty_name() {
        for (input, want) in [("(255)", true), ("8px", false)] {
            assert_eq!(matches!(parse_value(input), Value::Identifier(_)), want);
        }
    }

    #[test]
    fn parse_value_function_no_args() {
        let v = parse_value("f()");
        for (input, want) in [("f()", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Function { .. }), want);
        }
        assert_eq!(v.to_string(), "f()");
    }

    #[test]
    fn parse_value_var_without_dashes() {
        // Shorthand: a bare name is upgraded to a `--` custom property.
        let v = parse_value("var(brand)");
        for (input, want) in [("var(brand)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Function { .. }), want);
        }
        assert_eq!(v.to_string(), "var(--brand)");
    }

    #[test]
    fn parse_value_var_invalid_name_falls_back_to_raw() {
        let v = parse_value("var(--my var)");
        for (input, want) in [("var(--my var)", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Raw(_)), want);
        }
        assert_eq!(v.to_string(), "var(--my var)");
    }

    #[test]
    fn split_important_at_end() {
        assert_eq!(split_important("red !important"), ("red", true));
    }

    #[test]
    fn split_important_mid_string() {
        assert_eq!(split_important("red !important extra"), ("red", true));
    }

    #[test]
    fn split_important_suffix_not_boundary() {
        assert_eq!(split_important("red !importantx"), ("red !importantx", false));
    }

    #[test]
    fn split_important_absent() {
        assert_eq!(split_important("red"), ("red", false));
    }

    #[test]
    fn parse_value_unknown_unit_falls_back_to_raw() {
        for (input, want) in [("8foo", true), ("auto", false)] {
            assert_eq!(matches!(parse_value(input), Value::Raw(_)), want);
        }
    }

    // ── parse_decl_value ───────────────────────────────────────────

    #[test]
    fn parse_decl_value_single() {
        for (input, want) in [("8px", true), ("auto", false)] {
            assert_eq!(matches!(parse_decl_value(input), Value::Length(_)), want);
        }
    }

    #[test]
    fn parse_decl_value_split_single() {
        for (input, want) in [("8 px", true), ("auto", false)] {
            assert_eq!(matches!(parse_decl_value(input), Value::Length(_)), want);
        }
    }

    #[test]
    fn parse_decl_value_function_with_spaces() {
        for (input, want) in [("rgb(255, 0, 0)", true), ("8px", false)] {
            assert_eq!(matches!(parse_decl_value(input), Value::Color(_)), want);
        }
    }

    #[test]
    fn parse_decl_value_list_fallback() {
        // Whole-string parse yields Raw; the list retry wins.
        let v = parse_decl_value("8px 16px");
        assert!(matches!(v, Value::List(items) if items.len() == 2));
    }

    #[test]
    fn parse_decl_value_ident_list_fallback() {
        // Whole-string parse yields Identifier; the list retry wins.
        let v = parse_decl_value("1px solid red");
        assert!(matches!(v, Value::List(items) if items.len() == 3));
    }

    #[test]
    fn parse_decl_value_single_ident_kept() {
        for (input, want) in [("solid", true), ("8px", false)] {
            assert_eq!(matches!(parse_decl_value(input), Value::Identifier(_)), want);
        }
    }
}
