//! CA runner lib — Bart Massey 2025
//!
//! Generalized library for <https://github.com/pdx-cs-rust/hw-rule110>.

/// Rule Errors
#[derive(Debug, PartialEq, Eq)]
pub enum RuleError {
    EmptyStartRow,
    OverlongStartRow,
    BadStartChar(char),
}

impl std::fmt::Display for RuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyStartRow => write!(f, "start row has no characters"),
            Self::OverlongStartRow => write!(f, "start row exceeds 64 characters"),
            Self::BadStartChar(c) => write!(f, "bad character {c} in start row"),
        }
    }
}

impl std::error::Error for RuleError {}

/// A row is a sequence of bits.
#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    bits: u64,
    nbits: usize,
}

impl Row {
    /// Given a starting row description string, make and return
    /// the corresponding row.
    pub fn make_start(row: &str) -> Result<Self, RuleError> {
        let nbits = row.chars().count();
        if nbits == 0 {
            return Err(RuleError::EmptyStartRow);
        }
        if nbits > 64 {
            return Err(RuleError::OverlongStartRow);
        }

        let mut bits = 0;
        for c in row.chars() {
            match c {
                '*' => {
                    bits <<= 1;
                    bits |= 1;
                }
                '.' => {
                    bits <<= 1;
                }
                c => return Err(RuleError::BadStartChar(c)),
            }
        }

        let row = Self { bits, nbits };
        Ok(row)
    }

    /// Produce the renderable string for the row.
    pub fn row_string(&self) -> String {
        (0..self.nbits)
            .rev()
            .map(|i| {
                if ((self.bits >> i) & 1) == 1 {
                    '*'
                } else {
                    '.'
                }
            })
            .collect()
    }

    /// Use CA Rule 110 to make a new row.
    #[inline(always)]
    pub fn next_row(&self) -> Row {
        let nbits = self.nbits as u32;

        let cur = self.bits;
        let left = (cur >> 1) | (cur << (nbits - 1));
        let right = (cur << 1) | (cur >> (nbits - 1));
        let mask = u64::MAX >> (u64::BITS - nbits);
        let next = ((cur ^ right) | (!left & cur)) & mask;

        Row {
            bits: next,
            nbits: self.nbits,
        }
    }
}

#[test]
fn test_rows() {
    let rows = [
        "*.*..*..", "***.**.*", "..******", ".**....*", "***...**", "..*..**.", ".**.***.",
        "*****.*.", "*...****", "*..**...",
    ];

    for (i, (&r1, &r2)) in rows.iter().zip(rows[1..].iter()).enumerate() {
        let cur = Row::make_start(r1).unwrap();
        let expected = Row::make_start(r2).unwrap();
        let got = cur.next_row();
        assert_eq!(
            expected,
            got,
            "{}: {} != {}",
            i,
            expected.row_string(),
            got.row_string(),
        );
    }
}

#[test]
fn test_full_width_row() {
    let cur = Row::make_start(&"*.*..*..".repeat(8)).unwrap();
    let expected = Row::make_start(&"***.**.*".repeat(8)).unwrap();
    assert_eq!(cur.next_row(), expected);
}

#[test]
fn test_all_short_rows_against_direct_rule() {
    fn direct_next(row: &Row) -> Row {
        const RULE110: [bool; 8] = [false, true, true, true, false, true, true, false];

        let nbits = row.nbits;
        let mut bits = 0;
        for center in 0..nbits {
            let left = (row.bits >> ((center + 1) % nbits)) & 1;
            let middle = (row.bits >> center) & 1;
            let right = (row.bits >> ((center + nbits - 1) % nbits)) & 1;
            let neighborhood = (left << 2) | (middle << 1) | right;
            if RULE110[neighborhood as usize] {
                bits |= 1 << center;
            }
        }
        Row { bits, nbits }
    }

    for nbits in 1..=10 {
        for bits in 0..1 << nbits {
            let row = Row { bits, nbits };
            assert_eq!(row.next_row(), direct_next(&row));
        }
    }
}

#[test]
fn test_errors() {
    assert_eq!(Row::make_start(""), Err(RuleError::EmptyStartRow));
    assert_eq!(
        Row::make_start(&".".repeat(65)),
        Err(RuleError::OverlongStartRow)
    );
    assert_eq!(Row::make_start(".*x*."), Err(RuleError::BadStartChar('x')));

    for (error, expected) in [
        (RuleError::EmptyStartRow, "start row has no characters"),
        (
            RuleError::OverlongStartRow,
            "start row exceeds 64 characters",
        ),
        (RuleError::BadStartChar('x'), "bad character x in start row"),
    ] {
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }
}
