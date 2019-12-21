use crate::piece::Piece;
use crate::player::Player;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Copy, Clone)]
pub enum TTTPiece {
    Empty,
    Mark(Player),
}

impl Piece for TTTPiece {}

impl Default for TTTPiece {
    fn default() -> Self {
        TTTPiece::Empty
    }
}

impl Display for TTTPiece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let character = match self {
            TTTPiece::Empty => '·',
            TTTPiece::Mark(Player::One) => 'X',
            TTTPiece::Mark(Player::Two) => 'O',
        };
        write!(f, "{}", character)
    }
}
