# Chomp
Bart Massey 2023

This code plays the game of
[Chomp](https://en.wikipedia.org/wiki/Chomp).  It comprises
a library containing a perfect AI player, together with a
command-line program to play against it.

Run with `cargo run -- r c` where `r` is the number of rows and
`c` the number of columns. The maximum is four rows and five
columns. For a three-row, two-column game:

```sh
cargo run -- 3 2
```

Enter each move as a zero-based row followed by a column,
separated by a space. You move first. The program prints the
board before each human turn and after each human move, and
labels the computer's moves. A move at `0 0` eats poison and
loses. Invalid arguments or input end the program with an error.

The library implements the skeleton's public API, including
`ChompError`, fallible construction and moves, and queries for
dimensions and uneaten squares. Board storage is private.

Check the solution with:

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

# License

This work is licensed under the "MIT License". Please see
`LICENSE.txt` in this distribution for license terms.
