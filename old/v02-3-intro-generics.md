## Generic Types

* Like C++, Java, Haskell, etc, Rust has "type variables"

* Can write things like this

        struct Pair<T> {
            x: T,
            y: T,
        }

* And then things like this

        fn x<T>(pair: Pair<T>) -> T {
            pair.x
        }

        let pair = Pair { x: 0u32, y: 1 };
        let px: u32 = x(pair);

## Deriving and Generics

* Using `#[derive()]` for a struct with generics derives
  *iff* the underlying types are generic

* So `Newtype<T>` is `Copy` iff `T` is `Copy`

        #[derive(Clone, Copy)]
        struct Newtype<T>(T);

## Traits and Generics

* We haven't talked much about traits, but yeah `Copy` etc

* Can require a generic to "obey" a trait

        struct Value<T>(T);

        impl<T> Value<T> {
            fn display(&self) where T: Display {
                println!("{}", self.0);
            }
        }
