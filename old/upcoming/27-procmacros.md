## Procedural Macros

* Purest way to macro: take a token stream, process it with
  a Rust function, output a new token stream

* Can *really* generate a lot of code: slow to compile, big
  binaries

* Super-hard to write, test and debug

## What Can Proc Macro?

* Any item that can be decorated with a decorator `#[...]`,
  really

* Typically used with `#[derive(...)]` on structs / enums
  but not limited to that

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

* A proc macro crate declares proc macros using
  `#[proc_macro_derive(...)]`, for maximum irony

* This decorates a parsing function

        #[proc_macro_derive(...)]
        pub fn derive(input: TokenStream) -> TokenStream {
            ...
        }

* The `derive()` function must not fail, but can insert
  macro errors into the output stream

* Inside `derive()`, use `syn` to parse pieces of input
  stream and `quote::quote!` to make templates

* <http://github.com/dtolnay/proc-macro-workshop>

* <http://github.com/BartMassey/parsere>
