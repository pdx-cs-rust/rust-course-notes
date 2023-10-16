## Rust Has A Module System

* Better than most, not as good as SML

* Name-based, but with inter-module typechecking

* Two kinds of thing
    * "Crates": self-contained package
    * "Modules": namespaces within a package

* <http://crates.io> is the big repository of published
  crates
  
## Library and Binary Crates

* A package can have crates for

    * Programs that can be installed
    * A library that can be published
    * Both

* By default, directory structure gives Cargo what it needs
    * `src/main.rs` is a "binary" root
    * `src/lib.rs` is a library crate root (ideally)
    * `src/mod.rs` is a module top-level for a library

* You can override all of this in `Cargo.toml`

## More Default Paths

* `src/bin/prog.rs` is a named binary `prog`
* `src/bin/prog/main.rs` is a named binary `prog` with modules
* `examples/` is a bin directory for library crates
* `tests/` contains extra tests
* `benches/` contains benchmarks [unstable-ish]
 
## Cargo Fetches Crates

* The `[dependencies]` section of `Cargo.toml` allows
  specifying other crates used by this crate

* This is by far the sanest way to do separate compilation

* Can specify
    * `crates.io` crate: `foo = "0.4"`
    * Github crate: `foo = { git = "http://github.com/something/foo.git" }`
    * Local crate: `foo = { path = "/local/src/foo" }`

* Then you can use `foo::` in your program, or use a `use`
  decl

## Better TOML For Non-Standard Dependencies

* Instead of

        [dependencies]
        foo = { git = "http://github.com/something/foo.git" }

* Try 

        [dependencies.foo]
        git = "http://github.com/something/foo.git"

* Easier to read, allows specifying version and stuff

## Namespaces

* Namespaces are `::` separated as in some other languages
    * `std::io::stdin()`
    * `use std::io::stdin;` then `stdin();`
    * `use std::io::{self, stdin};`, then `io::stdin()` or `stdin()`

* The `mod` directive at the start of a block introduces a
  new namespace in the current spot

* Most of this machinery is available in any scope
  
## Modules In Separate Files

* In addition to explicit `mod { ... }` declarations, can
  make a file `foo.rs` a module by saying `mod foo;` at the
  top of its parent module

* The filename is inferred from the module name, so be
  careful, esp `-` vs `_`

* You can also make a directory contain a module and its
  submodules by putting a `mod.rs` at the root of the
  directory

## Making Things pub

* By default, nothing in a module is visible outside the
  module/crate in which it is declared

* You can put `pub` in front of an item to make it
  externally visible

* For things like structs and enums, the visibility of the
  thing and its fields is controlled separately

* The compiler will whine at you about an unused thing
  unless it is `pub`, in which case it assumes you just know
  it will be used somewhere

## Semantic Versioning

* Three-part version numbers: *breaking*.*major*.*minor*

    * Rust convention also says 0.*breaking*.*minor*

* Supposedly improves stability

* `Cargo.toml` has version matching rules: usually want
  just the *breaking* part

* `Cargo.lock` preserves Cargo's versioning choices

## Docs

* Doc comments have an extra slash `///`
  * Inline doc comments are `//!`
  * You can also have C-style doc comments `/**` or `/*!`

* Use `cargo doc --document-private-items` to get docs for
  stuff not visible outside the crate

* Doc comments are treated as Markdown

## Tests

* Marking a function with `#[test]` makes it a test

* Tests fail by panicking (`#[should_panic]` tests fail by
  not panicking)

* Test panics usually from asserts: `assert!()`, `assert_eq!()`

* Marking an item `#[cfg(test)]` makes it included only
  when compiling for testing

* Examples in doc comments are treated as "doc tests" by
  default: must compile and not panic

## Rustup, Toolchains, etc

* Can install Rust through distro or `rustup`: strongly
  recommend `rustup`

* Maintains separate "toolchains" with binaries for
  `rustc` and other tools

* Toolchains track "releases" for `nightly`, `beta`, `stable`

* Maintains separate "targets" for `rustc` and `libraries`

## Nightly Toolchains

* Avoid if at all possible

* Always lock to a particular nightly

## An Example: woodpieces

* <http://gitlab.com/BartMassey/woodpieces>

* Fairly complicated module structure

* Binary with libraries
