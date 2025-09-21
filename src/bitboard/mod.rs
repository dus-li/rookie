// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Duszku

use crate::loc::Loc;

/// A single bitboard.
///
/// Bitboards are a basic building block of bitboard board representation. Under
/// such representation a board is a collection of bitmasks. If bit n is set in
/// a bitboard, that represents a square with index n containing a piece.
#[derive(Default, Clone, Copy)]
pub struct Bitboard(u64);

/// An iterator over the set bits of a `Bitboard`.
///
/// This struct is created by the `Bitboard::iter()` method.
pub struct BitboardIter(u64);

/// Implements the iterator logic for `BitboardIter`.
///
/// This implementation uses a bit-twiddling hack to find and clear the least
/// significant bit on each call to `next()`.
impl Iterator for BitboardIter {
    type Item = Loc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let ret = self.0.trailing_zeros();
        self.0 &= !(1 << ret);

        Loc::from_index(ret as u8)
    }
}

/// Implements the bitwise-AND operator for `Bitboard`.
impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

/// Implements the bitwise-AND assignment operator for `Bitboard`.
impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Implements the bitwise-OR operator for `Bitboard`.
impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

/// Implements the bitwise-OR assignment operator for `Bitboard`.
impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

/// Implements the bitwise-XOR operator for `Bitboard`.
impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

/// Implements the bitwise-XOR assignment operator for `Bitboard`.
impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Bitboard {
    /// Creates a new bitboard.
    ///
    /// # Examples
    ///
    /// ```
    /// use rookie::bitboard::Bitboard;
    /// use rookie::loc::Loc;
    ///
    /// let mut indices = Vec::new();
    /// indices.push(Loc::from_index(0).unwrap());
    /// indices.push(Loc::from_index(13).unwrap());
    /// indices.push(Loc::from_index(22).unwrap());
    /// indices.push(Loc::from_index(5).unwrap());
    ///
    /// let bitboard = Bitboard::new(&indices);
    ///
    /// assert!(bitboard.at(Loc::from_index(0).unwrap()));
    /// assert!(bitboard.at(Loc::from_index(5).unwrap()));
    /// assert!(bitboard.at(Loc::from_index(13).unwrap()));
    /// assert!(bitboard.at(Loc::from_index(22).unwrap()));
    /// ```
    ///
    /// # Arguments
    ///
    /// * `indices`: A collection of indices to be set in the bitboard.
    ///
    /// # Returns
    ///
    /// A new bitboard populated with locations listen in `indices`.
    pub fn new(indices: &[Loc]) -> Self {
        Self(indices.iter().fold(0, |mut acc, loc| {
            acc |= 1 << loc.index();
            acc
        }))
    }

    /// Initializes a bitboard containing a single piece.
    pub fn from_single(loc: Loc) -> Self {
        Self(1 << loc.index())
    }

    /// Initialize a bitboard from a raw 64-bit unsigned integer.
    pub fn from_u64(raw: u64) -> Self {
        Self(raw)
    }

    /// Check if square is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use rookie::bitboard::Bitboard;
    /// use rookie::loc::Loc;
    ///
    /// let board = Bitboard::from_u64(0b0000_1001);
    ///
    /// // These are set
    /// assert!(board.at(Loc::from_index(0).unwrap()));
    /// assert!(board.at(Loc::from_index(3).unwrap()));
    ///
    /// // These are some of the squares that are not set
    /// assert!(!board.at(Loc::from_index(1).unwrap()));
    /// assert!(!board.at(Loc::from_index(2).unwrap()));
    /// ```
    ///
    /// # Arguments
    ///
    /// * `loc`: Location that is to be checked.
    ///
    /// # Returns
    ///
    /// State of the polled square.
    /// * `true`: when the square contains a piece.
    /// * `false`: when the square does not contain a piece.
    pub fn at(&self, loc: Loc) -> bool {
        (self.0 & (1 << loc.index())) != 0
    }

    /// Creates an iterator over the indices of the set bits in the bitboard.
    ///
    /// This iterator yields `loc::Loc` values, where each value is the square
    /// location of a piece. The iteration order is from the least significant
    /// bit to the most significant bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rookie::bitboard::Bitboard;
    ///
    /// let board = Bitboard::from_u64(0b0010_1000);
    /// let mut iter = board.iter();
    ///
    /// assert_eq!(iter.next().map(|loc| loc.index()), Some(3));
    /// assert_eq!(iter.next().map(|loc| loc.index()), Some(5));
    /// assert_eq!(iter.next().map(|loc| loc.index()), None);
    /// ```
    pub fn iter(&self) -> BitboardIter {
        BitboardIter(self.0)
    }
}
