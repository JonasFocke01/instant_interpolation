#![no_std]
use core::fmt::Display;

/// This holds a function that maps a number from one range to another.
/// This is designed to work in `no_std` environments
#[allow(private_bounds)]
pub trait MapRange:
    Sized + Copy + PartialOrd + CheckedNumberArithmetics + Display + CheckedNumberCastsToFloat
{
    /// Maps the value over the given ranges.
    ///
    /// The `inputvalue` must be inside the `from_range`,
    /// then this produces a `resultvalue` that lives inside `to_range`.
    ///
    /// ```
    /// use map_to_range::{MapRange};
    ///
    /// let test: u8 = 5;
    /// assert_eq!(Some(15), test.map_range((0, 10), (10, 20)));
    /// assert_eq!(None, test.map_range((10, 20), (20, 30)));
    /// ```
    ///
    /// This function internally upcasts any given number to f64 for maximum precision, and down again to the type
    /// provided for convenience. When you need every drop of performance, you can go around
    /// this by calling the `map_range_uncasted` directly (as this function also does after casting)
    fn map_range(&self, from_range: (Self, Self), to_range: (Self, Self)) -> Option<Self> {
        let value = self.checked_f64_cast()?;
        let from_range = (
            from_range.0.checked_f64_cast()?,
            from_range.1.checked_f64_cast()?,
        );
        let to_range = (
            to_range.0.checked_f64_cast()?,
            to_range.1.checked_f64_cast()?,
        );
        let result = value.map_range_uncasted(from_range, to_range)?;
        Self::checked_cast_back(result)
    }
    /// Maps the value over the given ranges.
    ///
    /// The `inputvalue` must be inside the `from_range`,
    /// then this produces a `resultvalue` that lives inside `to_range`.
    ///
    /// ```
    /// use map_to_range::{MapRange};
    ///
    /// let test: u8 = 5;
    /// assert_eq!(Some(15), test.map_range_uncasted((0, 10), (10, 20)));
    /// assert_eq!(None, test.map_range_uncasted((10, 20), (20, 30)));
    /// ```
    ///
    /// This is the more performant version of `map_range`, at the cost of precision and
    /// possible unexpected results. To be safe, just call `map_range`. That will handle the
    /// casting for you and ensures, that you get correct results.
    fn map_range_uncasted(&self, from_range: (Self, Self), to_range: (Self, Self)) -> Option<Self> {
        if *self < from_range.0 || *self > from_range.1 {
            return None;
        }

        let diff_self_from = self.checked_sub_mr(from_range.0)?;
        let diff_to = to_range.1.checked_sub_mr(to_range.0)?;
        let diff_from = from_range.1.checked_sub_mr(from_range.0)?;
        let product = diff_self_from.checked_mul_mr(diff_to)?;
        let quotient = product.checked_div_mr(diff_from)?;
        to_range.0.checked_add_mr(quotient)
    }
}

/// Holds functions for casts from and to f64.
/// This exists to fit different primitives in the `MapRange` trait.
trait CheckedNumberCastsToFloat: Sized {
    fn checked_f64_cast(&self) -> Option<f64>;
    fn checked_cast_back(other: f64) -> Option<Self>;
}
/// Wrapper for arithmetics on primitives.
/// This exists to fit different primitives in the `MapRange` trait
trait CheckedNumberArithmetics: Sized {
    fn checked_add_mr(&self, other: Self) -> Option<Self>;
    fn checked_sub_mr(&self, other: Self) -> Option<Self>;
    fn checked_mul_mr(&self, other: Self) -> Option<Self>;
    fn checked_div_mr(&self, other: Self) -> Option<Self>;
}

impl MapRange for f32 {}
#[rustfmt::skip]
impl CheckedNumberCastsToFloat for f32 {
    fn checked_f64_cast(&self) -> Option<f64> { Some(*self as f64) }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > f32::MAX as f64 || other < f32::MIN as f64 {
            return None;
        }
        Some(other as f32)
    }
}
impl CheckedNumberArithmetics for f32 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> {
        if Self::MAX - self <= other || Self::MAX - other <= *self {
            None
        } else {
            Some(self + other)
        }
    }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> {
        Some(self - other)
    }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> {
        if (*self != 0. || other != 0.)
            && ((Self::MAX / self) <= other && (Self::MAX / other) <= *self)
        {
            None
        } else {
            Some(*self * other)
        }
    }
    fn checked_div_mr(&self, other: Self) -> Option<Self> {
        if other == 0. {
            return None;
        }
        Some(self / other)
    }
}
impl MapRange for f64 {}
#[rustfmt::skip]
impl CheckedNumberCastsToFloat for f64 {
    fn checked_f64_cast(&self) -> Option<f64> { Some(*self) }
    fn checked_cast_back(other: f64) -> Option<Self> { Some(other) }
}
impl CheckedNumberArithmetics for f64 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> {
        if Self::MAX - self <= other || Self::MAX - other <= *self {
            None
        } else {
            Some(self + other)
        }
    }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> {
        Some(self - other)
    }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> {
        if (*self != 0. || other != 0.)
            && ((Self::MAX / self) <= other && (Self::MAX / other) <= *self)
        {
            None
        } else {
            Some(*self * other)
        }
    }
    fn checked_div_mr(&self, other: Self) -> Option<Self> {
        if other == 0. {
            return None;
        }
        Some(self / other)
    }
}
impl MapRange for u8 {}
impl CheckedNumberCastsToFloat for u8 {
    #[rustfmt::skip]
    fn checked_f64_cast(&self) -> Option<f64> { Some((*self) as f64) }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > u8::MAX as f64 || other < u8::MIN as f64 {
            return None;
        }
        Some(other as u8)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u8 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for u16 {}
impl CheckedNumberCastsToFloat for u16 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > u16::MAX as f64 || other < u16::MIN as f64 {
            return None;
        }
        Some(other as u16)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u16 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for u32 {}
impl CheckedNumberCastsToFloat for u32 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > u32::MAX as f64 || other < u32::MIN as f64 {
            return None;
        }
        Some(other as u32)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u32 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for u64 {}
impl CheckedNumberCastsToFloat for u64 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > u64::MAX as f64 || other < u64::MIN as f64 {
            return None;
        }
        Some(other as u64)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u64 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for usize {}
impl CheckedNumberCastsToFloat for usize {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > usize::MAX as f64 || other < usize::MIN as f64 {
            return None;
        }
        Some(other as usize)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for usize {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i8 {}
impl CheckedNumberCastsToFloat for i8 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > i8::MAX as f64 || other < i8::MIN as f64 {
            return None;
        }
        Some(other as i8)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i8 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i16 {}
impl CheckedNumberCastsToFloat for i16 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > i16::MAX as f64 || other < i16::MIN as f64 {
            return None;
        }
        Some(other as i16)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i16 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i32 {}
impl CheckedNumberCastsToFloat for i32 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > i32::MAX as f64 || other < i32::MIN as f64 {
            return None;
        }
        Some(other as i32)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i32 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i64 {}
impl CheckedNumberCastsToFloat for i64 {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > i64::MAX as f64 || other < i64::MIN as f64 {
            return None;
        }
        Some(other as i64)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i64 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for isize {}
impl CheckedNumberCastsToFloat for isize {
    fn checked_f64_cast(&self) -> Option<f64> {
        Some(*self as f64)
    }
    fn checked_cast_back(other: f64) -> Option<Self> {
        if other > isize::MAX as f64 || other < isize::MIN as f64 {
            return None;
        }
        Some(other as isize)
    }
}
#[rustfmt::skip]
impl CheckedNumberArithmetics for isize {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_linear_interpolation_unsigned() {
        assert_eq!(Some(15), 5_u8   .map_range((0, 10), (10, 20)));
        assert_eq!(Some(15), 5_u16  .map_range((0, 10), (10, 20)));
        assert_eq!(Some(15), 5_u32  .map_range((0, 10), (10, 20)));
        assert_eq!(Some(15), 5_u64  .map_range((0, 10), (10, 20)));
        assert_eq!(Some(127), 512_usize.map_range((0, 1024), (0, 255)));
    }
    #[test]
    #[rustfmt::skip]
    fn test_linear_interpolation_signed() {
        assert_eq!(Some(15), 5_i8   .map_range((0, 10), (10, 20)));
        assert_eq!(Some(15), 5_i16  .map_range((0, 10), (10, 20)));
        assert_eq!(Some(15), 5_i32  .map_range((0, 10), (10, 20)));
        assert_eq!(Some(15), 5_i64  .map_range((0, 10), (10, 20)));
        assert_eq!(Some(5), 15_i64  .map_range((10, 20), (0, 10)));
        assert_eq!(Some(127), 512_isize.map_range((0, 1024), (0, 255)));
    }
    #[test]
    #[rustfmt::skip]
    fn test_linear_interpolation_float() {
        assert_eq!(Some(15.), 5_f32.map_range((0., 10.), (10., 20.)));
        assert_eq!(Some(127.5), 512_f64.map_range((0., 1024.), (0., 255.)));
        assert_eq!(Some(15.), 5_f64.map_range((0., 10.), (10., 20.)));
    }
    #[test]
    fn test_casting() {
        assert_eq!(Some(5.), 5_u8.checked_f64_cast());
        assert_eq!(Some(0.), 0_u8.checked_f64_cast());
        assert_eq!(Some(10.), 10_u8.checked_f64_cast());
        assert_eq!(Some(20.), 20_u8.checked_f64_cast());
        assert_eq!(Some(15), u8::checked_cast_back(15_f64));
        assert_eq!(Some(15.), f64::checked_cast_back(15_f64));
    }
}
