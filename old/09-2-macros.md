## Macros

* Idea as old as computing; map from text to text using
  textual "functions"

* Famous macro systems: LISP, M4, TeX, CPP, Scheme

* Often a "preprocessor" like CPP: source-to-source

* Modern macro systems are a bit fancier

## Rust Macros

* Mappings from parsed source tokens to source tokens

* Two kinds:

  * "Declarative", which use rules and matching:
    e.g. `println!()`

  * "Procedural", which call Rust functions with token
    trees: e.g. `#[derive(Eq)]`

* Let's talk about declarative macros.

* Going to skip over procedural macros: they are *hard*

## Introducing A Macro

* `macro_rules!` itself looks / acts like a macro

* Argument is a sequence of rules

* Each rule has a LHS that is a token pattern to match, and a RHS
  that is tokens to rewrite using the match

* Both sides are lexed by the compiler: you can't use
  arbitrary text

* The input is parsed by the compiler: you can't use
  nonsense token streams

* `examples/debug-macro.rs`

## Rules Run In Order

* The macro rules match from top to bottom. The first
  matching rule is chosen

* A rule may suffer from type: the patterns match
  syntactically, but the pattern type is wrong. If this
  happens, compilation will fail right there

## Macro Bugs

* Double-expansion is dangerous, as with CPP. `examples/square-macro.rs`

* Macros are just tokenized, so weird errors in the macro
  rule bodies won't be caught at macro expansion time --
  they will be caught at code compile
  time.

## Macro Debugging

* `log_syntax!()` will print its arguments to the terminal
  *at compile time*

* `rustup run nightly rustc -Z unpretty=expanded` or the
  `cargo-expand` program can be used to show the
  preprocessed program as text

## Repetition and Condition

* Powerful, but easy to get wrong. `examples/debug-macro-rep.rs`

* Varargs is 70% of the reason for Rust macros

## Rules Can Be Recursive

* Note that our `debug!` example expands `eprintln!`. It can
  also expand itself, either directly or
  indirectly. `examples/macro-nargs.rs`

* Note that this expansion is *at compile time*: the source
  code can get huge and take a long time to generate and
  compile

* There is an expansion depth limit of 64 to prevent runaway
  macros from overrunning the compiler stack. The depth
  limit can be increased with `#![recursion_limit = "256"]`
  or something similar

* `#![feature(trace_macros)]` can be useful here for
  debugging expansions

## More Facilities

* Lots of compiler builtins, e.g. `line!()`. See the book
  for details

* Lots of "fragment types", e.g. `ident`, `ty`, `tt`

* A `tt` fragment is special: it matches any "token tree"
  the Rust compiler can build. This is either a list of
  stuff inside some kind of outer brackets, or it's a single
  token of arbitrary kind. `examples/macro-tt.rs`
  
## Scope Stuff

* Local variables and arguments created inside a macro
  "cannot escape": they are in a different namespace and
  thus "hygienic"
  
* This won't compile

        macro_rules! make_point {
            ($x:expr, $y:expr, $t:ty) =>
                { let x: $t = $x; let y: $t = $y; }
        }

        fn main() {
            make_point!(3, 2, u32);
            println!("{} {}", x, y);
        }

* Making macros visible to another crate requires
  `#[macro_export]` per-macro

* Macro import is controlled by normal module import rules

## Practical Examples

* <https://github.com/BartMassey/popcount/blob/master/popcount.rs>
* <https://github.com/pdx-cs-rust/debug-macros>

## A Book

* This is a bit dated, but otherwise great: [Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
