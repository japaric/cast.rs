//! Ergonomic, checked cast functions for primitive types
//!
//! This crate provides one checked cast function for each numeric primitive. Use these functions
//! to perform a cast from any other numeric primitive:
//!
//! ```
//! extern crate cast;
//!
//! use cast::{u8, u16, Error};
//!
//! # fn main() {
//! // Infallible operations, like integer promotion, are equivalent to a normal cast with `as`
//! assert_eq!(u16(0u8), 0u16);
//!
//! // Everything else will return a `Result` depending on the success of the operation
//! assert_eq!(u8(0u16), Ok(0u8));
//! assert_eq!(u8(256u16), Err(Error::Overflow));
//! assert_eq!(u8(-1i8), Err(Error::Underflow));
//! assert_eq!(u8(1. / 0.), Err(Error::Infinite));
//! assert_eq!(u8(0. / 0.), Err(Error::NaN));
//! # }
//! ```
//!
//! There are no namespace problems between these functions, the "primitive modules" in `core`/`std`
//! and the built-in primitive types, so all them can be in the same scope:
//!
//! ```
//! extern crate cast;
//!
//! use std::u8;
//! use cast::{u8, u16};
//!
//! # fn main() {
//! // `u8` as a type
//! let x: u8 = 0;
//! // `u8` as a module
//! let y = u16(u8::MAX);
//! // `u8` as a function
//! let z = u8(y).unwrap();
//! # }
//! ```
//!
//! The checked cast functionality is also usable with type aliases via the `cast` static method:
//!
//! ```
//! extern crate cast;
//!
//! use std::os::raw::c_ulonglong;
//! // NOTE avoid shadowing `std::convert::From` - cf. rust-lang/rfcs#1311
//! use cast::From as _0;
//!
//!
//! # fn main() {
//! assert_eq!(c_ulonglong::cast(0u8), 0u64);
//! # }
//! ```
//!
//! This crate also provides a `From` trait that can be used, for example, to create a generic
//! function that accepts any type that can be infallibly casted to `u32`.
//!
//! ```
//! extern crate cast;
//!
//! fn to_u32<T>(x: T) -> u32
//!     // reads as: "where u32 can be casted from T with output u32"
//!     where u32: cast::From<T, Output=u32>,
//! {
//!     cast::u32(x)
//! }
//!
//! # fn main() {
//! assert_eq!(to_u32(0u8), 0u32);
//! assert_eq!(to_u32(1u16), 1u32);
//! assert_eq!(to_u32(2u32), 2u32);
//!
//! // to_u32(-1i32);  // Compile error
//! # }
//! ```
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#![cfg_attr(all(feature = "unstable", test), feature(plugin))]
#![cfg_attr(all(feature = "unstable", test), plugin(quickcheck_macros))]

#[cfg(all(feature = "unstable", test))]
extern crate quickcheck;

#[cfg(test)]
mod test;

/// Cast errors
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Infinite value casted to a type that can only represent finite values
    Infinite,
    /// NaN value casted to a type that can't represent a NaN value
    NaN,
    /// Source value is greater than the maximum value that the destination type can hold
    Overflow,
    /// Source value is smaller than the minimum value that the destination type can hold
    Underflow,
}

/// The "cast from" operation
pub trait From<Src> {
    /// The result of the cast operation: either `Self` or `Result<Self, Error>`
    type Output;

    /// Checked cast from `Src` to `Self`
    fn cast(Src) -> Self::Output;
}

macro_rules! fns {
    ($($ty:ident),+) => {
        $(
            /// Checked cast function
            pub fn $ty<T>(x: T) -> <$ty as From<T>>::Output
                where $ty: From<T>
            {
                <$ty as From<T>>::cast(x)
            }
         )+
    }
}

fns!(f32, f64, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

/// `$dst` can hold any value of `$src`
macro_rules! promotion {
    ($($src:ty => $($dst: ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = $dst;

                    fn cast(src: $src) -> $dst {
                        src as $dst
                    }
                }
            )+
        )+
    }
}

/// `$dst` can hold any positive value of `$src`
macro_rules! half_promotion {
    ($($src:ty => $($dst:ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    fn cast(src: $src) -> Self::Output {
                        if src < 0 {
                            Err(Error::Underflow)
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

/// From an unsigned `$src` to a smaller `$dst`
macro_rules! from_unsigned {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    fn cast(src: $src) -> Self::Output {
                        use core::$dst;

                        if src > $dst::MAX as $src {
                            Err(Error::Overflow)
                        } else {
                            Ok(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

/// From a signed `$src` to a smaller `$dst`
macro_rules! from_signed {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    fn cast(src: $src) -> Self::Output {
                        use core::$dst;

                        Err(if src < $dst::MIN as $src {
                            Error::Underflow
                        } else if src > $dst::MAX as $src {
                            Error::Overflow
                        } else {
                            return Ok(src as $dst);
                        })
                    }
                }
            )+
        )+
    }
}

/// From a float `$src` to an integer `$dst`
macro_rules! from_float {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Result<$dst, Error>;

                    fn cast(src: $src) -> Self::Output {
                        use core::{$dst, $src};

                        Err(if src != src {
                            Error::NaN
                        } else if src == $src::INFINITY || src == $src::NEG_INFINITY {
                            Error::Infinite
                        } else if src < $dst::MIN as $src {
                            Error::Underflow
                        } else if src > $dst::MAX as $src {
                            Error::Overflow
                        } else {
                            return Ok(src as $dst);
                        })
                    }
                }
            )+
        )+
    }
}

// PLAY TETRIS! ;-)

#[cfg(target_pointer_width = "32")]
mod _32 {
    use {Error, From};

    // Signed
    promotion! {
        i8    => f32, f64, i8, i16, i32, isize, i64;
        i16   => f32, f64,     i16, i32, isize, i64;
        i32   => f32, f64,          i32, isize, i64;
        isize => f32, f64,          i32, isize, i64;
        i64   => f32, f64,                      i64;
    }

    half_promotion! {
        i8    =>                                     u8, u16, u32, usize, u64;
        i16   =>                                         u16, u32, usize, u64;
        i32   =>                                              u32, usize, u64;
        isize =>                                              u32, usize, u64;
        i64   =>                                                          u64;
    }

    from_signed! {

        i16   =>           i8,                       u8;
        i32   =>           i8, i16,                  u8, u16;
        isize =>           i8, i16,                  u8, u16;
        i64   =>           i8, i16, i32, isize,      u8, u16, u32, usize;
    }

    // Unsigned
    promotion! {
        u8    => f32, f64,     i16, i32, isize, i64, u8, u16, u32, usize, u64;
        u16   => f32, f64,          i32, isize, i64,     u16, u32, usize, u64;
        u32   => f32, f64,                      i64,          u32, usize, u64;
        usize => f32, f64,                      i64,          u32, usize, u64;
        u64   => f32, f64,                                                u64;
    }

    from_unsigned! {
        u8    =>           i8;
        u16   =>           i8, i16,                  u8;
        u32   =>           i8, i16, i32, isize,      u8, u16;
        usize =>           i8, i16, i32, isize,      u8, u16;
        u64   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize;
    }

    // Float
    promotion! {
        f32   => f32, f64;
        f64   =>      f64;
    }

    from_float! {
        f32   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize, u64;
        f64   =>           i8, i16, i32, isize, i64, u8, u16, u32, usize, u64;
    }
}

#[cfg(target_pointer_width = "64")]
mod _64 {
    use {Error, From};

    // Signed
    promotion! {
        i8    => f32, f64, i8, i16, i32, i64, isize;
        i16   => f32, f64,     i16, i32, i64, isize;
        i32   => f32, f64,          i32, i64, isize;
        i64   => f32, f64,               i64, isize;
        isize => f32, f64,               i64, isize;
    }

    half_promotion! {
        i8    =>                                     u8, u16, u32, u64, usize;
        i16   =>                                         u16, u32, u64, usize;
        i32   =>                                              u32, u64, usize;
        i64   =>                                                   u64, usize;
        isize =>                                                   u64, usize;
    }

    from_signed! {

        i16   =>           i8,                       u8;
        i32   =>           i8, i16,                  u8, u16;
        i64   =>           i8, i16, i32,             u8, u16, u32;
        isize =>           i8, i16, i32,             u8, u16, u32;
    }

    // Unsigned
    promotion! {
        u8    => f32, f64,     i16, i32, i64, isize, u8, u16, u32, u64, usize;
        u16   => f32, f64,          i32, i64, isize,     u16, u32, u64, usize;
        u32   => f32, f64,               i64, isize,          u32, u64, usize;
        u64   => f32, f64,                                         u64, usize;
        usize => f32, f64,                                         u64, usize;
    }

    from_unsigned! {
        u8    =>           i8;
        u16   =>           i8, i16,                  u8;
        u32   =>           i8, i16, i32,             u8, u16;
        u64   =>           i8, i16, i32, i64, isize, u8, u16, u32;
        usize =>           i8, i16, i32, i64, isize, u8, u16, u32;
    }

    // Float
    promotion! {
        f32  => f32, f64;
        f64  =>      f64;
    }

    from_float! {
        f32  =>           i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;
        f64  =>           i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;
    }
}

// The missing piece
impl From<f64> for f32 {
    type Output = Result<f32, Error>;

    fn cast(src: f64) -> Self::Output {
        use core::{f32, f64};

        if src != src || src == f64::INFINITY || src == f64::NEG_INFINITY {
            Ok(src as f32)
        } else if src < f32::MIN as f64 {
            Err(Error::Underflow)
        } else if src > f32::MAX as f64 {
            Err(Error::Overflow)
        } else {
            Ok(src as f32)
        }
    }
}
