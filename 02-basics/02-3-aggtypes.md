## Aggregate Types

* Tuples

* Arrays with size e.g. `[int; 12]`

* `Vec`

* Strings

* Structs and enums

## Arrays

* Array types *include their size:* [char; 7]

* Every size is a different type

* Arrays are `Copy` if their elements are

* The intent of arrays is that small ones are useful in
  numeric computing e.g. `[f32;4]` and large ones are
  usually used via slices

## Array Initialization

* Array initialization syntax is either by copyable value

        let a = [0u16; 12];

  or by list

        let a = ['a', 'b', 'c'];

  or by default (for arrays <= 32 elements, yuck)

        let a: [String; 16] = Default::default();

* Arrays must always be *created initialized.* (Will
  explain later)

## Slices

* Slice types are types like `[char]` that denote a sequence
  of values of *unspecified size*: the user can't create
  these directly

* Taking the address of an array normally produces a slice reference (or "slice", ugh)

        let a: &[char] = &['a', 'b'];

* A slice (reference) is a "fat pointer" that carries both
  the slice address and slice length

* Both arrays and slices have a `.len()` method: slices
  always know their length

## Vec

* A `Vec` is an owned value that lives on the heap (memory
  managed by the memory allocator). The size and capacity
  (these are different) are part of the `Vec`, and the `Vec`
  can be grown and shrunk, often with the `.push()` and
  `.pop()` methods

    * `vec.push(val)` pushes `val` on the end of `vec`,
      increasing its length. Ownership rules apply:
      `.push()` borrows `vec` mutably, and `val` moves into
      `vec`

    * `vec.pop()` tries to move the value on the end of
      `vec` out as the result, decreasing `vec`'s
      length. `.pop()` returns `Option<T>`: it returns
      `None` if the `vec` is empty and `Some(val)` otherwise

## Vec Creation

* A new empty `Vec` can be created via

        Vec::new()
        Vec::with_capacity(15)

  Note that that second thing is also empty — it just has
  reserved space for efficiency

* The `vec!` macro can be used to create vectors:

        vec![1, 2, 3]

## Other Vec Stuff

* A `Vec` is not `Copy`, but it is `Clone` if its elements
  are `Clone`

* You can access a `Vec` as if it were an array: `v[0]` etc.

* A `Vec` reference can be auto-converted to a slice

        let s: &[char] = &vec![1, 2, 3];

## String

* A `String` is an owned UTF-8 text on the heap. To say that
  it is a sequence of `char`s is wrong: the text is
  compressed according to the UTF-8 standard

* An empty `String` can be created with

        String::new()
        String::with_capacity(12)

  The capacity is in bytes, not chars

## More String Creation

* Strings are often created using the `.to_string()` method
  of various other datatypes

* The `format!` macro can be used to create strings
  `println!`-style. There is no `formatln!` for some reason

        format!("{} bottles of root beer", 99)


## String Misc

* A `String` is not `Copy` but it is `Clone`

* `.push()` and `.pop()` can be used to add or remove
  `char`s on the end of a `String`

## No String Indexing

* String indexing is not supported. If it were supported it
  would have to be in bytes because UTF-8. Use the
  `.chars()` method to iterate over the characters of a
  `String`

        // This will not compile.
        for i in 0..s.len() {
            println!("{}", s[i]);
        }

        // Write this instead.
        for c in s.chars() {
            println!("{}", c);
        }

## str

* `str` is a chunk of UTF-8 text of unspecified
  length. There is no obvious way to create a value of this
  type directly in a Rust program. Instead, this type is
  used by reference: `&str`. A `str` can be thought of as a
  slice, and some slice (reference) operations apply to
  `&str`, but it's a bit confusing…

* A string constant is a `str` slice

        let s: &str = "hello world";

## More str Stuff

* String constants are immutable. This is a compiler error:

        let s: &mut str = "hello world";

* A `String` reference normally autorefs to a string slice

        let s: &str = &5.to_string();

* There's a whole pile of methods for manipulating `&str`:
  see the docs
