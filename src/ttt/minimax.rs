use crate::board::Board;
use crate::ttt::piece::TTTPiece;
use crate::ttt::player::TTTPlayer;

pub fn pick_move(board: &Board, to_play: TTTPlayer) -> (usize, usize) {
    println!("Your move, Player {}!", to_play);

    for row in 0..board.height() {
        for col in 0..board.width() {
            if board.get(row, col) == TTTPiece::Empty {
                return (row, col);
            }
        }
    }

    (0, 0)
}
