#![no_std]
use core::fmt::Display;

/// This holds a function that maps a number from one range to another.
/// This is designed to work in `no_std` environments
#[allow(private_bounds)]
pub trait MapRange: Sized + Copy + PartialOrd + CheckedNumberArithmetics + Display {
    /// Maps the value over the given ranges.
    ///
    /// The `inputvalue` must be inside the `from_range`,
    /// then this produces a `resultvalue` that lives inside `to_range`.
    ///
    /// ```
    /// use map_to_range::{MapRange};
    ///
    /// let test: usize = 5;
    /// assert_eq!(Some(15), test.map_range((0, 10), (10, 20)));
    /// assert_eq!(None, test.map_range((10, 20), (20, 30)));
    /// ```
    ///
    /// WARNING: If the ranges are to big for the type used, this function returns None, even if
    /// the ranges __seem__ to fit the type, because the computations applyed can temporarrily
    /// produce results that do not fit.
    /// (We guarantee in those cases to return None and do not panic)
    /// If the performace allows it, its best to cast the input type to f32 or f64 before inputting
    /// it into this function and cast the result back to the type.
    fn map_range(&self, from_range: (Self, Self), to_range: (Self, Self)) -> Option<Self> {
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

/// Wrapper for arithmetics on primitives.
/// This exists to fit different primitives in the `MapRange` trait
trait CheckedNumberArithmetics: Sized {
    fn checked_add_mr(&self, other: Self) -> Option<Self>;
    fn checked_sub_mr(&self, other: Self) -> Option<Self>;
    fn checked_mul_mr(&self, other: Self) -> Option<Self>;
    fn checked_div_mr(&self, other: Self) -> Option<Self>;
}

impl MapRange for f32 {}
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
#[rustfmt::skip]
impl CheckedNumberArithmetics for u8 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for u16 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u16 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for u32 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u32 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for u64 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for u64 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for usize {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for usize {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i8 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i8 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i16 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i16 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i32 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i32 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for i64 {}
#[rustfmt::skip]
impl CheckedNumberArithmetics for i64 {
    fn checked_add_mr(&self, other: Self) -> Option<Self> { self.checked_add(other) }
    fn checked_sub_mr(&self, other: Self) -> Option<Self> { self.checked_sub(other) }
    fn checked_mul_mr(&self, other: Self) -> Option<Self> { self.checked_mul(other) }
    fn checked_div_mr(&self, other: Self) -> Option<Self> { self.checked_div(other) }
}
impl MapRange for isize {}
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
    }
}
