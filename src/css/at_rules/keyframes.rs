//! `@keyframes` at-rule rendering.

use std::fmt;

use super::Keyframe;

/// Render a `@keyframes name { ... }` block.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    name: &str,
    keyframes: &[Keyframe],
) -> fmt::Result {
    let mut s = format!("@keyframes {} {{", name);
    for kf in keyframes {
        s.push_str(&format!("\n  {}", kf));
    }
    if !keyframes.is_empty() { s.push('\n'); }
    s.push('}');
    f.write_str(&s)
}

/// Render an individual keyframe block.
pub fn render_keyframe(f: &mut fmt::Formatter<'_>, kf: &Keyframe) -> fmt::Result {
    let mut s = String::new();
    for (i, sel) in kf.selectors.iter().enumerate() {
        if i > 0 { s.push_str(", "); }
        s.push_str(sel);
    }
    s.push_str(" {");
    for d in &kf.declarations {
        s.push_str(&format!("\n    {}", d));
    }
    if !kf.declarations.is_empty() { s.push_str("\n  "); }
    s.push('}');
    f.write_str(&s)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::css::Declaration;
    use crate::css::properties::Value;
    use crate::css::values::Number;

    #[test]
    fn render_keyframe_with_declarations() {
        let kf = Keyframe {
            selectors: vec![Cow::Borrowed("from")],
            declarations: vec![Declaration::new("opacity", Value::Number(Number::from(0.0)))],
        };
        let s = kf.to_string();
        assert!(s.contains("from"));
        assert!(s.contains("opacity"));
    }

    #[test]
    fn render_keyframe_empty_declarations() {
        let kf = Keyframe {
            selectors: vec![Cow::Borrowed("from")],
            declarations: vec![],
        };
        let s = kf.to_string();
        assert!(s.contains("from"));
        assert!(!s.contains("opacity"));
    }

    #[test]
    fn render_keyframe_with_multiple_declarations() {
        let kf = Keyframe {
            selectors: vec![Cow::Borrowed("from")],
            declarations: vec![
                Declaration::new("opacity", Value::Number(Number::from(0.0))),
                Declaration::new("transform", Value::String("translateX(0)".into())),
            ],
        };
        let s = kf.to_string();
        assert!(s.contains("from"));
        assert!(s.contains("opacity"));
    }
}
