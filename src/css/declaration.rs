//! CSS declarations: [`Declaration`] and [`DeclarationBlock`].
//!
//! A declaration is a `name: value;` pair, optionally marked
//! `!important`.  A [`DeclarationBlock`] is an ordered collection
//! of declarations used inside rules and at-rules.

use std::borrow::Cow;
use std::fmt;

use crate::css::properties::Value;

/// A single CSS declaration: `name: value` (with optional `!important`).
#[derive(Debug, Clone)]
pub struct Declaration {
    /// Property name (e.g. `"color"`, `"--my-var"`).
    pub name: Cow<'static, str>,
    /// Typed value.
    pub value: Value,
    /// Whether this declaration is marked `!important`.
    pub important: bool,
}

impl Declaration {
    /// Create a new declaration.
    pub fn new(name: impl Into<Cow<'static, str>>, value: Value) -> Self {
        Declaration {
            name: name.into(),
            value,
            important: false,
        }
    }

    /// Mark this declaration as `!important`.
    pub fn important(mut self) -> Self {
        self.important = true;
        self
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.important {
            write!(f, "{}: {} !important;", self.name, self.value)
        } else {
            write!(f, "{}: {};", self.name, self.value)
        }
    }
}

/// An ordered collection of declarations inside a rule or at-rule.
#[derive(Debug, Clone, Default)]
pub struct DeclarationBlock {
    decls: Vec<Declaration>,
}

impl DeclarationBlock {
    /// Create an empty block.
    pub fn new() -> Self {
        DeclarationBlock { decls: Vec::new() }
    }

    /// Add a single declaration.
    pub fn decl(mut self, d: Declaration) -> Self {
        self.decls.push(d);
        self
    }

    /// Add multiple declarations.
    #[allow(dead_code)]
    pub fn decls(mut self, ds: impl IntoIterator<Item = Declaration>) -> Self {
        self.decls.extend(ds);
        self
    }

    /// Append all declarations from another block.
    #[allow(dead_code)]
    pub fn extend_from_block(mut self, other: &DeclarationBlock) -> Self {
        self.decls.extend(other.decls.iter().cloned());
        self
    }

    /// Access the declarations.
    #[allow(dead_code)]
    pub fn declarations(&self) -> &[Declaration] {
        &self.decls
    }

    /// Consume into the inner vec.
    pub fn into_declarations(self) -> Vec<Declaration> {
        self.decls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::values::Color;

    #[test]
    fn declaration_new() {
        let d = Declaration::new("color", Value::Color(Color::named("red")));
        assert_eq!(d.name, "color");
        assert!(!d.important);
    }

    #[test]
    fn declaration_important() {
        let d = Declaration::new("color", Value::Color(Color::named("red"))).important();
        assert!(d.important);
    }

    #[test]
    fn declaration_display() {
        let d = Declaration::new("color", Value::Color(Color::named("red")));
        assert_eq!(d.to_string(), "color: red;");
    }

    #[test]
    fn declaration_display_important() {
        let d = Declaration::new("color", Value::Color(Color::named("red"))).important();
        assert_eq!(d.to_string(), "color: red !important;");
    }

    #[test]
    fn declaration_block_new() {
        let b = DeclarationBlock::new();
        assert!(b.declarations().is_empty());
    }

    #[test]
    fn declaration_block_decl() {
        let b = DeclarationBlock::new()
            .decl(Declaration::new("color", Value::Color(Color::named("red"))));
        assert_eq!(b.declarations().len(), 1);
    }

    #[test]
    fn declaration_block_decls() {
        let b = DeclarationBlock::new().decls(vec![
            Declaration::new("color", Value::Color(Color::named("red"))),
            Declaration::new("margin", Value::Length(crate::css::values::Length::px(0.0))),
        ]);
        assert_eq!(b.declarations().len(), 2);
    }

    #[test]
    fn declaration_block_extend() {
        let b1 = DeclarationBlock::new()
            .decl(Declaration::new("color", Value::Color(Color::named("red"))));
        let b2 = DeclarationBlock::new().extend_from_block(&b1);
        assert_eq!(b2.declarations().len(), 1);
    }

    #[test]
    fn declaration_block_into_declarations() {
        let b = DeclarationBlock::new()
            .decl(Declaration::new("color", Value::Color(Color::named("red"))));
        assert_eq!(b.into_declarations().len(), 1);
    }

    #[test]
    fn declaration_block_default_is_empty() {
        let b: DeclarationBlock = Default::default();
        assert!(b.declarations().is_empty());
    }

    #[test]
    fn declaration_block_decls_extend() {
        let b = DeclarationBlock::new()
            .decl(Declaration::new("a", Value::Color(Color::named("red"))))
            .decls([Declaration::new("b", Value::Color(Color::named("blue")))]);
        assert_eq!(b.declarations().len(), 2);
    }

    #[test]
    fn declaration_block_extend_from_block() {
        let a = DeclarationBlock::new()
            .decl(Declaration::new("a", Value::Color(Color::named("red"))));
        let b = DeclarationBlock::new()
            .decl(Declaration::new("b", Value::Color(Color::named("blue"))))
            .extend_from_block(&a);
        assert_eq!(b.declarations().len(), 2);
    }

    #[test]
    fn declaration_block_declarations_accessor() {
        let b = DeclarationBlock::new()
            .decl(Declaration::new("color", Value::Color(Color::named("red"))));
        assert_eq!(b.declarations().len(), 1);
    }
}
