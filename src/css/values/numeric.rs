//! Internal trait for extracting an `f64` from a typed value.
//!
//! Used by the color-conversion math in [`super::convert`] so it can
//! promote `f32` fields (the public-facing precision) to `f64` for
//! arithmetic without changing the public API. Unit information
//! (e.g. `px` vs `em`) is preserved on the value type itself and is
//! not encoded here.
//!
//! This trait is `pub(crate)` only — it is not part of the public
//! surface.

/// Extract the magnitude of a typed value as `f64`.
///
/// For values with units (`Length`, `Time`, `Angle`, …) the unit is
/// dropped — only the numeric magnitude is returned. The
/// `convert.rs` math module is responsible for unit conversions when
/// it needs to combine values across spaces.
#[allow(dead_code)]
pub(crate) trait FloatConvert {
    /// Return the value as `f64`.
    fn to_f64(self) -> f64;
}

impl FloatConvert for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl FloatConvert for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}

impl FloatConvert for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl FloatConvert for u8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl FloatConvert for u16 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

#[cfg(test)]
mod tests {
    use super::FloatConvert;

    #[test]
    fn f32_extracts_as_f64() {
        let v: f32 = 1.5;
        assert_eq!(v.to_f64(), 1.5_f64);
    }

    #[test]
    fn f64_is_identity() {
        let v: f64 = 2.25;
        assert_eq!(v.to_f64(), 2.25);
    }

    #[test]
    fn i32_extracts_as_f64() {
        let v: i32 = 42;
        assert_eq!(v.to_f64(), 42.0);
    }

    #[test]
    fn u8_extracts_as_f64() {
        let v: u8 = 255;
        assert_eq!(v.to_f64(), 255.0);
    }

    #[test]
    fn u16_extracts_as_f64() {
        let v: u16 = 65535;
        assert_eq!(v.to_f64(), 65535.0);
    }

    #[test]
    fn negative_f32() {
        let v: f32 = -0.5;
        assert_eq!(v.to_f64(), -0.5_f64);
    }

    #[test]
    fn zero_f32() {
        let v: f32 = 0.0;
        assert_eq!(v.to_f64(), 0.0_f64);
    }
}
