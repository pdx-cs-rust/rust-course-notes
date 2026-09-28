## Background

A commonly desired operation in various algorithms is to
calculate an exponential modulo some (usually prime) number

    modexp(x, y, m) = x**y (mod m)

(where `**` is the exponentiation operator, e.g. `2**4 =
16`).

It turns out that there is a fast way to compute `modexp()`
without overflow. Here is some pseudocode:

>
> *modexp*(*x*,&nbsp;*y*,&nbsp;*m*):<br>
> &nbsp;&nbsp;&nbsp;&nbsp;**if**&nbsp;*x*&nbsp;=&nbsp;0<br>
> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**return**&nbsp;0<br>
> &nbsp;&nbsp;&nbsp;&nbsp;**if**&nbsp;*y*&nbsp;=&nbsp;0<br>
> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**return**&nbsp;1<br>
> &nbsp;&nbsp;&nbsp;&nbsp;*z*&nbsp;&#8592;&nbsp;*modexp*(*x*,&nbsp;*y*&nbsp;**div**&nbsp;2,&nbsp;*m*)<br>
> &nbsp;&nbsp;&nbsp;&nbsp;*z*&nbsp;&#8592;&nbsp;(*z*&nbsp;&#8901;&nbsp;*z*)&nbsp;**mod**&nbsp;*m*<br>
> &nbsp;&nbsp;&nbsp;&nbsp;**if**&nbsp;*y*&nbsp;is&nbsp;odd<br>
> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*z*&nbsp;&#8592;&nbsp;((*z*&nbsp;**mod**&nbsp;*m*)&nbsp;&#8901;&nbsp;*x*)&nbsp;**mod**&nbsp;*m*<br>
> &nbsp;&nbsp;&nbsp;&nbsp;**return**&nbsp;*z*

## Assignment

Implement a command-line calculator `modexp` as a Rust
"binary crate". Usage is the obvious: to compute
`modexp(2,20,17)` invoke your calculator with

    cargo run 2 20 17

Your program should print `16` on standard output in this case.

Your program should accept only non-negative `x` and `y` and
positive `m` with at least one of `x` and `y` positive: all
inputs should be less than `2**32`. If the program is run
with one or more invalid inputs it should print a usage
message on `stderr` and stop with exit status 1.

## Hints

* The command-line `gcd` example of Chapter 2 of the course
  text may be useful as a model.

* This function should be adequate for printing an error
  message and exiting the program.

        /// Print a usage error message and exit.
        fn error() -> ! {
            eprintln!("modexp: usage: modexp <x> <y> <m>");
            std::process::exit(1);
        }

* You will want to use the `parse()` method on string types
  to get numbers. Note that `parse()` returns a `Result`
  that you will need to deal with. Perhaps the easiest way is

        /// Parse the given string as a `u64`.
        fn parsenum(s: &str) -> u64 {
            let n: u32 = s.parse().unwrap_or_else(|_| error());
            u64::from(n)
        }

  This will return a value on success, and call the
  `error()` function defined above on error (throwing away
  the error information).

  Note that `parsenum()` expects an `&str`, not a
  `String`. You can get an `&str` from a `String` by using
  the `&` operator

        let s: String = "17".to_string();
        let n: u64 = parsenum(&s);

  Rust strings are kind of messy: we will talk about them soon.

* The type `u64` is probably what you want for working with
  numbers in this assignment. The expression
  `u64::from(u32::max_value())` gives the maximum possible
  32-bit number as a `u64`, if that is helpful.

* Here is an example of a Rust test:

        #[test]
        fn test_trivia() {
            assert!(1 != 0);
        }

  You can run your tests with `cargo test`. A test passes if
  none of its assertions fail.

* `assert!()` and related macros can be used outside tests
  as well. They are really useful tools for checking that
  your program is doing the right thing. Put in what you like
  and leave it in when you submit.

* Python provides arbitrary precision arithmetic, so you can
  use it to construct tests.

        >>> print(2**20 % 17)
        16

  The thing you are building should be faster, though.

## Requirements

Your `modexp` implementation should contain adequate tests
implemented using `#[test]` unit-testing.  Your code must be
formatted according to the official Rust formatting style —
use `cargo fmt` to reformat your code in-place. Your code
should produce no compiler warnings, and `cargo clippy`
should also produce no warnings.

You must have Rustdoc at the start of your program and at
the start of each function, for example

    //! Command-line modular exponentation tool
    //!
    //! My Name 2021

    /// Print a usage error message and exit.
    fn error() -> ! {

and so forth.

I suggest the `name` of the crate in `Cargo.toml` be
`modexp`, but this is not required. The easiest way to do
this is to use `cargo new` or `cargo init` to create your
project in a directory named `modexp` — `cargo` adopts the
crate name from the directory name.

Please submit a ZIP archive containing your source files
only: no `target/` directory or `.git` directory or MacOS
garbage files or whatever.

Your submission should contain a `README.md` file in
[Markdown](https://guides.github.com/features/mastering-markdown/)
format giving your name, the name of the project, a writeup
of what you did, how it went, and how you tested your
work.

tl;dr: the submitted ZIP should contain

    ./README.md
    ./Cargo.toml
    ./src/main.rs

and nothing else.
