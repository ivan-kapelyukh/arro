use crate::piece::Piece;
use crate::ttt::player::TTTPlayer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Copy, Clone, PartialEq)]
pub enum TTTPiece {
    Empty,
    Mark(TTTPlayer),
}

impl Piece for TTTPiece {}

impl Default for TTTPiece {
    fn default() -> Self {
        TTTPiece::Empty
    }
}

impl Display for TTTPiece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TTTPiece::Empty => write!(f, "·"),
            TTTPiece::Mark(p) => p.fmt(f),
        }
    }
}

impl From<TTTPlayer> for TTTPiece {
    fn from(player: TTTPlayer) -> Self {
        TTTPiece::Mark(player)
    }
}
