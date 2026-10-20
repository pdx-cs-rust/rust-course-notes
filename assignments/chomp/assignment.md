# HW: Chomp

## Background

Chomp is a very simple game played with a chocolate bar. This
[Wikipedia article](https://en.wikipedia.org/wiki/Chomp)
describes the rules of the game.

Chomp is a good game to build a computer player for. Chomp is a
two-player perfect-information terminating game, so a
straightforward implementation of the Negamax algorithm can play
perfectly on small boards.

The basic idea is that in a given position a best move can be
found by exploring all possible moves, all possible responses,
etc. The resulting move selected is one that gives the opponent
the worst possible end-of-game result.

In this assignment, you will write a library crate that
implements a Chomp AI (`src/lib.rs`), and a binary crate that
allows the game to be played on the command line (`src/main.rs`).

You must start from the [provided skeleton][skeleton] and
complete its library and command-line program. Keep the public
type names and method signatures so your work can be tested
automatically. Add fields to `Chomp` to represent the game state.
You may add helper functions and methods.

[skeleton]: https://github.com/pdx-cs-rust/hw-chomp-skeleton

## The library

The game state should be represented by a `struct Chomp` you
define. The skeleton supplies a unit struct as a placeholder:
replace it with a struct containing the fields you need. The
most important component of the state is the representation of
the board itself:

- One way: use an array

  - Use a fixed-size five-by-four (five columns, four rows) array
    of `bool`s to represent the game state. The element at index
    `[row][column]` should be `false` if that square has been
    eaten, and `true` otherwise.

  - Your program should support multiple board sizes. Since the
    size of a Rust array is fixed at compile time, your board
    type needs to have fields representing its logical width and
    height. Ignore squares outside these logical dimensions.

- Another way: use a set

  - You can make a `HashSet` of tuples, where tuple `(i, j)`
    being in the set means that position is not yet eaten. You
    can remove elements from the set as they are eaten.

Your `Chomp` type should support the following operations via
`impl`:

- Create a board with a given number of rows and columns,
  returning `Result<Chomp, ChompError>`. Return
  `ChompError::InvalidSize` for dimensions outside `1..=4` rows
  or `1..=5` columns.

- Print a graphical representation of a board.

- Chomp a given square, removing all squares below it and to the
  right of it. Return `Result<(), ChompError>`: `Ok(())` for a
  valid move, or `ChompError::InvalidMove` for coordinates
  outside the board or an eaten square. Invalid moves must leave
  the board unchanged. Eating the poison square is valid, but
  ends the game; all later moves are invalid.

- Return a winning move for the board, if any (see the next
  section). Return `None` for losing positions, including a
  board containing only poison, or one where poison has already
  been eaten.

- Clone a board (the `Clone` trait).

### The AI

The negamax algorithm solves any zero-sum perfect-information
two-player game (like Chomp). It takes as input a board state and
outputs a winning move, if one exists.

```text
winning-move(posn):
    for each remaining row r
        for each remaining column c in r
            if r = 0 and c = 0
                continue
            p ← copy of posn
            chomp r, c from p
            m ← winning-move(p)
            if no winning move is returned
                return the move r, c
    return no winning move
```

Understand this pseudocode as follows:

1. Check whether the board state is already lost. If so, then
   there is no winning move.

2. Otherwise, for each possible move `m`:

   1. Create a new board `p`.

   2. Perform the move `m` on `p`.

   3. Call `winning_move` recursively at `p`. (Since one player
      has just made a move, we are now trying to find a winning
      move for the *other* player.)

   4. If `winning_move` outputs a winning move for `p`, then `m`
      is *not* a winning move for the current player. (Why?)
      Continue on to the next move.

   5. Otherwise, `m` is a winning move. Return it.

For Chomp:

- The board state is lost if the upper-left square is the only
  one left.

- You can represent the move “chomp at a row and column” by a
  tuple `(row, column)`. There is one such possible move for each
  uneaten square (other than the top left one).

## The program

Your program will be a simple terminal interface for playing
Chomp against your AI. It should perform the following sequence:

1. Accept two command-line arguments for the board size: rows,
   then columns, as specified in the skeleton.

2. Repeatedly:

   1. Print the board.

   2. Ask the user to input a move and perform it.

   3. If the user ate the poison square, announce that the user
      lost and end the game.

   4. Try to find a winning move. If there is one, perform it.
      Otherwise, stall by chomping as little as possible. (You
      can implement this by chomping the furthest-right piece in
      the lowermost nonempty row.)

   5. Show the computer's move. If the computer ate the poison
      square, announce that the user won and end the game.

If the user ever gives invalid input, simply ask again until they
give valid input.

The game ends as soon as the upper-left poison square is eaten.
The player who eats that square loses.

## Hints

- The `prompted` crate from `crates.io` may be useful to get a
  line of user input.

- `Chomp::new` and `make_move` must return a `Result` with the
  appropriate `ChompError` on invalid input. Handle these errors
  without panicking. The skeleton provides the error type.

- Your `winning_move` function should return some kind of
  `Option`, since there may not be a winning move.

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
- Nothing else. Not your git repo. None of the funny Mac garbage
  files. Not your `target/` directory.

Name your ZIP archive `chomp.zip` (exactly that, in
lowercase).
