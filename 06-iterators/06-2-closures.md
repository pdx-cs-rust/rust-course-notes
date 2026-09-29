## First-Class Functions

* "First-class functions" are a thing. A thing is
  "first-class" if all the sensible operations of the
  language do indeed apply to it
    
* [`examples/prog.rs`][prog]
  
## Rust Has First Class Functions

* They have their own types

* They can be passed as parameters, returned as results,
  stored in arrays, used as struct and enum fields, etc

* They can be created in any block scope

* Function names act much like static function pointers

## Closures

* The problem with pure functions is that anything they
  compute has to be based on arguments

* A closure is a function that is allowed to access and
  mutate its "environment": names that are statically in
  scope at the point of its declaration

* This means that different calls to the function with the
  same parameters may produce different results

* Closures in Rust are mostly first-class, and can access
  parameters and locals like anything else
  
* Syntax has wacky pipes and stuff; types are optional and
  inferred
  
## Non-Capturing Closures

* Non-capturing closures can be used like ordinary functions

          fn call(f: fn(u64)) {
              f(12);
          }

          fn main() {
              call(|x| println!("{}", x));
          }

## Capture Closures

* Closures that capture the environment are each their own
  type, but all obey some corresponding `Fn` trait
  
          fn call<T: Fn(u64)>(f: T) {
              f(12);
          }

          fn main() {
              let y = 5;
              call(|x| println!("{}", x + y));
          }

* Be careful about generics *vs* trait objects

## The Problem With Closures (in a non-GC language)

* In the usual PL model, a closure combines a particular
  code body with an environment containing its captured values

* Need to keep closed-over memory alive as long as the
  closure

* To be useful, this needs to be longer than the scope in
  which the closure is created

* GC solution: who cares? Tracking lifetimes at runtime anyhow

* Rust solution: represent the environment as a value whose
  type implicitly identifies the particular code body

* Normal borrow checker rules then apply

## Closures As Code + Environment

* Think of a closure as some particular code plus an anonymous
  struct holding the values or refs captured by that code

* Each closure expression has a unique anonymous struct type;
  the code is associated with that type rather than stored in
  each concrete closure value
  
* Three kinds of implementation of the anonymous struct:

  * `&self`: Closure is of trait `Fn`
  
  * `&mut self`: Closure is of trait `FnMut`
  
  * `self`: Closure is of trait `FnOnce`
  
* Also, environment values themselves can be:

  * By reference (normal case)
  
  * Owned (`move` closure)
  
## Generics vs Trait Objects
  
* Each closure has a different size, so can't just handle
  them willy-nilly

* Each closure is of a different type, so have to be careful
  with generics

* Trait objects are often used with closures

* Erasing the concrete closure type requires making the code
  association explicit: a closure trait object is represented
  by a fat pointer

  * Pointer to the environment

  * Pointer to a vtable that identifies the implementation,
    including how to call the code

* [`examples/prog2.rs`][prog2]

## Final Notes

* Values implementing `Copy` confuse everybody

* This stuff is hard; get some experience with it right now

[prog]: https://github.com/pdx-cs-rust/examples/blob/main/prog.rs
[prog2]: https://github.com/pdx-cs-rust/examples/blob/main/prog2.rs
