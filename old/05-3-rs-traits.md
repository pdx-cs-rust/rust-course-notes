## Trait Garbage Bag

* There are a bunch of traits tied into Rust's internals or
  standard library with no real organizing principle
  
## Clone

* The `Clone` trait provides the `clone()` and
  `clone_into()` functions
  
* `clone_into()` is a good idea but little-used

* A `Clone` implementation should do a "deep copy"

* `Clone` is usually derived, but occasionally not

        impl Clone for MyType {
            fn clone(&self) -> Self {
                todo!()
            }
        }

        impl Copy for MyType {}

## Marker Traits

* "Marker traits" are a communication channel between
  compiled code and the compiler

* You can use a marker trait like any other trait

### Copy

* `Copy` is a marker trait that you implement when
  you want your values to be automatically copied
  by the compiler. It has no methods
  
* `Copy` provides `Clone` for free as a subtrait

* Use `Copy` sparingly:

  * Makes the implementation be careful
  * Expensive
  * Semantics sometimes surprising

### Drop

* You can implement the `Drop` trait to get control of a
  value right before it is freed

* This is used for e.g. closing files, flushing data, etc

* A type implementing `Copy` cannot also implement `Drop`,
  because the semantics are too confusing

## Sized

* `Sized` is a marker trait that says that the compiler
  knows the size of values of the type
  
* You cannot implement `Sized` yourself

* By default, generic types implicitly require instantiation
  with something `Sized`
  
* You can turn this off with `?Sized` ("questionably
  sized") in situations where you don't want it

## Default

* Trait to provide a "default value" for your type

* Perhaps a bad idea: what does "default value" even mean?

* … But often convenient

* Book provides a sketchy use case

* These days, Clippy tries to insist
