//! Checked scalar casting
//!
//! # Examples
//!
//! ```
//! extern crate cast;
//!
//! use cast::From;
//!
//! # fn main() {
//! // Infallible operations, like integer promotion, are equivalent to an `as` call
//! assert_eq!(u16::from(0u8), 0u16);
//!
//! // Everything else will return an `Option` depending on the success of the operation
//! assert_eq!(u8::from(256u16), None);  // Overflow
//! assert_eq!(u8::from(-1i8), None);  // Underflow
//! assert_eq!(u8::from(0. / 0.), None);  // NaN
//! assert_eq!(u8::from(127i8), Some(127u8));  // OK
//! # }
//! ```
//!
//! # `from_` vs `from`
//!
//! Importing `cast::From` shadows `std::convert::From`, so you can no longer write code like
//! `Cow::from("Hello")`. The logical solution is to rename the `cast::From` import:
//!
//! ``` ignore
//! extern crate cast;
//!
//! use std::borrow::Cow;
//!
//! // don't shadow `std::convert::From`
//! use cast::From as _0;
//!
//! # fn main() {
//! Cow::from("Hello");
//! u16::from(0u8);  //~ error: multiple applicable methods in scope
//! # }
//! ```
//!
//! But then you'll hit this [bug](https://github.com/rust-lang/rust/issues/24382). The workaround
//! for these cases where you want to use both `convert::From` and `cast::From` in the *same scope*
//! is to use the `from_` method to refer to the latter trait.
//!
//! ```
//! extern crate cast;
//!
//! use std::borrow::Cow;
//!
//! // don't shadow `std::convert::From`
//! use cast::From as _0;
//!
//! # fn main() {
//! Cow::from("Hello");
//! u16::from_(0u8);
//! # }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

#[cfg(test)]
mod test;

/// The "cast from" operation
pub trait From<Src> {
    /// The result of the cast operation: either `Self` or `Option<Self>`
    type Output;

    /// Checked cast from `Src` to `Self`
    fn from(Src) -> Self::Output;

    /// Workaround for rust-lang/rust#24382. See module docs for details
    ///
    /// NOTE: This function may be removed/deprecated after that bug has been fixed
    fn from_(src: Src) -> Self::Output {
        Self::from(src)
    }
}

macro_rules! promotion {
    ($($src:ty => $($dst: ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = $dst;

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
                impl From<$src> for $dst {
                    type Output = Option<$dst>;

                    fn from(src: $src) -> Option<$dst> {
                        if src < 0 {
                            None
                        } else {
                            Some(src as $dst)
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
                impl From<$src> for $dst {
                    type Output = Option<$dst>;

                    fn from(src: $src) -> Option<$dst> {
                        if src > <$dst>::max_value() as $src {
                            None
                        } else {
                            Some(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! from_signed {
    ($($src:ty => $($dst:ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Option<$dst>;

                    fn from(src: $src) -> Option<$dst> {
                        if src < <$dst>::min_value() as $src ||
                            src > <$dst>::max_value() as $src
                        {
                            None
                        } else {
                            Some(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

macro_rules! from_float {
    ($($src:ty => $($dst:ty),+);+;) => {
        $(
            $(
                impl From<$src> for $dst {
                    type Output = Option<$dst>;

                    fn from(src: $src) -> Option<$dst> {
                        if src.is_nan() ||
                            src < <$dst>::min_value() as $src ||
                                src > <$dst>::max_value() as $src
                        {
                            None
                        } else {
                            Some(src as $dst)
                        }
                    }
                }
            )+
        )+
    }
}

// PLAY TETRIS! ;-)

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
mod _32 {
    use From;

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
}

#[cfg(any(target_arch = "x86_64"))]
mod _64 {
    use From;

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
}

// The missing piece
impl From<f64> for f32 {
    type Output = Option<f32>;

    fn from(src: f64) -> Option<f32> {
        #![allow(deprecated)]

        use std::f32::{MAX_VALUE, MIN_VALUE};

        if src.is_nan() || src.is_infinite() {
            Some(src as f32)
        } else if src < MIN_VALUE as f64 || src > MAX_VALUE as f64 {
            None
        } else {
            Some(src as f32)
        }
    }
}
