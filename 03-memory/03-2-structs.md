## Rust struct

* Seen a bunch of these already

* Basic idea is the same as structures in any language

* Rust structs are fancy, and an essential building block

* Most of the standard API is built out of struct, enum, trait

## Basics

* Basic decl syntax is straightforward

          struct Point {
              x: i64,
              y: i64,
          }

* There is no default constructor: can create
  with again-obvious syntax
  
          let p = Point { x: 5, y: -2 };

## Fancy Features

* Common case of tag-name = varname is special

          let x = 5;
          let p = Point { x, y: -2 };

* Can have empty structures: these are inhabited by only one
  value
  
          struct Empty;
  
          let e = Empty;

* "Tuple structs" have anonymous fields

          struct Point(i64, i64);

          let p = Point(3, -2);
          let x = p.0;

* Single-element tuple structs are often used for "newtyping"

          struct Celsius(i64);
          struct Fahrenheit(i64);
          struct Newtons(i64);

## Ownership and Lifetimes

* Ownership is a bit complicated with structs

* A struct value owns all of its fields individually

          struct TwoString {
              s1: String,
              s2: String,
          }

## Partial Moves

* Moving a field out of a struct makes it "partially moved"

          let t = TwoString {
              s1: "hello".to_string(),
              s2: "world".to_string(),
          };
          let v1 = t.s1;
          println!("{}", v1); // prints "hello"
          // println!("{}", t.s1); // compiler error
          // some_function(&t); // compiler error: t partially moved
          // return t; // compiler error: t partially moved
          let v1 = t.s1;
          let v2 = t.s2;
          t.s1 = v2;
          t.s2 = v1;
          return t;

## Member Lifetimes

* Need to explicitly specify reference lifetime

* References stored in structs need to outlast the struct

          struct TwoStr<'a, 'b> {
              s1: &'a str,
              s2: &'b str,
          }

          let s = "hello".to_string();
          let t = TwoStr { s1: &s, s2: "world" };
          // drop(s);  // compiler error
          println!("{}", t.s1);

## impl

* Mechanism for getting that fancy OO-style syntax

* Implementor defines whether self argument is by
  move, reference or mut reference (`examples/point.rs`)

* Allows chaining of operators, which can be syntactically
  nice

## Deriving

* Mechanism for getting traits defined automatically for
  your struct
  
* Uses attribute syntax

* Common kind of thing to write (`examples/units.rs`)

          #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
          struct Temperature(i64);

  rather than e.g.

          impl std::cmp::PartialEq for Temperature {
             fn eq(&self, other: &Temperature) -> bool {
                 self.0 == other.0
             }
          }

## Misc

* Struct layout concepts are in the book: not too important

* Need to master structs: they are used everywhere
