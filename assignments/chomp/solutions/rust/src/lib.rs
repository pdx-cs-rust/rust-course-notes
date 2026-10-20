//! Chomp AI
//! Bart Massey 2023

/// Maximum number of rows the AI can handle.
const MAX_ROWS: usize = 4;
/// Maximum number of columns the AI can handle.
const MAX_COLS: usize = 5;

/// An invalid board size or move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChompError {
    /// Board dimensions are zero or exceed the supported limits.
    InvalidSize {
        /// The requested number of rows.
        nrows: usize,
        /// The requested number of columns.
        ncols: usize,
    },
    /// A move is outside the board or selects an eaten square.
    InvalidMove {
        /// The requested row, starting from zero.
        row: usize,
        /// The requested column, starting from zero.
        col: usize,
    },
}

impl std::fmt::Display for ChompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSize { nrows, ncols } => {
                write!(f, "invalid board size: {nrows} rows, {ncols} columns")
            }
            Self::InvalidMove { row, col } => {
                write!(
                    f,
                    "invalid move at ({row}, {col}): expected an uneaten square"
                )
            }
        }
    }
}

impl std::error::Error for ChompError {}

/// A Chomp board.
///
/// Clones must have the same dimensions and squares, with independent
/// state: making a move on a clone must not change the original.
#[derive(Debug, Clone)]
pub struct Chomp {
    /// The logical number of rows.
    nrows: usize,
    /// The logical number of columns.
    ncols: usize,
    /// Uneaten squares, indexed by row and column.
    board: [[bool; MAX_COLS]; MAX_ROWS],
}

impl Chomp {
    /// Make a new Chomp board with the given size for this game.
    /// All squares within the requested dimensions start uneaten.
    ///
    /// # Errors
    /// Returns [`ChompError::InvalidSize`] unless `nrows` is in
    /// `1..=4` and `ncols` is in `1..=5`. Invalid sizes must not panic.
    pub fn new(nrows: usize, ncols: usize) -> Result<Self, ChompError> {
        if nrows == 0 || ncols == 0 || nrows > MAX_ROWS || ncols > MAX_COLS {
            return Err(ChompError::InvalidSize { nrows, ncols });
        }
        Ok(Self {
            nrows,
            ncols,
            board: [[true; MAX_COLS]; MAX_ROWS],
        })
    }

    /// Return the logical board dimensions as `(nrows, ncols)`.
    /// These are the original dimensions, even after squares are eaten.
    /// This method must not change the board.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    /// Return whether the square at `(row, col)` is uneaten.
    /// Coordinates start from zero, with `(0, 0)` at the upper left.
    /// Return `false` for eaten squares and for coordinates outside the
    /// logical dimensions, including arbitrarily large coordinates.
    /// This method must not change the board or panic.
    pub fn is_present(&self, row: usize, col: usize) -> bool {
        row < self.nrows && col < self.ncols && self.board[row][col]
    }

    /// Make a move on the current board, "eating" all cells
    /// below `row` and to the right of `col` inclusive.
    /// That is, remove each square whose row is at least `row` and whose
    /// column is at least `col`. Leave all other squares unchanged.
    /// The logical dimensions do not change.
    /// Returns `Ok(())` after a valid move. Eating the poison square
    /// at `(0, 0)` is a valid move that ends the game.
    ///
    /// # Errors
    /// Returns [`ChompError::InvalidMove`] if the coordinates are
    /// outside the logical board or the selected square is already
    /// eaten. This includes every move after the poison is eaten.
    /// Invalid moves must leave the board unchanged and must not panic.
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), ChompError> {
        if !self.is_present(0, 0) || !self.is_present(row, col) {
            return Err(ChompError::InvalidMove { row, col });
        }
        for r in row..self.nrows {
            for c in col..self.ncols {
                self.board[r][c] = false;
            }
        }
        Ok(())
    }

    /// Returns `Some` winning move for this position as `(row, col)`.
    /// Returns `None` if there is no winning move in this position.
    /// This includes losing positions, a board containing only the
    /// poison square, and any board where the poison is already eaten.
    /// Any winning move is acceptable when several exist. A winning
    /// move selects an uneaten square other than `(0, 0)` and leaves
    /// the opponent without a winning move.
    /// This method must not change the board.
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
        if !self.is_present(0, 0) {
            return None;
        }
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                if (row, col) == (0, 0) || !self.is_present(row, col) {
                    continue;
                }
                let mut next = self.clone();
                if next.make_move(row, col).is_ok() && next.winning_move().is_none() {
                    return Some((row, col));
                }
            }
        }
        None
    }
}

#[test]
fn test_winning_move() -> Result<(), ChompError> {
    let mut c = Chomp::new(2, 2)?;
    assert!(c.winning_move().is_some());
    c.make_move(1, 1)?;
    assert!(c.winning_move().is_none());
    Ok(())
}

#[test]
fn test_invalid_sizes() {
    for (nrows, ncols) in [(0, 1), (1, 0), (0, 0), (5, 1), (1, 6), (usize::MAX, 1)] {
        assert!(matches!(
            Chomp::new(nrows, ncols),
            Err(ChompError::InvalidSize { .. })
        ));
    }
}

#[test]
fn test_initial_boards() -> Result<(), ChompError> {
    for nrows in 1..=MAX_ROWS {
        for ncols in 1..=MAX_COLS {
            let board = Chomp::new(nrows, ncols)?;
            assert_eq!(board.dimensions(), (nrows, ncols));
            for row in 0..=MAX_ROWS {
                for col in 0..=MAX_COLS {
                    assert_eq!(board.is_present(row, col), row < nrows && col < ncols);
                }
            }
            assert!(!board.is_present(usize::MAX, 0));
            assert!(!board.is_present(0, usize::MAX));
        }
    }
    Ok(())
}

#[test]
fn test_move_effects_and_errors() -> Result<(), ChompError> {
    let mut board = Chomp::new(3, 4)?;
    board.make_move(1, 2)?;
    for (row, col) in [(3, 0), (0, 4), (2, 3), (usize::MAX, 0), (0, usize::MAX)] {
        assert_eq!(
            board.make_move(row, col),
            Err(ChompError::InvalidMove { row, col })
        );
    }
    assert_eq!(board.dimensions(), (3, 4));
    for row in 0..3 {
        for col in 0..4 {
            assert_eq!(board.is_present(row, col), row < 1 || col < 2);
        }
    }
    Ok(())
}

#[test]
fn test_independent_clones() -> Result<(), ChompError> {
    let mut original = Chomp::new(2, 3)?;
    let mut copy = original.clone();
    copy.make_move(1, 2)?;
    assert!(original.is_present(1, 2));
    assert!(!copy.is_present(1, 2));
    original.make_move(0, 1)?;
    assert!(!original.is_present(0, 1));
    assert!(copy.is_present(0, 1));
    assert_eq!(copy.dimensions(), original.dimensions());
    Ok(())
}

#[test]
fn test_poison_and_finished_boards() -> Result<(), ChompError> {
    let mut board = Chomp::new(3, 2)?;
    board.make_move(0, 0)?;
    assert_eq!(board.winning_move(), None);
    assert_eq!(board.dimensions(), (3, 2));
    for row in 0..3 {
        for col in 0..2 {
            assert!(!board.is_present(row, col));
            assert_eq!(
                board.make_move(row, col),
                Err(ChompError::InvalidMove { row, col })
            );
        }
    }
    let mut poison = Chomp::new(1, 1)?;
    assert_eq!(poison.winning_move(), None);
    assert!(poison.is_present(0, 0));
    poison.make_move(0, 0)?;
    assert!(!poison.is_present(0, 0));
    Ok(())
}

#[test]
fn test_winning_move_preserves_board() -> Result<(), ChompError> {
    let board = Chomp::new(2, 2)?;
    assert_eq!(board.winning_move(), Some((1, 1)));
    for row in 0..2 {
        for col in 0..2 {
            assert!(board.is_present(row, col));
        }
    }
    for ncols in 2..=MAX_COLS {
        assert_eq!(Chomp::new(1, ncols)?.winning_move(), Some((0, 1)));
    }
    for nrows in 2..=MAX_ROWS {
        assert_eq!(Chomp::new(nrows, 1)?.winning_move(), Some((1, 0)));
    }
    Ok(())
}
