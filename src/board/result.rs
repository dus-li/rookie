// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Duszku

use thiserror::Error;

pub type Result<T> = std::result::Result<T, BoardError>;

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("board structure corruption detected: {0}")]
    BoardCorruption(String),
}

impl BoardError {
    pub fn board_corruption(msg: &str) -> Self {
        BoardError::BoardCorruption(msg.to_string())
    }
}
