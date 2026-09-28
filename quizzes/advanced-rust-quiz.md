# Advanced Rust Quiz

## Settings

- Type: graded quiz
- Total points: 100
- Question count: 7
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
- IP filtering: no
- Only visible to assignment overrides: no
- Anonymous submissions: no

## Instructions

Answer these questions to the best of your ability. You may
have as many tries as you like. You may use AI, but I suggest
not doing so as you should try to understand the material.

## Question 1: Rayon

- Type: multiple answer
- Points: 15

### Prompt

Given this code:

```rust
let mut a: Vec<u32> = (0..1024).collect();
a.iter_mut().for_each(*v = v.pow(*v));
```

Things needed to make this compile and run successfully in
parallel using Rayon include (choose all that apply)

### Answer 1

- Weight: 100

#### Text

Add `use rayon::prelude::*;` to make Rayon available.

### Answer 2

- Weight: 0

#### Text

Rewrite `for_each()` to a `for` loop, since `for` loops are
parallel by default.

### Answer 3

- Weight: 100

#### Text

Replace `iter_mut()` with `par_iter_mut()`.

### Answer 4

- Weight: 100

#### Text

Fix the closure passed to `for_each()` to take an argument.

### Answer 5

- Weight: 0

#### Text

Use `pow_rayon()` instead of `pow()`.

### Answer 6

- Weight: 100

#### Text

Use `wrapping_pow()` instead of `pow()` to avoid overflow in
debug mode.

## Question 2: Thread parallelism

- Type: multiple answer
- Points: 15

### Prompt

Given this code:

```rust
use std::thread;

fn main() {
    let mut s: Vec<f32> = (0..48_000)
        .map(|i| i / 48_000.0)
        .collect();
    for section in s.chunks_mut(s.len() / 16) {
        thread::spawn(|section| {
            for v in section {
                *v = v.sin();
            }
        });
    }
}
```

Things needed to make this compile and run successfully in
parallel include (choose all that apply)

### Answer 1

- Weight: 100

#### Text

In the `map()` operation, cast `i` to `f32` so that the division
will compile.

### Answer 2

- Weight: 100

#### Text

Lift the `s.len()` computation above the `for` loop so that `s`
is not borrowed both mutably and immutably in the loop header.

### Answer 3

- Weight: 100

#### Text

Remove the `section` argument from the closure passed to
`thread::spawn()`: closures passed to `spawn()` accept no
arguments.

### Answer 4

- Weight: 0

#### Text

Get the join handle from `thread::spawn()` and join it before
the next loop iteration.

### Answer 5

- Weight: 100

#### Text

Use `thread::scope()` or similar to avoid thread lifetime
errors.

## Question 3: Async/Await

- Type: multiple answer
- Points: 15

### Prompt

Consider the following program:

```rust
use tokio::{io::AsyncReadExt, net, sync::mpsc};

async fn report_messages(mut receiver: mpsc::Receiver<char>) {
    loop {
        let ch = receiver.recv().await.unwrap();
        eprintln!("message: {}", ch);
    }
}

async fn read_messages(
    mut client: net::TcpStream,
    sender: mpsc::Sender<char>,
) {
    let mut buf = [0u8];
    loop {
        match client.read(&mut buf).await {
            Ok(1) => {
                if let Some(ch) = char::from_u32(buf[0] as u32) {
                    if ch.is_ascii_alphabetic() {
                        sender.send(ch).await.unwrap();
                    }
                }
            }
            _ => return,
        }
    }
}

#[tokio::main]
async fn main() -> ! {
    let (sender, receiver) = mpsc::channel(1);
    tokio::spawn(async move {
        report_messages(receiver).await
    });
    let listener = net::TcpListener::bind("127.0.0.1:12453")
        .await
        .unwrap();
    loop {
        let (client, _) = listener.accept().await.unwrap();
        let sender = sender.clone();
        tokio::spawn(async move {
            read_messages(client, sender).await
        });
    }
}
```

### Answer 1

- Weight: 0

#### Text

The return types of `report_messages()` and `main()` are `!`
because they return async values.

### Answer 2

- Weight: 100

#### Text

There are no extra or missing `.await`s in the code.

### Answer 3

- Weight: 0

#### Text

Each trip through the `main()` loop spawns a new thread.

### Answer 4

- Weight: 0

#### Text

Each trip through the `main()` loop will only complete when the
new client has closed its connection.

### Answer 5

- Weight: 0

#### Text

The expensive `sender.clone()` in `main()` could be replaced
with `&sender`, since shared references are copy.

## Question 4: Declarative Macros

- Type: multiple answer
- Points: 15

### Prompt

Consider the following macro:

```rust
static LOG: Mutex<String> = Mutex::new(String::new());

macro_rules! log {
    () => {
        LOG.lock().unwrap().push_str("\n");
    };
    ($e:expr) => {
        LOG.lock().unwrap().push_str(&format!("{}\n", $e));
    };
    ($e:expr, $($es:expr)*) => {
        LOG.lock().unwrap().push_str(&format!("{} ", $e));
        log!($($es)*);
    };
}
```

Things true of this program include (choose all that apply)

### Answer 1

- Weight: 100

#### Text

The macro can be safely invoked as `log!();`.

### Answer 2

- Weight: 100

#### Text

The macro can be safely invoked as `log!("hello");`.

### Answer 3

- Weight: 100

#### Text

The macro can be safely invoked as `log!(7, 9);`.

### Answer 4

- Weight: 0

#### Text

The macro could be replaced with a `log()` function with the
same interface.

### Answer 5

- Weight: 0

#### Text

This macro cannot work, since recursive macros are illegal.

## Question 5: Unsafe Code

- Type: multiple answer
- Points: 15

### Prompt

What will the Rust compiler let happen inside an `unsafe`
block that can directly cause undefined behavior (UB)?
(choose all that apply)

### Answer 1

- Weight: 100

#### Text

Dereference a null raw mutable pointer, as in this `unsafe`
code:

```rust
let p = std::ptr::null_mut::<u8>();
*p = 0;
```

### Answer 2

- Weight: 0

#### Text

Call an unsafe function or method.

### Answer 3

- Weight: 0

#### Text

Assign a value of the wrong type through a raw mutable pointer,
as in this code:

```rust
let p = std::ptr::null_mut::<u8>();
*p = "hello";
```

### Answer 4

- Weight: 0

#### Text

There is no unsafe operation possible in Rust: the type system
prevents it.

## Question 6: Foreign Function Interface (FFI)

- Type: multiple answer
- Points: 15

### Prompt

Things that are true of Rust's C Foreign Function Interface
(FFI) include (choose all that apply)

### Answer 1

- Weight: 100

#### Text

Rust functions can be called from C.

### Answer 2

- Weight: 100

#### Text

C functions can be called from Rust.

### Answer 3

- Weight: 100

#### Text

Rust types declared with `#[repr(C)]` can be passed between
Rust and C when their fields are also FFI-compatible.

### Answer 4

- Weight: 0

#### Text

Rust FFI automatically converts C strings to Rust strings.

### Answer 5

- Weight: 0

#### Text

Rust will automatically allocate and deallocate memory on the C
heap as needed.

## Question 7: Memory Management

- Type: true or false
- Points: 10

### Prompt

Rust is a garbage-collected language.

### Answer 1

- Weight: 0

#### Text

True

### Answer 2

- Weight: 100

#### Text

False
