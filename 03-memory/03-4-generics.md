## Rust Generics

* Parameters of kind "type"

* Most commonly single capital letter (can be camel-case)

* Can parameterize a datatype, function or impl
  ([`examples/top.rs`][top])

* Turbofish supplies explicit type parameters

          t.update::<u32>(&0);

## Type Inference

* As with lifetimes, usually don't have to write generics
  out: compiler can match things up

* Sometimes gets lost, though. Also, will not make choices

          // Fails to compile because type mismatch.
          let s: u64 = [1u32, 2, 3].iter().cloned().sum();

## Materialization

* Generics are implemented by monomorphization: a new
  implementation of the datatype or function is created for
  each concrete combination of type arguments that is used

* This defines an infinity of potential instantiations:

          fn id<T>(x: T) -> T {
              x
          }

          id(3u16)
          id('c')

* Inlining may happen and avoid some of these functions

* Still, this can cause an explosion of implementations

          fn tuple<T, U, V> (x: T, y: U, z: V) -> (T, U, V) {
              (x, y, z)
          }

  can have a lot of implementations floating around

## Phantom Types

* Sometimes want to have the compiler track the type of a
  thing just to avoid confusion, even though no value of that
  type is stored

* `PhantomData` makes the otherwise-unused type parameter part
  of the type without adding stored data

          use std::marker::PhantomData;

          struct Id<T> {
              number: u64,
              kind: PhantomData<T>,
          }

          impl<T> Id<T> {
              fn new(number: u64) -> Id<T> {
                  Id {
                      number,
                      kind: PhantomData,
                  }
              }
          }

          struct User;
          struct Project;

          fn find_user(_id: Id<User>) {
              // ...
          }

          fn main() {
              let project = Id::<Project>::new(7);
              // Does not compile: wrong kind of ID.
              // find_user(project);
          }

* `Id<User>` and `Id<Project>` are distinct types even though
  both store just a `u64`

[top]: https://github.com/pdx-cs-rust/examples/blob/main/top.rs
