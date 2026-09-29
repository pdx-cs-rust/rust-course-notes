## How To Be unsafe

* In front of a function

        // must call from an unsafe place
        unsafe fn foo() {
            // unsafe stuff here
        }

* In Rust 2024, unsafe operations inside an `unsafe fn`
  should still appear in an explicit `unsafe` block

* In front of a block

        unsafe {
            // unsafe stuff here
        }

## Unsafe Obligations

* `unsafe` is a poorly chosen keyword: it does not mean that
  the code is necessarily dangerous or turn off normal type,
  borrow, or lifetime checking

* As the joke goes, a better keyword might have been
  `hold_my_beer`: the programmer asks the compiler to trust
  that extra safety obligations have been upheld

* An `unsafe` block permits a small, specific set of
  operations that the compiler cannot verify
  
    * Can call unsafe functions

    * Can dereference raw pointers

    * Can read or mutate mutable static variables

    * Can read fields of unions

* `unsafe` also marks declarations whose correctness depends
  on obligations the compiler cannot check

    * Unsafe functions, traits, and trait implementations

    * In Rust 2024, unsafe `extern` blocks used to declare
      Foreign-Function Interface (FFI) items

## Consequence Of unsafe Abuse

* Undefined Behavior (UB) when a program

    * Reads uninitialized memory

    * Creates invalid primitive values

        1. Invalid (including null) references or Boxes
        2. `bool` values that are not either 0 or 1
        3. `enum` values with bogus discriminants
        4. `char` values that are not Unicode scalar values
        5. `str` values that are not UTF-8

    * Violates the lifetime or sharing rules with references

    * Dereferences a bogus pointer — points somewhere
      invalid or *misaligned*

    * Has a data race

    * Unwinds across an ABI boundary that forbids unwinding

    * Violates standard library (or other) function
      contracts
  
* UB may do nothing, may cause trouble, or may only cause
  trouble when the optimizer gets clever
  
## Sidebar: Alignment

* Some types cannot start at just any byte address. A value's
  address must be a multiple of its type's alignment

* The exact requirement depends on the type and target. Ask
  Rust with `std::mem::align_of::<T>()`

* The compiler normally handles alignment. It becomes the
  programmer's responsibility when dereferencing raw pointers

* Why?

  * Some architectures don't even have a hardware
    instruction to access a misaligned word. So the compiler
    would have to do two aligned accesses plus some shifting
    and masking. The aligned accesses may not even be
    possible at the beginning or end of memory region

  * Architectures that allow unaligned access may impose a
    performance penalty, sometimes severe

## Unsafe "Contracts"

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

* Creating, copying and comparing raw pointers is safe;
  dereferencing and many pointer methods require `unsafe`

* Whole bunch of functions in `std::ptr` and `std::mem` for
  dealing with them
  
* [`examples/nullptr.rs`][nullptr]

* You can drop memory out from under a raw pointer. Watch
  out for ownership problems
  
* Creating a reference from a raw pointer requires `unsafe`;
  its lifetime is inferred from context. The programmer is
  responsible for ensuring that this lifetime does not
  outlive the referenced data

## Examples

* `RefWithFlag` from the book

  <https://github.com/ProgrammingRust/examples/blob/master/ref-with-flag/src/lib.rs>
  
* `SIVec`

  <https://github.com/BartMassey/sivec>

[nullptr]: https://github.com/pdx-cs-rust/examples/blob/main/nullptr.rs
