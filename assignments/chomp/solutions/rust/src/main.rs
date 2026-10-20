//! Command-line Chomp player
//! Bart Massey 2023
//!
//! The human moves first. Display the board, read a human move, and
//! respond with a winning computer move when possible. If no winning
//! move exists, eat the rightmost square in the lowest nonempty row.
//! Stop immediately when either player eats poison and announce the
//! outcome from the human's perspective.

use chomp_ai::Chomp;
use std::io::{self, Write};

/// Display the board using `#` for uneaten squares and `.` for eaten ones.
fn show_posn(posn: &Chomp) {
    let (nrows, ncols) = posn.dimensions();
    for row in 0..nrows {
        for col in 0..ncols {
            if posn.is_present(row, col) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/// Read a zero-based row and column, separated by whitespace.
/// The library validates the resulting move when it is made.
///
/// # Errors
/// Return an error if input cannot be read or parsed as two coordinates.
fn user_move() -> Result<(usize, usize), Box<dyn std::error::Error>> {
    print!("Your move (row col): ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let fields: Vec<_> = line.split_whitespace().collect();
    if fields.len() != 2 {
        return Err("expected a row and column".into());
    }
    Ok((fields[0].parse()?, fields[1].parse()?))
}

/// Choose a winning move or fall back to the last remaining square.
/// Return `None` after the game is over.
fn computer_move(posn: &Chomp) -> Option<(usize, usize)> {
    if !posn.is_present(0, 0) {
        return None;
    }
    if let Some(m) = posn.winning_move() {
        return Some(m);
    }
    let (nrows, ncols) = posn.dimensions();
    for row in (0..nrows).rev() {
        for col in (0..ncols).rev() {
            if posn.is_present(row, col) {
                return Some((row, col));
            }
        }
    }
    None
}

/// Play a game with two command-line arguments: rows, then columns.
///
/// ```text
/// cargo run -- 3 2
/// ```
///
/// # Errors
/// Return an error for invalid arguments, input, or library operations.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: chomp ROWS COLS".into());
    }
    let mut posn = Chomp::new(args[0].parse()?, args[1].parse()?)?;
    loop {
        show_posn(&posn);
        let (row, col) = user_move()?;
        posn.make_move(row, col)?;
        show_posn(&posn);
        if !posn.is_present(0, 0) {
            println!("You lose!");
            return Ok(());
        }

        let (row, col) = computer_move(&posn).ok_or("no computer move available")?;
        posn.make_move(row, col)?;
        println!("Computer: {row} {col}");
        if !posn.is_present(0, 0) {
            show_posn(&posn);
            println!("You win!");
            return Ok(());
        }
    }
}
