use crate::board::Board;
use crate::ttt::piece::TTTPiece;
use std::cmp::min;

//    \
//    /\
//   /  \

pub fn calculate_game_won(board: &Board) -> bool {
    let rows = board.cells();

    let mut cols = Vec::new();
    for col in 0..board.height() {
        let mut new_col = Vec::new();
        for row in 0..board.width() {
            new_col.push(board.get(row, col));
        }
        cols.push(new_col);
    }

    let min_dim = min(board.width(), board.height());
    let mut falling_diag = Vec::new();
    for i in 0..min_dim {
        falling_diag.push(board.get(i, i));
    }
    let mut rising_diag = Vec::new();
    for i in 0..min_dim {
        rising_diag.push(board.get(i, min_dim - 1 - i));
    }
    let diags = vec![falling_diag, rising_diag];

    let directions = [rows, &cols, &diags];

    #[rustfmt::skip]
    let game_won =
    directions.iter().map(|direction|
     direction.iter().map(|line| line[0] != TTTPiece::Empty &&
          line.iter().map(|c| *c == line[0])
             .all(|c| c)
         ).any(|line| line)
    ).any(|direction| direction);

    game_won
}

// Pre: game not won
pub fn calculate_game_drawn(board: &Board) -> bool {
    board
        .cells()
        .iter()
        .flatten()
        .all(|c| *c != TTTPiece::Empty)
}
