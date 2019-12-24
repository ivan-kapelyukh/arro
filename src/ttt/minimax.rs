use crate::board::Board;
use crate::ttt::judge::move_fair;
use crate::ttt::piece::TTTPiece;
use crate::ttt::player::TTTPlayer;

// Pre: game not over yet
pub fn pick_move(board: &mut Board, to_play: TTTPlayer) -> (usize, usize) {
    println!("Your move, Player {}!", to_play);

    let mut valid_moves: Vec<(usize, usize, i32)> = Vec::new();
    for row in 0..board.height() {
        for col in 0..board.width() {
            if move_fair(board, row, col) {
                valid_moves.push((row, col, 0));
            }
        }
    }

    for mut m in &mut valid_moves {
        let (row, col, _) = m;
        board.set(*row, *col, TTTPiece::from(to_play));
        m.2 = eval(board);
        board.undo(*row, *col);
    }

    // Unwrap safe due to precondition.
    let (best_row, best_col, _) = valid_moves.iter().max_by_key(|(_, _, val)| val).unwrap();
    (*best_row, *best_col)
}

fn eval(board: &Board) -> i32 {
    0
}
