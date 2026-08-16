//! `Ident` — a CSS identifier (keyword, custom-ident).
//!
//! Full implementation lands with Phase 1.9.

/// A CSS identifier: a keyword or custom-ident such as `auto`, `solid`,
/// or a user-defined name.
///
/// The text is stored internally; read it back via [`Display`](std::fmt::Display)
/// (`ident.to_string()`).
pub struct Ident(pub(crate) std::borrow::Cow<'static, str>);

impl Ident {
    /// Construct from a borrowed static string.
    pub fn new(s: &'static str) -> Self {
        Ident(std::borrow::Cow::Borrowed(s))
    }
}

impl From<&'static str> for Ident {
    fn from(s: &'static str) -> Self {
        Ident(std::borrow::Cow::Borrowed(s))
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Ident(std::borrow::Cow::Owned(s))
    }
}

impl Clone for Ident {
    fn clone(&self) -> Self {
        match &self.0 {
            std::borrow::Cow::Borrowed(s) => Ident(std::borrow::Cow::Borrowed(s)),
            std::borrow::Cow::Owned(s) => Ident(std::borrow::Cow::Owned(s.clone())),
        }
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}

impl Eq for Ident {}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ident").field(&self.0).finish()
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_borrowed() {
        let i = Ident::new("red");
        assert_eq!(i.to_string(), "red");
    }

    #[test]
    fn from_static_str() {
        let i = Ident::from("red");
        assert_eq!(i.to_string(), "red");
    }

    #[test]
    fn from_owned_string() {
        let i = Ident::from(String::from("dyn"));
        assert_eq!(i.to_string(), "dyn");
    }

    #[test]
    fn display_borrowed() {
        assert_eq!(Ident::from("red").to_string(), "red");
    }

    #[test]
    fn display_owned() {
        assert_eq!(Ident::from(String::from("dyn")).to_string(), "dyn");
    }

    #[test]
    fn equality_borrowed() {
        assert_eq!(Ident::from("a"), Ident::from("a"));
        assert_ne!(Ident::from("a"), Ident::from("b"));
    }

    #[test]
    fn equality_owned() {
        let a = Ident::from(String::from("x"));
        let b = Ident::from(String::from("x"));
        assert_eq!(a, b);
    }

    #[test]
    fn clone_borrowed() {
        let i = Ident::from("red");
        let j = i.clone();
        assert_eq!(i, j);
    }

    #[test]
    fn clone_owned() {
        let i = Ident::from(String::from("dyn"));
        let j = i.clone();
        assert_eq!(i, j);
        // Ensure cloned owned holds its own buffer.
        assert_eq!(j.0.as_ref(), "dyn");
    }

    #[test]
    fn debug_format() {
        let i = Ident::from("red");
        let s = format!("{:?}", i);
        assert!(s.contains("Ident"));
        assert!(s.contains("red"));
    }

    #[test]
    fn empty_string() {
        let i = Ident::from("");
        assert_eq!(i.to_string(), "");
    }

    #[test]
    fn unicode() {
        let i = Ident::from("café");
        assert_eq!(i.to_string(), "café");
    }
}
