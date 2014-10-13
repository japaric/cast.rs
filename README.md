# `cast.rs`

Machine scalar casting that meets your expectations:

- Integer/float promotion awareness
- NaN awareness
- No underflows nor overflows
- Use of `Result` only when it makes sense

# Examples

When this library compiles (see [blockers](#blockers)) you should be able to
do all these operations:

``` rust
// Promotion
// These casts always succeed, no need to return `Option`/`Result`
assert_eq!(0u8.to::<u16>(), 0u16);  // "casting to" operation
assert_eq!(f64::from(0f32), 0f64);  // "casting from" operation

// "Half" promotion
// e.g. i8 -> u16 can *only* underflow
assert_eq!(127i8.to::<u16>(), Ok(127u16));
assert_eq!(-1i8.to::<u16>(), Err(Underflow));

// From unsigned
// e.g. u16 -> u8 can *only* overflow
assert_eq!(255u16.to::<u8>(), Ok(255u8));
assert_eq!(256u16.to::<u8>(), Err(Overflow));

// From signed
// e.g. i16 -> i8 can underflow or overflow
assert_eq!(127i16.to::<i8>(), Ok(127i8));
assert_eq!(128i16.to::<i8>(), Err(FromInteger::Overflow));
assert_eq!(-129i16.to::<i8>(), Err(FromInteger::Underflow));

// From float
// Casting from floats has to deal with NaN, overflows and underflows
assert_eq!(127f32.to::<i8>(), Ok(127i8));
assert_eq!(128f32.to::<i8>(), Err(FromFloat::Overflow));
assert_eq!(-129f32.to::<i8>(), Err(FromFloat::Underflow));
assert_eq!((0f32 / 0.).to::<i8>(), Err(FromFloat::NaN));
```

# Blockers

This library **won't** compile at the moment because it uses features that have
not been yet implemented or are still WIP.

This is the list of blockers:

- Associated types
- Namespaced enums (optional, can be worked around)
- UFCS (also optional, but highly desired, e.g. `CastFrom::from` vs `u8::from`)

# Generic programming

Although I could define traits that could encode useful abstractions like:

- Any integer can be promoted to any float
- `f32` can be promoted to any float

Which are useful when writing "generic precision" code. I've decided not to
bake those in this library, instead I plan to create another crate that deals
specifically with the "generic precision" use case.

In other words, the traits defined here are not meant to be used for generic
programming, but instead as a "concrete" API.

# License

cast.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.
