## Rust Generics

* Parameters of kind "type"

* Most commonly single capital letter (can be camel-case)

* Can parameterize a datatype, function or impl (`examples/top.rs`)

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
  each place the the thing is used at a type

* This defines an infinity of potential instantations:

          fn id<T>(x: T) -> T {
              x
          }

          id(3u16)
          id('c')

* Inlining will happen and avoid some of these functions

* Still, this can cause an explosion of implementations

          fn tuple<T, U, V> (x: T, y: U, z: V) -> (T, U, V) {
              (x, y, z)
          }

  can have a lot of implementations floating around

## Phantom Types

* Sometimes want to have the compiler track the type of a
  thing just to avoid confusion

          struct Hash<T> {
              h: u128,
          }

          let h1: Hash<[u8; 3]> = Hash::new([0u8, 1, 2]);
          let h2: Hash<&str> = Hash::new("hello");
          if h1 == h2 {
              ...
          }

    * This is the hash of some specific type: even if the
      `h` values match, if the types they were derived from do not
      that should fail

    * Ask the compiler to track this

              use std::marker::PhantomData;

              struct Hash<T> {
                  h: u128,
                  p: PhantomData<T>,
              }

    * Now you can get typed hashes

              impl<T> Hash<T> {
                  fn hash(val: T) -> Hash<T> {
                      let h = unsafe { hasher(&T) };
                      Hash { h, p: PhantomData }
                  }
              }

    * `examples/phantom.rs` for the details
