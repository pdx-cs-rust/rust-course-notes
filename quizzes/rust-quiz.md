# HW: Rust Quiz

## Settings

- Type: graded quiz
- Total points: 100
- Question count: 15
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

You can re-take this quiz an unlimited number of times. In order
to receive credit, you must answer all questions correctly.

## Question 1

- Name: Question
- Type: multiple choice
- Points: 6

### Prompt

True or false: Rust is a garbage-collected language.

### Answer 1

- Weight: 100

#### Text

false

### Answer 2

- Weight: 0

#### Text

true

## Question 2

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

What is the most general trait for the closure `c` here?

```rust
let mut x = 0;
let c = |y: i32| {
    x += y;
    x
};
```

### Answer 1

- Weight: 100

#### Text

`FnMut(i32) -> i32`

#### Feedback

The closure mutably borrows `x` from its environment. Therefore,
it implements `FnMut` and `FnOnce`, but not `Fn`.

### Answer 2

- Weight: 0

#### Text

`Fn(i32) -> i32`

### Answer 3

- Weight: 0

#### Text

`FnOnce(i32) -> i32`

## Question 3

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

What is the type of the following block?

```rust
{
    let x: i32 = 0;
    &&*&&**&*&&x
}
```

### Answer 1

- Weight: 100

#### Text

`&&&i32`

### Answer 2

- Weight: 0

#### Text

there is a type error

### Answer 3

- Weight: 0

#### Text

`i32`

### Answer 4

- Weight: 0

#### Text

`&i32`

## Question 4

- Name: Question
- Type: multiple choice
- Points: 6

### Prompt

What is the most general trait for the closure `c` here?

```rust
let x = 0;
let c = |y: i32| { x + y };
```

### Answer 1

- Weight: 100

#### Text

`Fn(i32) -> i32`

#### Feedback

The closure can be used in an unrestricted way, so its most
general trait is `Fn`. Note that it *also* implements `FnMut` and
`FnOnce`, so it can be used wherever either of those traits are
expected.

### Answer 2

- Weight: 0

#### Text

`FnMut(i32) -> i32`

### Answer 3

- Weight: 0

#### Text

`FnOnce(i32) -> i32`

## Question 5

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

How could this function be made more idiomatic?

```rust
fn f(xs: &Vec<u32>) -> u32 {
    xs.iter().copied().map(|i| i.pow(i)).sum()
}
```

### Answer 1

- Weight: 100

#### Text

Give `xs` type `&[u32]`

#### Feedback

Then it could accept other slice types. This is normal in
Rust functions.

### Answer 2

- Weight: 0

#### Text

no problems

### Answer 3

- Weight: 0

#### Text

using copied() here is inefficient

### Answer 4

- Weight: 0

#### Text

`i` should explicitly have type `i32`

## Question 6

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

Is anything wrong with the following code?

```rust
let mut xs: Vec<i32> = vec![];
// ... fill xs with some numbers ...
xs.iter_mut().map(|i| *i += 1);
// ... do some more stuff ...
```

### Answer 1

- Weight: 100

#### Text

the call to map() has no effect

#### Feedback

Iterators are lazy and do not "do anything" on their own. Either of
the following would fix the problem here:

- replacing the whole thing with a `for`-loop (the best solution)

- replacing `.map()` with `.for_each()`

### Answer 2

- Weight: 0

#### Text

`xs` does not need an explicit type signature

#### Feedback

Rust cannot infer the type of `xs` here. We could instead give
the argument `i` a type, though.

### Answer 3

- Weight: 0

#### Text

no problems

### Answer 4

- Weight: 0

#### Text

`i` does not need to be dereferenced

## Question 7

- Name: Question
- Type: multiple choice
- Points: 6

### Prompt

What is wrong with the following code?

```rust
fn f(x: &mut i32) -> Box<dyn FnMut(i32)> {
    Box::new(|y| { *x = y; })
}
```

### Answer 1

- Weight: 100

#### Text

The returned function must live no longer than the referent of
`x`.

#### Feedback

Since the returned function writes to `x`, it cannot outlive the
location that `x` references.  This can be made explicit via:

```rust
fn f<'a>(x: &'a mut i32) -> Box<dyn FnMut(i32) + 'a>
```

### Answer 2

- Weight: 0

#### Text

the closure needs an explicit argument type

### Answer 3

- Weight: 0

#### Text

the returned function needs to be a `FnOnce`

### Answer 4

- Weight: 0

#### Text

the closure needs to be stored in a variable

## Question 8

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

Can the following code be simplified?

```rust
match f() {
    None => return None,
    Some(x) => x,
}
```

### Answer 1

- Weight: 100

#### Text

replace it with `f()?`

#### Feedback

This is exactly the meaning of the `?` operator (for `Option`).

### Answer 2

- Weight: 0

#### Text

remove the `return` keyword

#### Feedback

This is context-dependent. Removing the `return` keyword *might*
produce equivalent code if this is the last expression in a
function, but it will not generally work in any other context
(i.e., if the `return` is an early return).

### Answer 3

- Weight: 0

#### Text

cannot be simplified

### Answer 4

- Weight: 0

#### Text

replace it with `f()`

#### Feedback

This can't be right, since `f()` will neither return early on `None` nor
unwrap on `Some`.

### Answer 5

- Weight: 0

#### Text

remove the comma after the last `x`

#### Feedback

Final trailing commas are not only legal in Rust, but considered
good style.

## Question 9

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

What is the most general trait for the closure here?

```rust
let v = vec![0];
|y: i32| {
    let x = v[0];
    drop(v);
    x + y
}
```

### Answer 1

- Weight: 100

#### Text

`FnOnce(i32) -> i32`

#### Feedback

Calling the closure twice would result in a double-free
error, since `v` is dropped after the first call. Therefore,
the closure can only be called once, so it implements
`FnOnce`, but not `FnMut` or `Fn`.

### Answer 2

- Weight: 0

#### Text

`FnMut(i32) -> i32`

### Answer 3

- Weight: 0

#### Text

`Fn(i32) -> i32`

## Question 10

- Name: Question
- Type: multiple choice
- Points: 6

### Prompt

Consider the following code.

```rust
let xs: Vec<i32> = vec![];
// ... fill xs with some numbers ...
let ys = xs.iter().map(|i| i + 1);
// ... do some more stuff ...
```

What is the type of the parameter `i`?

### Answer 1

- Weight: 100

#### Text

`&i32`

### Answer 2

- Weight: 0

#### Text

`i32`

### Answer 3

- Weight: 0

#### Text

`&mut i32`

### Answer 4

- Weight: 0

#### Text

`Vec<i32>`

## Question 11

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

In the code from the previous question, why is the expression
`i + 1` not a type error?

### Answer 1

- Weight: 100

#### Text

the standard library implements `Add<i32> for &i32`

#### Feedback

If you ever implement `Add` for one of your own types, you will
find that it is not automatically implemented for references: you
have to implement it manually.

### Answer 2

- Weight: 0

#### Text

all operators automatically work on references

### Answer 3

- Weight: 0

#### Text

the operator `+` automatically works on references

### Answer 4

- Weight: 0

#### Text

there is a type error

## Question 12

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

Consider the following code.

```rust
struct Person {
    age: i32,
}

impl Person {
    fn adult_age(&self) -> i32 {
        self.age - 18
    }
}
```

What is the type of `self`?

### Answer 1

- Weight: 100

#### Text

`&Person`

### Answer 2

- Weight: 0

#### Text

`Person`

### Answer 3

- Weight: 0

#### Text

`i32`

## Question 13

- Name: Question
- Type: multiple choice
- Points: 6

### Prompt

In the code from the previous question, why is `self.age` not a
type error?

### Answer 1

- Weight: 100

#### Text

the operator `.` automatically works on references

### Answer 2

- Weight: 0

#### Text

all operators automatically work on references

### Answer 3

- Weight: 0

#### Text

`self` has type `Person`, which has an `age` field

### Answer 4

- Weight: 0

#### Text

there is a type error

## Question 14

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

Why will the following code fail to compile?

```rust
fn fib(i: i32) -> i32 {
    match i {
        0 => 0,
        1 => 1,
        2..=i32::MAX => fib(i - 1) + fib(i - 2),
    }
}
```

### Answer 1

- Weight: 100

#### Text

the `match` is not exhaustive

#### Feedback

When the `match` argument is an `i32`, all possible `i32` values
must be handled. This `match` misses the negative numbers.

### Answer 2

- Weight: 0

#### Text

ranges cannot be used as patterns in a `match` expression

### Answer 3

- Weight: 0

#### Text

variables like `i32::MAX` cannot be used as patterns in a `match`
expression

#### Feedback

Variables cannot be used, but constant symbols are okay.

### Answer 4

- Weight: 0

#### Text

no problems

## Question 15

- Name: Question
- Type: multiple choice
- Points: 7

### Prompt

Could the following code be made more general?

```rust
fn swap_ref_pair<'a>(p: (&'a i32, &'a i32)) -> (&'a i32, &'a i32) {
    let (x, y) = p;
    (y, x)
}
```

### Answer 1

- Weight: 100

#### Text

add an additional lifetime to the type signature

#### Feedback

Specifically, the signature should become

```rust
fn swap_ref_pair<'a, 'b>(p: (&'a i32, &'b i32)) -> (&'b i32, &'a i32)
```

### Answer 2

- Weight: 0

#### Text

remove the lifetimes from the type signature

#### Feedback

When exactly one input lifetime occurs, an elided output lifetime
is inferred from it. Here the two input references can have
independent lifetimes, so the output lifetimes must be written
explicitly.

### Answer 3

- Weight: 0

#### Text

no improvements necessary
