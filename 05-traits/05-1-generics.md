## Generics In impl Blocks

* You have some struct or something with a generic in its
  definition
  
        struct TwoOf<T>(T, T);
        
* Now you decide to `impl` some methods for your struct
  
        impl TwoOf<T> {
            fn as_array(&self) -> [&T; 2] {
                [self.0, self.1]
            }
        }
        
* The compiler refuses. Solution:

        impl<T> TwoOf<T> {
            fn as_array(&self) -> [&T; 2] {
                [self.0, self.1]
            }
        }

* This is a thing with lifetimes also

## Const Generic

* New Rust feature that allows compile-time constants (in a
  limited way, sometimes)
  
        fn share<T, const N: usize>(x: &T) -> [&T; N] {
            [x; N]
        }

* Rarely used in practice, but see for example `array::from_fn()`

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
