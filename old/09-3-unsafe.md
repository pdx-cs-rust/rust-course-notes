## The unsafe Keyword

* Turns off *some* of the compiler checks when
  applied to a block or function
  
    * Can call unsafe functions

    * Can dereference raw pointers

    * Can mutate global variables

    * Can use `extern` Foreign-Function Interface (FFI)
  
* That is *all* it gives you

## Consequence Of unsafe Abuse

* Undefined Behavior (UB) when a program

    * Reads uninitialized memory

    * Creates invalid primitive values

        1. Invalid (including null) references or Boxes
        2. `bool` values that are not either 0 or 1
        3. `enum` values with bogus discriminants
        4. `char` values that are not Unicode code points
        5. `str` values that are not UTF-8

    * Violates the lifetime or sharing rules with references

    * Dereferences a bogus pointer — points somewhere
      invalid or *misaligned*

    * Has a data race

    * Unwinds across FFI calls (obscure)

    * Violates standard library (or other) function
      contracts
  
* UB may do nothing, may cause trouble, or may only cause
  trouble when the optimizer gets clever
  
## Unsafe Function "Contracts"

* Code should be written so that *when properly used* UB
  cannot occur
  
* To inform the user of the code, a natural-language
  "contract" should be written specifying how an
  `unsafe` function can be safely called
  
* The contract should appear as a rustdoc comment before the
  function

* `unsafe` blocks inside a function must be used in such a
  way that no valid call to the function can cause UB

* e.g. [`from_utf8_unchecked()`](https://doc.rust-lang.org/std/str/fn.from_utf8_unchecked.html)

## Unsafe Block "Contracts"

* Every `unsafe` block should have a comment above that
  describes

    * *Need* (why should this block be `unsafe`?)

    * *Safety* (why is this block not UB?)

## Raw Pointers

* Declared for some type `T` via `*const T` or `*mut T`

* Are essentially just C pointers

* Not `unsafe` to do anything *other than* dereference them

* Whole bunch of functions in `std::ptr` and `std::mem` for
  dealing with them
  
* `examples/nullptr.rs`

* You can drop memory out from under a raw pointer. Watch
  out for ownership problems
  
* Can cast back to a ref with `as`, but be careful: default
  lifetime is `'static`, while the referenced data probably
  isn't at all

## Examples

* `RefWithFlag` from the book

  <https://github.com/ProgrammingRust/examples/blob/master/ref-with-flag/src/lib.rs>
  
* `SIVec`

  <http://github.com/BartMassey/sivec>
