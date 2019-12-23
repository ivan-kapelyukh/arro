use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq)]
pub enum TTTPlayer {
    X,
    O,
}

impl TTTPlayer {
    pub fn other(player: TTTPlayer) -> Self {
        match player {
            TTTPlayer::X => TTTPlayer::O,
            TTTPlayer::O => TTTPlayer::X,
        }
    }
}

impl Display for TTTPlayer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let character = match self {
            TTTPlayer::X => 'X',
            TTTPlayer::O => 'O',
        };
        write!(f, "{}", character)
    }
}
