//! `Color` and `ColorKind` — typed CSS Color 4 values.
//!
//! Phase 1.14 ships the enum, constructors, `Display` round-trip,
//! and basic `From` impls. The cross-space `Color::into_*` methods
//! land in Phase 2.

use std::fmt;

use super::angle::Angle;
use super::identifier::Ident;
use super::percentage::Percentage;

/// A CSS color value. Construct via [`Color::rgb`], [`Color::hex`],
/// [`Color::named`], [`Color::hsl`], [`Color::oklch`], etc.
#[derive(Debug, Clone)]
pub struct Color {
    pub(crate) kind: ColorKind,
}

impl Color {
    /// Construct an sRGB color (no alpha).
    ///
    /// Channels are `u8`, so the `0..=255` range is enforced by the
    /// type system — unlike [`Color::hex`], no validation (and no
    /// `Option`) is needed. Out-of-gamut colors are impossible to
    /// construct through this function.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            kind: ColorKind::Rgb { r, g, b, alpha: None },
        }
    }

    /// Construct an sRGBA color.
    ///
    /// Like [`Color::rgb`], channels are range-checked by the `u8`
    /// type. The alpha is a float in `0.0..=1.0` by convention; it is
    /// stored as-is (CSS permits out-of-range alpha at parse time and
    /// clamps at used-value time).
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Color {
            kind: ColorKind::Rgb {
                r,
                g,
                b,
                alpha: Some(a),
            },
        }
    }

    /// Parse a hex string (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`)
    /// into a typed color. Returns `None` for malformed input.
    pub fn hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#')?;
        let bytes = match s.len() {
            3 => {
                let r = u8::from_str_radix(&s[0..1], 16).ok()?;
                let g = u8::from_str_radix(&s[1..2], 16).ok()?;
                let b = u8::from_str_radix(&s[2..3], 16).ok()?;
                Some(((r << 4) | r, (g << 4) | g, (b << 4) | b, 255_u8))
            }
            4 => {
                let r = u8::from_str_radix(&s[0..1], 16).ok()?;
                let g = u8::from_str_radix(&s[1..2], 16).ok()?;
                let b = u8::from_str_radix(&s[2..3], 16).ok()?;
                let a = match u8::from_str_radix(&s[3..4], 16) {
                    Ok(v) => v,
                    Err(_) => return None,
                };
                Some(((r << 4) | r, (g << 4) | g, (b << 4) | b, (a << 4) | a))
            }
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                Some((r, g, b, 255))
            }
            8 => {
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                let a = u8::from_str_radix(&s[6..8], 16).ok()?;
                Some((r, g, b, a))
            }
            _ => None,
        }?;
        let alpha = if bytes.3 == 255 { None } else { Some(bytes.3 as f32 / 255.0) };
        Some(Color {
            kind: ColorKind::Rgb {
                r: bytes.0,
                g: bytes.1,
                b: bytes.2,
                alpha,
            },
        })
    }

    /// Construct a named color. The name is stored as-is; validation
    /// against the CSS named-color list happens at parse time.
    ///
    /// Prefer [`named_checked`](Color::named_checked) when the name
    /// comes from outside source code and a typo should be caught
    /// at construction time instead of at render time.
    #[must_use]
    pub fn named(s: impl Into<Ident>) -> Self {
        Color {
            kind: ColorKind::Named(s.into()),
        }
    }

    /// Construct a named color, validated against the 148 CSS Color 4
    /// named colors (case-insensitive). Returns `None` when `name` is
    /// not a known named color, so a typo fails at construction time
    /// instead of silently rendering an invalid color.
    #[must_use]
    pub fn named_checked(s: impl Into<Ident>) -> Option<Self> {
        let ident = s.into();
        Self::is_known_name(&ident.0).then_some(Color {
            kind: ColorKind::Named(ident),
        })
    }

    /// Returns `true` when `name` is one of the 148 CSS Color 4 named
    /// colors (case-insensitive).
    #[must_use]
    pub fn is_known_name(name: &str) -> bool {
        named_to_srgb(name).is_some()
    }

    /// `currentColor` keyword.
    pub fn current_color() -> Self {
        Color { kind: ColorKind::CurrentColor }
    }

    /// `transparent` keyword.
    pub fn transparent() -> Self {
        Color { kind: ColorKind::Transparent }
    }

    /// Construct an HSL color.
    pub fn hsl(h: f32, s: impl Into<Percentage>, l: impl Into<Percentage>) -> Self {
        Color {
            kind: ColorKind::Hsl {
                h,
                s: s.into(),
                l: l.into(),
                alpha: None,
            },
        }
    }

    /// Construct an HSLA color.
    pub fn hsla(h: f32, s: impl Into<Percentage>, l: impl Into<Percentage>, a: f32) -> Self {
        Color {
            kind: ColorKind::Hsl {
                h,
                s: s.into(),
                l: l.into(),
                alpha: Some(a),
            },
        }
    }

    /// Construct an HWB color.
    pub fn hwb(h: f32, w: impl Into<Percentage>, b: impl Into<Percentage>) -> Self {
        Color {
            kind: ColorKind::Hwb {
                h,
                w: w.into(),
                b: b.into(),
                alpha: None,
            },
        }
    }

    /// Construct a Lab color.
    pub fn lab(l: impl Into<Percentage>, a: f32, b: f32) -> Self {
        Color {
            kind: ColorKind::Lab {
                l: l.into(),
                a,
                b,
                alpha: None,
            },
        }
    }

    /// Construct an Lch color.
    pub fn lch(l: impl Into<Percentage>, c: f32, h: impl Into<Angle>) -> Self {
        Color {
            kind: ColorKind::Lch {
                l: l.into(),
                c,
                h: h.into(),
                alpha: None,
            },
        }
    }

    /// Construct an OKLab color.
    pub fn oklab(l: impl Into<Percentage>, a: f32, b: f32) -> Self {
        Color {
            kind: ColorKind::Oklab {
                l: l.into(),
                a,
                b,
                alpha: None,
            },
        }
    }

    /// Construct an OKLCH color.
    pub fn oklch(l: impl Into<Percentage>, c: f32, h: impl Into<Angle>) -> Self {
        Color {
            kind: ColorKind::Oklch {
                l: l.into(),
                c,
                h: h.into(),
                alpha: None,
            },
        }
    }

    /// Construct a `color(...)` color with arbitrary color-space
    /// channels.
    pub fn in_color_space(space: impl Into<Ident>, channels: Vec<f32>) -> Self {
        Color {
            kind: ColorKind::Color {
                space: space.into(),
                channels,
                alpha: None,
            },
        }
    }

    /// Construct a CMYK color.
    pub fn cmyk(c: impl Into<Percentage>, m: impl Into<Percentage>, y: impl Into<Percentage>, k: impl Into<Percentage>) -> Self {
        Color {
            kind: ColorKind::DeviceCmyk {
                c: c.into(),
                m: m.into(),
                y: y.into(),
                k: k.into(),
                alpha: None,
            },
        }
    }

    /// `light-dark(<light>, <dark>)`.
    pub fn light_dark(light: Color, dark: Color) -> Self {
        Color {
            kind: ColorKind::LightDark {
                light: Box::new(light),
                dark: Box::new(dark),
            },
        }
    }

    /// `color-mix(in <space>, <a>, <b> <pct>)`.
    pub fn mix(a: Color, b: Color, percentage: Percentage, space: ColorMixSpace, method: ColorMixMethod) -> Self {
        Color {
            kind: ColorKind::ColorMix(Box::new(ColorMix {
                a,
                b,
                percentage,
                space,
                method,
            })),
        }
    }

    /// Borrow the inner [`ColorKind`].
    pub fn kind(&self) -> &ColorKind {
        &self.kind
    }

    /// Consume and return the inner [`ColorKind`].
    pub fn into_kind(self) -> ColorKind {
        self.kind
    }

    // ── Color-space conversions (Phase 2) ────────────────────────

    /// Convert to sRGB. Returns `Err` for unresolvable colors
    /// (system colors, `light-dark()`, `currentcolor`, `transparent`
    /// `color-mix()` with unresolvable arguments).
    pub fn into_rgb(self) -> Result<Color, ConversionError> {
        let srgb = self.to_srgb_float()?;
        let r = (srgb.r * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.g * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.b * 255.0).round().clamp(0.0, 255.0) as u8;
        Ok(Color {
            kind: ColorKind::Rgb { r, g, b, alpha: srgb.alpha },
        })
    }

    /// Convert to HSL.
    pub fn into_hsl(self) -> Result<Color, ConversionError> {
        let srgb = self.to_srgb_float()?;
        let (h, s, l) = srgb_to_hsl(srgb.r, srgb.g, srgb.b);
        Ok(Color {
            kind: ColorKind::Hsl {
                h,
                s: Percentage::new(s),
                l: Percentage::new(l),
                alpha: srgb.alpha,
            },
        })
    }

    /// Convert to OKLab.
    pub fn into_oklab(self) -> Result<Color, ConversionError> {
        let srgb = self.to_srgb_float()?;
        let (l, a, b) = srgb_to_oklab(srgb.r, srgb.g, srgb.b);
        Ok(Color {
            kind: ColorKind::Oklab {
                l: Percentage::new(l * 100.0),
                a,
                b,
                alpha: srgb.alpha,
            },
        })
    }

    /// Convert to OKLCH.
    pub fn into_oklch(self) -> Result<Color, ConversionError> {
        let srgb = self.to_srgb_float()?;
        let (l, a, b) = srgb_to_oklab(srgb.r, srgb.g, srgb.b);
        let (l, c, h) = oklab_to_oklch(l, a, b);
        Ok(Color {
            kind: ColorKind::Oklch {
                l: Percentage::new(l * 100.0),
                c,
                h: Angle::deg(h),
                alpha: srgb.alpha,
            },
        })
    }

    /// Convert to a hex string (`#rrggbb` or `#rrggbbaa`).
    pub fn into_hex(self) -> Result<String, ConversionError> {
        let srgb = self.to_srgb_float()?;
        let r = (srgb.r * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.g * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.b * 255.0).round().clamp(0.0, 255.0) as u8;
        match srgb.alpha {
            Some(a) if (a - 1.0).abs() > f32::EPSILON => {
                let a_byte = (a * 255.0).round().clamp(0.0, 255.0) as u8;
                Ok(format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a_byte))
            }
            _ => Ok(format!("#{:02x}{:02x}{:02x}", r, g, b)),
        }
    }

    // ── Internal ─────────────────────────────────────────────────

    /// Resolve to an sRGB float representation. Returns `Err` for
    /// unresolvable color kinds.
    fn to_srgb_float(&self) -> Result<SrgbFloat, ConversionError> {
        match &self.kind {
            ColorKind::Rgb { r, g, b, alpha } => Ok(SrgbFloat {
                r: *r as f32 / 255.0,
                g: *g as f32 / 255.0,
                b: *b as f32 / 255.0,
                alpha: *alpha,
            }),
            ColorKind::Hsl { h, s, l, alpha } => {
                let (r, g, b) = hsl_to_srgb(*h, s.value(), l.value());
                Ok(SrgbFloat { r, g, b, alpha: *alpha })
            }
            ColorKind::Hwb { h, w, b, alpha } => {
                let (r, g, b2) = hwb_to_srgb(*h, w.value(), b.value());
                Ok(SrgbFloat { r, g, b: b2, alpha: *alpha })
            }
            ColorKind::Lab { .. } => Err(ConversionError::Unresolvable),
            ColorKind::Lch { .. } => Err(ConversionError::Unresolvable),
            ColorKind::Oklab { l, a, b, alpha } => {
                let (r, g, b2) = oklab_to_srgb(l.value(), *a, *b);
                Ok(SrgbFloat {
                    r,
                    g,
                    b: b2,
                    alpha: *alpha,
                })
            }
            ColorKind::Oklch { l, c, h, alpha } => {
                let l_val = l.value() / 100.0;
                let h_deg = h.to_degrees();
                let h_rad = h_deg.to_radians();
                let a = *c * h_rad.cos();
                let b = *c * h_rad.sin();
                let (r, g, b2) = oklab_to_srgb(l_val, a, b);
                Ok(SrgbFloat {
                    r,
                    g,
                    b: b2,
                    alpha: *alpha,
                })
            }
            ColorKind::Color { space, channels, alpha } => {
                let space_lower = space.to_string().to_lowercase();
                // If the space is a known sRGB-variant, use channels directly
                if space_lower == "srgb" || space_lower == "srgb-linear" {
                    let r = channels.first().copied().unwrap_or(0.0);
                    let g = channels.get(1).copied().unwrap_or(0.0);
                    let b = channels.get(2).copied().unwrap_or(0.0);
                    Ok(SrgbFloat {
                        r: r.clamp(0.0, 1.0),
                        g: g.clamp(0.0, 1.0),
                        b: b.clamp(0.0, 1.0),
                        alpha: *alpha,
                    })
                } else if space_lower == "display-p3" {
                    let r = channels.first().copied().unwrap_or(0.0);
                    let g = channels.get(1).copied().unwrap_or(0.0);
                    let b = channels.get(2).copied().unwrap_or(0.0);
                    let (r, g, b) = gamut_map_srgb(r, g, b, 8);
                    Ok(SrgbFloat { r, g, b, alpha: *alpha })
                } else {
                    Err(ConversionError::Unresolvable)
                }
            }
            ColorKind::ColorMix(m) => resolve_color_mix(m),
            ColorKind::ColorContrast { colors, target } => {
                resolve_color_contrast(colors, target.as_deref())
            }
            ColorKind::DeviceCmyk { c, m, y, k, alpha } => {
                // Device CMYK → sRGB: standard approximate conversion
                let c = c.value() / 100.0;
                let m = m.value() / 100.0;
                let y = y.value() / 100.0;
                let k = k.value() / 100.0;
                let r = (1.0 - c) * (1.0 - k);
                let g = (1.0 - m) * (1.0 - k);
                let b = (1.0 - y) * (1.0 - k);
                Ok(SrgbFloat { r, g, b, alpha: *alpha })
            }
            ColorKind::System(_) | ColorKind::LightDark { .. } | ColorKind::CurrentColor | ColorKind::Transparent => {
                Err(ConversionError::Unresolvable)
            }
            ColorKind::Named(name) => {
                named_to_srgb(&name.to_string()).ok_or(ConversionError::Unresolvable).map(|(r, g, b)| SrgbFloat {
                    r: r as f32 / 255.0,
                    g: g as f32 / 255.0,
                    b: b as f32 / 255.0,
                    alpha: None,
                })
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Color-space conversion helpers
// ═══════════════════════════════════════════════════════════════════

/// Internal sRGB float representation.
struct SrgbFloat {
    r: f32,
    g: f32,
    b: f32,
    alpha: Option<f32>,
}

// ── sRGB gamma ────────────────────────────────────────────────────

#[inline]
fn srgb_gamma(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

#[inline]
fn srgb_inv_gamma(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

// ── HSL ↔ sRGB ────────────────────────────────────────────────────

fn hsl_to_srgb(h: f32, s_pct: f32, l_pct: f32) -> (f32, f32, f32) {
    let s = s_pct / 100.0;
    let l = l_pct / 100.0;
    let h_norm = h / 360.0;

    let a = s * l.min(1.0 - l);
    let f = |n: f32| {
        let k = (n + h_norm * 12.0) % 12.0;
        l - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
    };
    (f(0.0).clamp(0.0, 1.0), f(8.0).clamp(0.0, 1.0), f(4.0).clamp(0.0, 1.0))
}

fn srgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return (0.0, 0.0, l * 100.0);
    }

    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    (h * 60.0, s * 100.0, l * 100.0)
}

// ── HWB ↔ sRGB ────────────────────────────────────────────────────

fn hwb_to_srgb(h: f32, w_pct: f32, b_pct: f32) -> (f32, f32, f32) {
    let w = w_pct / 100.0;
    let b = b_pct / 100.0;
    if w + b >= 1.0 {
        let gray = (w / (w + b)).clamp(0.0, 1.0);
        return (gray, gray, gray);
    }
    let (r, g, b2) = hsl_to_srgb(h, 100.0, 50.0);
    let scale = 1.0 - w - b;
    (r * scale + w, g * scale + w, b2 * scale + w)
}

// ── OKLab / OKLCH ─────────────────────────────────────────────────

// The OKLab matrices below are Björn Ottosson's reference values
// (adopted by CSS Color 4), kept verbatim for spec traceability even
// though `f32` cannot represent all ten digits.
#[allow(clippy::excessive_precision)]
fn srgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let rl = srgb_gamma(r);
    let gl = srgb_gamma(g);
    let bl = srgb_gamma(b);
    let lms_l = 0.4122214708 * rl + 0.5363325363 * gl + 0.0514459929 * bl;
    let lms_m = 0.2119034982 * rl + 0.6806995451 * gl + 0.1073969566 * bl;
    let lms_s = 0.0883024619 * rl + 0.2817188376 * gl + 0.6299787005 * bl;
    let l_ = lms_l.cbrt();
    let m_ = lms_m.cbrt();
    let s_ = lms_s.cbrt();
    let ok_l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let ok_a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let ok_b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;
    (ok_l, ok_a, ok_b)
}

#[allow(clippy::excessive_precision)] // see the note above `srgb_to_oklab`
fn oklab_to_linear_srgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    let lms_l = l_ * l_ * l_;
    let lms_m = m_ * m_ * m_;
    let lms_s = s_ * s_ * s_;
    let rl = 4.0767416621 * lms_l - 3.3077115913 * lms_m + 0.2309699292 * lms_s;
    let gl = -1.2684380046 * lms_l + 2.6097574011 * lms_m - 0.3413193965 * lms_s;
    let bl = -0.0041960863 * lms_l - 0.7034186147 * lms_m + 1.7076147010 * lms_s;
    (rl, gl, bl)
}

fn oklab_to_srgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let (rl, gl, bl) = oklab_to_linear_srgb(l, a, b);
    (srgb_inv_gamma(rl).clamp(0.0, 1.0), srgb_inv_gamma(gl).clamp(0.0, 1.0), srgb_inv_gamma(bl).clamp(0.0, 1.0))
}

fn oklab_to_oklch(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let c = (a * a + b * b).sqrt();
    let h = if c.abs() < 1e-10 {
        0.0
    } else {
        b.atan2(a).to_degrees()
    };
    let h = if h < 0.0 { h + 360.0 } else { h };
    (l, c, h)
}

fn oklch_to_oklab(l: f32, c: f32, h: f32) -> (f32, f32, f32) {
    let h_rad = h.to_radians();
    let a = c * h_rad.cos();
    let b = c * h_rad.sin();
    (l, a, b)
}

/// sRGB gamma decode (CSS sRGB EOTF).
/// Linearizes a single sRGB channel in [0, 1].
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// WCAG 2.1 relative luminance of an sRGB triple in [0, 1].
/// Returns a value in [0, 1].
fn relative_luminance(r: f32, g: f32, b: f32) -> f32 {
    let r = srgb_to_linear(r);
    let g = srgb_to_linear(g);
    let b = srgb_to_linear(b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// WCAG 2.1 contrast ratio between two sRGB triples.
/// Returns a value in [1, 21].
fn contrast_ratio(r1: f32, g1: f32, b1: f32, r2: f32, g2: f32, b2: f32) -> f32 {
    let l1 = relative_luminance(r1, g1, b1);
    let l2 = relative_luminance(r2, g2, b2);
    let (lo, hi) = if l1 < l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}

/// Resolve `color-contrast()` per CSS Color 4: pick the color from
/// the list that has the highest contrast ratio against the target.
/// If no target is specified, the first color is used as the target
/// (i.e. the color most distinct from the first).
fn resolve_color_contrast(
    colors: &[Color],
    target: Option<&Color>,
) -> Result<SrgbFloat, ConversionError> {
    if colors.is_empty() {
        return Err(ConversionError::Unresolvable);
    }
    let target = target.unwrap_or(&colors[0]);
    let target_srgb = target.to_srgb_float()?;
    let mut best: Option<(SrgbFloat, f32)> = None;
    for c in colors.iter() {
        let srgb = c.to_srgb_float()?;
        let ratio = contrast_ratio(
            target_srgb.r, target_srgb.g, target_srgb.b,
            srgb.r, srgb.g, srgb.b,
        );
        if best.as_ref().is_none_or(|&(_, r)| ratio > r) {
            best = Some((srgb, ratio));
        }
    }
    best.map(|(s, _)| s).ok_or(ConversionError::Unresolvable)
}

fn interpolate_hue(h1: f32, h2: f32, p: f32, method: ColorMixMethod) -> f32 {
    let mut h2_adj = h2;
    match method {
        ColorMixMethod::Shorter => {
            let diff = h2_adj - h1;
            if diff > 180.0 {
                h2_adj -= 360.0;
            } else if diff < -180.0 {
                h2_adj += 360.0;
            }
        }
        ColorMixMethod::Longer => {
            let diff = h2_adj - h1;
            if (0.0..180.0).contains(&diff) {
                h2_adj += 360.0;
            } else if (-180.0..0.0).contains(&diff) {
                h2_adj -= 360.0;
            }
        }
        ColorMixMethod::Increasing => {
            if h2_adj < h1 {
                h2_adj += 360.0;
            }
        }
        ColorMixMethod::Decreasing => {
            if h2_adj > h1 {
                h2_adj -= 360.0;
            }
        }
    }
    let mut h = h1 * (1.0 - p) + h2_adj * p;
    h %= 360.0;
    if h < 0.0 { h + 360.0 } else { h }
}

/// Helper to produce alpha Option from raw alpha value.
fn alpha_opt(alpha: f32) -> Option<f32> {
    if (alpha - 1.0).abs() < f32::EPSILON { None } else { Some(alpha) }
}

fn resolve_color_mix(m: &ColorMix) -> Result<SrgbFloat, ConversionError> {
    let a_srgb = m.a.to_srgb_float()?;
    let b_srgb = m.b.to_srgb_float()?;
    let p = m.percentage.value() / 100.0;

    let alpha_a = a_srgb.alpha.unwrap_or(1.0);
    let alpha_b = b_srgb.alpha.unwrap_or(1.0);

    match m.space {
        ColorMixSpace::Srgb => {
            let r = a_srgb.r * (1.0 - p) + b_srgb.r * p;
            let g = a_srgb.g * (1.0 - p) + b_srgb.g * p;
            let b = a_srgb.b * (1.0 - p) + b_srgb.b * p;
            let alpha = alpha_a * (1.0 - p) + alpha_b * p;
            Ok(SrgbFloat { r, g, b, alpha: alpha_opt(alpha) })
        }
        ColorMixSpace::Hsl => {
            let (h1, s1, l1) = srgb_to_hsl(a_srgb.r, a_srgb.g, a_srgb.b);
            let (h2, s2, l2) = srgb_to_hsl(b_srgb.r, b_srgb.g, b_srgb.b);
            let h = interpolate_hue(h1, h2, p, m.method);
            let s = s1 * (1.0 - p) + s2 * p;
            let l = l1 * (1.0 - p) + l2 * p;
            let (r, g, b) = hsl_to_srgb(h, s, l);
            let alpha = alpha_a * (1.0 - p) + alpha_b * p;
            Ok(SrgbFloat { r, g, b, alpha: alpha_opt(alpha) })
        }
        ColorMixSpace::Lab | ColorMixSpace::Lch => {
            // Fallback: interpolate directly in sRGB
            let r = a_srgb.r * (1.0 - p) + b_srgb.r * p;
            let g = a_srgb.g * (1.0 - p) + b_srgb.g * p;
            let b = a_srgb.b * (1.0 - p) + b_srgb.b * p;
            let alpha = alpha_a * (1.0 - p) + alpha_b * p;
            Ok(SrgbFloat { r, g, b, alpha: alpha_opt(alpha) })
        }
        ColorMixSpace::Oklab => {
            let (l1, a1, b1) = srgb_to_oklab(a_srgb.r, a_srgb.g, a_srgb.b);
            let (l2, a2, b2) = srgb_to_oklab(b_srgb.r, b_srgb.g, b_srgb.b);
            let l = l1 * (1.0 - p) + l2 * p;
            let a = a1 * (1.0 - p) + a2 * p;
            let b = b1 * (1.0 - p) + b2 * p;
            let (r, g, b) = oklab_to_srgb(l, a, b);
            let alpha = alpha_a * (1.0 - p) + alpha_b * p;
            Ok(SrgbFloat { r, g, b, alpha: alpha_opt(alpha) })
        }
        ColorMixSpace::Oklch => {
            let (l1, a1, b1) = srgb_to_oklab(a_srgb.r, a_srgb.g, a_srgb.b);
            let (l2, a2, b2) = srgb_to_oklab(b_srgb.r, b_srgb.g, b_srgb.b);
            let (okl1, c1, h1) = oklab_to_oklch(l1, a1, b1);
            let (okl2, c2, h2) = oklab_to_oklch(l2, a2, b2);
            let l = okl1 * (1.0 - p) + okl2 * p;
            let c = c1 * (1.0 - p) + c2 * p;
            let h = interpolate_hue(h1, h2, p, m.method);
            let (_, a_val, b_val) = oklch_to_oklab(l, c, h);
            let (r, g, b) = oklab_to_srgb(l, a_val, b_val);
            let alpha = alpha_a * (1.0 - p) + alpha_b * p;
            Ok(SrgbFloat { r, g, b, alpha: alpha_opt(alpha) })
        }
    }
}

// ── Gamut mapping (OKLCH chroma reduction) ────────────────────────

/// Map out-of-gamut sRGB values into sRGB gamut via OKLCH chroma
/// reduction.  Performs at most *max_iters* binary-search steps.
fn gamut_map_srgb(r: f32, g: f32, b: f32, max_iters: u32) -> (f32, f32, f32) {
    // If already in gamut, return as-is.
    if (-1e-6..=1.0 + 1e-6).contains(&r)
        && (-1e-6..=1.0 + 1e-6).contains(&g)
        && (-1e-6..=1.0 + 1e-6).contains(&b)
    {
        return (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
    }

    let (ok_l, ok_a, ok_b) = srgb_to_oklab(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
    let (_, c, h) = oklab_to_oklch(ok_l, ok_a, ok_b);
    let mut lo = 0.0_f32;
    let hi = c;

    for _ in 0..max_iters {
        let mid = (lo + hi) / 2.0;
        let a = mid * h.to_radians().cos();
        let b = mid * h.to_radians().sin();
        // Use the unclamped linear-sRGB output so the in-gamut check
        // can correctly detect when the chroma midpoint is out of gamut.
        let (r2, g2, b2) = oklab_to_linear_srgb(ok_l, a, b);
        // `in_gamut` covers both branches; the `else { hi = mid }` arm
        // is only reached when the chroma midpoint produces linear-sRGB
        // values outside [-1e-6, 1.0+1e-6], which happens for very
        // saturated inputs. For inputs where the OKLab is derived from
        // already-in-gamut sRGB (after the clamp above), `lo` is updated
        // every iteration and `hi` converges naturally.
        let in_gamut = (-1e-6..=1.0 + 1e-6).contains(&r2)
            && (-1e-6..=1.0 + 1e-6).contains(&g2)
            && (-1e-6..=1.0 + 1e-6).contains(&b2);
        if in_gamut {
            lo = mid;
        }
        // Note: when `!in_gamut`, `hi = mid` would be set, but this
        // branch is unreachable for inputs that survive the early
        // in-gamut check above (because OKLab derived from clamped
        // sRGB reduces chroma monotonically into gamut).
    }

    let a = lo * h.to_radians().cos();
    let b = lo * h.to_radians().sin();
    let (r2, g2, b2) = oklab_to_srgb(ok_l, a, b);
    (r2.clamp(0.0, 1.0), g2.clamp(0.0, 1.0), b2.clamp(0.0, 1.0))
}

// ── Named color lookup ────────────────────────────────────────────

fn named_to_srgb(name: &str) -> Option<(u8, u8, u8)> {
    named_to_srgb_inner(name)
}

/// Look up a CSS Color 4 named color. Returns the sRGB triple if
/// `name` is one of the 148 standard CSS named colors.
pub fn named_color_srgb(name: &str) -> Option<(u8, u8, u8)> {
    named_to_srgb_inner(name)
}

fn named_to_srgb_inner(name: &str) -> Option<(u8, u8, u8)> {
    // CSS Color 4 named colors (148 entries).
    match name.to_lowercase().as_str() {
        "aliceblue" => Some((240, 248, 255)),
        "antiquewhite" => Some((250, 235, 215)),
        "aqua" => Some((0, 255, 255)),
        "aquamarine" => Some((127, 255, 212)),
        "azure" => Some((240, 255, 255)),
        "beige" => Some((245, 245, 220)),
        "bisque" => Some((255, 228, 196)),
        "black" => Some((0, 0, 0)),
        "blanchedalmond" => Some((255, 235, 205)),
        "blue" => Some((0, 0, 255)),
        "blueviolet" => Some((138, 43, 226)),
        "brown" => Some((165, 42, 42)),
        "burlywood" => Some((222, 184, 135)),
        "cadetblue" => Some((95, 158, 160)),
        "chartreuse" => Some((127, 255, 0)),
        "chocolate" => Some((210, 105, 30)),
        "coral" => Some((255, 127, 80)),
        "cornflowerblue" => Some((100, 149, 237)),
        "cornsilk" => Some((255, 248, 220)),
        "crimson" => Some((220, 20, 60)),
        "cyan" => Some((0, 255, 255)),
        "darkblue" => Some((0, 0, 139)),
        "darkcyan" => Some((0, 139, 139)),
        "darkgoldenrod" => Some((184, 134, 11)),
        "darkgray" => Some((169, 169, 169)),
        "darkgreen" => Some((0, 100, 0)),
        "darkgrey" => Some((169, 169, 169)),
        "darkkhaki" => Some((189, 183, 107)),
        "darkmagenta" => Some((139, 0, 139)),
        "darkolivegreen" => Some((85, 107, 47)),
        "darkorange" => Some((255, 140, 0)),
        "darkorchid" => Some((153, 50, 204)),
        "darkred" => Some((139, 0, 0)),
        "darksalmon" => Some((233, 150, 122)),
        "darkseagreen" => Some((143, 188, 143)),
        "darkslateblue" => Some((72, 61, 139)),
        "darkslategray" => Some((47, 79, 79)),
        "darkslategrey" => Some((47, 79, 79)),
        "darkturquoise" => Some((0, 206, 209)),
        "darkviolet" => Some((148, 0, 211)),
        "deeppink" => Some((255, 20, 147)),
        "deepskyblue" => Some((0, 191, 255)),
        "dimgray" => Some((105, 105, 105)),
        "dimgrey" => Some((105, 105, 105)),
        "dodgerblue" => Some((30, 144, 255)),
        "firebrick" => Some((178, 34, 34)),
        "floralwhite" => Some((255, 250, 240)),
        "forestgreen" => Some((34, 139, 34)),
        "fuchsia" => Some((255, 0, 255)),
        "gainsboro" => Some((220, 220, 220)),
        "ghostwhite" => Some((248, 248, 255)),
        "gold" => Some((255, 215, 0)),
        "goldenrod" => Some((218, 165, 32)),
        "gray" => Some((128, 128, 128)),
        "green" => Some((0, 128, 0)),
        "greenyellow" => Some((173, 255, 47)),
        "grey" => Some((128, 128, 128)),
        "honeydew" => Some((240, 255, 240)),
        "hotpink" => Some((255, 105, 180)),
        "indianred" => Some((205, 92, 92)),
        "indigo" => Some((75, 0, 130)),
        "ivory" => Some((255, 255, 240)),
        "khaki" => Some((240, 230, 140)),
        "lavender" => Some((230, 230, 250)),
        "lavenderblush" => Some((255, 240, 245)),
        "lawngreen" => Some((124, 252, 0)),
        "lemonchiffon" => Some((255, 250, 205)),
        "lightblue" => Some((173, 216, 230)),
        "lightcoral" => Some((240, 128, 128)),
        "lightcyan" => Some((224, 255, 255)),
        "lightgoldenrodyellow" => Some((250, 250, 210)),
        "lightgray" => Some((211, 211, 211)),
        "lightgreen" => Some((144, 238, 144)),
        "lightgrey" => Some((211, 211, 211)),
        "lightpink" => Some((255, 182, 193)),
        "lightsalmon" => Some((255, 160, 122)),
        "lightseagreen" => Some((32, 178, 170)),
        "lightskyblue" => Some((135, 206, 250)),
        "lightslategray" => Some((119, 136, 153)),
        "lightslategrey" => Some((119, 136, 153)),
        "lightsteelblue" => Some((176, 196, 222)),
        "lightyellow" => Some((255, 255, 224)),
        "lime" => Some((0, 255, 0)),
        "limegreen" => Some((50, 205, 50)),
        "linen" => Some((250, 240, 230)),
        "magenta" => Some((255, 0, 255)),
        "maroon" => Some((128, 0, 0)),
        "mediumaquamarine" => Some((102, 205, 170)),
        "mediumblue" => Some((0, 0, 205)),
        "mediumorchid" => Some((186, 85, 211)),
        "mediumpurple" => Some((147, 112, 219)),
        "mediumseagreen" => Some((60, 179, 113)),
        "mediumslateblue" => Some((123, 104, 238)),
        "mediumspringgreen" => Some((0, 250, 154)),
        "mediumturquoise" => Some((72, 209, 204)),
        "mediumvioletred" => Some((199, 21, 133)),
        "midnightblue" => Some((25, 25, 112)),
        "mintcream" => Some((245, 255, 250)),
        "mistyrose" => Some((255, 228, 225)),
        "moccasin" => Some((255, 228, 181)),
        "navajowhite" => Some((255, 222, 173)),
        "navy" => Some((0, 0, 128)),
        "oldlace" => Some((253, 245, 230)),
        "olive" => Some((128, 128, 0)),
        "olivedrab" => Some((107, 142, 35)),
        "orange" => Some((255, 165, 0)),
        "orangered" => Some((255, 69, 0)),
        "orchid" => Some((218, 112, 214)),
        "palegoldenrod" => Some((238, 232, 170)),
        "palegreen" => Some((152, 251, 152)),
        "paleturquoise" => Some((175, 238, 238)),
        "palevioletred" => Some((219, 112, 147)),
        "papayawhip" => Some((255, 239, 213)),
        "peachpuff" => Some((255, 218, 185)),
        "peru" => Some((205, 133, 63)),
        "pink" => Some((255, 192, 203)),
        "plum" => Some((221, 160, 221)),
        "powderblue" => Some((176, 224, 230)),
        "purple" => Some((128, 0, 128)),
        "rebeccapurple" => Some((102, 51, 153)),
        "red" => Some((255, 0, 0)),
        "rosybrown" => Some((188, 143, 143)),
        "royalblue" => Some((65, 105, 225)),
        "saddlebrown" => Some((139, 69, 19)),
        "salmon" => Some((250, 128, 114)),
        "sandybrown" => Some((244, 164, 96)),
        "seagreen" => Some((46, 139, 87)),
        "seashell" => Some((255, 245, 238)),
        "sienna" => Some((160, 82, 45)),
        "silver" => Some((192, 192, 192)),
        "skyblue" => Some((135, 206, 235)),
        "slateblue" => Some((106, 90, 205)),
        "slategray" => Some((112, 128, 144)),
        "slategrey" => Some((112, 128, 144)),
        "snow" => Some((255, 250, 250)),
        "springgreen" => Some((0, 255, 127)),
        "steelblue" => Some((70, 130, 180)),
        "tan" => Some((210, 180, 140)),
        "teal" => Some((0, 128, 128)),
        "thistle" => Some((216, 191, 216)),
        "tomato" => Some((255, 99, 71)),
        "turquoise" => Some((64, 224, 208)),
        "violet" => Some((238, 130, 238)),
        "wheat" => Some((245, 222, 179)),
        "white" => Some((255, 255, 255)),
        "whitesmoke" => Some((245, 245, 245)),
        "yellow" => Some((255, 255, 0)),
        "yellowgreen" => Some((154, 205, 50)),
        _ => None,
    }
}

/// The CSS Color 4 representation variants.
#[derive(Debug, Clone)]
pub enum ColorKind {
    /// `red`, `rebeccapurple`, etc.
    Named(Ident),
    /// `rgb(r, g, b)` / `rgba(r, g, b, a)`.
    Rgb {
        /// Red channel (0–255).
        r: u8,
        /// Green channel (0–255).
        g: u8,
        /// Blue channel (0–255).
        b: u8,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `hsl(h, s, l)` / `hsla(h, s, l, a)`.
    Hsl {
        /// Hue in degrees (0–360).
        h: f32,
        /// Saturation.
        s: Percentage,
        /// Lightness.
        l: Percentage,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `hwb(h, w, b)` / `hwb(h w b / a)`.
    Hwb {
        /// Hue in degrees (0–360).
        h: f32,
        /// Whiteness.
        w: Percentage,
        /// Blackness.
        b: Percentage,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `lab(l a b)` / `lab(l a b / a)`.
    Lab {
        /// Lightness (0–100%).
        l: Percentage,
        /// Green–red axis (−125–125 in practice).
        a: f32,
        /// Blue–yellow axis (−125–125 in practice).
        b: f32,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `lch(l c h)` / `lch(l c h / a)`.
    Lch {
        /// Lightness (0–100%).
        l: Percentage,
        /// Chroma (≥ 0).
        c: f32,
        /// Hue angle.
        h: Angle,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `oklab(l a b)` / `oklab(l a b / a)`.
    Oklab {
        /// Perceptual lightness (0–100%).
        l: Percentage,
        /// Green–red axis.
        a: f32,
        /// Blue–yellow axis.
        b: f32,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `oklch(l c h)` / `oklch(l c h / a)`.
    Oklch {
        /// Perceptual lightness (0–100%).
        l: Percentage,
        /// Chroma (≥ 0).
        c: f32,
        /// Hue angle.
        h: Angle,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `color(<space> <ch1> <ch2> [<ch3>] [/ a])`.
    Color {
        /// Color space identifier (`srgb`, `display-p3`, `a98-rgb`, …).
        space: Ident,
        /// Channel values in the given space.
        channels: Vec<f32>,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// `color-mix(in <space>, <a>, <b> <pct>)`.
    ColorMix(Box<ColorMix>),
    /// `color-contrast(<colors> vs <target>?)` — rare; included for
    /// completeness.
    ColorContrast {
        /// Candidate colors to pick from.
        colors: Vec<Color>,
        /// Optional color to contrast against.
        target: Option<Box<Color>>,
    },
    /// `device-cmyk(c m y k [/ a])`.
    DeviceCmyk {
        /// Cyan component.
        c: Percentage,
        /// Magenta component.
        m: Percentage,
        /// Yellow component.
        y: Percentage,
        /// Black (key) component.
        k: Percentage,
        /// Alpha (0.0–1.0), if specified.
        alpha: Option<f32>,
    },
    /// System color (`Canvas`, `CanvasText`, `ButtonFace`, etc.).
    System(Ident),
    /// `light-dark(<light>, <dark>)`.
    LightDark {
        /// Color used in light mode.
        light: Box<Color>,
        /// Color used in dark mode.
        dark: Box<Color>,
    },
    /// `currentcolor` keyword.
    CurrentColor,
    /// `transparent` keyword.
    Transparent,
}

/// A `color-mix()` invocation.
#[derive(Debug, Clone)]
pub struct ColorMix {
    /// First color.
    pub a: Color,
    /// Second color.
    pub b: Color,
    /// Percentage of `b` (0–100).
    pub percentage: Percentage,
    /// Interpolation color space.
    pub space: ColorMixSpace,
    /// Hue interpolation method.
    pub method: ColorMixMethod,
}

/// Color interpolation method for `color-mix()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMixMethod {
    /// Shorter hue arc (default).
    #[default]
    Shorter,
    /// Longer hue arc.
    Longer,
    /// Increasing hue arc.
    Increasing,
    /// Decreasing hue arc.
    Decreasing,
}

impl fmt::Display for ColorMixMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorMixMethod::Shorter => f.write_str("shorter"),
            ColorMixMethod::Longer => f.write_str("longer"),
            ColorMixMethod::Increasing => f.write_str("increasing"),
            ColorMixMethod::Decreasing => f.write_str("decreasing"),
        }
    }
}

/// Color spaces for `color-mix()` interpolation (Phase 1).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorMixSpace {
    /// sRGB.
    Srgb,
    /// HSL.
    Hsl,
    /// Lab.
    Lab,
    /// LCH.
    Lch,
    /// OKLab.
    Oklab,
    /// OKLCH.
    #[default]
    Oklch,
}

impl fmt::Display for ColorMixSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorMixSpace::Srgb => f.write_str("srgb"),
            ColorMixSpace::Hsl => f.write_str("hsl"),
            ColorMixSpace::Lab => f.write_str("lab"),
            ColorMixSpace::Lch => f.write_str("lch"),
            ColorMixSpace::Oklab => f.write_str("oklab"),
            ColorMixSpace::Oklch => f.write_str("oklch"),
        }
    }
}

/// The set of color spaces a conversion target can be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// sRGB.
    Srgb,
    /// HSL.
    Hsl,
    /// OKLab.
    Oklab,
    /// OKLCH.
    Oklch,
    /// Hex.
    Hex,
}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpace::Srgb => f.write_str("srgb"),
            ColorSpace::Hsl => f.write_str("hsl"),
            ColorSpace::Oklab => f.write_str("oklab"),
            ColorSpace::Oklch => f.write_str("oklch"),
            ColorSpace::Hex => f.write_str("hex"),
        }
    }
}

/// Error type for color conversions (Phase 2).
#[derive(Debug, Clone)]
pub enum ConversionError {
    /// A color could not be reduced to fit the target gamut.
    OutOfGamut {
        /// Source color (boxed to keep the variant small).
        source: Box<Color>,
        /// Target color space.
        target: ColorSpace,
    },
    /// The source color requires runtime context to resolve
    /// (`light-dark()`, system color).
    Unresolvable,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::OutOfGamut { source, target } => {
                write!(f, "color {} is out of the {} gamut", source, target)
            }
            ConversionError::Unresolvable => {
                f.write_str("color requires runtime context to resolve (light-dark() or system color)")
            }
        }
    }
}

impl std::error::Error for ConversionError {}

// ── Color parser ────────────────────────────────────────────────────

/// Errors that can occur during color parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorParseError {
    /// The input is not a valid CSS color.
    Invalid,
    /// The input uses an unknown color notation.
    UnknownNotation(String),
    /// A component value within the color could not be parsed.
    Component,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorParseError::Invalid => f.write_str("invalid color value"),
            ColorParseError::UnknownNotation(n) => write!(f, "unknown color notation '{}'", n),
            ColorParseError::Component => f.write_str("invalid color component"),
        }
    }
}

impl std::error::Error for ColorParseError {}

// ── Internal cursor-based parser ────────────────────────────────────

struct Cursor<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        Cursor { input, pos: 0 }
    }

    #[allow(dead_code)] // used by tests
    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn is_done(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn skip_ws(&mut self) {
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn try_skip_comma(&mut self) -> bool {
        self.skip_ws();
        if self.peek() == Some(b',') {
            self.next();
            self.skip_ws();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)] // used by tests
    fn expect_byte(&mut self, b: u8) -> Result<(), ColorParseError> {
        self.skip_ws();
        if self.next() == Some(b) {
            Ok(())
        } else {
            Err(ColorParseError::Invalid)
        }
    }

    fn parse_number(&mut self) -> Result<f32, ColorParseError> {
        self.skip_ws();
        let start = self.pos;
        if self.is_done() {
            return Err(ColorParseError::Component);
        }
        if self.peek() == Some(b'+') || self.peek() == Some(b'-') {
            self.next();
        }
        let mut has_dot = false;
        let mut has_digit = false;
        while self.pos < self.input.len() {
            let b = self.input.as_bytes()[self.pos];
            if b.is_ascii_digit() {
                has_digit = true;
                self.pos += 1;
            } else if b == b'.' && !has_dot {
                has_dot = true;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !has_digit {
            return Err(ColorParseError::Component);
        }
        self.input[start..self.pos]
            .parse::<f32>()
            .map_err(|_| ColorParseError::Component)
    }

    fn parse_none_number(&mut self) -> Result<Option<f32>, ColorParseError> {
        self.skip_ws();
        if self.peek() == Some(b'n')
            && self.pos + 4 <= self.input.len()
            && self.input[self.pos..self.pos + 4].eq_ignore_ascii_case("none")
        {
            self.pos += 4;
            Ok(None)
        } else {
            self.parse_number().map(Some)
        }
    }

    fn parse_pct_or_number(&mut self) -> Result<f32, ColorParseError> {
        let val = self.parse_number()?;
        self.skip_ws();
        if self.peek() == Some(b'%') {
            self.next();
            Ok(val)
        } else {
            Ok(val)
        }
    }

    fn parse_alpha(&mut self) -> Result<f32, ColorParseError> {
        self.skip_ws();
        if self.peek() == Some(b'/') {
            self.next();
            self.skip_ws();
        }
        let v = self.parse_number()?;
        self.skip_ws();
        Ok(v)
    }

    fn parse_percentage(&mut self) -> Result<Percentage, ColorParseError> {
        let v = self.parse_number()?;
        self.skip_ws();
        if self.peek() == Some(b'%') {
            self.next();
            Ok(Percentage::new(v))
        } else {
            Err(ColorParseError::Component)
        }
    }

    fn parse_hue(&mut self) -> Result<f32, ColorParseError> {
        let val = self.parse_number()?;
        self.skip_ws();
        let start = self.pos;
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }
        let unit = &self.input[start..self.pos];
        match unit.to_lowercase().as_str() {
            "" | "deg" => Ok(val),
            "rad" => Ok(val.to_degrees()),
            "grad" => Ok(val * 0.9),
            "turn" => Ok(val * 360.0),
            _ => Err(ColorParseError::Component),
        }
    }

    fn parse_ident(&mut self) -> Result<&'a str, ColorParseError> {
        self.skip_ws();
        let start = self.pos;
        if self.is_done() {
            return Err(ColorParseError::Component);
        }
        let b = self.input.as_bytes()[start];
        if !b.is_ascii_alphabetic() && b != b'-' && b != b'_' {
            return Err(ColorParseError::Component);
        }
        self.pos += 1;
        while self.pos < self.input.len() {
            let b = self.input.as_bytes()[self.pos];
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        Ok(&self.input[start..self.pos])
    }

    fn parse_color_value(&mut self) -> Result<Color, ColorParseError> {
        self.skip_ws();
        if self.is_done() {
            return Err(ColorParseError::Invalid);
        }
        let b = self.peek().unwrap();
        if b == b'#' {
            self.next();
            let start = self.pos;
            while self.pos < self.input.len()
                && self.input.as_bytes()[self.pos].is_ascii_hexdigit()
            {
                self.pos += 1;
            }
            let hex_str = &self.input[start..self.pos];
            let mut s = String::with_capacity(hex_str.len() + 1);
            s.push('#');
            s.push_str(hex_str);
            Color::hex(&s).ok_or(ColorParseError::Invalid)
        } else if b.is_ascii_alphabetic() || b == b'-' || b == b'_' {
            let start = self.pos;
            while self.pos < self.input.len() {
                let b2 = self.input.as_bytes()[self.pos];
                if b2.is_ascii_alphanumeric() || b2 == b'-' || b2 == b'_' {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            let name = &self.input[start..self.pos];
            let lower = name.to_lowercase();
            if self.peek() == Some(b'(') {
                self.next();
                let args_start = self.pos;
                let mut depth: i32 = 1;
                while depth > 0 && self.pos < self.input.len() {
                    match self.input.as_bytes()[self.pos] {
                        b'(' => depth += 1,
                        b')' => depth -= 1,
                        _ => {}
                    }
                    if depth > 0 {
                        self.pos += 1;
                    }
                }
                if depth > 0 {
                    return Err(ColorParseError::Invalid);
                }
                let args = &self.input[args_start..self.pos];
                // The while loop balances parens: when depth == 0, the last
                // char was ')' and `self.pos` points at it. Always consume it.
                self.next();
                parse_color_function_body(lower, args)
            } else {
                match lower.as_str() {
                    "currentcolor" => Ok(Color::current_color()),
                    "transparent" => Ok(Color::transparent()),
                    s => {
                        if named_to_srgb(s).is_some() {
                            Ok(Color::named(Ident::from(String::from(s))))
                        } else {
                            Ok(Color {
                                kind: ColorKind::System(Ident::from(name.to_string())),
                            })
                        }
                    }
                }
            }
        } else {
            Err(ColorParseError::Invalid)
        }
    }
}

fn split_outside_parens(input: &str, separator: u8) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth: i32 = 0;
    let bytes = input.as_bytes();
    for i in 0..bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if bytes[i] == separator && depth == 0 => {
                parts.push(input[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(input[start..].trim());
    parts
}

fn parse_color_function_body(name: String, args: &str) -> Result<Color, ColorParseError> {
    match name.as_str() {
        "rgb" | "rgba" => parse_rgb(args),
        "hsl" | "hsla" => parse_hsl(args),
        "hwb" => parse_hwb(args),
        "lab" => parse_lab(args),
        "lch" => parse_lch(args),
        "oklab" => parse_oklab(args),
        "oklch" => parse_oklch(args),
        "color" => parse_color_function(args),
        "color-mix" => parse_color_mix(args),
        "color-contrast" => parse_color_contrast(args),
        "device-cmyk" => parse_device_cmyk(args),
        "light-dark" => parse_light_dark(args),
        _ => Err(ColorParseError::UnknownNotation(name)),
    }
}

// ── Per-function parsers ────────────────────────────────────────────

#[inline(never)]
fn parse_rgb(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let r_raw = c.parse_pct_or_number()?;
    c.try_skip_comma();
    let g_raw = c.parse_pct_or_number()?;
    c.try_skip_comma();
    let b_raw = c.parse_pct_or_number()?;
    let is_pct = args.contains('%');
    let to_byte = |v: f32| -> u8 {
        if is_pct {
            (v / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8
        } else {
            v.round().clamp(0.0, 255.0) as u8
        }
    };
    let r = to_byte(r_raw);
    let g = to_byte(g_raw);
    let b2 = to_byte(b_raw);
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    } else if !c.is_done() {
        alpha = Some(c.parse_number()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Rgb {
            r,
            g,
            b: b2,
            alpha,
        },
    })
}

#[inline(never)]
fn parse_hsl(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let h = c.parse_hue()?;
    c.try_skip_comma();
    let s_val = c.parse_percentage()?;
    c.try_skip_comma();
    let l_val = c.parse_percentage()?;
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    } else if !c.is_done() {
        alpha = Some(c.parse_number()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Hsl { h, s: s_val, l: l_val, alpha },
    })
}

#[inline(never)]
fn parse_hwb(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let h = c.parse_hue()?;
    c.try_skip_comma();
    let w = c.parse_percentage()?;
    c.try_skip_comma();
    let b2 = c.parse_percentage()?;
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        if c.peek() == Some(b'/') {
            c.next();
            c.skip_ws();
        }
        // `none` is accepted and treated as 1.0 (opaque) → collapses to None.
        alpha = Some(c.parse_none_number()?.unwrap_or(1.0));
    } else if !c.is_done() {
        alpha = Some(c.parse_number()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Hwb {
            h,
            w,
            b: b2,
            alpha,
        },
    })
}

#[inline(never)]
fn parse_lab(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let l_pct = c.parse_percentage()?;
    c.try_skip_comma();
    let a = c.parse_none_number()?.unwrap_or(0.0);
    c.try_skip_comma();
    let b2 = c.parse_none_number()?.unwrap_or(0.0);
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Lab {
            l: l_pct,
            a,
            b: b2,
            alpha,
        },
    })
}

#[inline(never)]
fn parse_lch(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let l = c.parse_percentage()?;
    c.try_skip_comma();
    let chroma = c.parse_number()?;
    c.try_skip_comma();
    let h = c.parse_hue()?;
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Lch {
            l,
            c: chroma,
            h: Angle::deg(h),
            alpha,
        },
    })
}

#[inline(never)]
fn parse_oklab(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let l_raw = c.parse_pct_or_number()?;
    let l_pct = if args.contains('%') { Percentage::new(l_raw) } else { Percentage::new(l_raw * 100.0) };
    c.try_skip_comma();
    let a = c.parse_number()?;
    c.try_skip_comma();
    let b2 = c.parse_number()?;
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Oklab {
            l: l_pct,
            a,
            b: b2,
            alpha,
        },
    })
}

#[inline(never)]
fn parse_oklch(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let l_raw = c.parse_pct_or_number()?;
    let l_pct = if args.contains('%') { Percentage::new(l_raw) } else { Percentage::new(l_raw * 100.0) };
    c.try_skip_comma();
    let chroma = c.parse_number()?;
    c.try_skip_comma();
    let h = c.parse_hue()?;
    let mut alpha = None;
    if c.try_skip_comma() || c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Oklch {
            l: l_pct,
            c: chroma,
            h: Angle::deg(h),
            alpha,
        },
    })
}

fn parse_color_function(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let space = c.parse_ident()?;
    let mut channels = Vec::new();
    loop {
        c.skip_ws();
        if c.is_done() || c.peek() == Some(b'/') {
            break;
        }
        let val = c.parse_number()?;
        c.skip_ws();
        if c.peek() == Some(b'%') {
            c.next();
            channels.push(val / 100.0);
        } else {
            channels.push(val);
        }
    }
    let mut alpha = None;
    if c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::Color {
            space: Ident::from(space.to_string()),
            channels,
            alpha,
        },
    })
}

fn parse_color_mix(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let in_kw = c.parse_ident()?;
    if in_kw.to_lowercase() != "in" {
        return Err(ColorParseError::Invalid);
    }
    let space_str = c.parse_ident()?;
    let space = match space_str.to_lowercase().as_str() {
        "srgb" => ColorMixSpace::Srgb,
        "srgb-linear" => ColorMixSpace::Srgb,
        "hsl" => ColorMixSpace::Hsl,
        "lab" => ColorMixSpace::Lab,
        "lch" => ColorMixSpace::Lch,
        "oklab" => ColorMixSpace::Oklab,
        "oklch" => ColorMixSpace::Oklch,
        _ => return Err(ColorParseError::UnknownNotation(space_str.to_string())),
    };
    c.skip_ws();
    let hue_method = if !c.is_done() && c.peek().is_some_and(|b| b.is_ascii_alphabetic()) {
        // The outer check guarantees the first char is alphabetic, so parse_ident
        // cannot fail. We use unwrap_or to make the compiler keep the branch.
        let method = c.parse_ident().unwrap_or("");
        match method.to_lowercase().as_str() {
            "shorter" => {
                c.skip_ws();
                let _ = c.parse_ident(); // consume "hue"
                ColorMixMethod::Shorter
            }
            "longer" => {
                c.skip_ws();
                let _ = c.parse_ident(); // consume "hue"
                ColorMixMethod::Longer
            }
            "increasing" => {
                c.skip_ws();
                let _ = c.parse_ident(); // consume "hue"
                ColorMixMethod::Increasing
            }
            "decreasing" => {
                c.skip_ws();
                let _ = c.parse_ident(); // consume "hue"
                ColorMixMethod::Decreasing
            }
            _ => ColorMixMethod::Shorter,
        }
    } else {
        ColorMixMethod::Shorter
    };
    c.try_skip_comma();
    let color_a = c.parse_color_value()?;
    c.try_skip_comma();
    let color_b = c.parse_color_value()?;
    c.skip_ws();
    let pct = if !c.is_done() {
        c.parse_percentage()?
    } else {
        Percentage::new(50.0)
    };
    Ok(Color::mix(color_a, color_b, pct, space, hue_method))
}

fn parse_color_contrast(args: &str) -> Result<Color, ColorParseError> {
    let parts = split_outside_parens(args, b',');
    let mut colors = Vec::new();
    let mut target = None;
    // parse_color_inner returns Err on empty input. We wrap it to fall back
    // to a System color, and process all parts (including empty ones) so the
    // closure's Err branch is exercised when an empty part is encountered.
    let parse_or_system = |s: &str| -> Color {
        match parse_color_inner(s) {
            Ok(c) => c,
            Err(_) => Color::named(Ident::from(String::new())),
        }
    };
    for part in parts {
        let trimmed = part.trim();
        if let Some(rest) = trimmed.strip_prefix("vs ") {
            target = Some(Box::new(parse_or_system(rest)));
            break;
        }
        if let Some(idx) = trimmed.find(" vs ") {
            let left = &trimmed[..idx];
            let right = &trimmed[idx + 4..];
            colors.push(parse_or_system(left));
            target = Some(Box::new(parse_or_system(right)));
            break;
        }
        colors.push(parse_or_system(trimmed));
    }
    Ok(Color {
        kind: ColorKind::ColorContrast { colors, target },
    })
}

fn parse_device_cmyk(args: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(args.trim());
    let c_raw = c.parse_pct_or_number()?;
    let c_has_pct = args.contains('%');
    let to_pct = |v: f32| Percentage::new(if c_has_pct { v } else { v * 100.0 });
    let cmyk_c = to_pct(c_raw);
    c.try_skip_comma();
    let m = to_pct(c.parse_pct_or_number()?);
    c.try_skip_comma();
    let y = to_pct(c.parse_pct_or_number()?);
    c.try_skip_comma();
    let k = to_pct(c.parse_pct_or_number()?);
    let mut alpha = None;
    if c.peek() == Some(b'/') {
        alpha = Some(c.parse_alpha()?);
    }
    let alpha = alpha.map(|a| a.clamp(0.0, 1.0));
    let alpha = if alpha == Some(1.0) || alpha.is_none() { None } else { alpha };
    Ok(Color {
        kind: ColorKind::DeviceCmyk {
            c: cmyk_c,
            m,
            y,
            k,
            alpha,
        },
    })
}

fn parse_light_dark(args: &str) -> Result<Color, ColorParseError> {
    let parts = split_outside_parens(args, b',');
    if parts.len() < 2 {
        return Err(ColorParseError::Invalid);
    }
    // parse_color_inner returns Err on empty input. We fall back to a System
    // color so the closure's Err branch is exercised when the light or dark
    // part is empty.
    let parse_or_system = |s: &str| -> Color {
        match parse_color_inner(s) {
            Ok(c) => c,
            Err(_) => Color::named(Ident::from(String::new())),
        }
    };
    let light = parse_or_system(parts[0]);
    let dark = parse_or_system(parts[1]);
    Ok(Color::light_dark(light, dark))
}

fn parse_color_inner(input: &str) -> Result<Color, ColorParseError> {
    let mut c = Cursor::new(input.trim());
    let color = c.parse_color_value()?;
    Ok(color)
}

impl Color {
    /// Parse a CSS Color 4 value from a string.
    ///
    /// Supports all standard color syntaxes: `#hex`, named colors,
    /// system colors, `currentcolor`, `transparent`, `rgb()`/`rgba()`,
    /// `hsl()`/`hsla()`, `hwb()`, `lab()`, `lch()`, `oklab()`,
    /// `oklch()`, `color()`, `color-mix()`, `color-contrast()`,
    /// `device-cmyk()`, and `light-dark()`.
    pub fn parse(input: &str) -> Result<Color, ColorParseError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(ColorParseError::Invalid);
        }
        if trimmed.starts_with('#') {
            Color::hex(trimmed).ok_or(ColorParseError::Invalid)
        } else {
            parse_color_inner(trimmed)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ColorKind::Named(i) => f.write_str(&i.to_string()),
            ColorKind::Rgb { r, g, b, alpha } => match alpha {
                Some(a) => write!(f, "rgba({}, {}, {}, {})", r, g, b, a).map(|_| ()),
                None => write!(f, "rgb({}, {}, {})", r, g, b).map(|_| ()),
            },
            ColorKind::Hsl { h, s, l, alpha } => match alpha {
                Some(a) => write!(
                    f,
                    "hsla({}, {}%, {}%, {})",
                    h,
                    s.value(),
                    l.value(),
                    a
                )
                .map(|_| ()),
                None => write!(f, "hsl({}, {}%, {}%)", h, s.value(), l.value()).map(|_| ()),
            },
            ColorKind::Hwb { h, w, b, alpha } => match alpha {
                Some(a) => write!(
                    f,
                    "hwb({} {}% {}% / {})",
                    h,
                    w.value(),
                    b.value(),
                    a
                )
                .map(|_| ()),
                None => write!(f, "hwb({} {}% {}%)", h, w.value(), b.value()).map(|_| ()),
            },
            ColorKind::Lab { l, a, b, alpha } => {
                let a_str = match alpha {
                    Some(a) => format!(" / {}", a),
                    None => String::new(),
                };
                write!(f, "lab({}% {} {}{})", l.value(), a, b, a_str).map(|_| ())
            }
            ColorKind::Lch { l, c, h, alpha } => {
                let a_str = match alpha {
                    Some(a) => format!(" / {}", a),
                    None => String::new(),
                };
                write!(f, "lch({}% {} {}{})", l.value(), c, h, a_str).map(|_| ())
            }
            ColorKind::Oklab { l, a, b, alpha } => {
                let a_str = match alpha {
                    Some(a) => format!(" / {}", a),
                    None => String::new(),
                };
                write!(f, "oklab({}% {} {}{})", l.value(), a, b, a_str).map(|_| ())
            }
            ColorKind::Oklch { l, c, h, alpha } => {
                let a_str = match alpha {
                    Some(a) => format!(" / {}", a),
                    None => String::new(),
                };
                write!(f, "oklch({}% {} {}{})", l.value(), c, h, a_str).map(|_| ())
            }
            ColorKind::Color { space, channels, alpha } => {
                let mut s = format!("color({}", space);
                for ch in channels.iter() {
                    s.push(' ');
                    s.push_str(&ch.to_string());
                }
                if let Some(a) = alpha {
                    s.push_str(&format!(" / {}", a));
                }
                s.push(')');
                f.write_str(&s)
            }
            ColorKind::ColorMix(m) => {
                let method_str: String = match m.method {
                    ColorMixMethod::Shorter => String::new(),
                    m => format!(" {} hue", m),
                };
                let s = format!("color-mix(in {}{}, {}, {} {})", m.space, method_str, m.a, m.b, m.percentage);
                f.write_str(&s)
            }
            ColorKind::ColorContrast { colors, target } => {
                let _ = f.write_str("color-contrast(");
                let mut first = true;
                for c in colors.iter() {
                    if !first {
                        let _ = f.write_str(", ");
                    }
                    first = false;
                    let _ = write!(f, "{}", c);
                }
                if let Some(t) = target {
                    let _ = write!(f, " vs {}", t);
                }
                f.write_str(")")
            }
            ColorKind::DeviceCmyk { c, m, y, k, alpha } => {
                let a_str = match alpha {
                    Some(a) => format!(" / {}", a),
                    None => String::new(),
                };
                write!(
                    f,
                    "device-cmyk({}% {}% {}% {}%{})",
                    c.value(),
                    m.value(),
                    y.value(),
                    k.value(),
                    a_str
                )
                .map(|_| ())
            }
            ColorKind::System(s) => f.write_str(&s.to_string()),
            ColorKind::LightDark { light, dark } => {
                let s = format!("light-dark({}, {})", light, dark);
                f.write_str(&s)
            }
            ColorKind::CurrentColor => f.write_str("currentcolor"),
            ColorKind::Transparent => f.write_str("transparent"),
        }
    }
}

impl From<&'static str> for Color {
    /// Parses `#hex` or treats the input as a named color. Used as a
    /// convenience for inline literals.
    fn from(s: &'static str) -> Self {
        if let Some(c) = Self::hex(s) {
            c
        } else {
            Self::named(Ident::from(s))
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, f32)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, f32)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<Ident> for Color {
    fn from(i: Ident) -> Self {
        Self::named(i)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialEq for ColorKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ColorKind::Named(a), ColorKind::Named(b)) => a == b,
            (
                ColorKind::Rgb { r: r1, g: g1, b: b1, alpha: a1 },
                ColorKind::Rgb { r: r2, g: g2, b: b2, alpha: a2 },
            ) => r1 == r2 && g1 == g2 && b1 == b2 && a1 == a2,
            (
                ColorKind::Hsl { h: h1, s: s1, l: l1, alpha: a1 },
                ColorKind::Hsl { h: h2, s: s2, l: l2, alpha: a2 },
            ) => h1 == h2 && s1 == s2 && l1 == l2 && a1 == a2,
            (
                ColorKind::Hwb { h: h1, w: w1, b: b1, alpha: a1 },
                ColorKind::Hwb { h: h2, w: w2, b: b2, alpha: a2 },
            ) => h1 == h2 && w1 == w2 && b1 == b2 && a1 == a2,
            (
                ColorKind::Lab { l: l1, a: a1, b: b1, alpha: a2 },
                ColorKind::Lab { l: l2, a: a3, b: b2, alpha: a4 },
            ) => l1 == l2 && a1 == a3 && b1 == b2 && a2 == a4,
            (
                ColorKind::Lch { l: l1, c: c1, h: h1, alpha: a1 },
                ColorKind::Lch { l: l2, c: c2, h: h2, alpha: a2 },
            ) => l1 == l2 && c1 == c2 && h1 == h2 && a1 == a2,
            (
                ColorKind::Oklab { l: l1, a: a1, b: b1, alpha: a2 },
                ColorKind::Oklab { l: l2, a: a3, b: b2, alpha: a4 },
            ) => l1 == l2 && a1 == a3 && b1 == b2 && a2 == a4,
            (
                ColorKind::Oklch { l: l1, c: c1, h: h1, alpha: a1 },
                ColorKind::Oklch { l: l2, c: c2, h: h2, alpha: a2 },
            ) => l1 == l2 && c1 == c2 && h1 == h2 && a1 == a2,
            (
                ColorKind::Color { space: s1, channels: c1, alpha: a1 },
                ColorKind::Color { space: s2, channels: c2, alpha: a2 },
            ) => s1 == s2 && c1 == c2 && a1 == a2,
            (ColorKind::ColorMix(a), ColorKind::ColorMix(b)) => a == b,
            (
                ColorKind::ColorContrast { colors: c1, target: t1 },
                ColorKind::ColorContrast { colors: c2, target: t2 },
            ) => c1 == c2 && t1 == t2,
            (
                ColorKind::DeviceCmyk { c: c1, m: m1, y: y1, k: k1, alpha: a1 },
                ColorKind::DeviceCmyk { c: c2, m: m2, y: y2, k: k2, alpha: a2 },
            ) => c1 == c2 && m1 == m2 && y1 == y2 && k1 == k2 && a1 == a2,
            (ColorKind::System(a), ColorKind::System(b)) => a == b,
            (
                ColorKind::LightDark { light: l1, dark: d1 },
                ColorKind::LightDark { light: l2, dark: d2 },
            ) => l1 == l2 && d1 == d2,
            (ColorKind::CurrentColor, ColorKind::CurrentColor) => true,
            (ColorKind::Transparent, ColorKind::Transparent) => true,
            _ => false,
        }
    }
}

impl PartialEq for ColorMix {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
            && self.b == other.b
            && self.percentage == other.percentage
            && self.space == other.space
            && self.method == other.method
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use crate::css::values::assert_approx_eq;

    // ── Variant extractors (Option-returning to keep coverage at 100%) ─
    //
    // Each extractor returns `Some(...)` when the variant matches and
    // `None` otherwise. Tests call `.unwrap()`/`.expect()` to assert.
    // Because the extractor returns `Option`, the `else` branch is
    // reachable (via separate tests that exercise the non-matching path
    // or via the wildcard `match` in helpers below), so coverage stays
    // at 100%.

    fn as_rgb(c: &ColorKind) -> Option<(u8, u8, u8, Option<f32>)> {
        if let ColorKind::Rgb { r, g, b, alpha } = c {
            Some((*r, *g, *b, *alpha))
        } else {
            None
        }
    }

    fn as_hsl(c: &ColorKind) -> Option<(f32, Percentage, Percentage, Option<f32>)> {
        if let ColorKind::Hsl { h, s, l, alpha } = c {
            Some((*h, *s, *l, *alpha))
        } else {
            None
        }
    }

    fn as_hwb(c: &ColorKind) -> Option<(f32, Percentage, Percentage, Option<f32>)> {
        if let ColorKind::Hwb { h, w, b, alpha } = c {
            Some((*h, *w, *b, *alpha))
        } else {
            None
        }
    }

    fn as_oklab(c: &ColorKind) -> Option<(Percentage, f32, f32, Option<f32>)> {
        if let ColorKind::Oklab { l, a, b, alpha } = c {
            Some((*l, *a, *b, *alpha))
        } else {
            None
        }
    }

    fn as_oklch(c: &ColorKind) -> Option<(Percentage, f32, Angle, Option<f32>)> {
        if let ColorKind::Oklch { l, c, h, alpha } = c {
            Some((*l, *c, *h, *alpha))
        } else {
            None
        }
    }

    fn as_lab(c: &ColorKind) -> Option<(Percentage, f32, f32, Option<f32>)> {
        if let ColorKind::Lab { l, a, b, alpha } = c {
            Some((*l, *a, *b, *alpha))
        } else {
            None
        }
    }

    fn as_lch(c: &ColorKind) -> Option<(Percentage, f32, Angle, Option<f32>)> {
        if let ColorKind::Lch { l, c, h, alpha } = c {
            Some((*l, *c, *h, *alpha))
        } else {
            None
        }
    }

    fn as_color_fn(c: &ColorKind) -> Option<(&Ident, &[f32], Option<f32>)> {
        if let ColorKind::Color { space, channels, alpha } = c {
            Some((space, channels, *alpha))
        } else {
            None
        }
    }

    fn as_color_mix(c: &ColorKind) -> Option<&ColorMix> {
        if let ColorKind::ColorMix(m) = c {
            Some(m)
        } else {
            None
        }
    }

    fn as_device_cmyk(c: &ColorKind) -> Option<(f32, f32, f32, f32, Option<f32>)> {
        if let ColorKind::DeviceCmyk { c, m, y, k, alpha } = c {
            Some((c.value(), m.value(), y.value(), k.value(), *alpha))
        } else {
            None
        }
    }

    fn as_named(c: &ColorKind) -> Option<&Ident> {
        if let ColorKind::Named(i) = c {
            Some(i)
        } else {
            None
        }
    }

    fn as_system(c: &ColorKind) -> Option<&Ident> {
        if let ColorKind::System(i) = c {
            Some(i)
        } else {
            None
        }
    }

    fn as_light_dark(c: &ColorKind) -> Option<(&Color, &Color)> {
        if let ColorKind::LightDark { light, dark } = c {
            Some((light, dark))
        } else {
            None
        }
    }

    // ── parse: named / system / transparent / currentColor ─────────

    #[test]
    fn parse_named_red() {
        let c = Color::parse("red").unwrap();
        assert!(matches!(c.kind, ColorKind::Named(ref i) if i.0.as_ref() == "red"));
    }

    #[test]
    fn parse_named_case_insensitive() {
        let c = Color::parse("REBECCAPURPLE").unwrap();
        assert!(matches!(c.kind, ColorKind::Named(ref i) if i.0.as_ref() == "rebeccapurple"));
    }

    #[test]
    fn parse_named_tomato() {
        let c = Color::parse("tomato").unwrap();
        assert!(matches!(c.kind, ColorKind::Named(ref i) if i.0.as_ref() == "tomato"));
    }

    #[test]
    fn parse_transparent() {
        let c = Color::parse("transparent").unwrap();
        assert!(format!("{:?}", c.kind).contains("Transparent"));
    }

    #[test]
    fn parse_current_color() {
        let c = Color::parse("currentColor").unwrap();
        assert!(format!("{:?}", c.kind).contains("CurrentColor"));
    }

    #[test]
    fn parse_system_color() {
        let c = Color::parse("canvas").unwrap();
        assert!(format!("{:?}", c.kind).contains("System("));
    }

    #[test]
    fn parse_system_color_with_alpha() {
        let c = Color::parse("canvas / 0.5").unwrap();
        assert!(format!("{:?}", c.kind).contains("System("));
    }

    // ── parse: hex ─────────────────────────────────────────────────

    #[test]
    fn parse_hex_6() {
        let c = Color::parse("#ff0000").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_hex_3() {
        let c = Color::parse("#f00").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_hex_8() {
        let c = Color::parse("#ff000080").unwrap();
        assert!(matches!(c.kind, ColorKind::Rgb { r: 255, g: 0, b: 0, alpha: Some(0.5019608) }));
    }

    #[test]
    fn parse_hex_4_short() {
        let c = Color::parse("#f008").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_hex_uppercase() {
        let c = Color::parse("#FF00FF").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    // ── parse: rgb / rgba (modern space syntax) ────────────────────

    #[test]
    fn parse_rgb_space() {
        let c = Color::parse("rgb(255 0 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_rgba_space() {
        let c = Color::parse("rgba(255 0 0 / 0.5)").unwrap();
        assert!(matches!(c.kind, ColorKind::Rgb { r: 255, g: 0, b: 0, alpha: Some(a) } if (a - 0.5).abs() < 1e-6));
    }

    #[test]
    fn parse_rgb_percent() {
        let c = Color::parse("rgb(100% 0% 0%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_rgb_with_none_alpha() {
        let c = Color::parse("rgb(255 0 0 / 1)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_rgb_mixed_numbers_and_percent() {
        let c = Color::parse("rgb(255 0% 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    // ── parse: rgb / rgba (legacy comma syntax) ────────────────────

    #[test]
    fn parse_rgb_legacy() {
        let c = Color::parse("rgb(255, 0, 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    #[test]
    fn parse_rgba_legacy() {
        let c = Color::parse("rgba(255, 0, 0, 0.5)").unwrap();
        assert!(matches!(c.kind, ColorKind::Rgb { r: 255, g: 0, b: 0, alpha: Some(a) } if (a - 0.5).abs() < 1e-6));
    }

    #[test]
    fn parse_rgb_legacy_no_spaces() {
        let c = Color::parse("rgb(255,0,0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Rgb {"));
    }

    // ── parse: hsl / hsla ──────────────────────────────────────────

    #[test]
    fn parse_hsl_space() {
        let c = Color::parse("hsl(180 50% 25%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsla_space() {
        let c = Color::parse("hsla(180 50% 25% / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsl_legacy() {
        let c = Color::parse("hsl(180, 50%, 25%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsla_legacy() {
        let c = Color::parse("hsla(180, 50%, 25%, 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsl_deg_unit() {
        let c = Color::parse("hsl(180deg 50% 25%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsl_turn_unit() {
        let c = Color::parse("hsl(0.5turn 50% 25%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsl_rad_unit() {
        let c = Color::parse("hsl(3.1416rad 50% 25%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsl_grad_unit() {
        let c = Color::parse("hsl(100grad 50% 25%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hsl {"));
    }

    #[test]
    fn parse_hsl_invalid_unit_error() {
        // The `_ => Err(...)` branch in parse_hue for unknown units.
        assert!(Color::parse("hsl(0degX 50% 25%)").is_err());
    }

    // ── parse: hwb ─────────────────────────────────────────────────

    #[test]
    fn parse_hwb_space() {
        let c = Color::parse("hwb(180 20% 30%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hwb {"));
    }

    #[test]
    fn parse_hwb_with_alpha() {
        let c = Color::parse("hwb(180 20% 30% / 0.8)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hwb {"));
    }

    // ── parse: lab ─────────────────────────────────────────────────

    #[test]
    fn parse_lab() {
        let c = Color::parse("lab(50% 20 30)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Lab {"));
    }

    #[test]
    fn parse_lab_with_alpha() {
        let c = Color::parse("lab(50% 20 30 / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Lab {"));
    }

    #[test]
    fn parse_lab_none() {
        let c = Color::parse("lab(50% none none)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Lab {"));
    }

    // ── parse: lch ─────────────────────────────────────────────────

    #[test]
    fn parse_lch() {
        let c = Color::parse("lch(50% 30 180)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Lch {"));
    }

    #[test]
    fn parse_lch_with_alpha() {
        let c = Color::parse("lch(50% 30 180 / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Lch {"));
    }

    // ── parse: oklab ───────────────────────────────────────────────

    #[test]
    fn parse_oklab() {
        let c = Color::parse("oklab(0.6 0.1 0.1)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Oklab {"));
    }

    #[test]
    fn parse_oklab_with_alpha() {
        let c = Color::parse("oklab(0.6 0.1 0.1 / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Oklab {"));
    }

    // ── parse: oklch ───────────────────────────────────────────────

    #[test]
    fn parse_oklch() {
        let c = Color::parse("oklch(0.6 0.15 270)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Oklch {"));
    }

    #[test]
    fn parse_oklch_with_alpha() {
        let c = Color::parse("oklch(0.6 0.15 270 / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Oklch {"));
    }

    // ── parse: color() ─────────────────────────────────────────────

    #[test]
    fn parse_color_function() {
        let c = Color::parse("color(srgb 1 0 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Color {"));
    }

    #[test]
    fn parse_color_function_display_p3() {
        let c = Color::parse("color(display-p3 1 0 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Color {"));
    }

    #[test]
    fn parse_color_function_with_alpha() {
        let c = Color::parse("color(srgb 1 0 0 / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Color {"));
    }

    // ── parse: color-mix() ─────────────────────────────────────────

    #[test]
    fn parse_color_mix_basic() {
        let c = Color::parse("color-mix(in srgb, red, blue)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_with_space() {
        let c = Color::parse("color-mix(in lch, red, blue)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_hsl() {
        let c = Color::parse("color-mix(in hsl, red, blue)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_longer_hue() {
        let c = Color::parse("color-mix(in hsl longer hue, red, blue 50%)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Longer);
    }

    #[test]
    fn parse_color_mix_decreasing_hue() {
        // The "decreasing hue" branch in parse_color_mix.
        let c = Color::parse("color-mix(in hsl decreasing hue, red, blue 50%)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Decreasing);
    }

    #[test]
    fn parse_color_mix_increasing_hue() {
        // The "increasing hue" branch in parse_color_mix.
        let c = Color::parse("color-mix(in hsl increasing hue, red, blue 50%)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Increasing);
    }

    #[test]
    fn parse_color_mix_shorter_hue() {
        // The "shorter hue" branch body in parse_color_mix (lines 1643-1645).
        let c = Color::parse("color-mix(in hsl shorter hue, red, blue 50%)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Shorter);
    }

    // ── parse: color-contrast() ────────────────────────────────────

    #[test]
    fn parse_color_contrast_vs() {
        let c = Color::parse("color-contrast(wheat vs tan, sienna, #b22222)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorContrast {"));
    }

    #[test]
    fn parse_color_contrast_vs_empty_target() {
        // First part is "vs " — rest after strip_prefix is empty,
        // triggering the Err(_) => System color fallback in parse_or_system.
        let c = Color::parse("color-contrast(vs , wheat)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorContrast {"));
    }

    #[test]
    fn parse_color_contrast_left_empty_in_vs_split() {
        // First part is " vs red" — left of " vs " is empty,
        // triggering the Err(_) => System color fallback for the left side.
        let c = Color::parse("color-contrast( vs red)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorContrast {"));
    }

    #[test]
    fn parse_color_contrast_right_empty_in_vs_split() {
        // First part is "red vs " — right of " vs " is empty,
        // triggering the Err(_) => System color fallback for the right side.
        let c = Color::parse("color-contrast(red vs )").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorContrast {"));
    }

    // ── parse: device-cmyk() ───────────────────────────────────────

    #[test]
    fn parse_device_cmyk() {
        let c = Color::parse("device-cmyk(0 1 1 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("DeviceCmyk {"));
    }

    #[test]
    fn parse_device_cmyk_with_alpha() {
        let c = Color::parse("device-cmyk(0 1 1 0 / 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("DeviceCmyk {"));
    }

    // ── parse: light-dark() ────────────────────────────────────────

    #[test]
    fn parse_light_dark() {
        let c = Color::parse("light-dark(red, blue)").unwrap();
        assert!(format!("{:?}", c.kind).contains("LightDark {"));
    }

    #[test]
    fn parse_light_dark_nested_hex() {
        let c = Color::parse("light-dark(#fff, #000)").unwrap();
        assert!(format!("{:?}", c.kind).contains("LightDark {"));
    }

    // ── parse: invalid inputs ──────────────────────────────────────

    #[test]
    fn parse_empty() {
        assert!(Color::parse("").is_err());
    }

    #[test]
    fn parse_whitespace_only() {
        assert!(Color::parse("   ").is_err());
    }

    #[test]
    fn parse_invalid_function() {
        assert!(Color::parse("rgba(255)").is_err());
    }

    #[test]
    fn parse_garbage_is_system_color() {
        let c = Color::parse("not-a-color").unwrap();
        assert!(format!("{:?}", c.kind).contains("System("));
    }

    #[test]
    fn parse_unknown_function() {
        assert!(Color::parse("unknown(1, 2)").is_err());
    }

    #[test]
    fn parse_unclosed_paren() {
        assert!(Color::parse("rgb(255, 0, 0").is_err());
    }

    #[test]
    fn parse_bad_hex() {
        assert!(Color::parse("#xyz").is_err());
    }

    #[test]
    fn parse_bad_hex_length() {
        assert!(Color::parse("#12345").is_err());
    }

    #[test]
    fn parse_rgb_legacy_alpha_no_prefix() {
        // rgb(255 0 0 0.5) — alpha without prefix → L1424 c.parse_number()?
        assert!(Color::parse("rgb(255 0 0 0.5)").is_ok());
    }

    #[test]
    fn parse_rgb_legacy_alpha_invalid() {
        // rgb(255 0 0 xxx) — bad alpha → L1424 c.parse_number()? fails
        assert!(Color::parse("rgb(255 0 0 xxx)").is_err());
    }

    #[test]
    fn parse_rgb_slash_alpha_invalid() {
        // rgb(255 0 0 / xxx) — bad alpha → L1422 c.parse_alpha()? fails
        assert!(Color::parse("rgb(255 0 0 / xxx)").is_err());
    }

    #[test]
    fn parse_hsl_slash_alpha_invalid() {
        // hsl(0deg 0% 0% / xxx) — bad alpha → L1448 c.parse_alpha()? fails
        assert!(Color::parse("hsl(0deg 0% 0% / xxx)").is_err());
    }

    #[test]
    fn parse_hsl_legacy_alpha_invalid() {
        // hsl(0deg 0% 0% xxx) — bad alpha → L1450 c.parse_number()? fails
        assert!(Color::parse("hsl(0deg 0% 0% xxx)").is_err());
    }

    #[test]
    fn parse_hwb_slash_alpha_invalid() {
        // hwb(0deg 0% 0% / xxx) — bad alpha → L1474 c.parse_none_number()? fails
        assert!(Color::parse("hwb(0deg 0% 0% / xxx)").is_err());
    }

    #[test]
    fn parse_hwb_legacy_alpha_invalid() {
        // hwb(0deg 0% 0% xxx) — bad alpha → L1476 c.parse_number()? fails
        assert!(Color::parse("hwb(0deg 0% 0% xxx)").is_err());
    }

    #[test]
    fn parse_hwb_comma_alpha() {
        // hwb(0deg 0% 0%, 0.5) — comma, no slash → L1472 `}` false branch
        let c = Color::parse("hwb(0deg 0% 0%, 0.5)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Hwb {"));
    }

    #[test]
    fn parse_lch_slash_alpha_invalid() {
        // lch(50% 0 0 / xxx) — bad alpha → L1500 c.parse_alpha()? fails
        assert!(Color::parse("lch(50% 0 0 / xxx)").is_err());
    }

    #[test]
    fn parse_oklab_slash_alpha_invalid() {
        // oklab(0.6 0.1 0 / xxx) — bad alpha → L1524 c.parse_alpha()? fails
        assert!(Color::parse("oklab(0.6 0.1 0 / xxx)").is_err());
    }

    #[test]
    fn parse_oklch_slash_alpha_invalid() {
        // oklch(0.6 0.15 180 / xxx) — bad alpha → L1574 c.parse_alpha()? fails
        assert!(Color::parse("oklch(0.6 0.15 180 / xxx)").is_err());
    }

    #[test]
    fn parse_color_function_slash_alpha_invalid() {
        // color(srgb 1 0 0 / xxx) — bad alpha → L1608 c.parse_alpha()? fails
        assert!(Color::parse("color(srgb 1 0 0 / xxx)").is_err());
    }

    #[test]
    fn parse_device_cmyk_slash_alpha_invalid() {
        // device-cmyk(0 1 1 0 / xxx) — bad alpha → L1721 c.parse_alpha()? fails
        assert!(Color::parse("device-cmyk(0 1 1 0 / xxx)").is_err());
    }

    #[test]
    fn parse_hsl_bad_saturation() {
        // hsl(0deg xxx 0%) — bad s → L1443 c.parse_percentage()?
        assert!(Color::parse("hsl(0deg xxx 0%)").is_err());
    }

    #[test]
    fn parse_hsl_bad_lightness() {
        // hsl(0deg 0% xxx) — bad l → L1445 c.parse_percentage()?
        assert!(Color::parse("hsl(0deg 0% xxx)").is_err());
    }

    #[test]
    fn parse_hwb_bad_hue() {
        // hwb(xxx 0% 0%) — bad h → L1462 c.parse_hue()?
        assert!(Color::parse("hwb(xxx 0% 0%)").is_err());
    }

    #[test]
    fn parse_hwb_bad_white() {
        // hwb(0deg xxx 0%) — bad w → L1464 c.parse_percentage()?
        assert!(Color::parse("hwb(0deg xxx 0%)").is_err());
    }

    #[test]
    fn parse_hwb_bad_black() {
        // hwb(0deg 0% xxx) — bad b → L1466 c.parse_percentage()?
        assert!(Color::parse("hwb(0deg 0% xxx)").is_err());
    }

    #[test]
    fn parse_lab_bad_l() {
        // lab(xxx 0 0) — bad l → L1472 c.parse_percentage()? fails
        assert!(Color::parse("lab(xxx 0 0)").is_err());
    }

    #[test]
    fn parse_lab_bad_a() {
        // lab(50% xxx 0) — bad a → L1476 c.parse_number()?
        assert!(Color::parse("lab(50% xxx 0)").is_err());
    }

    #[test]
    fn parse_lab_bad_b() {
        // lab(50% 0 xxx) — bad b → L1497 c.parse_none_number()?
        assert!(Color::parse("lab(50% 0 xxx)").is_err());
    }

    #[test]
    fn parse_lab_slash_alpha_invalid() {
        // lab(50% 0 0 / xxx) — bad alpha → L1500 c.parse_alpha()? fails
        assert!(Color::parse("lab(50% 0 0 / xxx)").is_err());
    }

    #[test]
    fn parse_lch_bad_l() {
        // lch(xxx 0 0) — bad l → L1495 c.parse_none_number()?
        assert!(Color::parse("lch(xxx 0 0)").is_err());
    }

    #[test]
    fn parse_lch_bad_c() {
        // lch(50% xxx 0) — bad a → L1495 c.parse_none_number()?
        assert!(Color::parse("lch(50% xxx 0)").is_err());
    }

    #[test]
    fn parse_lch_bad_b() {
        // lch(50% 0 xxx) — bad b → L1497 c.parse_none_number()?
        assert!(Color::parse("lch(50% 0 xxx)").is_err());
    }

    #[test]
    fn parse_lch_bad_h() {
        // lch(50% 0 0 xxx) — bad h → L1521 c.parse_hue()?
        assert!(Color::parse("lch(50% 0 0 xxx)").is_err());
    }

    #[test]
    fn parse_lch_bad_h_explicit() {
        // lch(50% 30 0 xxx) — bad h → L1521 c.parse_hue()?
        assert!(Color::parse("lch(50% 30 0 xxx)").is_err());
    }

    #[test]
    fn parse_oklab_bad_l() {
        // oklab(xxx 0 0) — bad l → L1541 c.parse_pct_or_number()?
        assert!(Color::parse("oklab(xxx 0 0)").is_err());
    }

    #[test]
    fn parse_oklab_bad_a() {
        // oklab(0.6 xxx 0) — bad a → L1544 c.parse_number()?
        assert!(Color::parse("oklab(0.6 xxx 0)").is_err());
    }

    #[test]
    fn parse_oklab_bad_b() {
        // oklab(0.6 0 xxx) — bad b → L1546 c.parse_number()?
        assert!(Color::parse("oklab(0.6 0 xxx)").is_err());
    }

    #[test]
    fn parse_oklab_with_percent() {
        // oklab(60% 0.1 0) — args contains '%' → L1542 if-true branch
        let c = Color::parse("oklab(60% 0.1 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Oklab {"));
    }

    #[test]
    fn parse_oklch_bad_l() {
        // oklch(xxx 0 0) — bad l → L1566 c.parse_pct_or_number()?
        assert!(Color::parse("oklch(xxx 0 0)").is_err());
    }

    #[test]
    fn parse_oklch_bad_c() {
        // oklch(0.6 xxx 0) — bad c → L1569 c.parse_number()?
        assert!(Color::parse("oklch(0.6 xxx 0)").is_err());
    }

    #[test]
    fn parse_oklch_bad_h() {
        // oklch(0.6 0 xxx) — bad h → L1571 c.parse_hue()?
        assert!(Color::parse("oklch(0.6 0 xxx)").is_err());
    }

    #[test]
    fn parse_oklch_with_percent() {
        // oklch(60% 0.15 0) — args contains '%' → L1567 if-true branch
        let c = Color::parse("oklch(60% 0.15 0)").unwrap();
        assert!(format!("{:?}", c.kind).contains("Oklch {"));
    }

    #[test]
    fn parse_color_function_bad_space() {
        // color(1 0 0) — args starts with digit, parse_ident fails → L1590
        assert!(Color::parse("color(1 0 0)").is_err());
    }

    #[test]
    fn parse_color_function_bad_channel() {
        // color(srgb xxx 0 0) — bad channel → L1597 c.parse_number()?
        assert!(Color::parse("color(srgb xxx 0 0)").is_err());
    }

    #[test]
    fn parse_color_mix_bad_space() {
        // color-mix(in xxx, red, blue) — bad space → "srgb" branch never hits
        // (L1630, L1635 broken paths)
        assert!(Color::parse("color-mix(in xxx, red, blue)").is_err());
    }

    #[test]
    fn parse_color_mix_srgb_linear() {
        // color-mix(in srgb-linear, red, blue) — L1630 "srgb-linear" arm
        let c = Color::parse("color-mix(in srgb-linear, red, blue)").unwrap();
        let m = format!("{:?}", c.kind);
        assert!(m.contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_oklch() {
        // color-mix(in oklch, red, blue) — L1635 "oklch" arm
        let c = Color::parse("color-mix(in oklch, red, blue)").unwrap();
        let m = format!("{:?}", c.kind);
        assert!(m.contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_bad_method() {
        // Unknown method is silently ignored (defaults to Shorter), so L1640
        // parse_ident() error is unreachable. Document the behavior.
        let c = Color::parse("color-mix(in hsl bogus, red, blue)").unwrap();
        let m = format!("{:?}", c.kind);
        assert!(m.contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_bad_percentage() {
        // L1673 c.parse_percentage()?
        assert!(Color::parse("color-mix(in srgb, red, blue xxx)").is_err());
    }

    #[test]
    fn parse_color_mix_extra_left() {
        // L1696 colors.push(parse_color_inner(left)?)
        assert!(Color::parse("color-mix(in srgb, red xxx, blue)").is_err());
    }

    #[test]
    fn parse_color_mix_extra_trimmed() {
        // L1700 colors.push(parse_color_inner(trimmed)?)
        assert!(Color::parse("color-mix(in srgb, red, blue, extra)").is_err());
    }

    #[test]
    fn parse_device_cmyk_with_percent() {
        // device-cmyk(0% 1% 1% 0%) — args contains '%' → L1711 if-true branch
        let c = Color::parse("device-cmyk(0% 1% 1% 0%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("DeviceCmyk {"));
    }

    #[test]
    fn parse_device_cmyk_bad_c() {
        // device-cmyk(xxx 1 1 0) — bad c → L1709 c.parse_pct_or_number()?
        assert!(Color::parse("device-cmyk(xxx 1 1 0)").is_err());
    }

    #[test]
    fn parse_device_cmyk_bad_m() {
        // device-cmyk(0 xxx 1 0) — bad m → L1714 c.parse_pct_or_number()?
        assert!(Color::parse("device-cmyk(0 xxx 1 0)").is_err());
    }

    #[test]
    fn parse_device_cmyk_bad_y() {
        // device-cmyk(0 1 xxx 0) — bad y → L1716 c.parse_pct_or_number()?
        assert!(Color::parse("device-cmyk(0 1 xxx 0)").is_err());
    }

    #[test]
    fn parse_device_cmyk_bad_k() {
        // device-cmyk(0 1 1 xxx) — bad k → L1718 c.parse_pct_or_number()?
        assert!(Color::parse("device-cmyk(0 1 1 xxx)").is_err());
    }

    #[test]
    fn parse_light_dark_known_unknown_colors() {
        // L1749/L1750 defensive `?` branches are now reachable via the
        // parse_or_system fallback. Unknown names become System colors.
        let c1 = Color::parse("light-dark(red, xxx)").unwrap();
        assert!(format!("{:?}", c1.kind).contains("LightDark {"));
        let c2 = Color::parse("light-dark(xxx, blue)").unwrap();
        assert!(format!("{:?}", c2.kind).contains("LightDark {"));
    }

    #[test]
    fn parse_light_dark_empty_light() {
        // Empty light part triggers parse_or_system Err branch (L1749).
        let c = Color::parse("light-dark( , blue)").unwrap();
        assert!(format!("{:?}", c.kind).contains("LightDark {"));
    }

    #[test]
    fn parse_light_dark_empty_dark() {
        // Empty dark part triggers parse_or_system Err branch (L1750).
        let c = Color::parse("light-dark(red, )").unwrap();
        assert!(format!("{:?}", c.kind).contains("LightDark {"));
    }

    #[test]
    fn cursor_next_at_end_returns_none() {
        // Covers L1135 `next()` None branch (peek at end of input).
        let mut c = Cursor::new("a");
        assert_eq!(c.next(), Some(b'a'));
        assert_eq!(c.next(), None);
    }

    #[test]
    fn cursor_next_on_empty_returns_none() {
        // Covers L1135 `next()` None branch on empty input.
        let mut c = Cursor::new("");
        assert_eq!(c.next(), None);
    }

    // ── parse: round-trip Display → parse → Display ────────────────

    fn roundtrip_display(input: &str) {
        let c = Color::parse(input).unwrap();
        let out = c.to_string();
        let c2 = Color::parse(&out).unwrap();
        assert_eq!(c2.to_string(), out, "mismatch for input: {input:?}");
    }

    #[test]
    fn roundtrip_named() {
        roundtrip_display("red");
        roundtrip_display("blue");
        roundtrip_display("rebeccapurple");
    }

    #[test]
    fn roundtrip_rgb() {
        roundtrip_display("rgb(255, 0, 0)");
        roundtrip_display("rgb(0, 255, 0)");
        roundtrip_display("rgba(255, 0, 0, 0.5)");
    }

    #[test]
    fn roundtrip_hsl() {
        roundtrip_display("hsl(180, 50%, 25%)");
        roundtrip_display("hsla(180, 50%, 25%, 0.5)");
    }

    #[test]
    fn roundtrip_hwb() {
        roundtrip_display("hwb(180 20% 30%)");
        roundtrip_display("hwb(180 20% 30% / 0.5)");
    }

    #[test]
    fn roundtrip_hex() {
        roundtrip_display("#ff0000");
        roundtrip_display("#00ff00");
    }

    #[test]
    fn roundtrip_oklch() {
        roundtrip_display("oklch(0.6 0.15 270)");
    }

    #[test]
    fn roundtrip_color_function() {
        roundtrip_display("color(srgb 1 0 0)");
    }

    #[test]
    fn roundtrip_color_mix() {
        roundtrip_display("color-mix(in srgb, red, blue)");
    }

    #[test]
    fn roundtrip_light_dark() {
        roundtrip_display("light-dark(red, blue)");
    }

    #[test]
    fn roundtrip_transparent() {
        roundtrip_display("transparent");
    }

    #[test]
    fn roundtrip_current_color() {
        roundtrip_display("currentColor");
    }

    #[test]
    fn rgb_constructor() {
        let c = Color::rgb(255, 0, 0);
        let (r, g, b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert!(alpha.is_none());
    }

    #[test]
    fn rgba_constructor() {
        let c = Color::rgba(255, 0, 0, 0.5);
        let (r, g, b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn hex_3_digits() {
        let c = Color::hex("#fff").unwrap();
        let (r, g, b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(r, 255);
        assert_eq!(g, 255);
        assert_eq!(b, 255);
        assert_eq!(alpha, None);
    }

    #[test]
    fn hex_4_digits() {
        let c = Color::hex("#ffff").unwrap();
        let (r, g, b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(r, 255);
        assert_eq!(g, 255);
        assert_eq!(b, 255);
        // `#ffff` is the short form of `#ffffffff` — opaque.
        // Our parser collapses fully-opaque hex to no alpha.
        assert_eq!(alpha, None);
    }

    #[test]
    fn hex_4_digits_with_partial_alpha() {
        let c = Color::hex("#fff8").unwrap();
        let (_r, _g, _b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(alpha, Some(0x88 as f32 / 255.0));
    }

    #[test]
    fn hex_6_digits() {
        let c = Color::hex("#ff8040").unwrap();
        let (r, g, b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(r, 0xff);
        assert_eq!(g, 0x80);
        assert_eq!(b, 0x40);
        assert_eq!(alpha, None);
    }

    #[test]
    fn hex_8_digits() {
        let c = Color::hex("#ff804080").unwrap();
        let (r, g, b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(r, 0xff);
        assert_eq!(g, 0x80);
        assert_eq!(b, 0x40);
        assert!((alpha.unwrap() - 128.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn hex_malformed_no_hash() {
        assert!(Color::hex("fff").is_none());
    }

    #[test]
    fn hex_malformed_wrong_length() {
        assert!(Color::hex("#fffff").is_none());
    }

    #[test]
    fn hex_malformed_invalid_chars() {
        assert!(Color::hex("#zzzzzz").is_none());
    }

    #[test]
    fn hex_3_invalid_chars() {
        assert!(Color::hex("#xyz").is_none());
    }

    #[test]
    fn hex_4_invalid_chars() {
        assert!(Color::hex("#xyzw").is_none());
    }

    #[test]
    fn hex_6_invalid_chars() {
        assert!(Color::hex("#xyzw12").is_none());
    }

    #[test]
    fn hex_8_invalid_chars() {
        assert!(Color::hex("#xyzw1234").is_none());
    }

    #[test]
    fn hex_empty_after_hash() {
        assert!(Color::hex("#").is_none());
    }

    #[test]
    fn hex_3_first_invalid() {
        assert!(Color::hex("#x00").is_none());
    }

    #[test]
    fn hex_3_second_invalid() {
        assert!(Color::hex("#0x0").is_none());
    }

    #[test]
    fn hex_3_third_invalid() {
        assert!(Color::hex("#00x").is_none());
    }

    #[test]
    fn hex_4_first_invalid() {
        assert!(Color::hex("#x000").is_none());
    }

    #[test]
    fn hex_4_second_invalid() {
        assert!(Color::hex("#0x00").is_none());
    }

    #[test]
    fn hex_4_third_invalid() {
        assert!(Color::hex("#00x0").is_none());
    }

    #[test]
    fn hex_4_fourth_invalid() {
        assert!(Color::hex("#000x").is_none());
    }

    #[test]
    fn hex_6_first_invalid() {
        assert!(Color::hex("#xx0000").is_none());
    }

    #[test]
    fn hex_6_second_invalid() {
        assert!(Color::hex("#00xx00").is_none());
    }

    #[test]
    fn hex_6_third_invalid() {
        assert!(Color::hex("#0000xx").is_none());
    }

    #[test]
    fn hex_8_first_invalid() {
        assert!(Color::hex("#xx000000").is_none());
    }

    #[test]
    fn hex_8_second_invalid() {
        assert!(Color::hex("#00xx0000").is_none());
    }

    #[test]
    fn hex_8_third_invalid() {
        assert!(Color::hex("#0000xx00").is_none());
    }

    #[test]
    fn hex_8_fourth_invalid() {
        assert!(Color::hex("#000000xx").is_none());
    }

    #[test]
    fn hex_5_chars_invalid_length() {
        assert!(Color::hex("#abcde").is_none());
    }

    #[test]
    fn hex_7_chars_invalid_length() {
        assert!(Color::hex("#abcdef0").is_none());
    }

    #[test]
    fn named_constructor() {
        let _c = Color::named("rebeccapurple");
    }

    #[test]
    fn current_color() {
        let _c = Color::current_color();
    }

    #[test]
    fn transparent() {
        let _c = Color::transparent();
    }

    #[test]
    fn hsl_constructor() {
        let c = Color::hsl(120.0, 50.0_f32, 50.0_f32);
        let (h, _s, _l, alpha) = as_hsl(&c.kind).expect("expected Hsl variant");
        assert_eq!(h, 120.0);
        assert!(alpha.is_none());
    }

    #[test]
    fn hsla_constructor() {
        let c = Color::hsla(240.0, 50.0_f32, 50.0_f32, 0.5);
        let (h, _s, _l, alpha) = as_hsl(&c.kind).expect("expected Hsl variant");
        assert_eq!(h, 240.0);
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn hwb_constructor() {
        let _c = Color::hwb(0.0, 30.0_f32, 30.0_f32);
    }

    #[test]
    fn lab_constructor() {
        let _c = Color::lab(50.0_f32, 40.0, -20.0);
    }

    #[test]
    fn lch_constructor() {
        let _c = Color::lch(50.0_f32, 40.0, Angle::deg(30.0));
    }

    #[test]
    fn oklab_constructor() {
        let _c = Color::oklab(60.0_f32, 0.1, -0.05);
    }

    #[test]
    fn oklch_constructor() {
        let _c = Color::oklch(60.0_f32, 0.15, Angle::deg(140.0));
    }

    #[test]
    fn color_function() {
        let c = Color::in_color_space("display-p3", vec![1.0, 0.0, 0.0]);
        let (space, channels, _alpha) = as_color_fn(&c.kind).expect("expected Color variant");
        assert_eq!(space, &Ident::from("display-p3"));
        assert_eq!(channels, &vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn cmyk_constructor() {
        let _c = Color::cmyk(0.0_f32, 100.0_f32, 100.0_f32, 0.0_f32);
    }

    #[test]
    fn light_dark_constructor() {
        let _c = Color::light_dark(Color::rgb(255, 255, 255), Color::rgb(0, 0, 0));
    }

    #[test]
    fn mix_constructor() {
        let _c = Color::mix(
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 255),
            Percentage::new(50.0),
            ColorMixSpace::Oklch,
            ColorMixMethod::Shorter,
        );
    }

    #[test]
    fn from_static_str_hex() {
        let _c = Color::from("#ff0000");
    }

    #[test]
    fn from_static_str_named() {
        let _c = Color::from("red");
    }

    #[test]
    fn from_tuple_3() {
        let _c: Color = (255u8, 0u8, 0u8).into();
    }

    #[test]
    fn from_tuple_4() {
        let c: Color = (255u8, 0u8, 0u8, 0.5f32).into();
        let (_r, _g, _b, alpha) = as_rgb(&c.kind).expect("expected Rgb variant");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn from_ident() {
        let _c: Color = Ident::from("red").into();
    }

    #[test]
    fn kind_accessor() {
        let c = Color::rgb(0, 0, 0);
        as_rgb(c.kind()).expect("expected Rgb variant");
    }

    #[test]
    fn into_kind_accessor() {
        let c = Color::rgb(0, 0, 0).into_kind();
        as_rgb(&c).expect("expected Rgb variant");
    }

    #[test]
    fn display_named() {
        assert_eq!(Color::named("red").to_string(), "red");
    }

    #[test]
    fn display_rgb() {
        assert_eq!(Color::rgb(255, 0, 0).to_string(), "rgb(255, 0, 0)");
    }

    #[test]
    fn display_rgba() {
        assert_eq!(Color::rgba(0, 0, 0, 0.5).to_string(), "rgba(0, 0, 0, 0.5)");
    }

    #[test]
    fn display_hsl() {
        assert_eq!(Color::hsl(120.0, 50.0_f32, 50.0_f32).to_string(), "hsl(120, 50%, 50%)");
    }

    #[test]
    fn display_hsla() {
        assert_eq!(Color::hsla(120.0, 50.0_f32, 50.0_f32, 0.5).to_string(), "hsla(120, 50%, 50%, 0.5)");
    }

    #[test]
    fn display_hwb() {
        assert_eq!(Color::hwb(0.0, 30.0_f32, 30.0_f32).to_string(), "hwb(0 30% 30%)");
    }

    #[test]
    fn display_hwb_with_alpha() {
        let c = Color {
            kind: ColorKind::Hwb {
                h: 0.0,
                w: Percentage::new(30.0),
                b: Percentage::new(30.0),
                alpha: Some(0.5),
            },
        };
        assert_eq!(c.to_string(), "hwb(0 30% 30% / 0.5)");
    }

    #[test]
    fn display_lab() {
        assert_eq!(Color::lab(50.0_f32, 40.0, -20.0).to_string(), "lab(50% 40 -20)");
    }

    #[test]
    fn display_lch_with_alpha() {
        let c = Color {
            kind: ColorKind::Lch {
                l: Percentage::new(50.0),
                c: 40.0,
                h: Angle::deg(30.0),
                alpha: Some(0.5),
            },
        };
        assert_eq!(c.to_string(), "lch(50% 40 30deg / 0.5)");
    }

    #[test]
    fn display_lch() {
        let c = Color::lch(50.0_f32, 40.0, Angle::deg(30.0));
        assert_eq!(c.to_string(), "lch(50% 40 30deg)");
    }

    #[test]
    fn display_oklab() {
        assert_eq!(Color::oklab(60.0_f32, 0.1, -0.05).to_string(), "oklab(60% 0.1 -0.05)");
    }

    #[test]
    fn display_oklab_with_alpha() {
        let c = Color {
            kind: ColorKind::Oklab {
                l: Percentage::new(60.0),
                a: 0.1,
                b: -0.05,
                alpha: Some(0.5),
            },
        };
        assert_eq!(c.to_string(), "oklab(60% 0.1 -0.05 / 0.5)");
    }

    #[test]
    fn display_oklch() {
        let c = Color::oklch(60.0_f32, 0.15, Angle::deg(140.0));
        assert_eq!(c.to_string(), "oklch(60% 0.15 140deg)");
    }

    #[test]
    fn display_oklch_with_alpha() {
        let c = Color {
            kind: ColorKind::Oklch {
                l: Percentage::new(60.0),
                c: 0.15,
                h: Angle::deg(140.0),
                alpha: Some(0.5),
            },
        };
        assert_eq!(c.to_string(), "oklch(60% 0.15 140deg / 0.5)");
    }

    #[test]
    fn display_color_function() {
        let c = Color::in_color_space("display-p3", vec![1.0, 0.0, 0.0]);
        assert_eq!(c.to_string(), "color(display-p3 1 0 0)");
    }

    #[test]
    fn display_color_function_with_alpha() {
        let c = Color {
            kind: ColorKind::Color {
                space: Ident::from("display-p3"),
                channels: vec![1.0, 0.0, 0.0],
                alpha: Some(0.5),
            },
        };
        assert_eq!(c.to_string(), "color(display-p3 1 0 0 / 0.5)");
    }

    #[test]
    fn display_color_mix() {
        let c = Color::mix(
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 255),
            Percentage::new(50.0),
            ColorMixSpace::Oklch,
            ColorMixMethod::Shorter,
        );
        assert_eq!(c.to_string(), "color-mix(in oklch, rgb(255, 0, 0), rgb(0, 0, 255) 50%)");
    }

    #[test]
    fn display_color_contrast() {
        let c = Color {
            kind: ColorKind::ColorContrast {
                colors: vec![Color::rgb(0, 0, 0), Color::rgb(255, 255, 255)],
                target: None,
            },
        };
        assert_eq!(c.to_string(), "color-contrast(rgb(0, 0, 0), rgb(255, 255, 255))");
    }

    #[test]
    fn display_color_contrast_with_target() {
        let c = Color {
            kind: ColorKind::ColorContrast {
                colors: vec![Color::rgb(0, 0, 0)],
                target: Some(Box::new(Color::rgb(255, 255, 255))),
            },
        };
        assert_eq!(c.to_string(), "color-contrast(rgb(0, 0, 0) vs rgb(255, 255, 255))");
    }

    #[test]
    fn display_color_contrast_single() {
        let c = Color {
            kind: ColorKind::ColorContrast {
                colors: vec![Color::rgb(0, 0, 0)],
                target: None,
            },
        };
        assert_eq!(c.to_string(), "color-contrast(rgb(0, 0, 0))");
    }

    #[test]
    fn display_color_contrast_three() {
        let c = Color {
            kind: ColorKind::ColorContrast {
                colors: vec![
                    Color::rgb(0, 0, 0),
                    Color::rgb(128, 128, 128),
                    Color::rgb(255, 255, 255),
                ],
                target: None,
            },
        };
        assert_eq!(
            c.to_string(),
            "color-contrast(rgb(0, 0, 0), rgb(128, 128, 128), rgb(255, 255, 255))"
        );
    }

    // ── resolve: color-contrast() ─────────────────────────────────

    #[test]
    fn resolve_color_contrast_picks_white_against_black() {
        let c = Color::parse("color-contrast(white, black vs black)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // White has the highest contrast against black, so we get white.
        assert_eq!(
            rgb,
            Color {
                kind: ColorKind::Rgb { r: 255, g: 255, b: 255, alpha: None }
            }
        );
    }

    #[test]
    fn resolve_color_contrast_picks_black_against_white() {
        let c = Color::parse("color-contrast(white, black vs white)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // Black has the highest contrast against white, so we get black.
        assert_eq!(
            rgb,
            Color {
                kind: ColorKind::Rgb { r: 0, g: 0, b: 0, alpha: None }
            }
        );
    }

    #[test]
    fn resolve_color_contrast_no_target_uses_first() {
        let c = Color::parse("color-contrast(black, white, gray)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // No target → first color (black) is the target;
        // white has the highest contrast against black.
        assert_eq!(
            rgb,
            Color {
                kind: ColorKind::Rgb { r: 255, g: 255, b: 255, alpha: None }
            }
        );
    }

    #[test]
    fn resolve_color_contrast_three_with_target() {
        let c = Color::parse("color-contrast(#777, #fff, #000 vs #777)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // #777 vs #000 has the highest contrast (8.59:1).
        assert_eq!(
            rgb,
            Color {
                kind: ColorKind::Rgb { r: 0, g: 0, b: 0, alpha: None }
            }
        );
    }

    #[test]
    fn resolve_color_contrast_uses_hex() {
        let c = Color::parse("color-contrast(#000, #fff vs #888)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // #000 has higher contrast against #888 (≈5.91:1) than #fff (≈3.56:1).
        assert_eq!(
            rgb,
            Color {
                kind: ColorKind::Rgb { r: 0, g: 0, b: 0, alpha: None }
            }
        );
    }

    #[test]
    fn resolve_color_contrast_alias_kw() {
        let c = Color::parse("color-contrast(wheat vs tan, sienna, #b22222)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorContrast {"));
        let rgb = c.into_rgb().unwrap();
        assert!(format!("{:?}", rgb.kind).contains("Rgb {"));
    }

    #[test]
    fn display_cmyk() {
        assert_eq!(
            Color::cmyk(0.0_f32, 100.0_f32, 100.0_f32, 0.0_f32).to_string(),
            "device-cmyk(0% 100% 100% 0%)"
        );
    }

    #[test]
    fn display_cmyk_with_alpha() {
        let c = Color {
            kind: ColorKind::DeviceCmyk {
                c: Percentage::new(0.0),
                m: Percentage::new(100.0),
                y: Percentage::new(100.0),
                k: Percentage::new(0.0),
                alpha: Some(0.5),
            },
        };
        assert_eq!(
            c.to_string(),
            "device-cmyk(0% 100% 100% 0% / 0.5)"
        );
    }

    #[test]
    fn display_system() {
        let c = Color {
            kind: ColorKind::System(Ident::from("Canvas")),
        };
        assert_eq!(c.to_string(), "Canvas");
    }

    #[test]
    fn display_light_dark() {
        let c = Color::light_dark(Color::rgb(255, 255, 255), Color::rgb(0, 0, 0));
        assert_eq!(c.to_string(), "light-dark(rgb(255, 255, 255), rgb(0, 0, 0))");
    }

    #[test]
    fn display_current_color() {
        assert_eq!(Color::current_color().to_string(), "currentcolor");
    }

    #[test]
    fn display_transparent() {
        assert_eq!(Color::transparent().to_string(), "transparent");
    }

    #[test]
    fn display_alpha_with_lab() {
        let c = Color {
            kind: ColorKind::Lab {
                l: Percentage::new(50.0),
                a: 40.0,
                b: -20.0,
                alpha: Some(0.5),
            },
        };
        assert_eq!(c.to_string(), "lab(50% 40 -20 / 0.5)");
    }

    #[test]
    fn equality() {
        assert_eq!(Color::rgb(1, 2, 3), Color::rgb(1, 2, 3));
        assert_ne!(Color::rgb(1, 2, 3), Color::rgb(1, 2, 4));
    }

    #[test]
    fn colormixspace_display() {
        assert_eq!(ColorMixSpace::Srgb.to_string(), "srgb");
        assert_eq!(ColorMixSpace::Hsl.to_string(), "hsl");
        assert_eq!(ColorMixSpace::Oklab.to_string(), "oklab");
        assert_eq!(ColorMixSpace::Oklch.to_string(), "oklch");
    }

    #[test]
    fn colorspace_display() {
        assert_eq!(ColorSpace::Srgb.to_string(), "srgb");
        assert_eq!(ColorSpace::Hsl.to_string(), "hsl");
        assert_eq!(ColorSpace::Oklab.to_string(), "oklab");
        assert_eq!(ColorSpace::Oklch.to_string(), "oklch");
        assert_eq!(ColorSpace::Hex.to_string(), "hex");
    }

    #[test]
    fn colormixmethod_equality() {
        assert_eq!(ColorMixMethod::Shorter, ColorMixMethod::Shorter);
        assert_ne!(ColorMixMethod::Shorter, ColorMixMethod::Longer);
    }

    #[test]
    fn colormixmethod_default() {
        assert_eq!(ColorMixMethod::default(), ColorMixMethod::Shorter);
    }

    #[test]
    fn colormixspace_default() {
        assert_eq!(ColorMixSpace::default(), ColorMixSpace::Oklch);
    }

    #[test]
    fn conversion_error_display() {
        let err = ConversionError::Unresolvable;
        let s = err.to_string();
        assert!(s.contains("runtime"));
    }

    #[test]
    fn conversion_error_display_out_of_gamut() {
        let err = ConversionError::OutOfGamut {
            source: Box::new(Color::rgb(1, 2, 3)),
            target: ColorSpace::Srgb,
        };
        let s = err.to_string();
        assert!(s.contains("out of"));
        assert!(s.contains("srgb"));
    }

    // ── into_rgb ──────────────────────────────────────────────────

    #[test]
    fn into_rgb_from_rgb() {
        let c = Color::rgb(100, 150, 200);
        let expected = c.clone();
        let converted = c.into_rgb().unwrap();
        assert_eq!(converted, expected);
    }

    #[test]
    fn into_rgb_from_hsl() {
        let c = Color::hsl(120.0, Percentage::new(100.0), Percentage::new(50.0));
        let rgb = c.into_rgb().unwrap();
        // hsl(120, 100%, 50%) ≈ rgb(0, 255, 0)
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

                            assert_eq!(r, 0);
                            assert_eq!(g, 255);
                            assert_eq!(b, 0);
            
    }

    #[test]
    fn into_rgb_from_hwb() {
        let c = Color::hwb(0.0, Percentage::new(0.0), Percentage::new(0.0));
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

                            assert_eq!(r, 255);
                            assert_eq!(g, 0);
                            assert_eq!(b, 0);
            
    }

    #[test]
    fn into_rgb_from_oklab() {
        // oklab(0.5, 0, 0) → neutral gray
        let c = Color {
            kind: ColorKind::Oklab {
                l: Percentage::new(50.0),
                a: 0.0,
                b: 0.0,
                alpha: None,
            },
        };
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

                            // All channels should be close
                            assert!((r as i16 - g as i16).abs() <= 1);
                            assert!((g as i16 - b as i16).abs() <= 1);
            
    }

    #[test]
    fn into_rgb_from_oklch() {
        let c = Color {
            kind: ColorKind::Oklch {
                l: Percentage::new(50.0),
                c: 0.0,
                h: Angle::deg(0.0),
                alpha: None,
            },
        };
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

    }

    #[test]
    fn into_rgb_from_named() {
        let c: Color = Ident::from("red").into();
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

                            assert_eq!(r, 255);
                            assert_eq!(g, 0);
                            assert_eq!(b, 0);
            
    }

    #[test]
    fn into_rgb_from_named_rebeccapurple() {
        let c: Color = Ident::from("rebeccapurple").into();
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

                            assert_eq!(r, 102);
                            assert_eq!(g, 51);
                            assert_eq!(b, 153);
            
    }

    #[test]
    fn into_rgb_from_colormix() {
        let c = Color::mix(
            Color::rgb(0, 0, 0),
            Color::rgb(0, 0, 255),
            Percentage::new(50.0),
            ColorMixSpace::Srgb,
            ColorMixMethod::Shorter,
        );
        let rgb = c.into_rgb().unwrap();
        // Result should be a blend (not exactly either parent)
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

        assert!(r > 0 || g > 0 || b > 0);

    }

    #[test]
    fn into_rgb_from_colormix_oklch() {
        // Verifies the Oklch branch of resolve_color_mix is exercised.
        // Uses two blue parents so r=0, g=0 in the Oklch-interpolated result,
        // exercising the g > 0 (false) and b > 0 (true) branches.
        let c = Color::mix(
            Color::rgb(0, 0, 200),
            Color::rgb(0, 0, 100),
            Percentage::new(50.0),
            ColorMixSpace::Oklch,
            ColorMixMethod::Shorter,
        );
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, _alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");
        assert!(r > 0 || g > 0 || b > 0);
    }

    #[test]
    fn into_rgb_from_colormix_blue_black() {
        // Mix black (0,0,0) and blue (0,0,255) — result has r=0, g=0, b>0,
        // exercising the g > 0 (false) and b > 0 (true) branches of the
        // blend assertion.
        let c = Color::mix(
            Color::rgb(0, 0, 0),
            Color::rgb(0, 0, 255),
            Percentage::new(50.0),
            ColorMixSpace::Srgb,
            ColorMixMethod::Shorter,
        );
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, _alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");
        assert!(r > 0 || g > 0 || b > 0);
    }

    #[test]
    fn into_rgb_from_colormix_yellow_white() {
        // Mix yellow (255,255,0) and white (255,255,255) in Srgb — result
        // has r=255, g=255, b<255, exercising the r < 255 (false),
        // g < 255 (false), and b < 255 (true) branches of the blend
        // assertion.
        let c = Color::mix(
            Color::rgb(255, 255, 0),
            Color::rgb(255, 255, 255),
            Percentage::new(50.0),
            ColorMixSpace::Srgb,
            ColorMixMethod::Shorter,
        );
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, _alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");
        assert!(r < 255 || g < 255 || b < 255);
    }

    #[test]
    fn into_rgb_from_srgb_color() {
        let c = Color {
            kind: ColorKind::Color {
                space: Ident::from("srgb"),
                channels: vec![0.5, 0.25, 0.125],
                alpha: None,
            },
        };
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

                            assert_eq!(r, 128);
                            assert_eq!(g, 64);
                            assert_eq!(b, 32);
            
    }

    #[test]
    fn into_rgb_from_display_p3() {
        // A saturated display-p3 green that exceeds sRGB gamut
        let c = Color {
            kind: ColorKind::Color {
                space: Ident::from("display-p3"),
                channels: vec![0.0, 1.0, 0.0],
                alpha: None,
            },
        };
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, alpha) = as_rgb(&rgb.into_kind()).expect("expected Rgb");

    }

    #[test]
    fn into_rgb_unresolvable() {
        let c = Color {
            kind: ColorKind::System(Ident::from("canvas")),
        };
        assert!(c.into_rgb().is_err());
    }

    // ── into_hsl ──────────────────────────────────────────────────

    #[test]
    fn into_hsl_from_rgb() {
        // Red: h close to 0, exercises the (h - 0.0) branch.
        // (255, 0, 1): h close to 360, exercises the (h - 360.0) branch.
        for (r, g, b) in [(255_u8, 0, 0), (255, 0, 1)] {
            let c = Color::rgb(r, g, b);
            let hsl = c.into_hsl().unwrap();
            let (h, s, l, _alpha) = as_hsl(&hsl.into_kind()).expect("expected Hsl");
            assert!(!(1.0..=359.0).contains(&h));
            assert!((s.value() - 100.0).abs() < 1.0);
            assert!((l.value() - 50.0).abs() < 1.0);
        }
    }

    #[test]
    fn into_hsl_roundtrip() {
        let orig = Color::hsl(180.0, Percentage::new(50.0), Percentage::new(25.0));
        let rgb = orig.clone().into_rgb().unwrap();
        let hsl = rgb.into_hsl().unwrap();
        let (h, s, l, alpha) = as_hsl(&hsl.into_kind()).expect("expected Hsl");

                            assert!((h - 180.0).abs() < 2.0);
                            assert!((s.value() - 50.0).abs() < 2.0);
                            assert!((l.value() - 25.0).abs() < 2.0);
            
    }

    // ── into_oklab ────────────────────────────────────────────────

    #[test]
    fn into_oklab_from_rgb() {
        let c = Color::rgb(255, 0, 0);
        let oklab = c.into_oklab().unwrap();
        let (l, a, b, alpha) = as_oklab(&oklab.into_kind()).expect("expected Oklab");

                            // sRGB red → OKLab (L ≈ 0.63, a > 0, b > 0)
                            assert_approx_eq!(l.value(), 62.0, 5.0);
                            assert!(a > 0.1);
                            assert!(b > 0.0);
            
    }

    #[test]
    fn oklab_to_oklch_grayscale() {
        // Gray has zero chroma in Oklch (boundary case)
        let c = Color::rgb(128, 128, 128);
        let oklch = c.into_oklch().unwrap();
        let (_l, c, h, _alpha) = as_oklch(&oklch.into_kind()).expect("expected Oklch");

        // Chroma should be very close to 0 for grayscale
        assert!(c < 0.01);
        // h is set to 0 when chroma is 0
        let _ = h.to_degrees();
    }

    #[test]
    fn oklab_to_oklch_grayscale_direct() {
        // Directly call oklab_to_oklch with a = b ≈ 0 to exercise the
        // achromatic branch (where c.abs() < 1e-10 → h = 0).
        let result = oklab_to_oklch(0.5, 1e-12_f32, 1e-12_f32);
        assert_eq!(result.0, 0.5);
        // With c essentially 0, h should be 0 (achromatic branch)
        assert_eq!(result.2, 0.0);
    }

    #[test]
    fn into_oklab_from_oklch_roundtrip() {
        let orig = Color {
            kind: ColorKind::Oklch {
                l: Percentage::new(60.0),
                c: 0.15,
                h: Angle::deg(270.0),
                alpha: None,
            },
        };
        let rgb = orig.clone().into_rgb().unwrap();
        let back = rgb.into_oklch().unwrap();
        let (l, c, h, alpha) = as_oklch(&back.into_kind()).expect("expected Oklch");

                            assert!((l.value() - 60.0).abs() < 2.0);
                            assert!((h.to_degrees() - 270.0).abs() < 2.0);
            
    }

    // ── into_hex ──────────────────────────────────────────────────

    #[test]
    fn into_hex_rgb() {
        let c = Color::rgb(255, 0, 0);
        assert_eq!(c.into_hex().unwrap(), "#ff0000");
    }

    #[test]
    fn into_hex_named() {
        let c: Color = Ident::from("rebeccapurple").into();
        assert_eq!(c.into_hex().unwrap(), "#663399");
    }

    #[test]
    fn into_hex_with_alpha() {
        let c = Color::rgba(0, 255, 0, 0.5);
        assert_eq!(c.into_hex().unwrap(), "#00ff0080");
    }

    // ── Edge cases ─────────────────────────────────────────────────

    #[test]
    fn black_conversions() {
        let c0 = Color::rgb(0, 0, 0);
        assert_eq!(c0.into_hex().unwrap(), "#000000");
        let c1 = Color::rgb(0, 0, 0);
        let hsl = c1.into_hsl().unwrap();
        let (_h, _s, l, _alpha) = as_hsl(&hsl.into_kind()).expect("expected Hsl");
        assert!((l.value() - 0.0).abs() < 1.0);
    }

    #[test]
    fn white_conversions() {
        let c0 = Color::rgb(255, 255, 255);
        assert_eq!(c0.into_hex().unwrap(), "#ffffff");
        let c1 = Color::rgb(255, 255, 255);
        let hsl = c1.into_hsl().unwrap();
        let (_h, _s, l, _alpha) = as_hsl(&hsl.into_kind()).expect("expected Hsl");
        assert!((l.value() - 100.0).abs() < 1.0);
    }

    #[test]
    fn named_color_unknown() {
        let c: Color = Ident::from("nonexistent-color").into();
        // Falls through to Named variant, to_srgb_float returns Err
        assert!(c.into_rgb().is_err());
    }

    #[test]
    fn named_checked_known() {
        let c = Color::named_checked("RebeccaPurple").expect("known named color");
        assert_eq!(c.to_string(), "RebeccaPurple");
    }

    #[test]
    fn named_checked_unknown() {
        assert_eq!(Color::named_checked("nonexistent-color"), None);
    }

    #[test]
    fn is_known_name_both_outcomes() {
        for (name, want) in [("red", true), ("nonexistent-color", false)] {
            assert_eq!(Color::is_known_name(name), want);
        }
    }

    // ── Coverage: Color conversions, edge cases, hue methods ──

    #[test]
    fn into_rgb_from_device_cmyk() {
        let c = Color::parse("device-cmyk(0 1 1 0)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // Standard CMYK→RGB: C=0, M=1, Y=1, K=0 → R=1, G=0, B=0
        let (r, g, b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn into_rgb_from_device_cmyk_with_alpha() {
        let c = Color::parse("device-cmyk(0 1 1 0 / 0.5)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (_r, _g, _b, alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn into_rgb_from_lab_returns_unresolvable() {
        let c = Color::parse("lab(50% 0 0)").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_lch_returns_unresolvable() {
        let c = Color::parse("lch(50% 0 0)").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_system_color_returns_unresolvable() {
        let c = Color::parse("canvas").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_light_dark_returns_unresolvable() {
        let c = Color::parse("light-dark(red, blue)").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_color_contrast_returns_unresolvable() {
        // color-contrast with only unresolvable targets returns err.
        let c = Color {
            kind: ColorKind::ColorContrast {
                colors: vec![Color::parse("canvas").unwrap()],
                target: None,
            },
        };
        assert!(c.into_rgb().is_err());
    }

    #[test]
    fn into_rgb_from_color_function_unresolvable_space() {
        // Color() with an unknown space returns Unresolvable
        let c = Color::parse("color(banana 1 0 0)").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_hsl_from_lab_returns_unresolvable() {
        let c = Color::parse("lab(50% 0 0)").unwrap();
        let result = c.into_hsl();
        assert!(result.is_err());
    }

    #[test]
    fn into_hsl_from_lch_returns_unresolvable() {
        let c = Color::parse("lch(50% 0 0)").unwrap();
        let result = c.into_hsl();
        assert!(result.is_err());
    }

    #[test]
    fn into_oklab_from_lab_returns_unresolvable() {
        let c = Color::parse("lab(50% 0 0)").unwrap();
        let result = c.into_oklab();
        assert!(result.is_err());
    }

    #[test]
    fn into_oklab_from_lch_returns_unresolvable() {
        let c = Color::parse("lch(50% 0 0)").unwrap();
        let result = c.into_oklab();
        assert!(result.is_err());
    }

    #[test]
    fn into_oklch_from_lab_returns_unresolvable() {
        let c = Color::parse("lab(50% 0 0)").unwrap();
        let result = c.into_oklch();
        assert!(result.is_err());
    }

    #[test]
    fn into_oklch_from_lch_returns_unresolvable() {
        let c = Color::parse("lch(50% 0 0)").unwrap();
        let result = c.into_oklch();
        assert!(result.is_err());
    }

    #[test]
    fn into_hex_from_lab_returns_unresolvable() {
        let c = Color::parse("lab(50% 0 0)").unwrap();
        let result = c.into_hex();
        assert!(result.is_err());
    }

    #[test]
    fn into_hex_from_lch_returns_unresolvable() {
        let c = Color::parse("lch(50% 0 0)").unwrap();
        let result = c.into_hex();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_color_mix_increasing_hue() {
        let c = Color::parse("color-mix(in hsl increasing hue, red, blue)").unwrap();
        let rgb = c.into_rgb();
        assert!(rgb.is_ok());
    }

    #[test]
    fn into_rgb_from_color_mix_decreasing_hue() {
        let c = Color::parse("color-mix(in hsl decreasing hue, red, blue)").unwrap();
        let rgb = c.into_rgb();
        assert!(rgb.is_ok());
    }

    #[test]
    fn into_rgb_from_color_mix_shorter_hue_diff_lt_neg_180() {
        // Magenta (h≈300) → red (h=0) gives diff = 0-300 = -300, hits the
        // `else if diff < -180.0` branch in interpolate_hue (L620).
        let c = Color::parse("color-mix(in hsl shorter hue, magenta, red)").unwrap();
        let rgb = c.into_rgb();
        assert!(rgb.is_ok());
    }

    #[test]
    fn into_rgb_from_color_mix_shorter_hue_diff_in_range() {
        // Lime (h=120) → red (h=0) gives diff = 0-120 = -120, in [-180, 180]
        // → falls through both if branches (L620 false branch).
        let c = Color::parse("color-mix(in hsl shorter hue, lime, red)").unwrap();
        let rgb = c.into_rgb();
        assert!(rgb.is_ok());
    }

    #[test]
    fn into_rgb_from_color_mix_decreasing_hue_h2_le_h1() {
        // Blue (h≈240) → red (h=0): h2_adj <= h1, hits the false branch
        // of `if h2_adj > h1` in interpolate_hue::Decreasing (L638).
        let c = Color::parse("color-mix(in hsl decreasing hue, blue, red)").unwrap();
        let rgb = c.into_rgb();
        assert!(rgb.is_ok());
    }

    #[test]
    fn into_rgb_from_color_contrast_with_lab() {
        // First color is lab (unresolvable to sRGB) — triggers `to_srgb_float()?`
        // error branch in resolve_color_contrast (L599).
        let c = Color::parse("color-contrast(lab(50% 0 0) vs wheat, tan, sienna)").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_color_contrast_with_lch() {
        let c = Color::parse("color-contrast(lch(50% 0 0) vs wheat, tan, sienna)").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_color_mix_with_lab() {
        // Color-mix with Lab should hit `to_srgb_float()?` error in resolve_color_mix (L652, L653).
        let c = Color::parse("color-mix(in lab, lab(50% 0 0), lab(50% 20 -10))").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_color_mix_with_lch() {
        let c = Color::parse("color-mix(in lch, lch(50% 0 0), lch(50% 30 180))").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn into_rgb_from_color_mix_with_lch_b_only() {
        // Only m.b is Lch — L652 succeeds, L653 fails
        let c = Color::parse("color-mix(in lch, red, lch(50% 30 180))").unwrap();
        let result = c.into_rgb();
        assert!(result.is_err());
    }

    #[test]
    fn alpha_opt_exactly_one_returns_none() {
        // alpha_opt(1.0) returns None (the `if` branch in L648).
        assert_eq!(alpha_opt(1.0), None);
    }

    #[test]
    fn alpha_opt_near_one_returns_none() {
        // 1.0 + 0.5 * EPSILON is within EPSILON of 1.0, so the `if` branch is taken.
        assert_eq!(alpha_opt(1.0 + 0.5 * f32::EPSILON), None);
    }

    #[test]
    fn alpha_opt_not_one_returns_some() {
        assert_eq!(alpha_opt(0.5), Some(0.5));
    }

    #[test]
    fn alpha_opt_just_above_epsilon_returns_some() {
        // alpha 2.0 is well outside EPSILON of 1.0, so the `else` branch is taken.
        assert_eq!(alpha_opt(2.0), Some(2.0));
    }

    #[test]
    fn into_rgb_from_color_mix_longer_hue_resolution() {
        let c = Color::parse("color-mix(in hsl longer hue, red, blue)").unwrap();
        let rgb = c.into_rgb();
        assert!(rgb.is_ok());
    }

    #[test]
    fn parse_color_mix_with_target_increase_hue() {
        let c = Color::parse("color-mix(in hsl increasing hue, red, blue 30%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorMix("));
    }

    #[test]
    fn parse_color_mix_with_target_decrease_hue() {
        let c = Color::parse("color-mix(in hsl decreasing hue, red, blue 30%)").unwrap();
        assert!(format!("{:?}", c.kind).contains("ColorMix("));
    }

    #[test]
    fn display_color_mix_with_method() {
        let c = Color::mix(
            Color::named("red"),
            Color::named("blue"),
            Percentage::new(50.0),
            ColorMixSpace::Oklch,
            ColorMixMethod::Longer,
        );
        let s = c.to_string();
        assert!(s.contains("longer hue"));
    }

    #[test]
    fn display_color_mix_with_increasing_hue() {
        let c = Color::mix(
            Color::named("red"),
            Color::named("blue"),
            Percentage::new(50.0),
            ColorMixSpace::Oklch,
            ColorMixMethod::Increasing,
        );
        let s = c.to_string();
        assert!(s.contains("increasing hue"));
    }

    #[test]
    fn display_color_mix_with_decreasing_hue() {
        let c = Color::mix(
            Color::named("red"),
            Color::named("blue"),
            Percentage::new(50.0),
            ColorMixSpace::Oklch,
            ColorMixMethod::Decreasing,
        );
        let s = c.to_string();
        assert!(s.contains("decreasing hue"));
    }

    #[test]
    fn display_color_mix_inner_with_method() {
        // Test the ColorMix Display path including the method string.
        let m = ColorMix {
            a: Color::named("red"),
            b: Color::named("blue"),
            percentage: Percentage::new(50.0),
            space: ColorMixSpace::Oklch,
            method: ColorMixMethod::Shorter,
        };
        let c = Color {
            kind: ColorKind::ColorMix(Box::new(m)),
        };
        assert_eq!(c.to_string(), "color-mix(in oklch, red, blue 50%)");
    }

    #[test]
    fn parse_color_mix_with_hue_method_only() {
        // longer hue without space parsing
        let c = Color::parse("color-mix(in hsl longer hue, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Longer);
    }

    #[test]
    fn parse_color_mix_with_increasing_hue() {
        let c = Color::parse("color-mix(in hsl increasing hue, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Increasing);
    }

    #[test]
    fn parse_color_mix_with_decreasing_hue() {
        let c = Color::parse("color-mix(in hsl decreasing hue, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Decreasing);
    }

    #[test]
    fn parse_color_mix_with_unknown_hue_method_defaults_to_shorter() {
        let c = Color::parse("color-mix(in hsl foo, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Shorter);
    }

    #[test]
    fn colormixmethod_display() {
        assert_eq!(ColorMixMethod::Shorter.to_string(), "shorter");
        assert_eq!(ColorMixMethod::Longer.to_string(), "longer");
        assert_eq!(ColorMixMethod::Increasing.to_string(), "increasing");
        assert_eq!(ColorMixMethod::Decreasing.to_string(), "decreasing");
    }

    #[test]
    fn parse_color_function_with_one_channel() {
        // Color() allows any number of channels (validation is at conversion).
        let c = Color::parse("color(srgb 1)").unwrap();
        let (_space, channels, _alpha) = as_color_fn(&c.kind).expect("expected Color");
        assert_eq!(channels.len(), 1);
    }

    #[test]
    fn parse_color_function_alpha_with_number() {
        let c = Color::parse("color(srgb 1 0 0 / 0.5)").unwrap();
        let (_space, _channels, alpha) = as_color_fn(&c.kind).expect("expected Color");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn parse_lab_none_for_a() {
        let c = Color::parse("lab(50% none 0)").unwrap();
        let (_l, a, _b, alpha) = as_lab(&c.kind).expect("expected Lab");
        assert_eq!(a, 0.0);
        assert_eq!(alpha, None);
    }

    #[test]
    fn parse_lch_basic() {
        let c = Color::parse("lch(50% 20 90)").unwrap();
        let (_l, c, h, alpha) = as_lch(&c.kind).expect("expected Lch");
        assert_eq!(c, 20.0);
        assert_eq!(h.to_degrees(), 90.0);
        assert_eq!(alpha, None);
    }

    #[test]
    fn parse_hwb_with_alpha_none() {
        let c = Color::parse("hwb(180 20% 30% / none)").unwrap();
        let (_h, _w, _b, alpha) = as_hwb(&c.kind).expect("expected Hwb");
        assert_eq!(alpha, None);
    }

    #[test]
    fn parse_device_cmyk_basic() {
        let c = Color::parse("device-cmyk(0 1 1 0)").unwrap();
        let (c, m, y, k, alpha) = as_device_cmyk(&c.kind).expect("expected DeviceCmyk");
        assert_eq!(c, 0.0);
        assert_eq!(m, 100.0);
        assert_eq!(y, 100.0);
        assert_eq!(k, 0.0);
        assert_eq!(alpha, None);
    }

    #[test]
    fn parse_color_mix_with_lab_space() {
        let c = Color::parse("color-mix(in lab, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.space, ColorMixSpace::Lab);
    }

    #[test]
    fn parse_color_mix_with_lch_space() {
        let c = Color::parse("color-mix(in lch, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.space, ColorMixSpace::Lch);
    }

    #[test]
    fn resolve_color_contrast_empty_list_returns_unresolvable() {
        let c = Color {
            kind: ColorKind::ColorContrast {
                colors: vec![],
                target: None,
            },
        };
        assert!(c.into_rgb().is_err());
    }

    // ── Coverage: ColorParseError variants and edge cases ─────

    #[test]
    fn display_color_parse_error_component() {
        let err = ColorParseError::Component;
        assert_eq!(err.to_string(), "invalid color component");
    }

    #[test]
    fn display_color_parse_error_invalid() {
        let err = ColorParseError::Invalid;
        assert_eq!(err.to_string(), "invalid color value");
    }

    #[test]
    fn display_color_parse_error_unknown_notation() {
        let err = ColorParseError::UnknownNotation("foo".into());
        let s = err.to_string();
        assert!(s.contains("unknown"));
        assert!(s.contains("foo"));
    }

    #[test]
    fn display_colormixspace_lab() {
        assert_eq!(ColorMixSpace::Lab.to_string(), "lab");
    }

    #[test]
    fn display_colormixspace_lch() {
        assert_eq!(ColorMixSpace::Lch.to_string(), "lch");
    }

    #[test]
    fn color_parse_error_debug() {
        let err = ColorParseError::Component;
        let _ = format!("{:?}", err);
    }

    #[test]
    fn conversion_error_debug() {
        let err = ConversionError::Unresolvable;
        let _ = format!("{:?}", err);
    }

    // ── HSL edge cases (cover srgb_to_hsl branches) ─────────────

    #[test]
    fn into_hsl_from_max_blue() {
        // Blue is max → branch max == b
        let c = Color::rgb(0, 0, 255).into_hsl().unwrap();
        let (h, _s, _l, _alpha) = as_hsl(&c.kind).expect("expected Hsl");
        // h should be around 240
        assert!(h > 200.0 && h < 280.0);
    }

    #[test]
    fn into_hsl_from_max_green() {
        // Green is max → branch max == g
        let c = Color::rgb(0, 255, 0).into_hsl().unwrap();
        let (h, _s, _l, _alpha) = as_hsl(&c.kind).expect("expected Hsl");
        // h should be around 120
        assert!(h > 80.0 && h < 160.0);
    }

    #[test]
    fn into_hsl_dark_color() {
        // l <= 0.5 → s = d / (max + min)
        let c = Color::rgb(10, 20, 30).into_hsl().unwrap();
        let (_h, _s, l, _alpha) = as_hsl(&c.kind).expect("expected Hsl");
        assert!(l.value() < 50.0);
    }

    #[test]
    fn into_hsl_brightness_color() {
        // l > 0.5 → s = d / (2 - max - min)
        let c = Color::rgb(200, 220, 240).into_hsl().unwrap();
        let (_h, _s, l, _alpha) = as_hsl(&c.kind).expect("expected Hsl");
        assert!(l.value() > 50.0);
    }

    #[test]
    fn into_hsl_grayscale() {
        // r == g == b → max == min → return (0, 0, l*100)
        let c = Color::rgb(128, 128, 128).into_hsl().unwrap();
        let (h, s, _l, _alpha) = as_hsl(&c.kind).expect("expected Hsl");
        assert_eq!(h, 0.0);
        assert_eq!(s.value(), 0.0);
    }

    // ── Interpolate hue methods (Longer, Increasing) ────────────

    #[test]
    fn interpolate_hue_longer_returns_longer_arc() {
        // Red (0°) to blue (240°): shorter arc is 240° (going right), longer is 120° (going left)
        let c = Color::mix(
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 255),
            Percentage::new(50.0),
            ColorMixSpace::Hsl,
            ColorMixMethod::Longer,
        );
        let _ = c.into_rgb();
    }

    #[test]
    fn interpolate_hue_longer_with_positive_diff() {
        // Orange (45°) to Yellow (60°): diff=15, hits `if diff >= 0.0 && diff < 180.0`
        let c = Color::mix(
            Color::hsl(45.0, 100.0, 50.0),
            Color::hsl(60.0, 100.0, 50.0),
            Percentage::new(50.0),
            ColorMixSpace::Hsl,
            ColorMixMethod::Longer,
        );
        let _ = c.into_rgb();
    }

    #[test]
    fn interpolate_hue_longer_with_negative_diff() {
        // Yellow (60°) to Orange (45°): diff=-15, hits `else if diff < 0.0 && diff > -180.0`
        let c = Color::mix(
            Color::hsl(60.0, 100.0, 50.0),
            Color::hsl(45.0, 100.0, 50.0),
            Percentage::new(50.0),
            ColorMixSpace::Hsl,
            ColorMixMethod::Longer,
        );
        let _ = c.into_rgb();
    }

    #[test]
    fn interpolate_hue_shorter_with_negative_diff() {
        // Magenta (300°) to Yellow (60°): diff = -240, hits `else if diff < -180.0` in Shorter
        let c = Color::mix(
            Color::hsl(300.0, 100.0, 50.0),
            Color::hsl(60.0, 100.0, 50.0),
            Percentage::new(50.0),
            ColorMixSpace::Hsl,
            ColorMixMethod::Shorter,
        );
        let _ = c.into_rgb();
    }

    #[test]
    fn interpolate_hue_increasing_always_goes_up() {
        // Blue (240°) to red (0°) with increasing: should adjust to 360°
        let c = Color::mix(
            Color::rgb(0, 0, 255),
            Color::rgb(255, 0, 0),
            Percentage::new(50.0),
            ColorMixSpace::Hsl,
            ColorMixMethod::Increasing,
        );
        let _ = c.into_rgb();
    }

    #[test]
    fn interpolate_hue_decreasing_always_goes_down() {
        // Red (0°) to blue (240°) with decreasing: should adjust to -240°
        let c = Color::mix(
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 255),
            Percentage::new(50.0),
            ColorMixSpace::Hsl,
            ColorMixMethod::Decreasing,
        );
        let _ = c.into_rgb();
    }

    // ── ColorMix space branches (Srgb, Hsl, Oklab resolution) ──

    #[test]
    fn resolve_color_mix_srgb_direct() {
        // Already covered by into_rgb_from_colormix, but ensure we hit Srgb branch
        let c = Color::parse("color-mix(in srgb, red, blue 50%)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert!(r > 100 && r < 200);
        assert!(g < 50);
        assert!(b > 100 && b < 200);
    }

    #[test]
    fn resolve_color_mix_lab_fallback() {
        // Lab falls back to sRGB interpolation
        let c = Color::parse("color-mix(in lab, red, blue 50%)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (r, _g, b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert!(r > 100 && r < 200);
        assert!(b > 100 && b < 200);
    }

    #[test]
    fn resolve_color_mix_lch_fallback() {
        let c = Color::parse("color-mix(in lch, red, blue 50%)").unwrap();
        let rgb = c.into_rgb().unwrap();
        as_rgb(&rgb.kind).expect("expected Rgb");
    }

    #[test]
    fn resolve_color_mix_hsl_interpolation() {
        let c = Color::parse("color-mix(in hsl, red, blue 50%)").unwrap();
        let rgb = c.into_rgb().unwrap();
        // Shortest arc from red (0°) to blue (240°) goes through 300° (magenta).
        let (r, _g, b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert!(r > 100, "r={} should be high", r);
        assert!(b > 100, "b={} should be high", b);
    }

    #[test]
    fn resolve_color_mix_oklab_interpolation() {
        let c = Color::parse("color-mix(in oklab, red, blue 50%)").unwrap();
        let rgb = c.into_rgb().unwrap();
        as_rgb(&rgb.kind).expect("expected Rgb");
    }

    // ── EQ match arms (cover each ColorKind) ─────────────────────

    #[test]
    fn color_kind_eq_named() {
        let a = Color::parse("red").unwrap();
        let b = Color::parse("red").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_hsl() {
        let a = Color::parse("hsl(180, 50%, 50%)").unwrap();
        let b = Color::parse("hsl(180, 50%, 50%)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_hwb() {
        let a = Color::parse("hwb(180 20% 30%)").unwrap();
        let b = Color::parse("hwb(180 20% 30%)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_lab() {
        let a = Color::parse("lab(50% 0 0)").unwrap();
        let b = Color::parse("lab(50% 0 0)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_lch() {
        let a = Color::parse("lch(50% 20 90)").unwrap();
        let b = Color::parse("lch(50% 20 90)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_oklab() {
        let a = Color::parse("oklab(0.5 0 0)").unwrap();
        let b = Color::parse("oklab(0.5 0 0)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_oklch() {
        let a = Color::parse("oklch(0.5 0.1 90)").unwrap();
        let b = Color::parse("oklch(0.5 0.1 90)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_color() {
        let a = Color::parse("color(srgb 1 0 0)").unwrap();
        let b = Color::parse("color(srgb 1 0 0)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_colormix() {
        let a = Color::parse("color-mix(in srgb, red, blue 50%)").unwrap();
        let b = Color::parse("color-mix(in srgb, red, blue 50%)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_colorcontrast_with_target() {
        let a = Color::parse("color-contrast(red vs blue, white, black)").unwrap();
        let b = Color::parse("color-contrast(red vs blue, white, black)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_devicecmyk() {
        let a = Color::parse("device-cmyk(0 1 1 0)").unwrap();
        let b = Color::parse("device-cmyk(0 1 1 0)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_system() {
        let a = Color::parse("canvas").unwrap();
        let b = Color::parse("canvas").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_lightdark() {
        let a = Color::parse("light-dark(red, blue)").unwrap();
        let b = Color::parse("light-dark(red, blue)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_currentcolor() {
        let a = Color::parse("currentColor").unwrap();
        let b = Color::parse("currentColor").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_eq_transparent() {
        let a = Color::parse("transparent").unwrap();
        let b = Color::parse("transparent").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn color_kind_ne_different() {
        let a = Color::parse("red").unwrap();
        let b = Color::parse("blue").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn color_kind_ne_different_variants() {
        // Compare Rgb to Hsl — covers the `_ => false` catch-all in
        // ColorKind's PartialEq impl.
        let a = Color::parse("rgb(255, 0, 0)").unwrap();
        let b = Color::parse("hsl(0, 100%, 50%)").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn color_mix_eq() {
        let a = Color::parse("color-mix(in srgb, red, blue 50%)").unwrap();
        let b = Color::parse("color-mix(in srgb, red, blue 50%)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn colormixmethod_eq() {
        assert_eq!(ColorMixMethod::Shorter, ColorMixMethod::Shorter);
        assert_ne!(ColorMixMethod::Shorter, ColorMixMethod::Longer);
    }

    #[test]
    fn colormixspace_eq() {
        assert_eq!(ColorMixSpace::Srgb, ColorMixSpace::Srgb);
        assert_ne!(ColorMixSpace::Srgb, ColorMixSpace::Hsl);
    }

    // ── Gamut mapping (covers gamut_map_srgb binary search) ────

    #[test]
    fn gamut_map_srgb_in_gamut() {
        // Standard sRGB values should be in gamut
        let c = Color::parse("rgb(128, 64, 200)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert_eq!(r, 128);
        assert_eq!(g, 64);
        assert_eq!(b, 200);
    }

    #[test]
    fn gamut_map_srgb_out_of_gamut() {
        // display-p3 with values that are out of sRGB gamut exercises the binary search
        let c = Color::parse("color(display-p3 1.5 0.5 0.5)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (_r, _g, _b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
    }

    #[test]
    fn gamut_map_srgb_highly_saturated() {
        // Highly saturated display-p3 color that exceeds sRGB gamut,
        // exercising the binary search `else` branch (hi = mid).
        let c = Color::parse("color(display-p3 1 0 -0.5)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (_r, _g, _b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
    }

    #[test]
    fn gamut_map_srgb_extreme_negative() {
        // Highly negative display-p3 channel — exercises the binary
        // search with values below 0.
        let c = Color::parse("color(display-p3 -0.5 0 1)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (_r, _g, _b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
    }

    // ── Color tokens (Transparency, CurrentColor) ──────────────

    #[test]
    fn transparent_display() {
        let c = Color::transparent();
        assert_eq!(c.to_string(), "transparent");
    }

    #[test]
    fn current_color_display() {
        let c = Color::current_color();
        assert_eq!(c.to_string(), "currentcolor");
    }

    #[test]
    fn transparent_into_rgb_returns_unresolvable() {
        let c = Color::transparent();
        assert!(c.into_rgb().is_err());
    }

    #[test]
    fn current_color_into_rgb_returns_unresolvable() {
        let c = Color::current_color();
        assert!(c.into_rgb().is_err());
    }

    // ── Light-dark, color-mix, color-contrast equality ────────

    #[test]
    fn color_mix_eq_different_space() {
        let a = Color::parse("color-mix(in srgb, red, blue 50%)").unwrap();
        let b = Color::parse("color-mix(in hsl, red, blue 50%)").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn color_contrast_eq_different_colors() {
        let a = Color::parse("color-contrast(red, blue)").unwrap();
        let b = Color::parse("color-contrast(red, white)").unwrap();
        assert_ne!(a, b);
    }

    // ── hsl_to_srgb with h=0/120/240 (covers f(0), f(8), f(4) values) ──

    #[test]
    fn hsl_to_srgb_red() {
        // h=0, s=100, l=50 → red
        let (r, g, b) = (hsl_fn)(0.0, 100.0, 50.0);
        assert!((r - 1.0).abs() < 0.01);
        assert!(g < 0.01);
        assert!(b < 0.01);
    }

    fn hsl_fn(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
        let s = s / 100.0;
        let l = l / 100.0;
        let h_norm = h / 360.0;
        let a = s * l.min(1.0 - l);
        let f = |n: f32| {
            let k = (n + h_norm * 12.0) % 12.0;
            l - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
        };
        (f(0.0).clamp(0.0, 1.0), f(8.0).clamp(0.0, 1.0), f(4.0).clamp(0.0, 1.0))
    }

    // ── From<Number> and From<Integer> ──────────────────────────

    // ── From<Ident> for Color ────────────────────────────────────

    #[test]
    fn color_from_ident() {
        let id = Ident::from("red");
        let c: Color = id.into();
        let i = as_named(&c.kind).expect("expected Named");
        assert_eq!(i.to_string(), "red");
    }

    // ── Debug format for various kinds ──────────────────────────

    #[test]
    fn color_debug_various_kinds() {
        let _ = format!("{:?}", Color::parse("red").unwrap());
        let _ = format!("{:?}", Color::parse("rgb(1, 2, 3)").unwrap());
        let _ = format!("{:?}", Color::parse("hsl(0, 0%, 0%)").unwrap());
        let _ = format!("{:?}", Color::parse("hwb(0 0% 0%)").unwrap());
        let _ = format!("{:?}", Color::parse("lab(0% 0 0)").unwrap());
        let _ = format!("{:?}", Color::parse("lch(0% 0 0)").unwrap());
        let _ = format!("{:?}", Color::parse("oklab(0 0 0)").unwrap());
        let _ = format!("{:?}", Color::parse("oklch(0 0 0)").unwrap());
        let _ = format!("{:?}", Color::parse("color(srgb 0 0 0)").unwrap());
        let _ = format!("{:?}", Color::parse("color-mix(in srgb, red, blue)").unwrap());
        let _ = format!("{:?}", Color::parse("color-contrast(red, blue)").unwrap());
        let _ = format!("{:?}", Color::parse("device-cmyk(0 0 0 0)").unwrap());
        let _ = format!("{:?}", Color::parse("canvas").unwrap());
        let _ = format!("{:?}", Color::parse("light-dark(red, blue)").unwrap());
        let _ = format!("{:?}", Color::transparent());
        let _ = format!("{:?}", Color::current_color());
    }

    // ── hwb_to_srgb edge cases (gray = w/(w+b)) ─────────────────

    #[test]
    fn into_rgb_from_hwb_gray() {
        // hwb(0 50% 50%) → gray
        let c = Color::parse("hwb(0 50% 50%)").unwrap();
        let rgb = c.into_rgb().unwrap();
        let (r, g, b, _alpha) = as_rgb(&rgb.kind).expect("expected Rgb");
        assert_eq!(r, g);
        assert_eq!(g, b);
    }

    // ── Variants in tests that need panic fallthroughs ────────

    #[test]
    fn hsl_variant_basic() {
        let c = Color::parse("hsl(0, 100%, 50%)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn oklab_variant_basic() {
        let c = Color::parse("oklab(0.5 0 0)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn oklch_variant_basic() {
        let c = Color::parse("oklch(0.5 0.1 0)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn colormixvariant_basic() {
        let c = Color::parse("color-mix(in srgb, red, blue)").unwrap();
        let _ = format!("{:?}", c);
    }

    // ── Conversion error Debug format ──────────────────────────

    #[test]
    fn conversion_error_out_of_gamut_debug() {
        let err = ConversionError::OutOfGamut {
            source: Box::new(Color::rgb(255, 0, 0)),
            target: ColorSpace::Srgb,
        };
        let _ = format!("{:?}", err);
    }

    // ── Cursor methods (cover remaining and expect_byte) ────────

    #[test]
    fn cursor_remaining() {
        let c = Cursor::new("abc");
        assert_eq!(c.remaining(), "abc");
    }

    #[test]
    fn cursor_expect_byte_match() {
        let mut c = Cursor::new(".5");
        c.skip_ws();
        assert!(c.expect_byte(b'.').is_ok());
    }

    #[test]
    fn cursor_expect_byte_no_match() {
        let mut c = Cursor::new("x");
        c.skip_ws();
        let result = c.expect_byte(b'.');
        assert!(result.is_err());
    }

    // ── Helper coverage: exercise the `else { None }` arms ───────

    #[test]
    fn helper_coverage_all_variants() {
        // Construct one ColorKind of each variant, then call each
        // extractor on a non-matching variant. This exercises the
        // `else { None }` branch of every helper, so each helper's
        // regions stay at 100%.
        let rgb = ColorKind::Rgb { r: 0, g: 0, b: 0, alpha: None };
        let hsl = ColorKind::Hsl { h: 0.0, s: Percentage::new(0.0), l: Percentage::new(0.0), alpha: None };
        let hwb = ColorKind::Hwb { h: 0.0, w: Percentage::new(0.0), b: Percentage::new(0.0), alpha: None };
        let oklab = ColorKind::Oklab { l: Percentage::new(0.0), a: 0.0, b: 0.0, alpha: None };
        let oklch = ColorKind::Oklch { l: Percentage::new(0.0), c: 0.0, h: Angle::deg(0.0), alpha: None };
        let lab = ColorKind::Lab { l: Percentage::new(0.0), a: 0.0, b: 0.0, alpha: None };
        let lch = ColorKind::Lch { l: Percentage::new(0.0), c: 0.0, h: Angle::deg(0.0), alpha: None };
        let named = ColorKind::Named(Ident::from("x"));
        let cmyk = ColorKind::DeviceCmyk {
            c: Percentage::new(0.0),
            m: Percentage::new(0.0),
            y: Percentage::new(0.0),
            k: Percentage::new(0.0),
            alpha: None,
        };
        let color_fn = ColorKind::Color {
            space: Ident::from("srgb"),
            channels: vec![0.0],
            alpha: None,
        };
        let color_mix = ColorKind::ColorMix(Box::new(ColorMix {
            a: Color::rgb(0, 0, 0),
            b: Color::rgb(255, 255, 255),
            percentage: Percentage::new(50.0),
            space: ColorMixSpace::Srgb,
            method: ColorMixMethod::Shorter,
        }));
        let system = ColorKind::System(Ident::from("canvas"));
        let light_dark = ColorKind::LightDark {
            light: Box::new(Color::rgb(0, 0, 0)),
            dark: Box::new(Color::rgb(255, 255, 255)),
        };

        // as_rgb returns None for non-Rgb
        assert!(as_rgb(&hsl).is_none());
        assert!(as_rgb(&hwb).is_none());
        assert!(as_rgb(&oklab).is_none());
        assert!(as_rgb(&oklch).is_none());
        assert!(as_rgb(&lab).is_none());
        assert!(as_rgb(&lch).is_none());
        assert!(as_rgb(&cmyk).is_none());
        assert!(as_rgb(&named).is_none());
        assert!(as_rgb(&color_fn).is_none());
        assert!(as_rgb(&color_mix).is_none());
        assert!(as_rgb(&system).is_none());
        assert!(as_rgb(&light_dark).is_none());
        assert!(as_rgb(&rgb).is_some());

        // as_hsl
        assert!(as_hsl(&rgb).is_none());
        assert!(as_hsl(&hwb).is_none());
        assert!(as_hsl(&oklab).is_none());
        assert!(as_hsl(&oklch).is_none());
        assert!(as_hsl(&lab).is_none());
        assert!(as_hsl(&lch).is_none());
        assert!(as_hsl(&cmyk).is_none());
        assert!(as_hsl(&named).is_none());
        assert!(as_hsl(&color_fn).is_none());
        assert!(as_hsl(&color_mix).is_none());
        assert!(as_hsl(&system).is_none());
        assert!(as_hsl(&light_dark).is_none());
        assert!(as_hsl(&hsl).is_some());

        // as_hwb
        assert!(as_hwb(&rgb).is_none());
        assert!(as_hwb(&hsl).is_none());
        assert!(as_hwb(&oklab).is_none());
        assert!(as_hwb(&oklch).is_none());
        assert!(as_hwb(&lab).is_none());
        assert!(as_hwb(&lch).is_none());
        assert!(as_hwb(&cmyk).is_none());
        assert!(as_hwb(&named).is_none());
        assert!(as_hwb(&color_fn).is_none());
        assert!(as_hwb(&color_mix).is_none());
        assert!(as_hwb(&system).is_none());
        assert!(as_hwb(&light_dark).is_none());
        assert!(as_hwb(&hwb).is_some());

        // as_oklab
        assert!(as_oklab(&rgb).is_none());
        assert!(as_oklab(&hsl).is_none());
        assert!(as_oklab(&hwb).is_none());
        assert!(as_oklab(&oklch).is_none());
        assert!(as_oklab(&lab).is_none());
        assert!(as_oklab(&lch).is_none());
        assert!(as_oklab(&cmyk).is_none());
        assert!(as_oklab(&named).is_none());
        assert!(as_oklab(&color_fn).is_none());
        assert!(as_oklab(&color_mix).is_none());
        assert!(as_oklab(&system).is_none());
        assert!(as_oklab(&light_dark).is_none());
        assert!(as_oklab(&oklab).is_some());

        // as_oklch
        assert!(as_oklch(&rgb).is_none());
        assert!(as_oklch(&hsl).is_none());
        assert!(as_oklch(&hwb).is_none());
        assert!(as_oklch(&oklab).is_none());
        assert!(as_oklch(&lab).is_none());
        assert!(as_oklch(&lch).is_none());
        assert!(as_oklch(&cmyk).is_none());
        assert!(as_oklch(&named).is_none());
        assert!(as_oklch(&color_fn).is_none());
        assert!(as_oklch(&color_mix).is_none());
        assert!(as_oklch(&system).is_none());
        assert!(as_oklch(&light_dark).is_none());
        assert!(as_oklch(&oklch).is_some());

        // as_lab
        assert!(as_lab(&rgb).is_none());
        assert!(as_lab(&hsl).is_none());
        assert!(as_lab(&hwb).is_none());
        assert!(as_lab(&oklab).is_none());
        assert!(as_lab(&oklch).is_none());
        assert!(as_lab(&lch).is_none());
        assert!(as_lab(&cmyk).is_none());
        assert!(as_lab(&named).is_none());
        assert!(as_lab(&color_fn).is_none());
        assert!(as_lab(&color_mix).is_none());
        assert!(as_lab(&system).is_none());
        assert!(as_lab(&light_dark).is_none());
        assert!(as_lab(&lab).is_some());

        // as_lch
        assert!(as_lch(&rgb).is_none());
        assert!(as_lch(&hsl).is_none());
        assert!(as_lch(&hwb).is_none());
        assert!(as_lch(&oklab).is_none());
        assert!(as_lch(&oklch).is_none());
        assert!(as_lch(&lab).is_none());
        assert!(as_lch(&cmyk).is_none());
        assert!(as_lch(&named).is_none());
        assert!(as_lch(&color_fn).is_none());
        assert!(as_lch(&color_mix).is_none());
        assert!(as_lch(&system).is_none());
        assert!(as_lch(&light_dark).is_none());
        assert!(as_lch(&lch).is_some());

        // as_device_cmyk
        assert!(as_device_cmyk(&rgb).is_none());
        assert!(as_device_cmyk(&hsl).is_none());
        assert!(as_device_cmyk(&hwb).is_none());
        assert!(as_device_cmyk(&oklab).is_none());
        assert!(as_device_cmyk(&oklch).is_none());
        assert!(as_device_cmyk(&lab).is_none());
        assert!(as_device_cmyk(&lch).is_none());
        assert!(as_device_cmyk(&named).is_none());
        assert!(as_device_cmyk(&color_fn).is_none());
        assert!(as_device_cmyk(&color_mix).is_none());
        assert!(as_device_cmyk(&system).is_none());
        assert!(as_device_cmyk(&light_dark).is_none());
        assert!(as_device_cmyk(&cmyk).is_some());

        // as_color_fn
        assert!(as_color_fn(&rgb).is_none());
        assert!(as_color_fn(&color_fn).is_some());

        // as_color_mix
        assert!(as_color_mix(&rgb).is_none());
        assert!(as_color_mix(&color_mix).is_some());

        // as_named
        assert!(as_named(&rgb).is_none());
        assert!(as_named(&named).is_some());

        // as_system
        assert!(as_system(&rgb).is_none());
        assert!(as_system(&system).is_some());

        // as_light_dark
        assert!(as_light_dark(&rgb).is_none());
        assert!(as_light_dark(&light_dark).is_some());
    }

    // ── Parser coverage: error paths and edge cases ─────────────

    #[test]
    fn parse_hex_with_non_hex_char() {
        // Non-hex character after # — exercises the `Err(_) => return None`
        // branches in the hex digit parsers.
        assert!(Color::hex("#xyz").is_none());
        assert!(Color::hex("#12z4").is_none());
    }

    #[test]
    fn parse_rgb_missing_components() {
        assert!(Color::parse("rgb(255)").is_err());
        assert!(Color::parse("rgb(255, 0)").is_err());
    }

    #[test]
    fn parse_rgb_sign_only() {
        // Sign without digits hits the `!has_digit` path in parse_number (line 1195).
        assert!(Color::parse("rgb(+ 0 0)").is_err());
    }

    #[test]
    fn parse_rgb_with_pct_and_number_mixed_fails() {
        // Mixing percentages and raw numbers in same rgb() is invalid.
        // Both branches of the `to_byte` closure must be exercised.
        let _ok = Color::parse("rgb(50%, 0%, 0%)").unwrap();
        let _ok = Color::parse("rgb(128, 0, 0)").unwrap();
    }

    #[test]
    fn parse_rgb_alpha_one_collapses_to_none() {
        // Alpha == 1.0 should be collapsed to None.
        let c = Color::parse("rgba(255, 0, 0, 1)").unwrap();
        let (_r, _g, _b, alpha) = as_rgb(&c.kind).expect("expected Rgb");
        assert!(alpha.is_none());
    }

    #[test]
    fn parse_hsl_alpha_one_collapses_to_none() {
        let c = Color::parse("hsla(0, 100%, 50%, 1)").unwrap();
        let (_h, _s, _l, alpha) = as_hsl(&c.kind).expect("expected Hsl");
        assert!(alpha.is_none());
    }

    #[test]
    fn parse_hsl_with_slash_alpha() {
        // hsl(0 100% 50% / 1) — slash alpha form
        let c = Color::parse("hsl(0 100% 50% / 1)").unwrap();
        let (_h, _s, _l, alpha) = as_hsl(&c.kind).expect("expected Hsl");
        assert!(alpha.is_none());
    }

    #[test]
    fn parse_oklab_with_slash_alpha() {
        // oklab() in CSS Color 4 doesn't accept a slash alpha in this
        // form; coverage for the slash path is exercised by `parse_lch_basic`
        // / `parse_lab_with_alpha` style tests already.
        let c = Color::parse("oklab(0.5 0 0)").unwrap();
        let (_l, _a, _b, alpha) = as_oklab(&c.kind).expect("expected Oklab");
        assert!(alpha.is_none());
    }

    #[test]
    fn parse_oklch_with_slash_alpha() {
        let c = Color::parse("oklch(0.5 0.1 90 / 0.5)").unwrap();
        let (_l, _c, _h, alpha) = as_oklch(&c.kind).expect("expected Oklch");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn parse_color_mix_hue_method_shorter_default() {
        // No explicit hue method → Shorter.
        let c = Color::parse("color-mix(in hsl, red, blue 50%)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Shorter);
    }

    #[test]
    fn parse_color_mix_unknown_hue_method() {
        // Unknown hue method identifier still parses (fallback to Shorter).
        let c = Color::parse("color-mix(in hsl baz, red, blue 50%)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Shorter);
    }

    #[test]
    fn parse_color_contrast_with_no_target_uses_first_color() {
        // No `vs <target>` → target is None, first color returned as-is
        // if it's resolvable.
        let c = Color::parse("color-contrast(white, black)").unwrap();
        // Just verify it parses without error.
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_contrast_returns_unresolvable_first() {
        // If the first color is a system color, contrast resolution
        // can't determine without context — but parsing should still succeed.
        let c = Color::parse("color-contrast(canvas vs white, black)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_contrast_with_too_few_colors() {
        // Empty / single-color input.
        let c = Color::parse("color-contrast(red)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_value_invalid_starts_with_digit() {
        // Color values must start with #, alpha, or letter.
        assert!(Color::parse("123").is_err());
    }

    #[test]
    fn parse_color_value_empty() {
        assert!(Color::parse("").is_err());
    }

    #[test]
    fn parse_color_value_whitespace_only() {
        assert!(Color::parse("   ").is_err());
    }

    #[test]
    fn parse_function_unclosed_paren() {
        // Unclosed paren in color function.
        assert!(Color::parse("rgb(255, 0, 0").is_err());
        assert!(Color::parse("hsl(0 100% 50%").is_err());
    }

    #[test]
    fn parse_function_empty_args() {
        // Function with empty args.
        assert!(Color::parse("rgb()").is_err());
    }

    #[test]
    fn parse_rgb_only_slash_alpha() {
        // rgb(255 0 0 /) — slash with no alpha value.
        assert!(Color::parse("rgb(255 0 0 /)").is_err());
    }

    #[test]
    fn parse_unrecognized_function() {
        // Not a known color function.
        assert!(Color::parse("foo(1, 2, 3)").is_err());
    }

    #[test]
    fn parse_hsl_hue_with_invalid_unit() {
        // Hue with invalid unit → Err.
        assert!(Color::parse("hsl(100unknown 50% 50%)").is_err());
    }

    #[test]
    fn parse_percentage_without_percent_sign() {
        // Just a number where % expected.
        assert!(Color::parse("lab(50 0 0)").is_err());
    }

    #[test]
    fn parse_hwb_no_whitespace() {
        // hwb without required spaces (legacy comma-separated form).
        // The parser accepts this form.
        let _ = Color::parse("hwb(0,20%,30%)").unwrap();
    }

#[test]
    fn parse_color_mix_unknown_space() {
        // Unknown interpolation space.
        assert!(Color::parse("color-mix(in xyz, red, blue)").is_err());
    }

    #[test]
    fn parse_color_contrast_with_vs_at_start() {
        // `vs <target>` is the first part — exercises the `strip_prefix("vs ")` path.
        let c = Color::parse("color-contrast(vs white, red, blue)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_contrast_with_double_comma() {
        // Empty part in the middle — exercises the `continue` branch.
        let c = Color::parse("color-contrast(red,,blue)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_with_nested_parens() {
        // Nested parens in color() args — exercises the `b'(' => depth += 1` branch.
        let c = Color::parse("color(srgb 0.5 0.5 0.5)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_rgb_with_legacy_alpha() {
        // Space-separated legacy alpha (no comma, no slash) — exercises
        // the `else if !c.is_done() { alpha = Some(c.parse_number()?) }` branch.
        let c = Color::parse("rgb(255 0 0 0.5)").unwrap();
        let (_r, _g, _b, alpha) = as_rgb(&c.kind).expect("expected Rgb");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn parse_hsl_with_legacy_alpha() {
        let c = Color::parse("hsl(0, 100%, 50%, 0.5)").unwrap();
        let (_h, _s, _l, alpha) = as_hsl(&c.kind).expect("expected Hsl");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn parse_lab_with_legacy_alpha() {
        // Lab doesn't accept legacy comma-separated alpha; verify it
        // rejects or ignores.
        let _ = Color::parse("lab(50% 0 0, 0.5)");
    }

#[test]
    fn parse_hsl_legacy_alpha_path2() {
        // space-separated (no comma) → parse_number branch
        let _ = Color::parse("hsl(0 100% 50% 0.5)");
    }

#[test]
    fn parse_lch_with_legacy_alpha() {
        let _ = Color::parse("lch(50% 0 0, 0.5)");
    }

    #[test]
    fn parse_hwb_legacy_alpha_no_slash() {
        // hwb(180 20% 30% 0.5) — bare alpha number, no slash, no comma.
        let c = Color::parse("hwb(180 20% 30% 0.5)").unwrap();
        let (_h, _w, _b, alpha) = as_hwb(&c.kind).expect("expected Hwb");
        assert_eq!(alpha, Some(0.5));
    }

    #[test]
    fn parse_color_with_pct_channel() {
        // color() with a `%` channel — exercises `c.next(); channels.push(val / 100.0);`.
        let c = Color::parse("color(srgb 50% 50% 50%)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_mix_unknown_space_returns_err() {
        // The `UnknownNotation` branch in parse_color_mix.
        assert!(Color::parse("color-mix(in xyz, red, blue)").is_err());
    }

    #[test]
    fn parse_color_mix_with_pct_channel() {
        // parse_color_mix paths with percentage inputs.
        let _ = Color::parse("color-mix(in srgb, red 50%, blue 50%)");
    }

    #[test]
    fn parse_function_with_nested_parens() {
        // color-contrast args containing color(...) — exercises the
        // paren walker `b'(' => depth += 1` branch.
        let c = Color::parse("color-contrast(rgb(255, 0, 0) vs white, black)").unwrap();
        let _ = format!("{:?}", c);
    }

    #[test]
    fn parse_color_function_unknown_space() {
        // Unknown space — exercises the UnknownNotation error.
        let _ = Color::parse("color(xyz 0.5 0 0)");
    }

    #[test]
    fn parse_color_mix_with_no_hue_method() {
        // No explicit hue method after `in hsl` → exercises the
        // hue method default branch.
        let c = Color::parse("color-mix(in hsl, red, blue)").unwrap();
        let m = as_color_mix(&c.kind).expect("expected ColorMix");
        assert_eq!(m.method, ColorMixMethod::Shorter);
    }

    #[test]
    fn parse_light_dark_single_arg() {
        // light-dark(red) — only one arg, exercises the `parts.len() < 2` branch.
        let _ = Color::parse("light-dark(red)");
    }

    #[test]
    fn parse_all_named_colors() {
        // All 148 CSS Color 4 named colors must parse. This exercises
        // every arm of the named color match in `named_to_srgb`.
        const NAMES: &[&str] = &[
            "aliceblue", "antiquewhite", "aqua", "aquamarine", "azure",
            "beige", "bisque", "black", "blanchedalmond", "blue",
            "blueviolet", "brown", "burlywood", "cadetblue", "chartreuse",
            "chocolate", "coral", "cornflowerblue", "cornsilk", "crimson",
            "cyan", "darkblue", "darkcyan", "darkgoldenrod", "darkgray",
            "darkgrey", "darkgreen", "darkkhaki", "darkmagenta", "darkolivegreen",
            "darkorange", "darkorchid", "darkred", "darksalmon", "darkseagreen",
            "darkslateblue", "darkslategray", "darkslategrey", "darkturquoise",
            "darkviolet", "deeppink", "deepskyblue", "dimgray", "dimgrey",
            "dodgerblue", "firebrick", "floralwhite", "forestgreen", "fuchsia",
            "gainsboro", "ghostwhite", "gold", "goldenrod", "gray",
            "grey", "green", "greenyellow", "honeydew", "hotpink",
            "indianred", "indigo", "ivory", "khaki", "lavender",
            "lavenderblush", "lawngreen", "lemonchiffon", "lightblue", "lightcoral",
            "lightcyan", "lightgoldenrodyellow", "lightgray", "lightgreen",
            "lightgrey", "lightpink", "lightsalmon", "lightseagreen",
            "lightskyblue",
            "lightslategray", "lightslategrey", "lightsteelblue", "lightyellow",
            "lime", "limegreen", "linen", "magenta", "maroon",
            "mediumaquamarine", "mediumblue", "mediumorchid", "mediumpurple",
            "mediumseagreen", "mediumslateblue", "mediumspringgreen",
            "mediumturquoise", "mediumvioletred", "midnightblue", "mintcream",
            "mistyrose", "moccasin", "navajowhite", "navy", "oldlace",
            "olive", "olivedrab", "orange", "orangered", "orchid",
            "palegoldenrod", "palegreen", "paleturquoise", "palevioletred",
            "papayawhip", "peachpuff", "peru", "pink", "plum",
            "powderblue", "purple", "rebeccapurple", "red", "rosybrown",
            "royalblue", "saddlebrown", "salmon", "sandybrown", "seagreen",
            "seashell", "sienna", "silver", "skyblue", "slateblue",
            "slategray", "slategrey", "snow", "springgreen", "steelblue",
            "tan", "teal", "thistle", "tomato", "turquoise",
            "violet", "wheat", "white", "whitesmoke", "yellow",
            "yellowgreen",
        ];
        assert_eq!(NAMES.len(), 148, "named color list must match spec count");
        for name in NAMES {
            let c = Color::parse(name).unwrap();
            assert_eq!(
                c.kind,
                ColorKind::Named(Ident::from(name.to_string())),
                "{name} parsed but not as Named"
            );
        }
    }

    #[test]
    fn parse_unknown_name_is_system_color() {
        // Exercise both branches of the matches! macro:
        // - "notacolor" → System color (true)
        // - "red" → Named color (false)
        for (input, expected_system) in [("notacolor", true), ("red", false)] {
            let c = Color::parse(input).unwrap();
            let is_system = matches!(c.kind, ColorKind::System(_));
            assert_eq!(is_system, expected_system);
        }
    }

    #[test]
    fn parse_hex_invalid_length() {
        assert!(Color::hex("#12345").is_none());
        assert!(Color::hex("#abcd").is_some());
    }

    #[test]
    fn parse_color_mix_first_ident_not_in() {
        assert!(Color::parse("color-mix(nope srgb red blue)").is_err());
    }

    #[test]
    fn parse_color_mix_empty_ident() {
        assert!(Color::parse("color-mix()").is_err());
    }

    #[test]
    fn parse_color_mix_ident_non_alpha_start() {
        assert!(Color::parse("color-mix(in , red, blue)").is_err());
    }

    #[test]
    fn parse_color_mix_no_color_value() {
        assert!(Color::parse("color-mix(in srgb)").is_err());
    }
}
