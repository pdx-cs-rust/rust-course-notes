//! Command-line Chomp player
//! Bart Massey 2023
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
//!
//! This continues until the game is over,
//! at which point either "you lose" or "you win"
//! is printed depending on the outcome.

use chomp_ai::*;

/// Display the current board. This should produce output in this format:
///
///    #####
///    #####
///    ####.
///    #....
///
fn show_posn(posn: &Chomp) {
    for r in 0..posn.nrows {
        for c in 0..posn.ncols {
            if posn.board[r][c] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
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
    let mut m = String::new();
    std::io::stdin().read_line(&mut m).unwrap();
    let fields: Vec<&str> = m.split_whitespace().collect();
    if fields.len() != 2 {
        return None;
    }
    let r: usize = fields[0].parse().ok()?;
    let c: usize = fields[1].parse().ok()?;
    if r >= posn.nrows || c >= posn.ncols || !posn.board[r][c] {
        None
    } else {
        Some((r, c))
    }
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
    let argv: Vec<String> = std::env::args().collect();
    assert!(argv.len() == 3);
    let nrows = argv[1].parse().unwrap();
    let ncols = argv[2].parse().unwrap();
    let mut posn = Chomp::new(nrows, ncols);
    loop {
        show_posn(&posn);
        let (r, c) = loop {
            let m = user_move(&posn);
            if let Some(m) = m {
                break m;
            }
            println!("bad move: try again");
        };
        posn.make_move(r, c);
        show_posn(&posn);
        if !posn.board[0][0] {
            println!("you lose");
            return;
        }

        if let Some((r, c)) = posn.winning_move() {
            posn.make_move(r, c);
            println!("{} {}", r, c);
        } else {
            let mut r = 0;
            for r0 in 0..posn.nrows {
                if posn.board[r0][0] {
                    r = r0;
                } else {
                    break;
                }
            }
            let mut c = 0;
            for c0 in 0..posn.ncols {
                if posn.board[r][c0] {
                    c = c0;
                } else {
                    break;
                }
            }
            posn.make_move(r, c);
            println!("{} {}", r, c);
        };
        if !posn.board[0][0] {
            println!("you win");
            return;
        }
    }
}
