## Rust Condition Handling

* No exceptions (except panics which should not be caught)
  
* Instead, three basic things a function can do when an
  error is encountered at runtime

    * Handle the error

    * Fail altogether via `panic!()`, various asserts

    * Return some kind of result indicating an error
      to the caller, who can then deal with it

## Error Handling: Panic

* `panic!` is that thing that should never happen in a
  production program

* Panic is a true exception: will be "unwound" by default,
  can be caught (!) but shouldn't be

* Why unwinding? Because drops do cleanup by default, so
  leave the world safe as possible before exiting

    * For example, flush buffer and close open file

## Error Handling: Option

* `Option` is Rust's alternative to special "sentinel"
  values, for example null pointers or negative numbers

* Enum defined more or less like this:

        enum Option<T> {
            Some(T),
            None,
        }
        use Option::*;

## Option Things

* Definitely read the API docs for `std::option::Option`
  carefully

* The `Option` type is sometimes used for returning certain
  kinds of "error"

    * There's a style question here. Best practice is to use
      `Option` for situations where nothing has really
      "failed" but no value is available

## Error Handling: Result

* `Result` is simple enum type with two value-carrying
  options: `Ok` and `Err`

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        use Result::*;

## Result Things

* Definitely read the API docs for `std::result::Result`
  carefully

* "Normal" way to deal with errors in standard library and
  your own code

* `Err` normally contains a value implementing
  `std::error::Error`; there are many kinds of these in the
  standard library

    * `std::io::Error` is particularly a thing

## Implicit Error Types

* In `std::io` we have

        pub type Result<T> = std::result::Result<T, std::io::Error>;

* Thus we can `use std::io::Result` and then say things like

        fn f() -> Result<u32>

* Just a convenience, not that useful; more useful would be (textbook)

        type GenericError =
            Box<dyn std::error::Error + Send + Sync + 'static>;
        type GenericResult<T> = std::result::Result<T, GenericError>;

## Error Printing

* Things that implement `std::error::Error` must implement
  `std::fmt::Display` and `std::fmt::Debug` also

* Thus, "{}" and "{:?}" will work as format specifiers for
  errors

* These, especially "{:?}", may expose private internals; 
  *do not* format to string and then parse

## Error Handling: Unwrap and Expect

* Methods of `Option` and `Result`. `.unwrap()` panics with
  a generic message on failure, `.expect("x")` panics with
  the message `"x"` on failure

* Panic on `None` or `Err`, otherwise return the contents of
  `Some` or `Ok`

        let n = Some(7).unwrap(); // n is now 7
        let n = None.unwrap(); // program panics

* Note `.unwrap_or()` and `unwrap_or_else()`. Useful for
  supplying a default value in a few situations

## An Example: Pop On Empty Stack

* Can use a `Vec` as a stack:

        let mut stack = Vec::new();
        let v = stack.pop();

* Could return random garbage. This is the C way

* Could return a "default value", but what? And ugh

* Could `panic!()`: pretty drastic. This is the Python way

## Other Possible Pop Results

* Could return a `Result` with a custom error indicating
  popping an empty stack. Right if intent is for `pop()` to
  be a "partial function"

* Could return an `Option` — what `Vec` does. This indicates
  that popping an empty stack is not to be regarded as an
  error, but you should handle it. Right if intent
  is for `pop()` to be a "total function"

## Handling Potential Errors: Pattern Matching

* Easiest to understand, but most verbose: just deal with
  the enum directly

        // Print a str backward.
        let mut s = "hello".to_string();
        while let Some(c) = s.pop() {
            print!("{}", c);
        }
        println!();

## Handling Potential Errors: ?

* Rust has the `?` operator. This takes an `Option` or
  `Result` and unwraps if good, early-returns from the
  calling function with the error otherwise

        fn is_negative(s: &str) -> Result<bool, std::num::ParseIntError> {
            let n: i64 = s.parse()?;
            Ok(n < 0)
        }

        fn main() {
            assert!(is_negative("-100").unwrap());
            assert!(is_negative("-x").is_err());
        }

## Main Can Return Result Or Option

* It is allowed to declare `main()` to return a `Result` or `Option`.
  If `main()` returns an `Err` you will get an error message
  formatted some debuggy way

        fn main() -> Result<(), std::num::ParseIntError> {
            println!("{}", is_negative("")?);
            Ok(())
        }

## Handling Potential Errors: Error Combinators

* It is common to handle errors in a "functional" style
  using methods provided by `Result` and `Option`. This is
  handy when you want to keep going after an error

        let scaled_min =
            (0..n).map(f).min().and_then(|s| s / m).or_else(|| g());

* This usage can be worth it, but can also be a bit
  tricky. It will take some practice

## Making Your Own Errors

* You can add your own error type

    * Manually

    * Via some specialty crate. There are a million of these,
      and it's hard to keep up-to-date on which are in
      fashion

    * As of this writing, `anyhow` and `thiserror`
