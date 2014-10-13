#![feature(associated_types, macro_rules)]

//! Machine scalar casting that meets your expectations

/// Overflow error
pub struct Overflow;

/// Underflow error
pub struct Underflow;

/// Errors that may arise when casting from integers
pub enum FromInteger {
    Overflow,
    Underflow,
}

/// Errors that may arise when casting from floats
pub enum FromFloat {
    NaN,
    Overflow,
    Underflow,
}

/// The "casting from" operation
pub trait CastFrom<Src> {
    type Output;

    fn from(src: Src) -> Output;
}

/// The "casting to" operation
pub trait CastTo {
    fn to<Dst>(self) -> Dst::Output where Dst: CastFrom<Self> {
        Dst::from(self)
    }
}

macro_rules! impl_cast_to {
    ($($ty:ty),+) => {$(
        impl CastTo for $ty {})+
    }
}

impl_cast_to!(int, i8, i16, i32, i64, uint, u8, u16, u32, u64, f32, f64)

macro_rules! promotion {
    ($($src:ty -> $($dst: ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = $dst;

            #[inline(always)]
            fn from(src: $src) -> $dst {
                 // NB Sanity check
                debug_assert!(mem::size_of::<$src>() <= mem::size_of::<$dst>());

                src as $dst
            }
        })+)+
    }
}

macro_rules! half_promotion {
    ($($src:ty -> $($dst:ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = Result<$dst, Underflow>;

            fn from(src: $src) -> Result<$dst, Underflow> {
                 // NB Sanity check
                debug_assert!(mem::size_of::<$src>() <= mem::size_of::<$dst>());

                if self < 0 {
                    Err(Underflow)
                } else {
                    Ok(src as $dst)
                }
            }
        })+)+
    }
}

macro_rules! from_unsigned {
    ($($src:ty -> $($dst:ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = Result<$dst, Overflow>;

            fn from(src: $src) -> Result<$dst, Overflow> {
                // NB Sanity check
                debug_assert!(mem::size_of::<$src>() >= mem::size_of::<$dst>());

                if self > $dst::max_value() as $src {
                    Err(Overflow)
                } else {
                    Ok(src as $dst)
                }
            }
        })+)+
    }
}

macro_rules! from_signed {
    ($($src:ty -> $($dst:ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = Result<$dst, FromInteger>;

            fn cast(self) -> Result<$dst, FromInteger> {
                // NB Sanity check
                debug_assert!(mem::size_of::<$src>() > mem::size_of::<$dst>());

                if src < $dst::min_value() as $src {
                    Err(FromInteger::Underflow)
                } else if src > $dst::max_value() as $src {
                    Err(FromInteger::Overflow)
                } else {
                    Ok(src as $dst)
                }
            }
        })+)+
    }
}

macro_rules! from_float {
    ($($src:ty -> $($dst:ty),+);+;) => {$($(
        impl CastFrom<$src> for $dst {
            type Output = Result<$dst, FromFloat>;

            fn from(src: $src) -> Result<$dst, FromFloat> {
                if src.is_nan() {
                    Err(FromFloat::Nan)
                } else if src < $dst::min_value() as $src {
                    Err(FromFloat::Underflow)
                } else if src > $dst::max_value() as $src {
                    Err(FromFloat::Overflow)
                } else {
                    Ok(src as $dst)
                }
            }
        })+)+
    }
}

// Signed
promotion!{
    i8   -> f32, f64, i8, i16, i32, i64;
    i16  -> f32, f64,     i16, i32, i64;
    i32  -> f32, f64,          i32, i64;
    i64  -> f32, f64,               i64;
}

half_promotion!{
    i8   ->                              u8, u16, u32, u64;
    i16  ->                                  u16, u32, u64;
    i32  ->                                       u32, u64;
    i64  ->                                            u64;
}

from_signed!{

    i16  ->           i8,                u8;
    i32  ->           i8, i16,           u8, u16;
    i64  ->           i8, i16, i32,      u8, u16, u32;
}

// Unsigned
promotion!{
    u8   -> f32, f64,     i16, i32, i64, u8, u16, u32, u64;
    u16  -> f32, f64,          i32, i64,     u16, u32, u64;
    u32  -> f32, f64,               i64,          u32, u64;
    u64  -> f32, f64,                                  u64;
}

from_unsigned!{
    u8   ->           i8;
    u16  ->           i8, i16,           u8;
    u32  ->           i8, i16, i32,      u8, u16;
    u64  ->           i8, i16, i32, i64, u8, u16, u32;
}

// Float
promotion!{
    f32  -> f32, f64;
    f64  ->      f64;
}

from_float!{
    f32  ->           i8, i16, i32, i64, u8, u16, u32, u64;
    f64  -> f32,      i8, i16, i32, i64, u8, u16, u32, u64;
}

// Machine dependent scalars
#[cfg(any(target_arch = "x86", target_arch = "arm"))]
promotion!{
    int  -> f32, f64,          i32, i64;
}

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
half_promotion!{
    int  ->                                       u32, u64;
}

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
from_signed!{
    int  ->           i8, i16,           u8, u16;
}

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
promotion!{
    uint -> f32, f64,               i64,          u32, u64;
}

#[cfg(any(target_arch = "x86", target_arch = "arm"))]
from_unsigned!{
    uint ->           i8, i16, i32,      u8, u16;
}

#[cfg(any(target_arch = "x86_64"))]
promotion!{
    int  -> f32, f64,               i64;
}

#[cfg(any(target_arch = "x86_64"))]
half_promotion!{
    int  ->                                            u64;
}

#[cfg(any(target_arch = "x86_64"))]
from_signed!{
    int  ->           i8, i16, i32,      u8, u16, u32;
}

#[cfg(any(target_arch = "x86_64"))]
promotion!{
    uint -> f32, f64,                                  u64;
}

#[cfg(any(target_arch = "x86_64"))]
from_unsigned!{
    uint ->           i8, i16, i32, i64, u8, u16, u32;
}
