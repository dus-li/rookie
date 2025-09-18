// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Duszku

/// Coordinatates of a square on a board.
///
/// Internally rookie uses Little-Endian, Rank-File mapping. That is:
/// * `A1` maps to `0`
/// * `B1` maps to `1`
/// * `A2` maps to `8`
#[derive(Debug)]
pub struct Loc(u8);

const BOARD_DIM: u8 = 8;

impl Loc {
    /// Creates new square coordinates.
    ///
    /// # Arguments
    ///
    /// * `rank`: Rank of the square, counted from zero (eg. 0, 3, 7)
    /// * `file`: File of the square, counted from zero (eg. 0, 4, 5)
    pub fn new(rank: u8, file: u8) -> Option<Self> {
        if (rank | file) >= 8 {
            return None;
        }

        Some(Loc(rank * BOARD_DIM + file))
    }

    /// Create new square coordinates from a LE-RF index.
    pub fn from_index(index: u8) -> Option<Self> {
        if index >= 64 {
            return None;
        }

        Some(Loc(index))
    }

    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn rank_file(&self) -> (u8, u8) {
        (self.0 / BOARD_DIM, self.0 % BOARD_DIM)
    }
}

/// Implements the display logic for `Loc`.
///
/// This makes board coordinates printable.
///
/// # Examples
///
/// ```
/// use rookie::loc::Loc;
///
/// assert_eq!(format!("{}", Loc::new(0, 0).unwrap()), "A1");
/// assert_eq!(format!("{}", Loc::new(0, 1).unwrap()), "B1");
/// assert_eq!(format!("{}", Loc::new(1, 0).unwrap()), "A2");
/// ```
impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rank, file) = self.rank_file();
        let file = ('A' as u8 + file) as char;

        write!(f, "{}{}", file, 1 + rank)
    }
}
