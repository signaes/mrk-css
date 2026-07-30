//! CSS selector AST.
//!
//! Selectors are the building blocks of CSS rule matching. The AST
//! in this module is deliberately minimal — just enough structure
//! to round-trip a selector to its CSS source form. The free
//! [`selector()`] function builds a [`Selector`] from any arbitrary
//! selector string (rendered verbatim via [`Selector::Raw`]).

use std::borrow::Cow;

/// A CSS selector AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    /// The universal selector `*`.
    Universal,
    /// A type / tag selector (e.g. `div`, `svg`).
    Type(Cow<'static, str>),
    /// A class selector (`.foo`).
    Class(Cow<'static, str>),
    /// An ID selector (`#foo`).
    Id(Cow<'static, str>),
    /// An attribute selector (`[foo=bar]`, `[foo^="bar"]`, etc.).
    Attribute {
        /// Attribute name.
        name: Cow<'static, str>,
        /// Comparison operator.
        op: AttrOp,
        /// Comparison value.
        value: Cow<'static, str>,
        /// Case sensitivity flag.
        case: AttrCase,
    },
    /// A bare attribute selector with no comparison (`[foo]`,
    /// `[disabled]`).
    AttributeBare(Cow<'static, str>),
    /// A pseudo-class or pseudo-element (`:hover`, `::before`,
    /// `:nth-child(2n+1)`, etc.).
    Pseudo(PseudoSelector),
    /// Compound selector: a sequence of simple selectors with no
    /// combinator (e.g. `a.btn#main:hover`).
    Compound(Vec<Selector>),
    /// Descendant combinator: `A B`.
    Descendant(Box<Selector>, Box<Selector>),
    /// Child combinator: `A > B`.
    Child(Box<Selector>, Box<Selector>),
    /// Adjacent-sibling combinator: `A + B`.
    Sibling(Box<Selector>, Box<Selector>),
    /// General-sibling combinator: `A ~ B`.
    GeneralSibling(Box<Selector>, Box<Selector>),
    /// The nesting reference `&` for CSS nesting.
    NestingRef,
    /// A raw selector string, rendered verbatim. Escape hatch for
    /// selectors the AST (or the `css!` token grammar) cannot
    /// express, e.g. compound class chains like `.btn.primary`.
    Raw(Cow<'static, str>),
}

/// Comparison operator in an attribute selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrOp {
    /// `=`.
    Equals,
    /// `^=`.
    StartsWith,
    /// `$=`.
    EndsWith,
    /// `*=`.
    Contains,
    /// `~=`.
    Includes,
    /// `|=`.
    DashMatch,
}

/// Case sensitivity flag for attribute selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrCase {
    /// `[foo=bar]` — ASCII case-sensitive (default).
    Sensitive,
    /// `[foo=bar i]` — ASCII case-insensitive.
    Insensitive,
}

/// Pseudo-class / pseudo-element selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoSelector {
    /// `:foo`.
    Class(Cow<'static, str>),
    /// `#foo`.
    Id(Cow<'static, str>),
    /// `::foo`.
    Element(Cow<'static, str>),
    /// `:foo(<args>)` — function-style pseudo.
    Function {
        /// Pseudo-class / -element name.
        name: Cow<'static, str>,
        /// Function arguments.
        args: Vec<SelectorArg>,
    },
    /// `:lang(<ident>)`.
    Lang(Cow<'static, str>),
    /// `:dir(<ltr|rtl>)`.
    Dir(Cow<'static, str>),
    /// `:not(...)`.
    Not(Vec<Selector>),
    /// `:is(...)`.
    Is(Vec<Selector>),
    /// `:where(...)`.
    Where(Vec<Selector>),
    /// `:has(...)`.
    Has(Vec<Selector>),
    /// `:nth-child(<an+b>)`.
    NthChild(Cow<'static, str>),
    /// `:nth-last-child(<an+b>)`.
    NthLastChild(Cow<'static, str>),
    /// `:nth-of-type(<an+b>)`.
    NthOfType(Cow<'static, str>),
    /// `:nth-last-of-type(<an+b>)`.
    NthLastOfType(Cow<'static, str>),
}

/// Argument to a function-style pseudo-class or pseudo-element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectorArg {
    /// `:is(.a, .b)` — list of selectors.
    Selectors(Vec<Selector>),
    /// `:lang(en)` — identifier.
    Ident(Cow<'static, str>),
    /// `:nth-child(2n+1)` — `<an+b>` expression.
    AnPlusB(Cow<'static, str>),
}

impl Selector {
    /// Convenience constructor for [`Selector::Class`].
    pub fn class(name: impl Into<Cow<'static, str>>) -> Self {
        Selector::Class(name.into())
    }

    /// Convenience constructor for [`Selector::Id`].
    pub fn id(name: impl Into<Cow<'static, str>>) -> Self {
        Selector::Id(name.into())
    }

    /// Convenience constructor for [`Selector::Type`].
    pub fn type_(name: impl Into<Cow<'static, str>>) -> Self {
        Selector::Type(name.into())
    }

    /// Convenience constructor for a pseudo-class.
    pub fn pseudo_class(name: impl Into<Cow<'static, str>>) -> Self {
        Selector::Pseudo(PseudoSelector::Class(name.into()))
    }

    /// Convenience constructor for a pseudo-element.
    pub fn pseudo_element(name: impl Into<Cow<'static, str>>) -> Self {
        Selector::Pseudo(PseudoSelector::Element(name.into()))
    }

    /// Convenience constructor for the universal selector.
    pub fn universal() -> Self {
        Selector::Universal
    }

    /// Convenience constructor for the nesting reference `&`.
    pub fn nesting_ref() -> Self {
        Selector::NestingRef
    }

    /// Convenience constructor for [`Selector::Raw`].
    pub fn raw(s: impl Into<Cow<'static, str>>) -> Self {
        Selector::Raw(s.into())
    }
}

/// Build a [`Selector`] from an arbitrary selector string.
///
/// The string is stored as [`Selector::Raw`] and rendered verbatim,
/// so any selector the typed AST cannot express (compound class
/// chains, attribute selectors, functional pseudo-classes with
/// complex arguments, …) can still be used:
///
/// ```
/// use mrk_css::css::selector::selector;
///
/// assert_eq!(selector(".btn.primary:hover").to_string(), ".btn.primary:hover");
/// ```
pub fn selector(s: impl Into<Cow<'static, str>>) -> Selector {
    Selector::Raw(s.into())
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Universal => f.write_str("*"),
            Selector::Type(t) => f.write_str(t),
            Selector::Class(c) => write!(f, ".{}", c),
            Selector::Id(i) => write!(f, "#{}", i),
            Selector::Attribute { name, op, value, case } => {
                let mut s = String::from("[");
                s.push_str(name);
                match op {
                    AttrOp::Equals => s.push('='),
                    AttrOp::StartsWith => s.push_str("^="),
                    AttrOp::EndsWith => s.push_str("$="),
                    AttrOp::Contains => s.push_str("*="),
                    AttrOp::Includes => s.push_str("~="),
                    AttrOp::DashMatch => s.push_str("|="),
                }
                s.push('"');
                s.push_str(value);
                s.push('"');
                match case {
                    AttrCase::Sensitive => {}
                    AttrCase::Insensitive => s.push_str(" i"),
                }
                s.push(']');
                f.write_str(&s)
            }
            Selector::AttributeBare(name) => write!(f, "[{}]", name),
            Selector::Pseudo(p) => write!(f, "{}", p),
            Selector::Compound(parts) => {
                let mut s = String::new();
                for part in parts.iter() {
                    s.push_str(&part.to_string());
                }
                f.write_str(&s)
            }
            Selector::Descendant(a, b) => write!(f, "{} {}", a, b),
            Selector::Child(a, b) => write!(f, "{} > {}", a, b),
            Selector::Sibling(a, b) => write!(f, "{} + {}", a, b),
            Selector::GeneralSibling(a, b) => write!(f, "{} ~ {}", a, b),
            Selector::NestingRef => f.write_str("&"),
            Selector::Raw(s) => f.write_str(s),
        }
    }
}

impl std::fmt::Display for PseudoSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PseudoSelector::Class(c) => write!(f, ":{}", c),
            PseudoSelector::Id(i) => write!(f, ":{}", i),
            PseudoSelector::Element(e) => write!(f, "::{}", e),
            PseudoSelector::Function { name, args } => {
                let mut s = format!(":{}(", name);
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { s.push_str(", "); }
                    s.push_str(&arg.to_string());
                }
                s.push(')');
                f.write_str(&s)
            }
            PseudoSelector::Lang(l) => write!(f, ":lang({})", l),
            PseudoSelector::Dir(d) => write!(f, ":dir({})", d),
            PseudoSelector::Not(sels) => write!(f, ":not({})", sels_selector_string(sels)),
            PseudoSelector::Is(sels) => write!(f, ":is({})", sels_selector_string(sels)),
            PseudoSelector::Where(sels) => write!(f, ":where({})", sels_selector_string(sels)),
            PseudoSelector::Has(sels) => write!(f, ":has({})", sels_selector_string(sels)),
            PseudoSelector::NthChild(n) => write!(f, ":nth-child({})", n),
            PseudoSelector::NthLastChild(n) => write!(f, ":nth-last-child({})", n),
            PseudoSelector::NthOfType(n) => write!(f, ":nth-of-type({})", n),
            PseudoSelector::NthLastOfType(n) => write!(f, ":nth-last-of-type({})", n),
        }
    }
}

impl std::fmt::Display for SelectorArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectorArg::Selectors(sels) => write!(f, "{}", sels_selector_string(sels)),
            SelectorArg::Ident(i) => f.write_str(i),
            SelectorArg::AnPlusB(s) => f.write_str(s),
        }
    }
}

fn sels_selector_string(sels: &[Selector]) -> String {
    let mut out = String::new();
    for (i, sel) in sels.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&sel.to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_universal() {
        assert_eq!(Selector::Universal.to_string(), "*");
    }

    #[test]
    fn display_type() {
        assert_eq!(Selector::Type(Cow::Borrowed("div")).to_string(), "div");
    }

    #[test]
    fn display_class() {
        assert_eq!(Selector::Class(Cow::Borrowed("btn")).to_string(), ".btn");
    }

    #[test]
    fn display_id() {
        assert_eq!(Selector::Id(Cow::Borrowed("main")).to_string(), "#main");
    }

    #[test]
    fn display_attribute_equals() {
        let s = Selector::Attribute {
            name: Cow::Borrowed("data-x"),
            op: AttrOp::Equals,
            value: Cow::Borrowed("foo"),
            case: AttrCase::Sensitive,
        };
        assert_eq!(s.to_string(), "[data-x=\"foo\"]");
    }

    #[test]
    fn display_attribute_starts_with_insensitive() {
        let s = Selector::Attribute {
            name: Cow::Borrowed("data-x"),
            op: AttrOp::StartsWith,
            value: Cow::Borrowed("foo"),
            case: AttrCase::Insensitive,
        };
        assert_eq!(s.to_string(), "[data-x^=\"foo\" i]");
    }

    #[test]
    fn display_pseudo_class() {
        let s = Selector::Pseudo(PseudoSelector::Class(Cow::Borrowed("hover")));
        assert_eq!(s.to_string(), ":hover");
    }

    #[test]
    fn display_pseudo_element() {
        let s = Selector::Pseudo(PseudoSelector::Element(Cow::Borrowed("before")));
        assert_eq!(s.to_string(), "::before");
    }

    #[test]
    fn display_pseudo_nth_child() {
        let s = Selector::Pseudo(PseudoSelector::NthChild(Cow::Borrowed("2n+1")));
        assert_eq!(s.to_string(), ":nth-child(2n+1)");
    }

    #[test]
    fn display_pseudo_not() {
        let s = Selector::Pseudo(PseudoSelector::Not(vec![Selector::Class(Cow::Borrowed("active"))]));
        assert_eq!(s.to_string(), ":not(.active)");
    }

    #[test]
    fn display_compound() {
        let s = Selector::Compound(vec![
            Selector::Type(Cow::Borrowed("a")),
            Selector::Class(Cow::Borrowed("btn")),
            Selector::Pseudo(PseudoSelector::Class(Cow::Borrowed("hover"))),
        ]);
        assert_eq!(s.to_string(), "a.btn:hover");
    }

    #[test]
    fn display_descendant() {
        let s = Selector::Descendant(
            Box::new(Selector::Class(Cow::Borrowed("a"))),
            Box::new(Selector::Class(Cow::Borrowed("b"))),
        );
        assert_eq!(s.to_string(), ".a .b");
    }

    #[test]
    fn display_child() {
        let s = Selector::Child(
            Box::new(Selector::Class(Cow::Borrowed("a"))),
            Box::new(Selector::Class(Cow::Borrowed("b"))),
        );
        assert_eq!(s.to_string(), ".a > .b");
    }

    #[test]
    fn display_sibling() {
        let s = Selector::Sibling(
            Box::new(Selector::Class(Cow::Borrowed("a"))),
            Box::new(Selector::Class(Cow::Borrowed("b"))),
        );
        assert_eq!(s.to_string(), ".a + .b");
    }

    #[test]
    fn display_general_sibling() {
        let s = Selector::GeneralSibling(
            Box::new(Selector::Class(Cow::Borrowed("a"))),
            Box::new(Selector::Class(Cow::Borrowed("b"))),
        );
        assert_eq!(s.to_string(), ".a ~ .b");
    }

    #[test]
    fn display_nesting_ref() {
        assert_eq!(Selector::NestingRef.to_string(), "&");
    }

    #[test]
    fn class_constructor() {
        assert_eq!(Selector::class("btn").to_string(), ".btn");
    }

    #[test]
    fn id_constructor() {
        assert_eq!(Selector::id("main").to_string(), "#main");
    }

    #[test]
    fn type_constructor() {
        assert_eq!(Selector::type_("div").to_string(), "div");
    }

    #[test]
    fn pseudo_class_constructor() {
        assert_eq!(Selector::pseudo_class("hover").to_string(), ":hover");
    }

    #[test]
    fn pseudo_element_constructor() {
        assert_eq!(Selector::pseudo_element("before").to_string(), "::before");
    }

    #[test]
    fn universal_constructor() {
        assert_eq!(Selector::universal().to_string(), "*");
    }

    #[test]
    fn nesting_ref_constructor() {
        assert_eq!(Selector::nesting_ref().to_string(), "&");
    }

    #[test]
    fn display_raw() {
        let s = Selector::Raw(Cow::Borrowed(".btn.primary:hover"));
        assert_eq!(s.to_string(), ".btn.primary:hover");
    }

    #[test]
    fn raw_constructor() {
        assert_eq!(Selector::raw(".a.b").to_string(), ".a.b");
    }

    #[test]
    fn selector_fn() {
        assert_eq!(selector("[data-x=\"foo\" i]").to_string(), "[data-x=\"foo\" i]");
    }

    #[test]
    fn equality() {
        assert_eq!(Selector::Class(Cow::Borrowed("a")), Selector::Class(Cow::Borrowed("a")));
        assert_ne!(Selector::Class(Cow::Borrowed("a")), Selector::Class(Cow::Borrowed("b")));
    }

    #[test]
    fn clone_selector() {
        let s = Selector::class("a");
        let s2 = s.clone();
        assert_eq!(s, s2);
    }

    #[test]
    fn debug_format() {
        let s = Selector::class("a");
        let _ = format!("{:?}", s);
    }

    // ── Coverage: every AttrOp and PseudoSelector variant ─────

    #[test]
    fn display_attribute_bare() {
        let s = Selector::AttributeBare(Cow::Borrowed("disabled"));
        assert_eq!(s.to_string(), "[disabled]");
    }

    #[test]
    fn display_attribute_ends_with() {
        let s = Selector::Attribute {
            name: Cow::Borrowed("a"),
            op: AttrOp::EndsWith,
            value: Cow::Borrowed("foo"),
            case: AttrCase::Sensitive,
        };
        assert_eq!(s.to_string(), "[a$=\"foo\"]");
    }

    #[test]
    fn display_attribute_contains() {
        let s = Selector::Attribute {
            name: Cow::Borrowed("a"),
            op: AttrOp::Contains,
            value: Cow::Borrowed("foo"),
            case: AttrCase::Sensitive,
        };
        assert_eq!(s.to_string(), "[a*=\"foo\"]");
    }

    #[test]
    fn display_attribute_includes() {
        let s = Selector::Attribute {
            name: Cow::Borrowed("a"),
            op: AttrOp::Includes,
            value: Cow::Borrowed("foo"),
            case: AttrCase::Sensitive,
        };
        assert_eq!(s.to_string(), "[a~=\"foo\"]");
    }

    #[test]
    fn display_attribute_dash_match() {
        let s = Selector::Attribute {
            name: Cow::Borrowed("a"),
            op: AttrOp::DashMatch,
            value: Cow::Borrowed("foo"),
            case: AttrCase::Sensitive,
        };
        assert_eq!(s.to_string(), "[a|=\"foo\"]");
    }

    #[test]
    fn display_pseudo_id() {
        let s = Selector::Pseudo(PseudoSelector::Id(Cow::Borrowed("main")));
        assert_eq!(s.to_string(), ":main");
    }

    #[test]
    fn display_pseudo_function() {
        let s = Selector::Pseudo(PseudoSelector::Function {
            name: Cow::Borrowed("has"),
            args: vec![SelectorArg::Ident(Cow::Borrowed("hi"))],
        });
        assert_eq!(s.to_string(), ":has(hi)");
    }

    #[test]
    fn display_pseudo_function_multiple_args() {
        let s = Selector::Pseudo(PseudoSelector::Function {
            name: Cow::Borrowed("is"),
            args: vec![
                SelectorArg::Ident(Cow::Borrowed("a")),
                SelectorArg::Ident(Cow::Borrowed("b")),
            ],
        });
        assert_eq!(s.to_string(), ":is(a, b)");
    }

    #[test]
    fn display_pseudo_lang() {
        let s = Selector::Pseudo(PseudoSelector::Lang(Cow::Borrowed("en")));
        assert_eq!(s.to_string(), ":lang(en)");
    }

    #[test]
    fn display_pseudo_dir() {
        let s = Selector::Pseudo(PseudoSelector::Dir(Cow::Borrowed("ltr")));
        assert_eq!(s.to_string(), ":dir(ltr)");
    }

    #[test]
    fn display_pseudo_is() {
        let s = Selector::Pseudo(PseudoSelector::Is(vec![Selector::Class(Cow::Borrowed("a"))]));
        assert_eq!(s.to_string(), ":is(.a)");
    }

    #[test]
    fn display_pseudo_where() {
        let s = Selector::Pseudo(PseudoSelector::Where(vec![Selector::Class(Cow::Borrowed("a"))]));
        assert_eq!(s.to_string(), ":where(.a)");
    }

    #[test]
    fn display_pseudo_has() {
        let s = Selector::Pseudo(PseudoSelector::Has(vec![Selector::Class(Cow::Borrowed("a"))]));
        assert_eq!(s.to_string(), ":has(.a)");
    }

    #[test]
    fn display_pseudo_nth_last_child() {
        let s = Selector::Pseudo(PseudoSelector::NthLastChild(Cow::Borrowed("2n+1")));
        assert_eq!(s.to_string(), ":nth-last-child(2n+1)");
    }

    #[test]
    fn display_pseudo_nth_of_type() {
        let s = Selector::Pseudo(PseudoSelector::NthOfType(Cow::Borrowed("2n+1")));
        assert_eq!(s.to_string(), ":nth-of-type(2n+1)");
    }

    #[test]
    fn display_pseudo_nth_last_of_type() {
        let s = Selector::Pseudo(PseudoSelector::NthLastOfType(Cow::Borrowed("2n+1")));
        assert_eq!(s.to_string(), ":nth-last-of-type(2n+1)");
    }

    #[test]
    fn display_selector_arg_selectors() {
        // Direct test for SelectorArg::Selectors via Display
        let arg = SelectorArg::Selectors(vec![Selector::Class(Cow::Borrowed("a")), Selector::Class(Cow::Borrowed("b"))]);
        assert_eq!(arg.to_string(), ".a, .b");
    }

    #[test]
    fn display_selector_arg_an_plus_b() {
        // Direct test for SelectorArg::AnPlusB via Display
        let arg = SelectorArg::AnPlusB(Cow::Borrowed("2n+1"));
        assert_eq!(arg.to_string(), "2n+1");
    }

    #[test]
    fn display_selector_arg_ident() {
        let arg = SelectorArg::Ident(Cow::Borrowed("en"));
        assert_eq!(arg.to_string(), "en");
    }

    #[test]
    fn display_selector_arg_single_selector() {
        // sels_selector_string with one element — i > 0 branch not hit.
        let arg = SelectorArg::Selectors(vec![Selector::Class(Cow::Borrowed("only"))]);
        assert_eq!(arg.to_string(), ".only");
    }
}