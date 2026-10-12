# Ownership and Lifetimes

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

## Question 1: Array Types

- Type: multiple choice
- Points: 5

### Prompt

Which is the type of this array?

```rust
let a = [0u16; 12];
```

### Answer 1

- Weight: 100

#### Text

`[u16; 12]`

### Answer 2

- Weight: 0

#### Text

`[u16]`

### Answer 3

- Weight: 0

#### Text

`Vec<u16>`

### Answer 4

- Weight: 0

#### Text

`[u16; 0]`

## Question 2: Slices

- Type: multiple choice
- Points: 5

### Prompt

What does `&[i32]` describe?

### Answer 1

- Weight: 100

#### Text

A borrowed sequence of `i32` values with a known runtime length.

### Answer 2

- Weight: 0

#### Text

An owned vector of `i32` values.

### Answer 3

- Weight: 0

#### Text

An array whose length is part of its type.

### Answer 4

- Weight: 0

#### Text

A reference to exactly one `i32` value.

## Question 3: Length and Capacity

- Type: multiple choice
- Points: 5

### Prompt

What does this print?

```rust
let v: Vec<i32> = Vec::with_capacity(12);
println!("{}", v.len());
```

### Answer 1

- Weight: 100

#### Text

`0`

### Answer 2

- Weight: 0

#### Text

`12`

### Answer 3

- Weight: 0

#### Text

The length is uninitialized.

### Answer 4

- Weight: 0

#### Text

The code does not compile.

## Question 4: Popping a Vector

- Type: multiple choice
- Points: 5

### Prompt

What is the value of `last`?

```rust
let mut v: Vec<i32> = Vec::new();
let last = v.pop();
```

### Answer 1

- Weight: 100

#### Text

`None`

### Answer 2

- Weight: 0

#### Text

`Some(0)`

### Answer 3

- Weight: 0

#### Text

The code panics.

### Answer 4

- Weight: 0

#### Text

The code does not compile.

## Question 5: Creating a Struct

- Type: multiple choice
- Points: 5

### Prompt

Given this definition, which expression creates a `Point`?

```rust
struct Point {
    x: i64,
    y: i64,
}
```

### Answer 1

- Weight: 100

#### Text

`Point { x: 3, y: 5 }`

### Answer 2

- Weight: 0

#### Text

`Point(3, 5)`

### Answer 3

- Weight: 0

#### Text

`(3, 5)`

### Answer 4

- Weight: 0

#### Text

`Point::new(3, 5)` without defining a `new` method.

## Question 6: Method Receivers

- Type: multiple choice
- Points: 5

### Prompt

What does `&self` mean in a method defined in an `impl` block?

### Answer 1

- Weight: 100

#### Text

The method borrows its receiver through a shared reference.

### Answer 2

- Weight: 0

#### Text

The method takes ownership of its receiver.

### Answer 3

- Weight: 0

#### Text

The method borrows its receiver through a mutable reference.

### Answer 4

- Weight: 0

#### Text

The method clones its receiver.

## Question 7: Enums

- Type: multiple answer
- Points: 5

### Prompt

Things that are true of Rust enums include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

A value has one variant at a time.

### Answer 2

- Weight: 100

#### Text

A variant can carry values.

### Answer 3

- Weight: 0

#### Text

Every variant must carry the same types of values.

### Answer 4

- Weight: 0

#### Text

An enum value contains all of its variants at once.

## Question 8: Option

- Type: multiple choice
- Points: 5

### Prompt

Which type represents either a value of type `T` or no value?

### Answer 1

- Weight: 100

#### Text

`Option<T>`

### Answer 2

- Weight: 0

#### Text

`Vec<T>`

### Answer 3

- Weight: 0

#### Text

`&T`

### Answer 4

- Weight: 0

#### Text

`(T,)`

## Question 9: Result

- Type: multiple choice
- Points: 5

### Prompt

In `Result<T, E>`, which variants represent success and failure?

### Answer 1

- Weight: 100

#### Text

`Ok(T)` and `Err(E)`

### Answer 2

- Weight: 0

#### Text

`Some(T)` and `None`

### Answer 3

- Weight: 0

#### Text

`True(T)` and `False(E)`

### Answer 4

- Weight: 0

#### Text

`Ok(E)` and `Err(T)`

## Question 10: Unwrap

- Type: multiple answer
- Points: 5

### Prompt

Things that are true of `.unwrap()` include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

On `Some(value)` or `Ok(value)`, it returns `value`.

### Answer 2

- Weight: 100

#### Text

On `None` or `Err(error)`, it panics.

### Answer 3

- Weight: 0

#### Text

On `None`, it returns a default value.

### Answer 4

- Weight: 0

#### Text

On `Err(error)`, it returns `error` to the caller.

## Question 11: Propagating Errors

- Type: multiple choice
- Points: 5

### Prompt

What happens if parsing fails at `?`?

```rust
fn twice(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(2 * n)
}
```

### Answer 1

- Weight: 100

#### Text

`twice` returns the parse error as `Err` immediately.

### Answer 2

- Weight: 0

#### Text

`twice` panics.

### Answer 3

- Weight: 0

#### Text

`n` becomes zero and execution continues.

### Answer 4

- Weight: 0

#### Text

Parsing is retried until it succeeds.

## Question 12: Reference Lifetimes

- Type: multiple choice
- Points: 5

### Prompt

Why does this code fail to compile?

```rust
let r;
{
    let n = 7;
    r = &n;
}
println!("{}", r);
```

### Answer 1

- Weight: 100

#### Text

The reference `r` would outlive `n`.

### Answer 2

- Weight: 0

#### Text

Integer values cannot be borrowed.

### Answer 3

- Weight: 0

#### Text

Returning a reference always moves the value it refers to.

### Answer 4

- Weight: 0

#### Text

The function needs to clone `n` before borrowing it.

## Question 13: Borrow Rules

- Type: multiple answer
- Points: 5

### Prompt

Under the usual borrow rules, which are allowed?
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

Several shared references to the same value used together.

### Answer 2

- Weight: 100

#### Text

One mutable reference used to change a value.

### Answer 3

- Weight: 0

#### Text

Two mutable references to the same value used together.

### Answer 4

- Weight: 0

#### Text

Changing a value while shared references to it are still in use.

## Question 14: Exclusive Borrowing

- Type: multiple choice
- Points: 5

### Prompt

Why does this code fail to compile?

```rust
let mut n = 7;
let r = &mut n;
n = 8;
println!("{}", *r);
```

### Answer 1

- Weight: 100

#### Text

`n` is changed directly while its mutable borrow is still in use.

### Answer 2

- Weight: 0

#### Text

`n` has an immutable binding.

### Answer 3

- Weight: 0

#### Text

A mutable reference cannot be read.

### Answer 4

- Weight: 0

#### Text

A mutable reference must point to a heap allocation.

## Question 15: Moving a String

- Type: multiple choice
- Points: 5

### Prompt

Why does this code fail to compile?

```rust
let first = String::from("hello");
let second = first;
println!("{} {}", first, second);
```

### Answer 1

- Weight: 100

#### Text

Ownership moved to `second`, leaving `first` unusable.

### Answer 2

- Weight: 0

#### Text

A `String` cannot be printed with `{}`.

### Answer 3

- Weight: 0

#### Text

Assignment requires both bindings to be mutable.

### Answer 4

- Weight: 0

#### Text

Two strings cannot be printed together.

## Question 16: Copying an Integer

- Type: multiple choice
- Points: 5

### Prompt

What does this print?

```rust
let first = 7u32;
let second = first;
println!("{} {}", first, second);
```

### Answer 1

- Weight: 100

#### Text

`7 7`

### Answer 2

- Weight: 0

#### Text

`0 7`

### Answer 3

- Weight: 0

#### Text

The code does not compile because `first` was moved.

### Answer 4

- Weight: 0

#### Text

The code panics because both bindings own the same value.

## Question 17: Cloning a String

- Type: multiple choice
- Points: 5

### Prompt

Which statement is true?

```rust
let first = String::from("hello");
let second = first.clone();
```

### Answer 1

- Weight: 100

#### Text

Both bindings remain usable and own their own text buffers.

### Answer 2

- Weight: 0

#### Text

`first` is unusable because cloning moves it.

### Answer 3

- Weight: 0

#### Text

`second` is a shared reference to `first`.

### Answer 4

- Weight: 0

#### Text

The code fails because `String` is not `Copy`.

## Question 18: Passing Ownership

- Type: multiple choice
- Points: 5

### Prompt

Which statement is true after the call?

```rust
fn show(s: String) {
    println!("{}", s);
}

let message = String::from("hello");
show(message);
```

### Answer 1

- Weight: 100

#### Text

`show` took ownership, so `message` is no longer usable.

### Answer 2

- Weight: 0

#### Text

`show` borrowed `message` through a shared reference.

### Answer 3

- Weight: 0

#### Text

`show` automatically cloned `message`.

### Answer 4

- Weight: 0

#### Text

Ownership returns to `message` when `show` returns.

## Question 19: Dropping Values

- Type: multiple choice
- Points: 5

### Prompt

Under normal execution, what happens to this string at
the end of the block?

```rust
{
    let message = String::from("hello");
    println!("{}", message);
}
```

### Answer 1

- Weight: 100

#### Text

It is dropped when its owner leaves scope.

### Answer 2

- Weight: 0

#### Text

It stays allocated until the next garbage collection.

### Answer 3

- Weight: 0

#### Text

The programmer must call `free()` on it.

### Answer 4

- Weight: 0

#### Text

It is moved into a hidden global variable.

## Question 20: References in Structs

- Type: multiple choice
- Points: 5

### Prompt

What does `'a` describe here?

```rust
struct Label<'a> {
    text: &'a str,
}
```

### Answer 1

- Weight: 100

#### Text

How long the borrowed text must remain valid.

### Answer 2

- Weight: 0

#### Text

The number of characters in the text.

### Answer 3

- Weight: 0

#### Text

An instruction to copy the text into the struct.

### Answer 4

- Weight: 0

#### Text

A request to keep the text allocated forever.
