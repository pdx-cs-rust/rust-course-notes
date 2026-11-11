# Special Memory Types

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

## Question 1: Box

- Type: multiple choice
- Points: 10

### Prompt

What does `Box::new(42u64)` do?

### Answer 1

- Weight: 100

#### Text

Puts the value in a heap allocation owned by the box.

### Answer 2

- Weight: 0

#### Text

Borrows the value without allocating.

### Answer 3

- Weight: 0

#### Text

Puts the value in a reference-counted allocation.

### Answer 4

- Weight: 0

#### Text

Stores the value directly inside the box, with no separate
allocation.

## Question 2: Interior Mutability

- Type: multiple answer
- Points: 10

### Prompt

Which statements apply to both `Cell<u32>` and `RefCell<u32>`
(choose all that apply)?

### Answer 1

- Weight: 100

#### Text

You can change the contained value through a shared reference to
the wrapper.

### Answer 2

- Weight: 100

#### Text

Neither wrapper allocates heap storage to hold the `u32`.

### Answer 3

- Weight: 0

#### Text

The wrapper uses a reference count to decide when to drop its
value.

### Answer 4

- Weight: 0

#### Text

The wrapper provides thread-safe sharing of the contained value.

## Question 3: Getting a Cell Value

- Type: multiple choice
- Points: 10

### Prompt

What does this print?

```rust
use std::cell::Cell;

let score = Cell::new(3u32);
let before = score.get();
score.set(5);
println!("{} {}", before, score.get());
```

### Answer 1

- Weight: 100

#### Text

`3 5`

#### Feedback

`get()` copies the value. Later changes to the cell do not change
that copy.

### Answer 2

- Weight: 0

#### Text

`5 5`

### Answer 3

- Weight: 0

#### Text

`3 3`

### Answer 4

- Weight: 0

#### Text

It does not compile because `score` is not declared `mut`.

## Question 4: Replacing a Cell Value

- Type: multiple choice
- Points: 10

### Prompt

Which expression puts `"new"` in the cell and evaluates to
the old `String`?

```rust
use std::cell::Cell;

let label = Cell::new(String::from("old"));
```

### Answer 1

- Weight: 100

#### Text

`label.replace(String::from("new"))`

#### Feedback

The `replace()` expression evaluates to the old value without
copying it.

### Answer 2

- Weight: 0

#### Text

`label.set(String::from("new"))`

### Answer 3

- Weight: 0

#### Text

`label.get()`

### Answer 4

- Weight: 0

#### Text

`label.borrow_mut()`

## Question 5: Conflicting RefCell Borrows

- Type: multiple choice
- Points: 10

### Prompt

What happens with this code?

```rust
use std::cell::RefCell;

let score = RefCell::new(3);
let read = score.borrow();
*score.borrow_mut() += 1;
println!("{}", *read);
```

### Answer 1

- Weight: 100

#### Text

It panics at `borrow_mut()`.

#### Feedback

The shared guard is still live. A `RefCell` checks for
conflicting borrows at runtime.

### Answer 2

- Weight: 0

#### Text

It prints `4`.

### Answer 3

- Weight: 0

#### Text

It prints `3` because `read` holds a copy.

### Answer 4

- Weight: 0

#### Text

The compiler rejects the overlapping borrows.

## Question 6: Releasing a RefCell Borrow

- Type: multiple choice
- Points: 10

### Prompt

Why does the last borrow succeed?

```rust
use std::cell::RefCell;

let score = RefCell::new(3);
{
    let mut write = score.borrow_mut();
    *write += 1;
}
println!("{}", *score.borrow());
```

### Answer 1

- Weight: 100

#### Text

The exclusive guard is dropped at the end of the inner block.

### Answer 2

- Weight: 0

#### Text

Writing through the guard immediately releases it.

### Answer 3

- Weight: 0

#### Text

A shared guard can coexist with an exclusive guard.

### Answer 4

- Weight: 0

#### Text

`borrow()` copies the value without acquiring a guard.

## Question 7: Taking a Value

- Type: multiple choice
- Points: 10

### Prompt

What is inside `message` after this code runs?

```rust
use std::cell::RefCell;

let message = RefCell::new(Some(String::from("hello")));
let old = message.borrow_mut().take();
```

### Answer 1

- Weight: 100

#### Text

`None`

#### Feedback

`take()` moves out the old `Option` and leaves an initialized
`None` in its place.

### Answer 2

- Weight: 0

#### Text

`Some(String::from("hello"))`

### Answer 3

- Weight: 0

#### Text

`Some(String::new())`

### Answer 4

- Weight: 0

#### Text

The cell is uninitialized.

## Question 8: Cloning an Rc

- Type: multiple choice
- Points: 10

### Prompt

Which statement describes `second` after this code runs?

```rust
use std::rc::Rc;

let first = Rc::new(String::from("hello"));
let second = Rc::clone(&first);
drop(first);
```

### Answer 1

- Weight: 100

#### Text

It still points to the original allocation and keeps the string
alive.

#### Feedback

Cloning an `Rc` creates another handle pointing to the same
allocation and increments the reference count. Dropping a handle
decrements the count. In this example, when the count reaches
zero, the cell containing the value is dropped and its storage
is freed.

### Answer 2

- Weight: 0

#### Text

It points to a separate allocation containing a copy of the
string.

### Answer 3

- Weight: 0

#### Text

It is a borrow of the string, so the compiler rejects
`drop(first)`.

### Answer 4

- Weight: 0

#### Text

It can no longer access the string because the first handle was
dropped.

## Question 9: Reference Cycles

- Type: multiple choice
- Points: 10

### Prompt

Two allocations each contain an `Rc` handle pointing to the
other. All handles outside the pair are dropped. Why can the
two values leak?

### Answer 1

- Weight: 100

#### Text

Each allocation still has a handle pointing to it from inside
the other.

### Answer 2

- Weight: 0

#### Text

Dropping a handle does not decrement the reference count.

### Answer 3

- Weight: 0

#### Text

`Rc` waits for a garbage collector to find unused values.

### Answer 4

- Weight: 0

#### Text

Once the count has been greater than one, the value cannot be
dropped.

## Question 10: Sharing and Mutation

- Type: multiple answer
- Points: 10

### Prompt

Why does this print `8` (choose all that apply)?

```rust
use std::cell::RefCell;
use std::rc::Rc;

let score = Rc::new(RefCell::new(7));
let observer = Rc::clone(&score);

*score.borrow_mut() += 1;
println!("{}", *observer.borrow());
```

### Answer 1

- Weight: 100

#### Text

Both `Rc` handles point to the same allocation.

### Answer 2

- Weight: 100

#### Text

The `RefCell` allows mutation through shared access.

### Answer 3

- Weight: 0

#### Text

Cloning the `Rc` makes an independent copy of the score.

### Answer 4

- Weight: 0

#### Text

`borrow()` obtains an exclusive guard.
