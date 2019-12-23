#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    One = 1,
    Two = 2,
}

impl Player {
    pub fn other(player: Player) -> Self {
        match player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}
