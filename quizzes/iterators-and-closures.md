# Iterators and Closures

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

## Question 1: Advancing an Iterator

- Type: multiple choice
- Points: 10

### Prompt

What are the values of `first`, `second`, and `third`, in that
order?

```rust
let mut numbers = 4u32..6;
let first = numbers.next();
let second = numbers.next();
let third = numbers.next();
```

### Answer 1

- Weight: 100

#### Text

`Some(4)`, `Some(5)`, `None`

#### Feedback

Each call advances this range. Its end is excluded, and `next()`
evaluates to `None` when the range is exhausted.

### Answer 2

- Weight: 0

#### Text

`Some(4)`, `Some(5)`, `Some(6)`

### Answer 3

- Weight: 0

#### Text

`Some(4)`, `Some(4)`, `Some(4)`

### Answer 4

- Weight: 0

#### Text

`4`, `5`, `None`

## Question 2: Iterator Adapters

- Type: multiple choice
- Points: 10

### Prompt

What is `doubled`?

```rust
let doubled = (1..=3).map(|n| n * 2);
```

### Answer 1

- Weight: 100

#### Text

An iterator that yields `2`, `4`, and `6`.

### Answer 2

- Weight: 0

#### Text

A vector containing `2`, `4`, and `6`.

### Answer 3

- Weight: 0

#### Text

An iterator that yields every integer from `2` through `6`.

### Answer 4

- Weight: 0

#### Text

The original range, with each value changed in place.

## Question 3: Collecting Items

- Type: multiple choice
- Points: 10

### Prompt

Which trait does `Vec<u32>` implement so that `collect()` can
build it from the items?

```rust
let values: Vec<u32> = (1..=3).collect();
```

### Answer 1

- Weight: 100

#### Text

`FromIterator<u32>`

### Answer 2

- Weight: 0

#### Text

`IntoIterator`

### Answer 3

- Weight: 0

#### Text

`Iterator`

### Answer 4

- Weight: 0

#### Text

`From<u32>`

## Question 4: For Loops

- Type: multiple answer
- Points: 10

### Prompt

Which operations does a `for` loop use to obtain and advance
its iterator (choose all that apply)?

### Answer 1

- Weight: 100

#### Text

It uses `IntoIterator::into_iter()` to obtain an iterator.

### Answer 2

- Weight: 100

#### Text

It uses `Iterator::next()` to advance the iterator.

### Answer 3

- Weight: 0

#### Text

It calls `collect()` to gather all the items before running the
loop body.

### Answer 4

- Weight: 0

#### Text

It calls `map()` to run the loop body for each item.

## Question 5: Implementing an Iterator

- Type: multiple choice
- Points: 10

### Prompt

Why does `next()` subtract one from `remaining`?

```rust
struct Repeat {
    value: u32,
    remaining: usize,
}

impl Iterator for Repeat {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.value)
        }
    }
}
```

### Answer 1

- Weight: 100

#### Text

`remaining` records how many more items to yield before producing
`None`.

### Answer 2

- Weight: 0

#### Text

`remaining` is the next value yielded by the iterator.

### Answer 3

- Weight: 0

#### Text

The `Iterator` trait requires each yielded value to be different
from the previous one.

### Answer 4

- Weight: 0

#### Text

The `Iterator` trait needs a count so it can allocate space for
all the items.

## Question 6: Capturing a Local

- Type: multiple choice
- Points: 10

### Prompt

What is the value of `result`?

```rust
let offset = 5u32;
let add_offset = |n| n + offset;
let result = add_offset(7u32);
```

### Answer 1

- Weight: 100

#### Text

`12`

#### Feedback

The closure can use `offset` from its environment, and Rust
infers the type of `n`.

### Answer 2

- Weight: 0

#### Text

`7`

### Answer 3

- Weight: 0

#### Text

The code does not compile because `offset` is not a parameter of
the closure.

### Answer 4

- Weight: 0

#### Text

The code does not compile because the type of `n` must be written
explicitly.

## Question 7: Read-Only Captures

- Type: multiple choice
- Points: 10

### Prompt

Why does this closure satisfy the `Fn` bound?

```rust
fn check<F: Fn(u32) -> bool>(f: F) -> bool {
    f(4)
}

let limit = 10u32;
let result = check(|n| n < limit);
```

### Answer 1

- Weight: 100

#### Text

It reads the captured `limit` without changing it.

### Answer 2

- Weight: 0

#### Text

It captures nothing from its environment.

### Answer 3

- Weight: 0

#### Text

It moves `limit` out of the closure when called.

### Answer 4

- Weight: 0

#### Text

The `Fn` bound is satisfied because `check` calls the closure
only once.

## Question 8: Mutating a Capture

- Type: multiple choice
- Points: 10

### Prompt

Why does `run_twice` use `FnMut` rather than `Fn` for this
closure?

```rust
fn run_twice<F: FnMut()>(mut f: F) {
    f();
    f();
}

let mut total = 0u32;
run_twice(|| total += 1);
```

### Answer 1

- Weight: 100

#### Text

The closure needs to mutate the captured `total` on each call.

### Answer 2

- Weight: 0

#### Text

Any closure called more than once must use `FnMut` rather than
`Fn`.

### Answer 3

- Weight: 0

#### Text

The closure moves `total` out of its environment on its first
call.

### Answer 4

- Weight: 0

#### Text

The closure needs `FnMut` because it has no arguments.

## Question 9: Move Captures

- Type: multiple answer
- Points: 10

### Prompt

Which statements about this code are true
(choose all that apply)?

```rust
let name = String::from("Ada");
let length = move || name.len();
let first = length();
let second = length();
```

### Answer 1

- Weight: 100

#### Text

The closure takes ownership of the string.

#### Feedback

The `move` closure holds the string. Calling `len()` borrows the
string from the closure, so another call is allowed.

### Answer 2

- Weight: 100

#### Text

Both calls to `length()` evaluate to `3`.

### Answer 3

- Weight: 0

#### Text

The string is still available through `name` after the closure is
created.

### Answer 4

- Weight: 0

#### Text

The first call moves the string out of the closure, so the second
call cannot compile.

## Question 10: Closure Types

- Type: multiple choice
- Points: 10

### Prompt

Why use a trait object type for the array elements?

```rust
let scale = 2u32;
let offset = 5u32;
let times = |n| n * scale;
let plus = |n| n + offset;
let transforms: [&dyn Fn(u32) -> u32; 2] = [&times, &plus];
```

### Answer 1

- Weight: 100

#### Text

The closures have distinct types. The trait object gives the
references one common type.

### Answer 2

- Weight: 0

#### Text

The trait object makes both closures use the same captured
values.

### Answer 3

- Weight: 0

#### Text

A trait object is required whenever a closure captures a
variable.

### Answer 4

- Weight: 0

#### Text

The trait object copies both captured environments into the
array.
