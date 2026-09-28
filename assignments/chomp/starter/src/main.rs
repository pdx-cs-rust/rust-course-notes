//! Command-line Chomp player
//! Your Name and Bart Massey 2023
//!
//! This player repeatedly
//! * Displays the board
//! * Prompts the human for a move until a legal move is obtained
//! * Makes the human move on the board
//! * Displays the board
//! * Gets a winning computer move from the AI
//!   * If the AI has no winning move, chooses a computer move
//!     by going to the last available row and eating the last
//!     available square in that row
//! * Makes the computer move on the board
//! * Displays the computer move
//! This continues until the game is over,
//! at which point either "you lose" or "you win"
//! is printed depending on the outcome.

use chomp_ai::*;
use prompted::input;

/// Display the current board. This should produce output in this format:
///
///    #####
///    #####
///    ####.
///    #....
///
fn show_posn(posn: &Chomp) {
    todo!()
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
    todo!()
}

/// Play a game, as described above.
///
/// The program should take two command-line arguments
/// representing the board size: a number of rows and a
/// number of columns for the board. The program should fail
/// (somehow) if the requested board size is too large or
/// negative or not numbers etc.
///
/// Thus, a typical run of the program on a 3×4 board might look like
/// ```text
/// cargo run 3 4
/// ```
fn main() {
    todo!()
}
