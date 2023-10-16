## Writing Programs

* Programs look a lot like any imperative language

* Can't have mutable global variables (kind of)

* Variable definition is done with let statement, needn't
  provide types (static type inference)

* Types must agree completely, conversions are explicit with `as`
  or type conversion operators

## Writing Programs (cont)

* Blocks must be bracketed. (Brackets and parens are kind of
  interchangeable)

* Semicolon is statement separator; optional null statements
  and expressions (comma)

* Last expression in block is value of that block

* `if` statement is ternary expression (`?:`)

* Fairly normal-looking `while` and `for`, also
  unconditional `loop`

* `match` is `switch` equivalent, but more powerful
  because pattern matching

## Rust Datatypes

* Emphasis on safe, efficient, simple

* Type system borrows heavily from all the other
  parametric/template type systems out there
  
* "Borrow checker" tangled in with the type system
  statically checks "lifetimes"
  
## Simple Datatypes

* Unit type: `()` with value `()`

* Boolean type: `bool` with values `true`, `false`

* Integer types:
    * `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`
    * `isize`, `usize`

* Floating types: `f32`, `f64`

* Character type: `char`
    * 32-bit Unicode code point
    * Conversion-to implies checking: `from_u32` vs `from_u32_unchecked`
    * Strings are UTF-8, so not `char`s

## Simple Datatype Notes

* These types can be "copied" implicitly

* Numeric literals can have type suffix (*e.g.* 0u64) and
  a base prefix (*e.g.* 0b0110)

* You can have `_` inside numbers as a separator: it is ignored

## Tuples

* Fixed, immutable number of values of diverse type

          (u64, f64, char)

  with obvious value representation

          let t = (12, 1.2, 'x');

## Commas

* Like other comma-separated things in Rust, tuple values can
  have a trailing comma `(12, 1.2, 'x',)`. This is useful
  for separate lines

          (
              12,
              1.2,
              'x',
          )

  and for making 1-tuples

          ("hello",)

## Tuples Are Handy

* Great for multiple return values, points, and the like

* "Copyable" if elements are

* Pattern-matching syntax

* Accessible by dot-index

          let x = ('a', 'b');
          println!("{}", x.1); // prints 'b'
