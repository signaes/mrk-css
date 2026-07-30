# mrk-css

Type-safe CSS authoring for Rust — the standalone home of what used to
be the `css` module of [`mrk`](https://github.com/signaes/mrk).

- Fluent builders: `StyleSheet`, `Rule`, every standard at-rule,
  typed selectors, declarations, and value types.
- The CSS Color 4 parser and conversions (sRGB, HSL, OKLab, OKLCH,
  hex) with out-of-gamut chroma reduction.
- A canonical pretty-printer.
- The `css!` macro: CSS-like syntax compiled at macro-expansion time,
  with `{ expr }` interpolation.

```rust
use mrk_css::{css, Renderable};

let sheet = css! {
    :root { --brand: rebeccapurple; }
    .btn {
        color: var(--brand);
        padding: 8px 16px;
        &:hover { color: blue; }
    }
    @media (min-width: 800px) {
        .btn { padding: 16px 32px; }
    }
};

let css = sheet.render();
```

`mrk-css` depends on `mrk` only for the `Renderable` trait and the
`StyleSheet → Node` conversion, so a stylesheet can be embedded in a
markup tree.

## License

MIT — see [LICENSE-MIT](LICENSE-MIT).
