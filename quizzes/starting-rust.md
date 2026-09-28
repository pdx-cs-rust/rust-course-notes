# Starting Rust

> AI disclosure: This quiz was authored by Codex, an AI system
> from OpenAI, at the instructor's direction.

## Settings

- Type: graded quiz
- Total points: 100
- Question count: 16
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

## Question 1: Cargo Workflow

- Type: multiple answer
- Points: 6

### Prompt

Things that are true of the usual Cargo workflow include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

`cargo new --bin hello` creates a new binary project.

### Answer 2

- Weight: 100

#### Text

`cargo build` builds the project.

### Answer 3

- Weight: 100

#### Text

`cargo run` builds the project if needed, then runs it.

### Answer 4

- Weight: 0

#### Text

`cargo new` runs an existing project.

### Answer 5

- Weight: 0

#### Text

`cargo build` installs Rust on the machine.

## Question 2: Cargo Tools

- Type: multiple answer
- Points: 6

### Prompt

Things that are true of the standard Cargo tools include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

`cargo fmt` formats Rust source code.

### Answer 2

- Weight: 100

#### Text

`cargo clippy` suggests ways to improve Rust code.

### Answer 3

- Weight: 100

#### Text

`cargo test` builds and runs the project's tests.

### Answer 4

- Weight: 0

#### Text

`cargo fmt` checks whether all tests pass.

### Answer 5

- Weight: 0

#### Text

`cargo clippy` replaces the Rust compiler.

## Question 3: Memory Management

- Type: multiple choice
- Points: 6

### Prompt

Which best describes Rust's memory management?

### Answer 1

- Weight: 100

#### Text

The compiler uses statically checked rules to arrange automatic
memory management.

### Answer 2

- Weight: 0

#### Text

A tracing garbage collector finds unused objects while the
program runs.

### Answer 3

- Weight: 0

#### Text

The programmer must manually free every allocation.

### Answer 4

- Weight: 0

#### Text

Every value is reference-counted at runtime.

## Question 4: Type Inference

- Type: multiple choice
- Points: 6

### Prompt

Consider this code:

```rust
let mut n = 12;
n = 14u8 + 12;
```

Which statement is true?

### Answer 1

- Weight: 100

#### Text

Rust infers one static type for `n`.

### Answer 2

- Weight: 0

#### Text

The type of `n` can change on each assignment.

### Answer 3

- Weight: 0

#### Text

The code cannot compile without an explicit type for `n`.

### Answer 4

- Weight: 0

#### Text

`mut` disables type checking for `n`.

## Question 5: Numeric Conversion

- Type: multiple choice
- Points: 6

### Prompt

Which expression completes this code?

```rust
let n: u32 = 7;
let m: u64 = /* expression */;
```

### Answer 1

- Weight: 100

#### Text

`n as u64`

### Answer 2

- Weight: 0

#### Text

`n`

### Answer 3

- Weight: 0

#### Text

`n: u64`

### Answer 4

- Weight: 0

#### Text

`&n`

## Question 6: Block Values

- Type: multiple choice
- Points: 6

### Prompt

What is the value of `n`?

```rust
let n = {
    let x = 4;
    x + 1
};
```

### Answer 1

- Weight: 100

#### Text

`5`

### Answer 2

- Weight: 0

#### Text

`4`

### Answer 3

- Weight: 0

#### Text

`()`

### Answer 4

- Weight: 0

#### Text

The code does not compile.

## Question 7: Semicolons

- Type: multiple choice
- Points: 6

### Prompt

What is the type of `n`?

```rust
let n = {
    let x = 4;
    x + 1;
};
```

### Answer 1

- Weight: 100

#### Text

`()`

### Answer 2

- Weight: 0

#### Text

`i32`

### Answer 3

- Weight: 0

#### Text

`bool`

### Answer 4

- Weight: 0

#### Text

The type cannot be inferred.

## Question 8: If Expressions

- Type: multiple choice
- Points: 6

### Prompt

Which statement compiles?

Assume `ready` has type `bool`.

### Answer 1

- Weight: 100

#### Text

```rust
let x = if ready { 1u32 } else { 2u32 };
```

### Answer 2

- Weight: 0

#### Text

```rust
let x = if ready { 1u32 } else { "two" };
```

### Answer 3

- Weight: 0

#### Text

```rust
let x = if ready { 1u32 } else { 2u64 };
```

### Answer 4

- Weight: 0

#### Text

```rust
let x = if ready { 1u32 };
```

## Question 9: Numeric Types

- Type: multiple answer
- Points: 6

### Prompt

Which are built-in Rust numeric types? (choose all that apply)

### Answer 1

- Weight: 100

#### Text

`i32`

### Answer 2

- Weight: 100

#### Text

`isize`

### Answer 3

- Weight: 100

#### Text

`f64`

### Answer 4

- Weight: 0

#### Text

`int`

### Answer 5

- Weight: 0

#### Text

`float`

## Question 10: Numeric Literals

- Type: multiple answer
- Points: 6

### Prompt

Things that are true of Rust numeric literals include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

`0xffu32` is a hexadecimal `u32` literal.

### Answer 2

- Weight: 100

#### Text

`0b0110` is a binary literal.

### Answer 3

- Weight: 100

#### Text

`48_000` has the same numeric value as `48000`.

### Answer 4

- Weight: 100

#### Text

`1.0f32` has type `f32`.

### Answer 5

- Weight: 0

#### Text

Underscores in a numeric literal may sometimes change its value.

## Question 11: Characters and Strings

- Type: multiple choice
- Points: 6

### Prompt

Which statement is true?

### Answer 1

- Weight: 100

#### Text

A Rust `char` represents a Unicode code point, while Rust
strings use UTF-8.

### Answer 2

- Weight: 0

#### Text

A Rust `char` is always an 8-bit byte.

### Answer 3

- Weight: 0

#### Text

A Rust string stores every character as a 32-bit `char`.

### Answer 4

- Weight: 0

#### Text

`'hello'` is a valid Rust `char` literal.

## Question 12: Unit

- Type: multiple choice
- Points: 6

### Prompt

What is the type of `x`?

```rust
let x = ();
```

### Answer 1

- Weight: 100

#### Text

`()`

### Answer 2

- Weight: 0

#### Text

`bool`

### Answer 3

- Weight: 0

#### Text

An empty tuple has no type.

### Answer 4

- Weight: 0

#### Text

The code does not compile.

## Question 13: Tuple Access

- Type: multiple choice
- Points: 7

### Prompt

What does this print?

```rust
let t: (u64, f64, char) = (12, 1.2, 'x');
println!("{}", t.2);
```

### Answer 1

- Weight: 100

#### Text

`x`

### Answer 2

- Weight: 0

#### Text

`12`

### Answer 3

- Weight: 0

#### Text

`1.2`

### Answer 4

- Weight: 0

#### Text

The code does not compile because tuple fields have no names.

## Question 14: One-Element Tuples

- Type: multiple choice
- Points: 7

### Prompt

Which expression is a one-element tuple?

### Answer 1

- Weight: 100

#### Text

`(12,)`

### Answer 2

- Weight: 0

#### Text

`(12)`

### Answer 3

- Weight: 0

#### Text

`[12]`

### Answer 4

- Weight: 0

#### Text

`{12}`

## Question 15: Copying Tuples

- Type: multiple answer
- Points: 7

### Prompt

Things that are true of tuples include (choose all that apply)

### Answer 1

- Weight: 100

#### Text

A tuple is `Copy` when all of its elements are `Copy`.

### Answer 2

- Weight: 100

#### Text

A tuple can contain values of different types.

### Answer 3

- Weight: 0

#### Text

Every tuple is `Copy`.

### Answer 4

- Weight: 0

#### Text

Tuple fields are accessed with square brackets, as in `t[0]`.

## Question 16: Control Flow

- Type: multiple answer
- Points: 7

### Prompt

Things that are true of Rust control flow include
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

`for` is used to visit values from an iterator.

### Answer 2

- Weight: 100

#### Text

`while` repeats while its condition is true.

### Answer 3

- Weight: 100

#### Text

`loop` expresses an unconditional loop.

### Answer 4

- Weight: 100

#### Text

`match` selects among patterns.

### Answer 5

- Weight: 0

#### Text

`if` is the only way to choose between alternatives in Rust.
