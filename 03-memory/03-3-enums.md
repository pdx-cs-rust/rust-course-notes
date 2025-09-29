## Enums Are Disjoint Unions

* AKA "sum types". Idea is to have a single value represent
  one of several alternatives

        enum EnumType { Value1, Value2 }

* Alternatives can be names, or *value-carrying* names

        enum Point {
            Point0,
            Point1(u64),
            Point2(u64, u64),
            Point3(u64, u64, u64),
        }

## Enum Names

* Alternatives normally carry type-qualified names

        enum EnumType { Value1, Value2 }

        ...

        let x = EnumType::Value1;

  but there is a trick:

          use EnumType::*;

          let x = Value1;
  
## Enum Misc

* Can impl an enum just as with a struct

* Enum values come in tuple or structured form: tuple is
  more "normal", because…
  
* Pattern matching is the common way to extract carried
  values from an enum
  
* Compiler checks that all alternatives are matched, as with
  other types

* Generics and lifetimes work the same as with structs

## The Two Most Important Enums In Rust: Option

          pub enum Option<T> {
              None,
              Some(T),
          }
          pub use Option::*;

          // let x: &u8 = 0; // Compiler says no
          let x: Option<&u8> = None;
          // let Some(ptr) = x; // Compiler says no
          match x {
              None => {
                  Err("missing x")
              }
              Some(&x) if x == 7 => {
                  println!("all good");
                  Ok(())
              }
              Some(&x) => {
                  Err("bad x")
              }
          }

## The Two Most Important Enums In Rust: Result

          enum Result<T, E> {
              Ok(T),
              Err(E),
          }
          use Result::*;

## Non-Value-Carrying Enums Are "Special"

* Alternatives have a corresponding usize value: default 0, 1, 2 …

* Can set values for elements (unspecified elements
  autoincrement, be careful)

* Can cast alternative to usize, but no built-in way to
  convert usize to alternative

          enum CLike {
              A,
              B,
              C,
          }

          let x = CLike::B as usize;
          // let y = 2 as CLike; // compiler error

## The "Void" enum

* What should `enum Void;` mean?

          let x = // ???

* Apparently no inhabitants of this type

* Thus, can't really pass it around or whatever

* The type "!" is around…

        fn fail(cond: bool) -> ! {
            std::process::exit(1);
        }

## Why Are Sum Types A Big Deal?

* Avoid redundant storage, as with C union

* "Tags" are managed by the language, so no tag mismatch
  issues
  
* Combine (limited) convenience with type and memory safety

* "And-or trees" are a thing

## Examples

<http://github.com/pdx-cs-rust/playing-cards>

<http://github.com/PoHuit/plan-b>
