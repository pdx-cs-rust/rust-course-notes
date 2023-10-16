## About This Course

* We will learn to program in Rust

  * Work through a textbook
  * Write weekly(ish) homeworks
  * Everyone: Write a 1000-line project
  * Grads: In addition, write a 3-page research paper

* Goal: leave as competent Rust programmer
  (course objectives in syllabus)

## About Rust

* Rust is a "systems programming language" intended as a
  replacement for C/C++

* Rust offers a unique type system that allows static
  automatic memory management: mallocs and frees can be
  inserted by the program as needed

    * The automatic memory management constraints make
      writing Rust programs harder than one might expect

* Rust provides safety guarantees against common types of
  error. It also provides an "unsafe" mode to escape some of
  these guarantees when needed

## More About Rust

* Rust programs tend to have similar performance to C
  programs, similar readability to C++ programs, be harder
  to write than either, and to be reliable and
  safe. (Compile times are not great right now, tho)

* Concurrency and functional programming features are
  first-class in Rust

* Nice module and separate compilation system with package
  manager

## Installing Rust

* Install and run [`rustup`](https://rustup.rs) tool, using
  the instructions on the linked page.

* Make sure your PATH environment variable is set up

## Hello World In Rust

* Start a new Cargo project

        cargo new --bin hello
        cd hello

* Edit provided program (not actually needed)

* Build and run program

        cargo build
        cargo run

## Rust Decency

* Use `cargo fmt` to format your program according to Rust
  standards. This is what everyone in the community expects:
  *do not* fight with it

* Use `cargo clippy` to find ways to improve your
  program. At this stage assume Clippy is right (or ask me):
  *do not* fight with it unnecessarily

## Rust Testing

* Built-in (OK) support for unit tests integrated with
  language and Cargo. Use `cargo test` to run tests

* Can express positive and negative tests, conditional tests

## Rust Program Documentation

* Integrated doc comments as expected

* Neat feature: doc tests

* Integrated with Cargo

## Rust Language Documentation

* "The Rust Book" online

* The course textbook

* Other books: see LMS for a list

* The API docs
