## Traits Are Interface Types

* Define methods, fields and types that must be
  provided by an implementation of a struct or enum

* Principal mechanism for code reuse in Rust

* Interact strongly with and enhance
  "generic" types (parametric polymorphism)

## Defining A Trait

* Just an interface specification

* Here's a hairy version (`examples/addy.rs`)

* Note the following:

  * Trait bounds
  * Associated type
  * Static (default) method
  * Use of Sized marker type
  * Use of Self
  * Clever type inference
  * Implementation of our trait for standard types

## Trait Bounds vs Trait Objects

* Book example (`examples/bno-salad.rs`)

* A specialized version of `generic_salad.add(*v)` will be
  constructed, no argument need be passed, the whole thing
  will be inlined

* Note the awkwardness of working with trait objects. This
  code is fairly fragile

## The Orphan Rules

* Idea: When you call something like `foo.bar()` where `bar`
  is a trait method, the compiler needs to know what
  implementation of `bar()` to use.
  
* Consider one crate providing

         impl ToString for u32 {
              fn to_string(x: &u32) -> String {
                 format!("{}!", x)
              }
         }

  and another

         impl ToString for u32 {
              fn to_string(x: &u32) -> String {
                 format!("{}?", x)
              }
         }

* "Orphan Rules" forbid implementing another crate's trait
  on another trait's type (or thereabouts) to solve this problem

* The details are complicated

* Technical discussion

  * <https://github.com/Ixrec/rust-orphan-rules>

  * <http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/>

## Subtraits

* You can insist on some trait requiring all the methods of
  another trait

* For example, `std` has `trait Copy: Clone`

  * This means that anything that is `Copy` must also be
    `Clone`

  * It also guarantees (the compiler's internal)
    implementations of `Copy` access to `Clone` methods on
    `self`

          impl Clone for MyType {
              fn clone(&self) -> Self {
                  ...
              }
          }

          impl Copy for MyType {}
