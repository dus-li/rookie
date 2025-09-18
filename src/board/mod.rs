// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Duszku

use crate::bitboard::Bitboard;
use crate::loc::Loc;

/// All the different types of chess pieces.
#[derive(Debug, PartialEq, Eq)]
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
    /// assert_eq!(board.at(&a1).unwrap(), white_rook);
    /// assert_eq!(board.at(&e5).unwrap(), black_pawn);
    /// assert!(board.at(&f2).is_none());
    /// ```
    pub fn add_piece(mut self, piece: &Piece, loc: &Loc) -> Self {
        use PieceKind::*;

        let new = Bitboard::from_single(loc);

        match piece.kind {
            Pawn => self.board.pawns |= new,
            Knight => self.board.knights |= new,
            Bishop => self.board.bishops |= new,
            Rook => self.board.rooks |= new,
            Queen => self.board.queens |= new,
            King => self.board.kings |= new,
        };

        if piece.white {
            self.board.white |= new;
        } else {
            self.board.black |= new;
        }

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
    ///
    /// # Panics
    ///
    /// FIXME: If a board is corrupted (two colour occupying the same square or
    /// a square with a color but no piece) this function panics. Ideally it
    /// should return an error instead.
    pub fn at(&self, loc: &Loc) -> Option<Piece> {
        use PieceKind::*;

        let white = match (self.white.at(loc), self.black.at(loc)) {
            (false, false) => return None,
            (false, true) => false,
            (true, false) => true,
            (true, true) => panic!("Board corruption: two colors"),
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
        .find_map(|(bitboard, kind)| bitboard.at(loc).then_some(kind))
        .expect("Board corruption: color with no piece");

        Some(Piece { kind, white })
    }

    /// Yields a `Bitboard` of all pawns of a given colour.
    pub fn pawns(&self, white: bool) -> Bitboard {
        self.pawns & (if white { self.white } else { self.black })
    }

    /// Yields a `Bitboard` of all knights of a given colour.
    pub fn knights(&self, white: bool) -> Bitboard {
        self.knights & (if white { self.white } else { self.black })
    }

    /// Yields a `Bitboard` of all bishops of a given colour.
    pub fn bishops(&self, white: bool) -> Bitboard {
        self.bishops & (if white { self.white } else { self.black })
    }

    /// Yields a `Bitboard` of all rooks of a given colour.
    pub fn rooks(&self, white: bool) -> Bitboard {
        self.rooks & (if white { self.white } else { self.black })
    }

    /// Yields a `Bitboard` of all queens of a given colour.
    pub fn queens(&self, white: bool) -> Bitboard {
        self.queens & (if white { self.white } else { self.black })
    }

    /// Yields a `Bitboard` of all kings of a given colour.
    pub fn kings(&self, white: bool) -> Bitboard {
        self.kings & (if white { self.white } else { self.black })
    }
}
