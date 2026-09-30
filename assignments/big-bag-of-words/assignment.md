# HW: Big Bag Of Words

In this exercise, we will focus on maintaining a "Big Bag Of
Words" (BBOW) from a source text.

## Background

A BBOW contains the set of words present in a document, together
with the occurrence count of each word.

You will be processing the words in a collection of documents
stored as strings in memory. You could make an owned list of all
the words you find, but that would increase storage use and
involve expensive string copying. Instead, you cook up a map that
stores each word as a slice into the original text.

Sadly, you want to ignore case in words. This means that you
might not be able to just store a slice. Fortunately,
`std::borrow::Cow` is there for you.

## Assignment

Clone the library template at
<https://github.com/pdx-cs-rust/hw-bbow> and fill in the
`todo!()` items you find there. Then add tests as needed to make
sure your code is working as intended.

## Hints

- You may struggle a bit with reference types and lifetimes in
  this assignment. That's normal.

- You will need to figure out `std::borrow::Cow`. The easiest way
  to do this is to read the comments in *Programming Rust*
  carefully, then the documentation in the Rust Standard Library,
  then write some test code of your own until you're comfortable
  with how it works. Trying to skip this step may spell doom.

  Note that `Cow::from()` can be used to construct a `Cow` from
  either a borrowed or an owned type, and will "do the right
  thing".

- Things I used in my implementation (in no particular order):

  - `s.chars()` is an iterator that returns each character in a
    string slice `s` in turn.

  - `s.split_whitespace()` is an iterator that returns each slice
    of non-whitespace characters in a string slice `s` in turn.

  - `c.is_alphabetic()` returns true iff the character `c` is
    alphabetic (a "letter").

  - `c.is_uppercase()` returns true iff the character `c` is
    uppercase (an "uppercase letter").

  - `s.trim_matches(|c: char| ...)` is a bit tricky. This method
    takes a boolean function ("closure" / "lambda") that should
    return `true` iff the given character should be trimmed. The
    method returns a subslice of the string slice `s` that has
    matching characters trimmed off the start and end of the
    string.

- If you are feeling functional-programmy, check out the `map()`
  and `filter()` operations on iterators, which can be pretty
  useful.

## Requirements

Your library crate implementation should contain adequate tests
(implemented using `#[test]` unit-testing) and assertions
(implemented using `assert!()` and related macros). Your code
should be formatted according to the official Rust formatting
style — use `cargo fmt` to reformat your code in-place. Your
code should produce no compiler warnings, and `cargo clippy`
should also produce no warnings. Please do not disable warnings
except in the most unusual circumstances: fix them instead.

## Submission

Please submit a ZIP archive containing:

- Your `Cargo.toml` and `Cargo.lock`.

- Your `src/lib.rs`.

- Any other source or other text files that are necessary/useful.

- The `README.md` file in Markdown format described above.

- *Nothing else.* Not your git repo. None of the funny Mac
  garbage files. Not your `target/` directory.

Name your ZIP archive `bbow.zip` (exactly that, in lowercase).
\[It's fine if Canvas renames the file on resubmission: that's
not on you.\]
