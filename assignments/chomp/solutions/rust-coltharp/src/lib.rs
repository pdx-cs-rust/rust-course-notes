//! Chomp AI
//! Your Name and Bart Massey 2023

/// Maximum number of rows the AI can handle.
pub const MAX_ROWS: usize = 4;
/// Maximum number of columns the AI can handle.
pub const MAX_COLS: usize = 5;

/// A Chomp board.
#[derive(Debug, Clone)]
pub struct Chomp {
    /// The number of rows for this game.
    pub nrows: usize,

    /// The number of columns for this game.
    pub ncols: usize,

    /// The board. `true` is an un-eaten square, `false is
    /// an eaten square.
    pub board: [[bool; MAX_COLS]; MAX_ROWS],
}

impl Chomp {
    /// Make a new Chomp board with the given size for this game.
    ///
    /// # Panics
    /// Panics if the requested board size is larger than the AI
    /// can handle, or has zero rows or columns.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        assert!(nrows > 0, "not enough rows to play");
        assert!(ncols > 0, "not enough columns to play");
        assert!(
            nrows <= MAX_ROWS,
            "too many rows ({} > {}) for AI",
            nrows,
            MAX_ROWS
        );
        assert!(
            ncols <= MAX_COLS,
            "too many columns ({} > {}) for AI",
            ncols,
            MAX_COLS
        );
        let board = [[true; MAX_COLS]; MAX_ROWS];
        Self {
            nrows,
            ncols,
            board,
        }
    }

    pub fn square(&self, row: usize, col: usize) -> bool {
        // Type inference seems to poop out sometimes when double-indexing; use
        // this method to get around that.
        self.board[row][col]
    }

    /// Make a move on the current board, "eating" all cells
    /// below `row` and to the right of `col` inclusive.
    pub fn make_move(&mut self, row: usize, col: usize) {
        for row in self.board.iter_mut().skip(row) {
            row[col..].fill(false);
        }
    }

    /// Returns `Some` winning move for this position as `(row, col)`.
    /// Returns `None` if there is no winning move in this position.
    ///
    /// # Strategy
    ///
    /// ```text
    /// winning-move(posn):
    ///     for each remaining row r
    ///         for each remaining column c in r
    ///             if r = 0 and c = 0
    ///                 continue
    ///             p ← copy of posn
    ///             chomp r, c from p
    ///             m ← winning-move(p)
    ///             if no winning move is returned
    ///                 return the move r, c
    ///    return no winning move
    /// ```
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        for (i, row) in self.board.iter().take(self.nrows).enumerate() {
            for j in row
                .iter()
                .take(self.ncols)
                .take_while(|&&square| square)
                .enumerate()
                .map(|p| p.0)
            {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut board = self.clone();
                board.make_move(i, j);
                if board.winning_move().is_none() {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

#[test]
fn test_winning_move() {
    let mut c = Chomp::new(2, 2);
    assert!(c.winning_move().is_some());
    c.make_move(1, 1);
    assert!(c.winning_move().is_none());
}
