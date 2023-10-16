## Refs, Ownership, Lifetimes

* Can take a reference to an owned thing

          let y = 5;
          let ry = &y;

* Now you have an obligation: `ry` must not outlive `y`

## The C Way: Dangling Pointers

* C/C++ will happily let you write things like

        int y = 5;
        int *ry = &y;
        return ry;

* The returned pointer is now pointing into whereever on the
  stack the now-lost `y` was

## The Rust Way: Static "Borrow Checking"

* Rust compiler tracks lifetimes for you, so that

          let y = 5;
          let ry = &y;
          return ry;

  will give a compile-time error

## Lifetimes

* Rust tracks ownership via explicit or implicit lifetime
  specifiers
  
* Compiler's "borrow checker" tracks lifetime and ownership
  of values, throws a static error when can't work

* Usually implicit, but can be made explicit when needed /
  wanted
  
        struct MyStruct<'a> {
            my_field: &'a u64,
        }

## Mutable Refs

* Refs come in two varieties: "mutable" and "immutable"
  (really "exclusive" and "shared")

* At any given point a storage location is in one of three
  states

  * No outstanding refs: owner can access and change
  
  * Outstanding shared refs: all borrowers can access

  * One outstanding exclusive ref: *the* borrower can 
    access and change

## Mutable Ref

* Can only take mutable ref to mutable thing

          let mut y = 5;
          let ry = &mut y;
          println!("{}", ry);
          y = 7;

* Once you take a mutable ref, you've essentially "borrowed"
  the value referred to
  
  * Must not go out of scope
  * Cannot take any more references while it is live
  * Owner can't do anything with the value while it is live
    (read it, change it, move it, drop it)

## Immutable Ref

* An immutable ref is essentially "shared". You can take
  lots of them if you want

          let mut y = 5;
          let ry1 = &y;
          let ry2 = &y;
          println!("{}", *ry1, *ry2);

* You are still restricted for safety: owner cannot drop or
  move value (nor mutate, duh) while refs are live

## No Null References

* There's no such thing as a "null reference": no way to
  produce one, no need to guard against them

  * If you need a "nullable" value, use the `Option` type

          let y = 10;
          let mut ory = Some(&y);
          ory = None;

  * This is true for refs or anything else

## Ref Misc

* You can get a reference to an anonymous variable
  implicitly defined by an expression

          let ry = &10;

* A lot of automatic derefing happens

    * With the "." structure / enum operator
    * With some other operators

## Explicit Lifetimes

* The machinery that the compiler uses to check lifetimes is
  by default "under the hood": does the checking for you
  without intervention

* Sometimes, though, you need (or want) to get explicit
  access to that machinery to allow a program to compile
  that is safe but won't by default

## Lifetime Syntax
  
* "Named lifetimes" start with a tick, e.g. `'a` ("tick-a")

* In many contexts, explicit lifetime names can be declared
  
          fn f<'a, 'b>(x: &u64, y: &u64) -> &u64

* These names can then be used to describe lifetime
  constraints for referenced data
  
          fn f<'a, 'b>(x: &'a u64, y: &'b u64) -> &'a u64
  
## Simple Example

    fn fst<'a, 'b: 'a>(x: &'a u64, y: &'b u64) -> &'b u64 {
        &x
    }

    fn main() {
        let x = 7;
        {
            let y = 2;
            println!("{}", *fst(&x, &y));
        }
        println!("{}", *fst(&x, &2));
    }

## Book Example

    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s {
                s = r;
            }
        }
        s
    }

    fn main() {
        let s;
        {
            let v = [2,1,3];
            s = smallest(&v);
        }
        println!("{}", *s)
    }

