//! # mrk-css
//!
//! Type-safe CSS authoring for Rust: [`StyleSheet`](css::StyleSheet),
//! rules, at-rules, selectors, declarations, value types, the CSS
//! Color 4 parser and conversions, the pretty-printer, and the
//! [`css!`](crate::css!) macro.
//!
//! This crate is the standalone home of what used to be the `css`
//! module of the [`mrk`](https://github.com/signaes/mrk) crate. See
//! the [module documentation](mod@css) for the full overview.
//!
//! [`Renderable`] is re-exported from `mrk` for convenience, so the
//! familiar idiom keeps working:
//!
//! ```
//! use mrk_css::{css, Renderable};
//!
//! let sheet = css! { .btn { color: blue; } };
//! assert!(sheet.render().contains("color: rgb(0, 0, 255);"));
//! ```

#![deny(missing_docs)]
// The `css!` macro recognizes structure with token-munching helper
// macros that cost one recursion frame per token; lift the default
// limit. Downstream crates compiling very large stylesheets may need
// to raise their own limit.
#![recursion_limit = "256"]

pub mod css;

#[doc(inline)]
pub use mrk::Renderable;
