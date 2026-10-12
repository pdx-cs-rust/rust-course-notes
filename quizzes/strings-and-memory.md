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

What does this print?

```rust
let s = "é!";
println!("{} {}", s.len(), s.chars().count());
```

### Answer 1

- Weight: 100

#### Text

`3 2`

### Answer 2

- Weight: 0

#### Text

`2 2`

### Answer 3

- Weight: 0

#### Text

`3 3`

### Answer 4

- Weight: 0

#### Text

`2 3`

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

## Question 14: Heap Allocation

- Type: multiple answer
- Points: 5

### Prompt

Which types put their contents in a heap allocation?
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

`Box<T>`

### Answer 2

- Weight: 100

#### Text

`Rc<T>`

### Answer 3

- Weight: 0

#### Text

`Cell<T>` by itself.

### Answer 4

- Weight: 0

#### Text

`RefCell<T>` by itself.

## Question 15: Cell

- Type: multiple choice
- Points: 5

### Prompt

What does this print?

```rust
use std::cell::Cell;

let score = Cell::new(0);
let shared = &score;
shared.set(shared.get() + 1);
println!("{}", score.get());
```

### Answer 1

- Weight: 100

#### Text

`1`

### Answer 2

- Weight: 0

#### Text

`0`

### Answer 3

- Weight: 0

#### Text

The code does not compile because `score` is not declared `mut`.

### Answer 4

- Weight: 0

#### Text

The code panics because `shared` is a shared reference.

## Question 16: RefCell Guards

- Type: multiple choice
- Points: 5

### Prompt

Which statement is true of `RefCell<T>`?

### Answer 1

- Weight: 100

#### Text

`borrow_mut()` gives exclusive access checked at runtime.

### Answer 2

- Weight: 0

#### Text

`borrow_mut()` makes a clone of the contents.

### Answer 3

- Weight: 0

#### Text

`borrow()` moves the contents out of the cell.

### Answer 4

- Weight: 0

#### Text

All borrow conflicts are rejected at compile time.

## Question 17: RefCell Borrow Conflicts

- Type: multiple choice
- Points: 5

### Prompt

What happens when this code runs?

```rust
use std::cell::RefCell;

let value = RefCell::new(7);
let reader = value.borrow();
let writer = value.borrow_mut();
println!("{} {}", *reader, *writer);
```

### Answer 1

- Weight: 100

#### Text

`borrow_mut()` panics because the shared guard is still live.

### Answer 2

- Weight: 0

#### Text

It prints `7 7`.

### Answer 3

- Weight: 0

#### Text

The code does not compile because both guards are in one scope.

### Answer 4

- Weight: 0

#### Text

`borrow_mut()` waits until the shared guard is dropped.

## Question 18: Cloning Rc

- Type: multiple choice
- Points: 5

### Prompt

What does the clone do?

```rust
use std::rc::Rc;

let first = Rc::new(String::from("hello"));
let second = Rc::clone(&first);
```

### Answer 1

- Weight: 100

#### Text

It adds another owner of the same allocation.

### Answer 2

- Weight: 0

#### Text

It copies the text into a new allocation.

### Answer 3

- Weight: 0

#### Text

It moves ownership away from `first`.

### Answer 4

- Weight: 0

#### Text

It creates a borrowed `&str` without adding an owner.

## Question 19: Dropping Rc

- Type: multiple choice
- Points: 5

### Prompt

For an `Rc` allocation with no reference cycle, when are
its contents dropped?

### Answer 1

- Weight: 100

#### Text

When the last owning `Rc` is dropped.

### Answer 2

- Weight: 0

#### Text

When the first owning `Rc` is dropped.

### Answer 3

- Weight: 0

#### Text

At the next garbage collection.

### Answer 4

- Weight: 0

#### Text

Only when the programmer manually frees them.

## Question 20: Shared Mutable Data

- Type: multiple choice
- Points: 5

### Prompt

Why combine `Rc` and `RefCell` as `Rc<RefCell<T>>`?

### Answer 1

- Weight: 100

#### Text

For shared ownership with mutation checked by runtime borrow
rules.

### Answer 2

- Weight: 0

#### Text

To allow any number of simultaneous mutable borrows.

### Answer 3

- Weight: 0

#### Text

To share mutable data safely across threads.

### Answer 4

- Weight: 0

#### Text

To guarantee that reference cycles cannot leak.
