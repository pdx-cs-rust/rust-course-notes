## Background

Most operating systems these days have filesystems. Most of
those filesystems have tree-structured directories. Working
with directory trees is an essential part of filesystem
operation.

In most directory trees the tree itself owns the path
component names. However, this reduces opportunities for
sharing in an in-memory directory tree. Perhaps it would be
useful for a directory tree to *borrow* its component names?

## Assignment

In this assignment, you will write a library crate that
provides a directory tree implementation, and a bit of
operating system state to model the concept of "current
working directory".

The code skeleton for the assignment is at
<http://github.com/pdx-cs-rust/hw-dtree> and includes
full Rustdoc including some examples and doctest. Please do
not alter the skeleton as given. Simply clone it and then:

* Replace all the `todo!()` annotations with working code.

* Write sufficient additional unit tests to convince
  yourself that your implementation is correct.

## Hints

* As things stand, the paths produced by `DTree::paths()`
  and `OsState::paths()` always start and end with `/`.

* Some of the methods here are easily implemented in terms
  of others of the methods. In particular,
  `DTree::with_subdir()` and `DTree::with_subdir_mut()` are
  pretty powerful building blocks.

* It is fine to add private stuff to your library, as long
  as you don't break the public interface.

## Requirements

* Your library must build with current `stable` Rust.

* Your library implementations should contain adequate tests
  (implemented using `#[test]` unit-testing) and assertions
  (implemented using `assert!()` and related macros).

* Your code must be formatted according to the official Rust
  formatting style — use `cargo fmt` to reformat your code
  in-place.

* Your code must produce no compiler warnings, and `cargo
  clippy` must also produce no warnings. Please do not
  disable warnings except in the most unusual circumstances:
  fix them instead.

Please submit a ZIP archive containing:

* Your `Cargo.toml` and `Cargo.lock`.

* Your `src/lib.rs`.

* Any other source or other text files that are
  necessary/useful.

* A `README.md` file in Markdown format giving a writeup of
  what you did, how it went, and how you tested your work.

* Nothing else. Not your git repo. None of the funny Mac
  garbage files. Not your `target/` directory.
