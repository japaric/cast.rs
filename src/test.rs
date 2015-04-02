use From;

#[test]
fn promotion() {
    assert_eq!(i8::from(0i8), 0i8);
    assert_eq!(i16::from(0i8), 0i16);
    assert_eq!(i32::from(0i8), 0i32);
    assert_eq!(i64::from(0i8), 0i64);

    assert_eq!(i16::from(0i16), 0i16);
    assert_eq!(i32::from(0i16), 0i32);
    assert_eq!(i64::from(0i16), 0i64);

    assert_eq!(i32::from(0i32), 0i32);
    assert_eq!(i64::from(0i32), 0i64);

    assert_eq!(i64::from(0i64), 0i64);

    assert_eq!(u8::from(0u8), 0u8);
    assert_eq!(u16::from(0u8), 0u16);
    assert_eq!(u32::from(0u8), 0u32);
    assert_eq!(u64::from(0u8), 0u64);
    assert_eq!(i16::from(0u8), 0i16);
    assert_eq!(i32::from(0u8), 0i32);
    assert_eq!(i64::from(0u8), 0i64);

    assert_eq!(u16::from(0u16), 0u16);
    assert_eq!(u32::from(0u16), 0u32);
    assert_eq!(u64::from(0u16), 0u64);
    assert_eq!(i32::from(0u16), 0i32);
    assert_eq!(i64::from(0u16), 0i64);

    assert_eq!(u32::from(0u32), 0u32);
    assert_eq!(u64::from(0u32), 0u64);
    assert_eq!(i64::from(0u32), 0i64);

    assert_eq!(u64::from(0u64), 0u64);

    assert_eq!(f32::from(0f32), 0f32);
    assert_eq!(f64::from(0f32), 0f64);

    assert_eq!(f64::from(0f64), 0f64);
}

#[test]
fn half_promotion() {
    assert_eq!(u8::from(1i8), Some(1u8));
    assert_eq!(u16::from(1i8), Some(1u16));
    assert_eq!(u32::from(1i8), Some(1u32));
    assert_eq!(u64::from(1i8), Some(1u64));

    assert_eq!(u8::from(-1i8), None);
    assert_eq!(u16::from(-1i8), None);
    assert_eq!(u32::from(-1i8), None);
    assert_eq!(u64::from(-1i8), None);

    assert_eq!(u16::from(1i16), Some(1u16));
    assert_eq!(u32::from(1i16), Some(1u32));
    assert_eq!(u64::from(1i16), Some(1u64));

    assert_eq!(u16::from(-1i16), None);
    assert_eq!(u32::from(-1i16), None);
    assert_eq!(u64::from(-1i16), None);

    assert_eq!(u32::from(1i32), Some(1u32));
    assert_eq!(u64::from(1i32), Some(1u64));

    assert_eq!(u32::from(-1i32), None);
    assert_eq!(u64::from(-1i32), None);

    assert_eq!(u64::from(1i64), Some(1u64));

    assert_eq!(u64::from(-1i64), None);
}

#[test]
fn nan() {
    assert_eq!(u8::from(0f32 / 0f32), None);
    assert_eq!(u16::from(0f32 / 0f32), None);
    assert_eq!(u32::from(0f32 / 0f32), None);
    assert_eq!(u64::from(0f32 / 0f32), None);
    assert_eq!(i8::from(0f32 / 0f32), None);
    assert_eq!(i16::from(0f32 / 0f32), None);
    assert_eq!(i32::from(0f32 / 0f32), None);
    assert_eq!(i64::from(0f32 / 0f32), None);

    assert!(f32::from(0f32 / 0f32).is_nan());
    assert!(f64::from(0f32 / 0f32).is_nan());

    assert!(f32::from(0f64 / 0f64).unwrap().is_nan());
    assert!(f64::from(0f64 / 0f64).is_nan());
}

#[test]
fn neg_inf() {
    assert_eq!(f32::from(-1f32 / 0f32), -1f32 / 0f32);
    assert_eq!(f64::from(-1f32 / 0f32), -1f64 / 0f64);

    assert_eq!(f32::from(-1f64 / 0f64), Some(-1f32 / 0f32));
    assert_eq!(f64::from(-1f64 / 0f64), -1f64 / 0f64);
}

#[test]
fn plus_inf() {
    assert_eq!(f32::from(1f32 / 0f32), 1f32 / 0f32);
    assert_eq!(f64::from(1f32 / 0f32), 1f64 / 0f64);

    assert_eq!(f32::from(1f64 / 0f64), Some((1f32 / 0f32)));
    assert_eq!(f64::from(1f64 / 0f64), 1f64 / 0f64);
}
