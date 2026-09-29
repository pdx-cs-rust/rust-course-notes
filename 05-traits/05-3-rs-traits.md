## Trait Garbage Bag

* There are a bunch of traits tied into Rust's internals or
  standard library with no real organizing principle
  
## Clone

* The `Clone` trait provides the `clone()` and
  `clone_from()` functions
  
* `clone_from()` is a good idea but little-used

* `Clone` explicitly duplicates a value. The result should
  behave like a duplicate rather than doing unrelated work

* Cloning recursively duplicates ordinary data, but may stop
  at reference-like types such as references, `Rc` and `Arc`

* `Clone` is usually derived, but occasionally not

* `#[derive(Clone)]` clones each field and may add generic
  bounds

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

        #[derive(Clone)]
        struct MyStruct(u32, f64);

        impl Copy for MyStruct {}


* `Copy` requires `Clone` as a supertrait; implement or
  derive both

* Use `Copy` sparingly:

  * Encourages less careful use of data
  * Expensive
  * Semantics sometimes surprising

### Drop

* You can implement the `Drop` trait to get control of a
  value right before it is freed

* This is used for e.g. closing files, flushing data, etc

* A type implementing `Copy` cannot also implement `Drop`,
  because the semantics are too confusing

### Sized

* `Sized` is a marker trait that says that the compiler
  knows "the size" of values of the type
  
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
