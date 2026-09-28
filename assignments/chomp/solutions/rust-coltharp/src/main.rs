//! Command-line Chomp player
//! Your Name and Bart Massey 2023
//!
//! This player repeatedly
//! * Displays the board
//! * Prompts the human for a move until a legal move is obtained
//! * Makes the human move on the board
//! * Displays the board
//! * Gets a winning computer move from the AI
//! * If the AI has no winning move, chooses a
//!   random computer move
//! * Makes the computer move on the board
//! * Displays the computer move
//! This continues until the game is over,
//! at which point either "you lose" or "you win"
//! is printed depending on the outcome.

use chomp_ai::*;
use prompted::input;
use rand::prelude::*;

/// Display the current board. This should produce output in this format:
///
///    #####
///    #####
///    ####.
///    #....
///
fn show_posn(posn: &Chomp) {
    for row in posn.board.iter().take(posn.nrows) {
        for &square in row.iter().take(posn.ncols) {
            print!("{}", if square { "#" } else { "." });
        }
        println!();
    }
    println!();
}

/// Get a move from the human player. The human should
/// supply the move as a row and column (starting from 0)
/// separated by a space, like this.
///
///    2 3
///
/// If the human makes a "bad" move (badly formatted or
/// illegal), this function returns `None`. Otherwise it
/// returns `Some` row and column coordinates of the human
/// move.
fn user_move(posn: &Chomp) -> Option<(usize, usize)> {
    let s = input!("Choose a move: ");
    let mut s = s.chars();
    let row = s.next()?.to_string().parse().ok()?;
    if s.next()? != ' ' {
        return None;
    }
    let col = s.next()?.to_string().parse().ok()?;
    if s.next().is_some() || row >= posn.nrows || col >= posn.ncols || !posn.square(row, col) {
        return None;
    }
    Some((row, col))
}

fn ai_move(posn: &Chomp) -> (usize, usize) {
    match posn.winning_move() {
        Some((row, col)) => (row, col),
        None => {
            let mut rng = thread_rng();
            loop {
                let row = rng.gen_range(0..posn.nrows);
                let col = rng.gen_range(0..posn.ncols);
                if posn.square(row, col) {
                    break (row, col);
                }
            }
        }
    }
}

/// Play a game, as described above.
fn main() {
    let mut posn = Chomp::new(4, 4);
    loop {
        show_posn(&posn);
        let (row, col) = loop {
            match user_move(&posn) {
                Some(mv) => break mv,
                _ => println!("invalid move"),
            }
        };
        posn.make_move(row, col);
        show_posn(&posn);
        if !posn.square(0, 0) {
            println!("you lose");
            break;
        }
        let (row, col) = ai_move(&posn);
        posn.make_move(row, col);
        if !posn.square(0, 0) {
            println!("you win");
            break;
        }
    }
}
