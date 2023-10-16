## Expressions

* Blocks have value

* [Operator precedence table](https://doc.rust-lang.org/reference/expressions.html#expression-precedence)
  is in the reference manual. 18 levels. Left
  associativity. Left-to-right evaluation?

* Conversion is mostly explicit through `as`. Only
  conversions that are "safe" work (where safe has
  a somewhat non-intuitive definition)

* Reference magic happens

  * Auto dereference in certain situations as discussed
    earlier
  * Auto reference conversions in certain situations, e.g.
    `String`, `Vec`, `Box`

## Expressions and Control Flow

* In Rust, some of what we would normally do with control
  flow is done with simple expressions. This is part of
  Rust's functional-language heritage

    * `if` is an expression type
    * "method" chaining is a standard mechanism

## "Method" Chaining

* Many Rust operations are used in an "object-oriented style":

        "hello".to_uppercase()

  vs

        to_uppercase("hello")

* This allows stringing together expressions in more
  readable syntax (and also facilitates proper borrowing/sharing).

  <https://play.rust-lang.org/?edition=2021&gist=0842a2a805a02e8a2d031e77f2bb5a6e>

## Explicit Control Flow

* `if`-`else`

* `if let`, `while let`

* `match`

* `while`

* `loop`

* `for` and iterators
