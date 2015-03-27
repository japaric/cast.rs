//! Machine scalar casting that meets your expectations
//!
//! This library implements two "smart" checked cast traits: `CastTo` and `CastFrom`. These traits
//! are similar to the `ToPrimitive` and `FromPrimitive` traits from stdlib, but are "aware" of
//! integer and float promotions.
//!
//! Take as an example an integer promotion from `u8` to `u16`:
//!
//! ```
//! #![feature(core)]
//!
//! use cast::CastTo;
//! use std::num::ToPrimitive;
//!
//! // ToPrimitive
//! assert_eq!(0u8.to_u16(), Some(0u16));
//!
//! // CastTo
//! assert_eq!(0u8.to::<u16>(), 0u16);
//! ```
//!
//! Here the `ToPrimitive` version returns a superfluous (because the operation can't "fail")
//! `Option` and incurs in unnecessary bounds checking under the hood. Whereas the `CastTo` version
//! is equivalent to a built-in cast operation: `0u8 as u16`.
//!
//! For other cast operations that are "lossy", the `CastTo` version returns a `Result`, where
//! the `Err` variant explains the error.
//!
//! ```
//! #![feature(core)]
//!
//! use cast::prelude::*;
//! use std::num::ToPrimitive;
//!
//! // ToPrimitive
//! assert_eq!((0f32 / 0f32).to_u8(), Some(0u8));  // ARGH! This is wrong! This should be `None`
//! assert_eq!((-1i16).to_u8(), None);
//! assert_eq!(256i16.to_u8(), None);
//! assert_eq!(0u16.to_u8(), Some(0u8));
//!
//! // CastTo
//! assert_eq!((0f32 / 0f32).to::<u8>(), Err(NaN));
//! assert_eq!(256i16.to::<u8>(), Err(Overflow));
//! assert_eq!((-1i16).to::<u8>(), Err(Underflow));
//! assert_eq!(0u16.to::<u8>(), Ok(0u8));
//! ```
//!
//! If the `CastFrom` trait is in scope you can perform "cast from" operations:
//!
//! ```
//! use cast::prelude::*;
//!
//! assert_eq!(u8::from(0f32 / 0f32), Err(NaN));
//! assert_eq!(u8::from(256i16), Err(Overflow));
//! assert_eq!(u8::from(-1i16), Err(Underflow));
//! assert_eq!(u8::from(0i16), Ok(0u8));
//! ```
//!
//! # Cargo
//!
//! - `Cargo.toml`
//!
//! ``` ignore
//! [dependencies.cast]
//! git = "https://github.com/japaric/cast.rs"
//! ```
//!
//! - Crate file
//!
//! ``` ignore
//! extern crate cast;
//!
//! use cast::prelude::*;
//! ```

#![allow(trivial_numeric_casts)]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(core)]
#![feature(std_misc)]

use Error::*;

pub mod prelude;

#[cfg(test)]
mod test;

/// Errors that may arise from the cast operation: `(value: Source) -> Destination`
#[derive(Copy, Debug, PartialEq)]
pub enum Error {
    /// `value` is `NaN`, but `Destination` can't represent it
    NaN,
    /// `value` > `Destination::max_value()`
    Overflow,
    /// `value` < `Destination::min_value()`
    Underflow,
}

/// The result of a cast operation
pub type Result<T> = ::std::result::Result<T, Error>;

/// The "cast from" operation
pub trait CastFrom<Source> {
    /// The result of the cast operation: either `Self` or `Result<Self>`
    type Output;

    /// Checked cast from `Source` to `Self`
    fn from(Source) -> Self::Output;
}

/// The "cast to" operation
pub trait CastTo: Sized {
    /// Checked cast from `Self` to `Destination`
    fn to<Destination>(self) -> <Destination as CastFrom<Self>>::Output where
        Destination: CastFrom<Self>,
    {
        <Destination as CastFrom<Self>>::from(self)
    }
}

/// A supertrait over `std::num::Float`, with additional information about casts.
///
/// FIXME(rust-lang/rust#22279) Add more `CastFrom<i8, Output=Self>` supertrait bounds
/// FIXME(rust-lang/rust#20671) Add `where i8: CastFrom<Self, Output=Result<i8>>` bounds
pub trait Float: Float_ + CastFrom<f32, Output=Self> {}

/// Hack to have multiple `CastFrom` bounds on a supertrait
pub trait Float_: std::num::Float + CastFrom<usize, Output=Self> {}

impl Float_ for f32 {}
impl Float_ for f64 {}

impl Float for f32 {}
impl Float for f64 {}

macro_rules! impl_cast_to {
    ($($ty:ty),+) => {$(
        impl CastTo for $ty {})+
    }
}

impl_cast_to!(isize, i8, i16, i32, i64, usize, u8, u16, u32, u64, f32, f64);

macro_rules! promotion {
    ($($src:ty => $($dst: ty),+);+;) => {
        $(
            $(
                impl CastFrom<$src> for $dst {
                    type Output = $dst;

                    #[inline(always)]
                    fn from(src: $src) -> $dst {
                        src as $dst
                    }
                }
            )+
        )+
    }
}

macro_rules! half_promotion {
    ($($src:ty => $($dst:ty),+);+;) => {
        $(
            $(
                impl CastFrom<$src> for $dst {
                    type Output = Result<$dst>;

                    fn from(src: $src) -> Result<$dst> {
                        if src < 0 {
                            Err(Underflow)
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! from_unsigned {
    ($($src:ty => $($dst:ty),+);+;) => {
        $(
            $(
                impl CastFrom<$src> for $dst {
                    type Output = Result<$dst>;

                    fn from(src: $src) -> Result<$dst> {
                        let upper_bound: $dst = Int::max_value();
                        if src > upper_bound as $src {
                            Err(Overflow)
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! from_signed {
    ($($src:ty => $($dst:ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = Result<$dst>;

            fn from(src: $src) -> Result<$dst> {
                let lower_bound: $dst = Int::min_value();
                let upper_bound: $dst = Int::max_value();
                if src < lower_bound as $src {
                    Err(Underflow)
                } else if src > upper_bound as $src {
                    Err(Overflow)
                } else {
                    Ok(src as $dst)
                }
            }
        })+)+
    }
}

macro_rules! from_float {
    ($($src:ty => $($dst:ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = Result<$dst>;

            fn from(src: $src) -> Result<$dst> {
                let lower_bound: $dst = Int::min_value();
                let upper_bound: $dst = Int::min_value();
                if src.is_nan() {
                    Err(NaN)
                } else if src < lower_bound as $src {
                    Err(Underflow)
                } else if src > upper_bound as $src {
                    Err(Overflow)
                } else {
                    Ok(src as $dst)
                }
            }
        })+)+
    }
}

// PLAY TETRIS! ;-)

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
mod _32 {
    use std::num::{Float, Int};

    use {CastFrom, Result};
    use Error::*;

    // Signed
    promotion!{
        i8    => f32, f64, i8, i16, i32, isize, i64;
        i16   => f32, f64,     i16, i32, isize, i64;
        i32   => f32, f64,          i32, isize, i64;
        isize => f32, f64,          i32, isize  i64;
        i64   => f32, f64,                      i64;
    }

    half_promotion!{
        i8    =>                                     u8, u16, u32, usize, u64;
        i16   =>                                         u16, u32, usize, u64;
        i32   =>                                              u32, usize, u64;
        isize =>                                              u32, usize, u64;
        i64   =>                                                          u64;
    }

    from_signed!{

        i16   =>           i8,                       u8;
        i32   =>           i8, i16,                  u8, u16;
        isize =>           i8, i16,                  u8, u16;
        i64   =>           i8, i16, i32, isize,      u8, u16, u32, usize;
    }

    // Unsigned
    promotion!{
        u8    => f32, f64,     i16, i32, isize, i64, u8, u16, u32, usize, u64;
        u16   => f32, f64,          i32, isize, i64,     u16, u32, usize, u64;
        u32   => f32, f64,               isize, i64,          u32, usize, u64;
        usize => f32, f64,               isize, i64,          u32, usize, u64;
        u64   => f32, f64,                                                u64;
    }

    from_unsigned!{
        u8    =>           i8;
        u16   =>           i8, i16,                  u8;
        u32   =>           i8, i16, i32, isize,      u8, u16;
        usize =>           i8, i16, i32, isize,      u8, u16;
        u64   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize;
    }

    // Float
    promotion!{
        f32   => f32, f64;
        f64   =>      f64;
    }

    from_float!{
        f32   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize, u64;
        f64   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize, u64;
    }

    impl CastFrom<f64> for f32 {
        type Output = Result<f32>;

        fn from(src: f64) -> Result<f32> {
            let lower_bound: f32 = Float::min_value();
            let upper_bound: f32 = Float::min_value();
            if src.is_nan() || src.is_infinite() {
                Ok(src as f32)
            } else if src < lower_bound as f64 {
                Err(Underflow)
            } else if src > upper_bound as f64 {
                Err(Overflow)
            } else {
                Ok(src as f32)
            }
        }
    }
}

#[cfg(any(target_arch = "x86_64"))]
mod _64 {
    use std::num::{Float, Int};

    use {CastFrom, Result};
    use Error::*;

    // Signed
    promotion!{
        i8    => f32, f64, i8, i16, i32, i64, isize;
        i16   => f32, f64,     i16, i32, i64, isize;
        i32   => f32, f64,          i32, i64, isize;
        i64   => f32, f64,               i64, isize;
        isize => f32, f64,               i64, isize;
    }

    half_promotion!{
        i8    =>                                     u8, u16, u32, u64, usize;
        i16   =>                                         u16, u32, u64, usize;
        i32   =>                                              u32, u64, usize;
        i64   =>                                                   u64, usize;
        isize =>                                                   u64, usize;
    }

    from_signed!{

        i16   =>           i8,                       u8;
        i32   =>           i8, i16,                  u8, u16;
        i64   =>           i8, i16, i32,             u8, u16, u32;
        isize =>           i8, i16, i32,             u8, u16, u32;
    }

    // Unsigned
    promotion!{
        u8    => f32, f64,     i16, i32, i64, isize, u8, u16, u32, u64, usize;
        u16   => f32, f64,          i32, i64, isize,     u16, u32, u64, usize;
        u32   => f32, f64,               i64, isize,          u32, u64, usize;
        u64   => f32, f64,                                         u64, usize;
        usize => f32, f64,                                         u64, usize;
    }

    from_unsigned!{
        u8    =>           i8;
        u16   =>           i8, i16,                  u8;
        u32   =>           i8, i16, i32,             u8, u16;
        u64   =>           i8, i16, i32, i64, isize, u8, u16, u32;
        usize =>           i8, i16, i32, i64, isize, u8, u16, u32;
    }

    // Float
    promotion!{
        f32  => f32, f64;
        f64  =>      f64;
    }

    from_float!{
        f32  =>           i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;
        f64  =>           i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;
    }

    impl CastFrom<f64> for f32 {
        type Output = Result<f32>;

        fn from(src: f64) -> Result<f32> {
            let lower_bound: f32 = Float::min_value();
            let upper_bound: f32 = Float::min_value();
            if src.is_nan() || src.is_infinite() {
                Ok(src as f32)
            } else if src < lower_bound as f64 {
                Err(Underflow)
            } else if src > upper_bound as f64 {
                Err(Overflow)
            } else {
                Ok(src as f32)
            }
        }
    }
}
