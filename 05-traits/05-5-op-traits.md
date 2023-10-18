## Operator Overloading

* This is an "easy, boring" chapter

* There are traits that can be used to tell the Rust
  compiler to use standard operators for
  your standard type
  
* Compromise between Nickle-style ("operator overloading
  makes programs unreadable") and C++-style ("overload any
  operator for any reason anytime")

## What You Cannot (Must Not) Do

* Cannot change precedence or associativity of standard
  operators

* Overloaded operators are "expected" (not required, but
  really?) to obey basic arithmetic laws "as appropriate",
  for example

  * Associativity of `*`

        a * (b * c) == (a * b) * c

  * Transitivity of inequality

        a < b < c ⇒ a < c

* In general, it is good style to not be polymorphic with
  operators, e.g. `1.0 + 2` should maybe be rejected

## What You Can Do

* Pick any operator from the table in the chapter and
  specify its function on your own datatypes (`examples/matmul.rs`)
  
* The arithmetic operators consume their
  arguments. Sorry. Usually derive `Copy` for arithmetics
  
* Compound assignment operators are separate.

* Operators should be derivable, but currently aren't
  
* `Index` and `IndexMut` allow overloading `[]` in various
  contexts. `IndexMut` requires producing a reference to
  valid memory, which is borked for types that want to do an
  initial assignment
