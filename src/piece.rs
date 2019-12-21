use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Copy, Clone)]
pub enum Piece {
    Empty,
    Nought,
    Cross,
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let character = match self {
            Piece::Empty => '·',
            Piece::Nought => 'O',
            Piece::Cross => 'X',
        };
        write!(f, "{}", character)
    }
}
