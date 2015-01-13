use CastTo;
use Error::*;

#[test]
fn promotion() {
    assert_eq!(0i8.to::<i8>(), 0i8);
    assert_eq!(0i8.to::<i16>(), 0i16);
    assert_eq!(0i8.to::<i32>(), 0i32);
    assert_eq!(0i8.to::<i64>(), 0i64);

    assert_eq!(0i16.to::<i16>(), 0i16);
    assert_eq!(0i16.to::<i32>(), 0i32);
    assert_eq!(0i16.to::<i64>(), 0i64);

    assert_eq!(0i32.to::<i32>(), 0i32);
    assert_eq!(0i32.to::<i64>(), 0i64);

    assert_eq!(0i64.to::<i64>(), 0i64);

    assert_eq!(0u8.to::<u8>(), 0u8);
    assert_eq!(0u8.to::<u16>(), 0u16);
    assert_eq!(0u8.to::<u32>(), 0u32);
    assert_eq!(0u8.to::<u64>(), 0u64);
    assert_eq!(0u8.to::<i16>(), 0i16);
    assert_eq!(0u8.to::<i32>(), 0i32);
    assert_eq!(0u8.to::<i64>(), 0i64);

    assert_eq!(0u16.to::<u16>(), 0u16);
    assert_eq!(0u16.to::<u32>(), 0u32);
    assert_eq!(0u16.to::<u64>(), 0u64);
    assert_eq!(0u16.to::<i32>(), 0i32);
    assert_eq!(0u16.to::<i64>(), 0i64);

    assert_eq!(0u32.to::<u32>(), 0u32);
    assert_eq!(0u32.to::<u64>(), 0u64);
    assert_eq!(0u32.to::<i64>(), 0i64);

    assert_eq!(0u64.to::<u64>(), 0u64);

    assert_eq!(0f32.to::<f32>(), 0f32);
    assert_eq!(0f32.to::<f64>(), 0f64);

    assert_eq!(0f64.to::<f64>(), 0f64);
}

#[test]
fn half_promotion() {
    assert_eq!(1i8.to::<u8>(), Ok(1u8));
    assert_eq!(1i8.to::<u16>(), Ok(1u16));
    assert_eq!(1i8.to::<u32>(), Ok(1u32));
    assert_eq!(1i8.to::<u64>(), Ok(1u64));

    assert_eq!((-1i8).to::<u8>(), Err(Underflow));
    assert_eq!((-1i8).to::<u16>(), Err(Underflow));
    assert_eq!((-1i8).to::<u32>(), Err(Underflow));
    assert_eq!((-1i8).to::<u64>(), Err(Underflow));

    assert_eq!(1i16.to::<u16>(), Ok(1u16));
    assert_eq!(1i16.to::<u32>(), Ok(1u32));
    assert_eq!(1i16.to::<u64>(), Ok(1u64));

    assert_eq!((-1i16).to::<u16>(), Err(Underflow));
    assert_eq!((-1i16).to::<u32>(), Err(Underflow));
    assert_eq!((-1i16).to::<u64>(), Err(Underflow));

    assert_eq!(1i32.to::<u32>(), Ok(1u32));
    assert_eq!(1i32.to::<u64>(), Ok(1u64));

    assert_eq!((-1i32).to::<u32>(), Err(Underflow));
    assert_eq!((-1i32).to::<u64>(), Err(Underflow));

    assert_eq!(1i64.to::<u64>(), Ok(1u64));

    assert_eq!((-1i64).to::<u64>(), Err(Underflow));
}

#[test]
fn nan() {
    use std::num::Float;

    assert_eq!((0f32 / 0f32).to::<u8>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<u16>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<u32>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<u64>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<i8>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<i16>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<i32>(), Err(NaN));
    assert_eq!((0f32 / 0f32).to::<i64>(), Err(NaN));

    assert!((0f32 / 0f32).to::<f32>().is_nan());
    assert!((0f32 / 0f32).to::<f64>().is_nan());

    assert!((0f64 / 0f64).to::<f32>().unwrap().is_nan());
    assert!((0f64 / 0f64).to::<f64>().is_nan());
}

#[test]
fn neg_inf() {
    assert_eq!((-1f32 / 0f32).to::<f32>(), -1f32 / 0f32);
    assert_eq!((-1f32 / 0f32).to::<f64>(), -1f64 / 0f64);

    assert_eq!((-1f64 / 0f64).to::<f32>(), Ok(-1f32 / 0f32));
    assert_eq!((-1f64 / 0f64).to::<f64>(), -1f64 / 0f64);
}

#[test]
fn plus_inf() {
    assert_eq!((1f32 / 0f32).to::<f32>(), 1f32 / 0f32);
    assert_eq!((1f32 / 0f32).to::<f64>(), 1f64 / 0f64);

    assert_eq!((1f64 / 0f64).to::<f32>(), Ok((1f32 / 0f32)));
    assert_eq!((1f64 / 0f64).to::<f64>(), 1f64 / 0f64);
}
