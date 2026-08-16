//! `Url` — a CSS `url(...)` value.

use std::borrow::Cow;
use std::fmt;

/// The kind of URL a [`Url`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlKind {
    /// A relative or absolute path or fragment (`url("...")`).
    Local,
    /// An absolute URL (`url("https://...")`).
    Absolute,
    /// A `data:` URL (`url("data:image/png;base64,...")`).
    Data,
}

/// A CSS `url(...)` value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    pub(crate) raw: Cow<'static, str>,
    pub(crate) kind: UrlKind,
}

impl Url {
    /// Construct a local URL (relative path).
    pub fn local(s: impl Into<Cow<'static, str>>) -> Self {
        Url {
            raw: s.into(),
            kind: UrlKind::Local,
        }
    }

    /// Construct an absolute URL.
    pub fn absolute(s: impl Into<Cow<'static, str>>) -> Self {
        Url {
            raw: s.into(),
            kind: UrlKind::Absolute,
        }
    }

    /// Construct a `data:` URL.
    pub fn data(s: impl Into<Cow<'static, str>>) -> Self {
        Url {
            raw: s.into(),
            kind: UrlKind::Data,
        }
    }

    /// Return the raw URL string.
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Return the URL kind.
    pub fn kind(&self) -> UrlKind {
        self.kind
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Escape characters that would break the double-quoted URL
        // syntax: backslash first, then double quotes.
        let escaped = self.raw.replace('\\', "\\\\").replace('"', "\\\"");
        write!(f, "url(\"{}\")", escaped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_borrowed() {
        let u = Url::local("foo.png");
        assert_eq!(u.raw(), "foo.png");
        assert_eq!(u.kind(), UrlKind::Local);
    }

    #[test]
    fn local_owned() {
        let owned = String::from("bar.png");
        let u = Url::local(owned);
        assert_eq!(u.raw(), "bar.png");
        assert_eq!(u.kind(), UrlKind::Local);
    }

    #[test]
    fn absolute() {
        let u = Url::absolute("https://example.com/x.png");
        assert_eq!(u.kind(), UrlKind::Absolute);
    }

    #[test]
    fn data() {
        let u = Url::data("data:image/png;base64,AAAA");
        assert_eq!(u.kind(), UrlKind::Data);
    }

    #[test]
    fn display_local() {
        assert_eq!(Url::local("foo.png").to_string(), "url(\"foo.png\")");
    }

    #[test]
    fn display_absolute() {
        assert_eq!(
            Url::absolute("https://example.com/x.png").to_string(),
            "url(\"https://example.com/x.png\")"
        );
    }

    #[test]
    fn display_data() {
        assert_eq!(
            Url::data("data:image/png;base64,AAAA").to_string(),
            "url(\"data:image/png;base64,AAAA\")"
        );
    }

    #[test]
    fn display_with_quotes_escaped() {
        let cases: [(&str, &str); 4] = [
            ("a\"b", "url(\"a\\\"b\")"),
            ("a\\b", "url(\"a\\\\b\")"),
            ("a\\\"b", "url(\"a\\\\\\\"b\")"),
            ("plain.png", "url(\"plain.png\")"),
        ];
        for (input, expected) in cases {
            assert_eq!(Url::local(input).to_string(), expected);
        }
    }

    #[test]
    fn equality_borrowed() {
        assert_eq!(Url::local("a"), Url::local("a"));
        assert_ne!(Url::local("a"), Url::local("b"));
    }

    #[test]
    fn equality_kinds() {
        let a = Url::local("foo");
        let b = Url::absolute("foo");
        assert_ne!(a, b);
    }

    #[test]
    fn clone() {
        let u = Url::local("x");
        let u2 = u.clone();
        assert_eq!(u, u2);
    }

    #[test]
    fn debug_format() {
        let u = Url::local("foo");
        let _ = format!("{:?}", u);
    }
}
