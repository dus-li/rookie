// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Duszku

mod result;

use crate::bitboard::Bitboard;
use crate::loc::Loc;
use self::result::{Result, BoardError};

/// All the different types of chess pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// A complete description of a chess piece.
#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub white: bool,
}

/// A complete information on piece placement.
///
/// This strucure encapsulates positioning of every piece on the chessboard, but
/// does not keep all information regarding state of the game, such as active
/// colour or potential en passant.
#[derive(Default, Clone)]
pub struct Board {
    black: Bitboard,
    white: Bitboard,
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
}

/// A builder for `Board` structs.
#[derive(Default)]
pub struct BoardBuilder {
    board: Board,
}

impl std::ops::Index<PieceKind> for Board {
    type Output = Bitboard;

    fn index(&self, index: PieceKind) -> &Self::Output {
        use PieceKind::*;

        match index {
            Pawn => &self.pawns,
            Knight => &self.knights,
            Bishop => &self.bishops,
            Rook => &self.rooks,
            Queen => &self.queens,
            King => &self.kings,
        }
    }
}

impl std::ops::IndexMut<PieceKind> for Board {
    fn index_mut(&mut self, index: PieceKind) -> &mut Self::Output {
        use PieceKind::*;

        match index {
            Pawn => &mut self.pawns,
            Knight => &mut self.knights,
            Bishop => &mut self.bishops,
            Rook => &mut self.rooks,
            Queen => &mut self.queens,
            King => &mut self.kings,
        }
    }
}

impl std::ops::Index<bool> for Board {
    type Output = Bitboard;

    fn index(&self, index: bool) -> &Self::Output {
        if index {
            &self.white
        } else {
            &self.black
        }
    }
}

impl std::ops::IndexMut<bool> for Board {
    fn index_mut(&mut self, index: bool) -> &mut Self::Output {
        if index {
            &mut self.white
        } else {
            &mut self.black
        }
    }
}

impl BoardBuilder {
    /// Places a piece on a board.
    ///
    /// # Examples
    ///
    /// ```
    /// use rookie::board::{Board, Piece, PieceKind};
    /// use rookie::loc::Loc;
    ///
    /// let a1 = Loc::new(0, 0).unwrap();
    /// let e5 = Loc::new(4, 4).unwrap();
    /// let f2 = Loc::new(1, 5).unwrap();
    ///
    /// let white_rook = Piece { kind: PieceKind::Rook, white: true };
    /// let black_pawn = Piece { kind: PieceKind::Pawn, white: false };
    ///
    /// let board = Board::builder()
    ///     .add_piece(&white_rook, &a1)
    ///     .add_piece(&black_pawn, &e5)
    ///     .build();
    ///
    /// assert_eq!(board.at(&a1).unwrap(), Some(white_rook));
    /// assert_eq!(board.at(&e5).unwrap(), Some(black_pawn));
    /// assert!(board.at(&f2).unwrap().is_none());
    /// ```
    pub fn add_piece(mut self, piece: &Piece, loc: &Loc) -> Self {
        let new = Bitboard::from_single(loc);

        self.board[piece.kind] |= new;
        self.board[piece.white] |= new;

        self
    }

    /// Completes the build process and yields a `Board` instance.
    ///
    /// The same builder may be used multiple times to create several instances
    /// of identical boards.
    pub fn build(self) -> Board {
        self.board.clone()
    }
}

impl Board {
    /// Starts `Board` build process and yields a `BoardBuilder`.
    pub fn builder() -> BoardBuilder {
        BoardBuilder::default()
    }

    /// Returns piece (if any) located at a given square.
    pub fn at(&self, loc: &Loc) -> Result<Option<Piece>> {
        use PieceKind::*;

        let white = match (self.white.at(loc), self.black.at(loc)) {
            (false, false) => return Ok(None),
            (false, true) => false,
            (true, false) => true,
            (true, true) => return Err(BoardError::board_corruption("two colors")),
        };

        let kind = [
            (&self.pawns, Pawn),
            (&self.knights, Knight),
            (&self.bishops, Bishop),
            (&self.rooks, Rook),
            (&self.queens, Queen),
            (&self.kings, King),
        ]
        .into_iter()
        .find_map(|(bitboard, kind)| bitboard.at(loc).then_some(kind));

        match kind {
            Some(kind) => Ok(Some(Piece { kind, white })),
            None => Err(BoardError::board_corruption("color with no piece"))
        }
    }

    pub fn pieces(&self, pattern: &Piece) -> Bitboard {
        self[pattern.kind] & self[pattern.white]
    }
}
