## Rust Type Conversions

* Rust is a quite strictly statically typed language. Things
  like this won't succeed

        let x: u16 = 0u8;

  because the types don't exactly match

* To make the language livable, there's a bunch of type conversion
  that goes on

* Let's focus on the stuff that is applicable to
  numbers, leaving the fancy conversions for a future chat

## Four Kinds Of Conversion

* Via `From`, `Into`

* Via `TryFrom`, `TryInto`

* Via `as` casts

## `From`, `Into`

* `From` and `Into` are *traits*, which are a fancy Rust
  thing that we don't need to worry about much for now

* These traits are in-scope in standard Rust: you may use
  the `from()` and `into()` methods they provide for a lot
  of conversions

## Using `from()` and `into()`

* `from()` requires an explicit type

        let x = u16::from(0u8);

* `into()` requires a valid type context

        let x: u16 = 0u8.into();

## Limitations of `From` and `Into`

* These conversions must always succeed: they must return a
  correct result and cannot panic

* Because of this, some conversions are just not possible. This

        let x: i16 = 0u16.into();

  won't work, because *some* `u16`s are too big to fit in an `i16`

## `TryFrom` and `TryInto`

y* These traits provide `try_from()` and `try_into()` methods
  that return a `Result` that is either an `Ok` converted value
  or an `Err` error

        let x: i16 = 0u16.try_into().unwrap();

  is a perfectly valid conversion that we know will succeed

* If your `Cargo.toml` has `edition = "2021"` you get these
  traits automatically. Otherwise you must

        use std::convert::{TryFrom, TryInto};

  to get them

## "Casting" with `as`

* A variety of conversions are possible by using `as`
  "casts"

* These casts may be coercions or conversions (like C)

* `as` casts will never panic: they are guaranteed to "succeed"

## Dangers of `as`

* This is legal

        let x = 0i16 as u16

  but bad practice in general, because if the conversion
  would fail you have to read the manual to find out what
  happens, and it's probably bad

* Guess what these do?

        let x = 0xffffu16 as i16;
        let x = 0xfffffffeu32 as f32 as u32;
        let x = 1.0e20_f32 as u32;
        let x = (0.0f32 / 0.0f32) as u32;

## Beyond Numeric Types

* `Vec<u8>`, `&[u8]`, `Box<[u8]>`

* `String`, `&str`, `char`

* Other standard conversion traits, e.g. `Owned`

* Implementing for your own types
