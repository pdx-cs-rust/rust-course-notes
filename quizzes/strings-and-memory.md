# Strings and Memory

> AI disclosure: This quiz was authored by Codex, an AI system
> from OpenAI, at the instructor's direction.

## Settings

- Type: graded quiz
- Total points: 100
- Question count: 20
- Shuffle answers: yes
- Attempts: unlimited
- Scoring policy: keep highest
- Time limit: none
- Show correct answers: immediately
- Hide correct answers: never
- Show correct answers only after last attempt: no
- Hide results: never
- One-time results: no
- One question at a time: no
- Allow backtracking: yes
- Access code: none
- IP filter: none
- Only visible to assignment overrides: no
- Anonymous submissions: no

## Instructions

Choose the best answer for each question. Some questions ask
you to choose all answers that apply. You may retake this quiz
as many times as you like.

## Question 1: Char

- Type: multiple choice
- Points: 5

### Prompt

What does a Rust `char` represent?

### Answer 1

- Weight: 100

#### Text

One Unicode scalar value.

### Answer 2

- Weight: 0

#### Text

One UTF-8 byte.

### Answer 3

- Weight: 0

#### Text

One complete word.

### Answer 4

- Weight: 0

#### Text

Exactly one displayed character, even if it uses several scalar
values.

## Question 2: String and str

- Type: multiple answer
- Points: 5

### Prompt

Things that are true of Rust strings include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

A `String` owns a UTF-8 text buffer.

### Answer 2

- Weight: 100

#### Text

An `&str` borrows UTF-8 text.

### Answer 3

- Weight: 0

#### Text

A `String` stores a fixed-width `char` for each scalar value.

### Answer 4

- Weight: 0

#### Text

An `&str` can only refer to a string literal.

## Question 3: Byte Length

- Type: multiple choice
- Points: 5

### Prompt

For a Rust string `s`, which statement is always true?

### Answer 1

- Weight: 100

#### Text

`s.chars().count()` is never greater than `s.len()`.

### Answer 2

- Weight: 0

#### Text

`s.chars().count()` is always equal to `s.len()`.

### Answer 3

- Weight: 0

#### Text

`s.chars().count()` is always less than `s.len()`.

### Answer 4

- Weight: 0

#### Text

`s.chars().count()` can be greater than `s.len()` when the
string contains non-ASCII characters.

## Question 4: String Iteration

- Type: multiple choice
- Points: 5

### Prompt

Which loop visits a string's Unicode scalar values as `char`s?

Assume `s` has type `&str`.

### Answer 1

- Weight: 100

#### Text

```rust
for c in s.chars() {
    println!("{}", c);
}
```

### Answer 2

- Weight: 0

#### Text

```rust
for c in s.bytes() {
    println!("{}", c);
}
```

### Answer 3

- Weight: 0

#### Text

```rust
for i in 0..s.len() {
    println!("{}", s[i]);
}
```

### Answer 4

- Weight: 0

#### Text

```rust
for c in s {
    println!("{}", c);
}
```

## Question 5: Parsing Text

- Type: multiple choice
- Points: 5

### Prompt

What is the value of `n`?

```rust
let n = "42".parse::<u32>();
```

### Answer 1

- Weight: 100

#### Text

`Ok(42)`

### Answer 2

- Weight: 0

#### Text

`Some(42)`

### Answer 3

- Weight: 0

#### Text

`42`

### Answer 4

- Weight: 0

#### Text

The code does not compile because strings cannot be parsed.

## Question 6: Tuple Patterns

- Type: multiple choice
- Points: 5

### Prompt

What does this print?

```rust
let (n, word) = (3u8, "hello");
println!("{} {}", n, word);
```

### Answer 1

- Weight: 100

#### Text

`3 hello`

### Answer 2

- Weight: 0

#### Text

`hello 3`

### Answer 3

- Weight: 0

#### Text

The code does not compile because a pattern cannot bind two
names.

### Answer 4

- Weight: 0

#### Text

The code does not compile because the types differ.

## Question 7: Exhaustive Matching

- Type: multiple choice
- Points: 5

### Prompt

Why does this code fail to compile?

```rust
let n = Some(3u8);
let value = match n {
    Some(x) => x,
};
```

### Answer 1

- Weight: 100

#### Text

The match does not cover `None`.

### Answer 2

- Weight: 0

#### Text

A match arm cannot return a value.

### Answer 3

- Weight: 0

#### Text

`Some(x)` is not a valid pattern.

### Answer 4

- Weight: 0

#### Text

Matching an `Option` requires a mutable binding.

## Question 8: Match Order

- Type: multiple choice
- Points: 5

### Prompt

What is the value of `word`?

```rust
let n = 3;
let word = match n {
    1..=3 => "small",
    3..=5 => "medium",
    _ => "other",
};
```

### Answer 1

- Weight: 100

#### Text

`"small"`

### Answer 2

- Weight: 0

#### Text

`"medium"`

### Answer 3

- Weight: 0

#### Text

`"other"`

### Answer 4

- Weight: 0

#### Text

The code does not compile because the ranges overlap.

## Question 9: let Patterns

- Type: multiple choice
- Points: 5

### Prompt

Why does this code fail to compile?

```rust
let maybe: Option<u8> = None;
let Some(n) = maybe;
```

### Answer 1

- Weight: 100

#### Text

An ordinary `let` pattern must also handle the `None` case.

### Answer 2

- Weight: 0

#### Text

`Some` must always be written as `Option::Some`.

### Answer 3

- Weight: 0

#### Text

`maybe` must be declared `mut` before it can be matched.

### Answer 4

- Weight: 0

#### Text

An `Option` cannot contain an integer.

## Question 10: Borrowing in Patterns

- Type: multiple choice
- Points: 5

### Prompt

What is the type of `text`?

```rust
let item = Some(String::from("hello"));
match item {
    Some(ref text) => println!("{}", text),
    None => (),
}
```

### Answer 1

- Weight: 100

#### Text

`&String`

### Answer 2

- Weight: 0

#### Text

`String`

### Answer 3

- Weight: 0

#### Text

`&mut String`

### Answer 4

- Weight: 0

#### Text

`Option<String>`

## Question 11: Generic Functions

- Type: multiple choice
- Points: 5

### Prompt

What are the types of `n` and `c`?

```rust
fn id<T>(x: T) -> T {
    x
}

let n = id(3u16);
let c = id('c');
```

### Answer 1

- Weight: 100

#### Text

`u16` and `char`

### Answer 2

- Weight: 0

#### Text

`T` and `T`, with no concrete types.

### Answer 3

- Weight: 0

#### Text

`i32` and `char`

### Answer 4

- Weight: 0

#### Text

The code does not compile because both calls must use the same
type.

## Question 12: Explicit Type Arguments

- Type: multiple choice
- Points: 5

### Prompt

What does `::<u32>` specify?

```rust
let n = "42".parse::<u32>();
```

### Answer 1

- Weight: 100

#### Text

The type to parse the text into.

### Answer 2

- Weight: 0

#### Text

The number of bytes to parse.

### Answer 3

- Weight: 0

#### Text

The type of the parse error.

### Answer 4

- Weight: 0

#### Text

The memory address where the result is stored.

## Question 13: Box Ownership

- Type: multiple choice
- Points: 5

### Prompt

Under normal execution, what happens when this `Box` leaves
scope?

```rust
{
    let message = Box::new(String::from("hello"));
    println!("{}", message);
}
```

### Answer 1

- Weight: 100

#### Text

The boxed value is dropped and the box's allocation is freed.

### Answer 2

- Weight: 0

#### Text

The value stays allocated until a garbage collector finds it.

### Answer 3

- Weight: 0

#### Text

The programmer must free the allocation manually.

### Answer 4

- Weight: 0

#### Text

Only the box is dropped; the string buffer always leaks.

## Question 14: String Mutation

- Type: multiple answer
- Points: 5

### Prompt

Things that are true of `String` include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

Its contents can be modified through a mutable reference.

### Answer 2

- Weight: 100

#### Text

Its safe methods preserve valid UTF-8.

### Answer 3

- Weight: 0

#### Text

Its length is fixed when it is created.

### Answer 4

- Weight: 0

#### Text

Its safe methods can leave it containing invalid UTF-8.

## Question 15: Converting to String

- Type: multiple choice
- Points: 5

### Prompt

What is the type of `s`?

```rust
let s = 42u32.to_string();
```

### Answer 1

- Weight: 100

#### Text

`String`

### Answer 2

- Weight: 0

#### Text

`&str`

### Answer 3

- Weight: 0

#### Text

`u32`

### Answer 4

- Weight: 0

#### Text

`Result<String, std::num::ParseIntError>`

## Question 16: Borrowing String Bytes

- Type: multiple choice
- Points: 5

### Prompt

Which expression borrows the bytes of a `String` named `s`,
leaving `s` usable?

### Answer 1

- Weight: 100

#### Text

`s.as_bytes()`

### Answer 2

- Weight: 0

#### Text

`s.into_bytes()`

### Answer 3

- Weight: 0

#### Text

`s.chars()`

### Answer 4

- Weight: 0

#### Text

`s.len()`

## Question 17: Struct Patterns

- Type: multiple choice
- Points: 5

### Prompt

What does this print?

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 3, y: 5 };
let Point { x, .. } = point;
println!("{}", x);
```

### Answer 1

- Weight: 100

#### Text

`3`

### Answer 2

- Weight: 0

#### Text

`5`

### Answer 3

- Weight: 0

#### Text

`3 5`

### Answer 4

- Weight: 0

#### Text

The code does not compile because the pattern omits `y`.

## Question 18: Pattern Alternatives

- Type: multiple choice
- Points: 5

### Prompt

In a `match` pattern, what does `1 | 2` mean?

### Answer 1

- Weight: 100

#### Text

Match either `1` or `2`.

### Answer 2

- Weight: 0

#### Text

Match the result of the expression `1 | 2`, which is `3`.

### Answer 3

- Weight: 0

#### Text

Match the tuple `(1, 2)`.

### Answer 4

- Weight: 0

#### Text

Match any value except `1` and `2`.

## Question 19: Generic Structs

- Type: multiple choice
- Points: 5

### Prompt

Given this definition, which expression creates a `Pair<u32>`?

```rust
struct Pair<T> {
    first: T,
    second: T,
}
```

### Answer 1

- Weight: 100

#### Text

`Pair { first: 1u32, second: 2u32 }`

### Answer 2

- Weight: 0

#### Text

`Pair { first: 1u32, second: 2u64 }`

### Answer 3

- Weight: 0

#### Text

`Pair { first: 1u32, second: "two" }`

### Answer 4

- Weight: 0

#### Text

`Pair { first: 'a', second: 'b' }`

## Question 20: Memory Leaks

- Type: multiple choice
- Points: 5

### Prompt

Which statement about memory leaks in Rust is true?

### Answer 1

- Weight: 100

#### Text

Memory leaks are possible in safe Rust.

### Answer 2

- Weight: 0

#### Text

Rust's garbage collector prevents memory leaks.

### Answer 3

- Weight: 0

#### Text

Only unsafe code can leak memory.

### Answer 4

- Weight: 0

#### Text

A `Box` always leaks unless its allocation is freed manually.
