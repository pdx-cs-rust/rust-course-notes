## Procedural Macros

* Purest way to macro: take a token stream, process it with
  a Rust function, output a new token stream

* Can *really* generate a lot of code: slow to compile, big
  binaries

* Super-hard to write, test and debug

## What Can Proc Macro?

* Three forms: function-like `foo!(...)`, attribute `#[foo]`
  and derive `#[derive(Foo)]` macros

## Proc Macro Crates

* The `proc_macro` and `proc_macro2` crates provide compiler
  interface

* The `syn` crate and related crates handle a *ton* of token
  stream parsing, making nice Rust token trees to deal with

* The `quote` crate allows a sort of templating for macro
  output, vaguely like decl macro rule bodies

## Setting Up

* Because compiler, proc macro implementations must be in
  their own crate: no mixing with other code

* So usually have a lib crate `foo` that provides glue and
  depends on another crate `foo-derive` that has the proc
  macros

* Cargo workspaces are good for this: both crates live in
  same repo, but are semi-independent

## Interface

* `#[proc_macro_derive(...)]` on a function declares a
  derive macro, for maximum irony

* This decorates a parsing function

        #[proc_macro_derive(...)]
        pub fn derive(input: TokenStream) -> TokenStream {
            ...
        }

* The `derive()` function can report errors by panicking or
  emitting `compile_error!`

* Inside `derive()`, use `syn` to parse pieces of input
  stream and `quote::quote!` to make templates

* <https://github.com/dtolnay/proc-macro-workshop>

* <https://github.com/BartMassey/parsere>
