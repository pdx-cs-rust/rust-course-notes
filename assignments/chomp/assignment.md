# HW: Chomp

Complete the [Chomp library skeleton][skeleton] to build an AI
player, then write a command-line program for playing against it.

## Background

Chomp is a two-player game played with a chocolate bar. Players
take turns choosing an uneaten square and eating it along with
all squares below and to its right. The upper-left square is
poison: the player who eats it loses. This
[Wikipedia article](https://en.wikipedia.org/wiki/Chomp)
describes the game in more detail.

Chomp is a good game for a computer player. On small boards,
recursive search can explore the possible moves and responses
to find a winning move, if one exists.

## Getting started

You must use the skeleton for this assignment.

[skeleton]: https://github.com/pdx-cs-rust/hw-chomp-skeleton

Clone the skeleton repository or download it as a ZIP archive.
Work in its Cargo project, which contains:

- `src/lib.rs`: the `chomp_ai` library, including `Chomp`,
  `ChompError`, and the required public methods.
- `sample-gameplay.txt`: an example of a complete game.
- `Cargo.toml`, `Cargo.lock`, `README.md`, and `LICENSE.txt`.

Read the Rustdoc comments in `src/lib.rs` before starting.
They specify the library interface and behavior for automated
testing. You can also render the comments with:

```sh
cargo doc --document-private-items --open
```

Complete the `todo!()` placeholders. Keep the supplied public
library types, error variants, and method signatures so your
library can be tested automatically. You may add fields to
`Chomp` and add helper functions and methods. You will write
the CLI in a new `src/main.rs`, as described below.

The skeleton builds, but its game tests will not pass until you
implement the missing functionality. Add tests as you work;
the supplied tests do not cover everything.

## The library

Replace the placeholder unit struct `Chomp` with a struct that
stores the board state and its logical dimensions. Your library
must support boards with one to four rows and one to five
columns. Coordinates are zero-based, with `(0, 0)` at the upper
left, and always appear in row-then-column order.

Choose your own board representation. For example:

- Use a fixed-size `[[bool; 5]; 4]` array, with `true` for
  uneaten squares and `false` for eaten squares. Store the
  logical row and column counts separately, and ignore array
  entries outside those dimensions.
- Use a `HashSet<(usize, usize)>` containing the coordinates of
  uneaten squares, together with the logical dimensions.

Either representation must implement the same public interface:

- Complete `Chomp::new` to initialize an uneaten board. The
  skeleton already checks the size and returns `ChompError`
  for invalid dimensions.
- Implement `dimensions` and `is_present` so callers can inspect
  the board without knowing its representation.
- Implement `make_move` to validate and perform a move, returning
  the specified `Result`. Invalid moves must leave the board
  unchanged. Eating poison is a valid move that ends the game.
- Implement `winning_move` as described below.
- Keep `Chomp` cloneable. A clone must have independent state:
  moves on either board must not change the other.

Follow the comments for error conditions and return values.
Handle invalid sizes and moves with `ChompError`, without
panicking. Board dimensions do not change as squares are eaten.

### The AI

The skeleton includes pseudocode for a recursive winning-move
search. Understand the algorithm before implementing
`winning_move`:

1. Consider each uneaten square except the poison square.
2. Clone the board and make that move on the clone.
3. Recursively ask whether the opponent has a winning move
   from the resulting position.
4. If the opponent has no winning move, the move you just tried
   is winning. Return it. Otherwise, try another square.
5. If no move wins, return `None`.

Each recursive call considers a board with fewer uneaten
squares. A board containing only poison has no winning move,
and neither does a board where poison has already been eaten.

Your search must leave the original board unchanged. Any winning
move is acceptable when several exist; no particular search
order is required.

## The program

Create `src/main.rs` with a `main` function for a simple terminal
interface that lets a human play against your AI. Use your
library through `chomp_ai`, for example with
`use chomp_ai::Chomp;`. Cargo will detect the new file as the
`chomp` binary; no binary entry in `Cargo.toml` is needed.

Get the board size using prompts or command-line arguments.
For example, an interface using arguments might start a
three-row, four-column game with:

```sh
cargo run -- 3 4
```

The human moves first. Display the board, get a valid human move,
and then let the computer respond. Use a winning computer move
when one exists. Otherwise, choose another uneaten nonpoison
square if possible, or eat poison if it is the only square left.
The rightmost square in the lowest nonempty row is a simple
fallback. End the game when either player eats poison, and
announce who won.

Choose readable prompts, a clear board display, and an input
format that is easy to use. You can use `#` and `.` for uneaten
and eaten squares, or another display you prefer. A simple move
format is a zero-based row followed by a column, separated by
whitespace, such as `2 3`. Helpers for displaying the board and
reading a move may be useful; their interfaces are up to you.
You may assume the human follows your input instructions;
keep error handling simple.

Your CLI should produce a trace roughly like
`sample-gameplay.txt` in the skeleton: show the initial board,
prompt for human moves, identify computer moves, show the board
after each move, and announce the final result. Exact wording
and display layout may differ, but the trace should make it
easy to follow how the game progresses.

### Gameplay transcript

The library will be graded automatically. The CLI will be
reviewed by a human using a transcript of you playing your game.

Play at least one complete game that shows both human and
computer moves and the board changing. Save the terminal session
as plain text in `gameplay.txt`. Include the command used to run
the program, the board displays, your input, the computer's
moves, and the final result. Copying and pasting from your
terminal is fine. You do not need to win.

See `sample-gameplay.txt` in the skeleton for an example on a
three-row, two-column board.

The transcript should make it easy to see that your CLI lets
someone play a game. Note any known issues in your README.

## Requirements

*(These requirements are for every exercise in this course,
not just this one.)*

- You must create or update a `README.md` for the homework.
  This file will have

  - Your authorship information.
  - A description of the work: what it is/does.
  - How to build and run the work.
  - How you tested your work.
  - Comments on any issues you encountered.
  - Anything else a reader might find useful.

- Your crates must be adequately documented with Rustdoc comments
  for each top-level datatype, function and method.

- Your crates must build with current `stable` Rust.

- Your crates should contain adequate tests (implemented using
  `#[test]` unit-testing) and assertions (implemented using
  `assert!()` and related macros) for you to be comfortable that
  everything is working correctly.

  There is no minimum amount of testing required, but good tests
  can help you turn in a working solution.

- Your code must be formatted according to the official Rust
  formatting style --- use `cargo fmt` to reformat your code
  in-place.

- Your code must produce no warnings.

  - The compiler must produce no warnings.

  - `cargo clippy` must produce no warnings.

  - Do not disable warnings except in the most unusual
    circumstances: fix them instead.

## Submission

Please submit a ZIP archive containing:

- Your Cargo project directory, including `Cargo.toml`,
  `Cargo.lock`, `src/lib.rs`, and `src/main.rs`.
- Any other source or other text files that are necessary/useful.
- The `README.md` file in Markdown format described above.
- Your `gameplay.txt` transcript as described above.
- Nothing else. Not your git repo. None of the funny Mac garbage
  files. Not your `target/` directory.

Name your ZIP archive `chomp.zip` (exactly that, in
lowercase).
