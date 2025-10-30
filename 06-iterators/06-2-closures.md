## First-Class Functions

* "First-class functions" are a thing. A thing is
  "first-class" if all the sensible operations of the
  language do indeed apply to it
    
* `examples/prog.rs`
  
## Rust Has First Class Functions

* They have their own types

* They can be passed as parameters, returned as results,
  stored in arrays, used as struct and enum fields, etc

* They can be created in any block scope

* References are available (indeed, a function essentially
  *is* a static reference)

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
  
## Pure Closures

* "Pure" closures are just functions

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

* Need to keep closed-over memory alive as long as the
  closure

* To be useful, this needs to be longer than the scope in
  which the closure is created

* GC solution: who cares? Tracking lifetimes at runtime anyhow

* Rust solution: Make a closure pointer a fat pointer

    * Pointer to the code
    * Pointer to a struct that closes over the environment

* Normal borrow checker rules then apply

## Closures As Environment Structs

* Think of a closure as a code pointer and some anonymous
  struct holding the captured vars or refs to them and its
  implementation
  
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

* `examples/prog2.rs`

## Final Notes

* Values implementing `Copy` confuse everybody

* This stuff is hard; get some experience with it right now
