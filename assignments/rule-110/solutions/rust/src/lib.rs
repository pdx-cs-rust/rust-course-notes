//! CA runner lib — Bart Massey 2025
//!
//! Generalized library for <https://github.com/pdx-cs-rust/hw-rule110>.

/// Rule Errors
#[derive(Debug)]
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
fn test_error_messages() {
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
