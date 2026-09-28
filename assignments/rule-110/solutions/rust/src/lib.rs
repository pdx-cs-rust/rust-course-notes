//! CA runner lib — Bart Massey 2025
//!
//! Generalized library for <https://github.com/pdx-cs-rust/hw-rule110>.

extern crate thiserror;

/// Rule Errors
#[derive(Debug, thiserror::Error)]
pub enum RuleError {
    #[error("start row has no characters")]
    EmptyStartRow,
    #[error("start row exceeds 64 characters")]
    OverlongStartRow,
    #[error("bad character {0} in start row")]
    BadStartChar(char),
}

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
        let mask = (1u64 << nbits).wrapping_sub(1);
        let next = ((cur ^ right) | (!left & cur)) & mask;

        Row { bits: next, nbits: self.nbits }
    }
}

#[test]
fn test_rows() {
    let rows = [
        "*.*..*..",
        "***.**.*",
        "..******",
        ".**....*",
        "***...**",
        "..*..**.",
        ".**.***.",
        "*****.*.",
        "*...****",
        "*..**...",
    ];

    for (i, (&r1, &r2)) in rows.iter().zip(rows[1..].iter()).enumerate() {
        let cur = Row::make_start(r1).unwrap();
        let expected = Row::make_start(r2).unwrap();
        let got = cur.next_row();
        assert_eq!(
            expected, got,
            "{}: {} != {}",
            i, expected.row_string(), got.row_string(),
        );
    }
}
