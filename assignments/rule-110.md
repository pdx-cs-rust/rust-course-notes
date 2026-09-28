# HW: Rule 110

## Background

In this exercise you will create an ASCII rendering of the [Rule
110](https://en.wikipedia.org/wiki/Rule_110) [Elementary Cellular
Automaton](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)
(CA). Not as scary as it sounds, and a nice simple Rust thing to
play with.

A CA starts with a random row of bits. We will represent 1 bits
with `*` and 0 bits with `.`. Our starting row will specifically
be this 8-bit position for now.

    *.*..*..

To produce the next row, take bits three at a time, "wrapping
around" if a boundary is hit (row 7 is next to row 0 and so
forth). A group of three bits in row *n* will determine the
center bit position in row *n + 1* according to Rule 110:

    111 → 0
    110 → 1
    101 → 1
    100 → 0
    011 → 1
    010 → 1
    001 → 1
    000 → 0

So for our start, the first two rows will be

    *.*..*..
    ***.**.*

(Notice the wraparound at the beginning and end.)

## The Program

Write a program that prints ten rows starting with the two given.

## Hints

- You could use either `u8` or `[bool; 8]` to represent a row.
  The second thing is easier, so I'll assume that.

- If you print out as you go, you only need to keep track of two
  rows: the current row and the next row.

- Arrays in Rust are first-class: you can copy-assign them, pass
  them as function arguments, etc.

- You should probably write a function `rule110()` with a
  signature something like

  ```rust
  fn rule110(bits: [bool; 3]) -> bool {
      todo!()
  }
  ```

  in the array case, or

  ```rust
  fn rule110(bits: u8) -> u8 {
      todo!()
  }
  ```

  in the bit case. The `match` statement is your friend here.

  You may want other functions as well.

- You can't index a string in Rust, because UTF-8 encoded Unicode
  code points are variable-width. `s.chars()` is an iterator over
  the characters of a string.

- Do *not* be afraid to ask for help. This is the hardest
  assignment, probably, because things are new.

## Challenges

*(Challenges are ***not*** required, but give a chance to do
something extra. Turn them in and write them up in the README.)*

- Make your program take a command-line argument for the starting
  row. You may limit the length of the argument. If no argument
  is given, use the default for the program.

  This code

  ```rust
  let arg = std::env::args().nth(1);
  ```

  will assign `arg` either `Some(String)` or `None` depending on
  whether an argument is given.

- Time your program. See how fast you can make it go. Compare
  performance with a C or C++ implementation. You will probably
  have to modify the program to print *a lot* more rows: maybe
  some number given by a command-line argument.

## Requirements

- You must create a `README.md`. This file will have

  - Your authorship information.
  - A description of the program: what it does.
  - How to build and run the program.
  - Comments on any issues you encountered,
  - Anything else a reader might find useful.

- Your crate must be adequately documented with Rustdoc comments
  for each top-level datatype, function and method.

- Your crate must build with current `stable` Rust.

- Your crates should contain adequate tests (implemented using
  `#[test]` unit-testing) and assertions (implemented using
  `assert!()` and related macros) for you to be comfortable that
  everything is working correctly.

  There is no minimum amount of testing required, but good tests
  can help you turn in a working program.

- Your code must be formatted according to the official Rust
  formatting style --- use `cargo fmt` to reformat your code
  in-place.

- Your code must produce no warnings.

  - The compiler must produce no warnings.

  - `cargo clippy` must produce no warnings.

  - Do not disable warnings except in the most unusual
    circumstances: fix them instead.

## Submission

Please submit a ZIP archive containing:

- Your `Cargo.toml` and `Cargo.lock`.

- Your `src/main.rs`.

- Any other source or other text files that are necessary/useful.

- The `README.md` file in Markdown format described above.

- *Nothing else.* Not your git repo. None of the funny Mac
  garbage files. Not your `target/` directory.

Name your ZIP archive `rule-110.zip` (exactly that, in
lowercase).
