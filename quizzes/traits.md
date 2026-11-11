# Traits

> AI disclosure: This quiz was authored by Codex, an AI system
> from OpenAI, at the instructor's direction.

## Settings

- Type: graded quiz
- Total points: 100
- Question count: 10
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

## Question 1: Trait Interfaces

- Type: multiple answer
- Points: 10

### Prompt

What can a trait specify (choose all that apply)?

### Answer 1

- Weight: 100

#### Text

Methods.

### Answer 2

- Weight: 100

#### Text

Associated types.

### Answer 3

- Weight: 100

#### Text

Associated constants.

### Answer 4

- Weight: 0

#### Text

Data fields that every implementing type must store.

## Question 2: Default Methods

- Type: multiple choice
- Points: 10

### Prompt

Which method must an implementation of this trait provide?

```rust
trait Named {
    fn name(&self) -> &str;

    fn label(&self) -> &str {
        self.name()
    }
}
```

### Answer 1

- Weight: 100

#### Text

`name()` only.

#### Feedback

`label()` already has a default implementation.

### Answer 2

- Weight: 0

#### Text

`label()` only.

### Answer 3

- Weight: 0

#### Text

Both `name()` and `label()`.

### Answer 4

- Weight: 0

#### Text

Neither method.

## Question 3: Trait Bounds

- Type: multiple choice
- Points: 10

### Prompt

What does the bound on `T` require?

```rust
use std::fmt::Display;

fn report<T: Display>(value: T) {
    println!("{}", value);
}
```

### Answer 1

- Weight: 100

#### Text

The type used for `T` must implement `Display`.

### Answer 2

- Weight: 0

#### Text

The argument must be a `String`.

### Answer 3

- Weight: 0

#### Text

The argument must be converted to a `String` before the function
runs.

### Answer 4

- Weight: 0

#### Text

The argument must be a reference to a trait object.

## Question 4: Trait Bounds and Trait Objects

- Type: multiple choice
- Points: 10

### Prompt

Which statement describes these two functions?

```rust
trait Area {
    fn area(&self) -> u32;
}

fn area_a<T: Area>(shape: &T) -> u32 {
    shape.area()
}

fn area_b(shape: &dyn Area) -> u32 {
    shape.area()
}
```

### Answer 1

- Weight: 100

#### Text

`area_a` will be specialized for each concrete type used;
`area_b` calls through a trait object.

### Answer 2

- Weight: 0

#### Text

`area_b` will be specialized for each concrete type used;
`area_a` calls through a trait object.

### Answer 3

- Weight: 0

#### Text

Both functions use trait objects because both take references.

### Answer 4

- Weight: 0

#### Text

Both functions use a type parameter because both mention the
trait `Area`.

## Question 5: The Orphan Rule

- Type: multiple choice
- Points: 10

### Prompt

Your crate defines the type `Ticket` and the trait `Describe`.
Which implementation is forbidden by the orphan rule?

### Answer 1

- Weight: 100

#### Text

`Display` from the standard library for `Vec<u32>`.

#### Feedback

Neither `Display` nor `Vec<u32>` is defined in your crate.

### Answer 2

- Weight: 0

#### Text

`Display` from the standard library for `Ticket`.

### Answer 3

- Weight: 0

#### Text

`Describe` for `Vec<u32>`.

### Answer 4

- Weight: 0

#### Text

`Describe` for `Ticket`.

## Question 6: Copy and Clone

- Type: multiple answer
- Points: 10

### Prompt

Which statements about `Copy` are true (choose all that apply)?

### Answer 1

- Weight: 100

#### Text

A type that implements `Copy` must also implement `Clone`.

### Answer 2

- Weight: 100

#### Text

`Copy` is a marker trait with no methods of its own.

### Answer 3

- Weight: 0

#### Text

A type can implement both `Copy` and `Drop`.

### Answer 4

- Weight: 0

#### Text

Implementing `Clone` makes the type `Copy` automatically.

## Question 7: From and Into

- Type: multiple choice
- Points: 10

### Prompt

This `From` implementation also provides an `Into` conversion.
Which direction does `.into()` go?

```rust
struct Score(u32);

impl From<Score> for u32 {
    fn from(score: Score) -> Self {
        score.0
    }
}
```

### Answer 1

- Weight: 100

#### Text

From `Score` to `u32`.

### Answer 2

- Weight: 0

#### Text

From `u32` to `Score`.

### Answer 3

- Weight: 0

#### Text

Both directions.

### Answer 4

- Weight: 0

#### Text

From `&Score` to `u32`.

## Question 8: Fallible Conversion

- Type: multiple choice
- Points: 10

### Prompt

What happens when this code runs?

```rust
use std::convert::TryFrom;

let result = u8::try_from(300u16);
```

### Answer 1

- Weight: 100

#### Text

`result` is an `Err` because 300 does not fit in a `u8`.

#### Feedback

`TryFrom` reports conversion failure in a `Result`.

### Answer 2

- Weight: 0

#### Text

`result` is `Ok(44)` because the value wraps.

### Answer 3

- Weight: 0

#### Text

`result` is `Ok(255)` because the value is capped.

### Answer 4

- Weight: 0

#### Text

The conversion panics because the value is too large.

## Question 9: Borrowing with AsRef

- Type: multiple choice
- Points: 10

### Prompt

What does the bound `P: AsRef<Path>` let this function do?

```rust
use std::path::Path;

fn show_path<P: AsRef<Path>>(path: P) {
    let borrowed: &Path = path.as_ref();
    println!("{}", borrowed.display());
}
```

### Answer 1

- Weight: 100

#### Text

Get a shared reference to a `Path` from the argument.

### Answer 2

- Weight: 0

#### Text

Take ownership of a new `PathBuf` returned by `as_ref()`.

### Answer 3

- Weight: 0

#### Text

Get a mutable reference to the path through `as_ref()`.

### Answer 4

- Weight: 0

#### Text

Check at compile time that the path exists.

## Question 10: Default Values

- Type: multiple choice
- Points: 10

### Prompt

If `value` is `None`, what does `T::default()` evaluate to?

```rust
fn fallback<T: Default>(value: Option<T>) -> T {
    match value {
        Some(value) => value,
        None => T::default(),
    }
}
```

### Answer 1

- Weight: 100

#### Text

A value of type `T` supplied by its `Default` implementation.

### Answer 2

- Weight: 0

#### Text

A reference to a shared default value of type `T`.

### Answer 3

- Weight: 0

#### Text

A value of type `T` made by filling its memory with zeros.

### Answer 4

- Weight: 0

#### Text

`None` of type `Option<T>`.
