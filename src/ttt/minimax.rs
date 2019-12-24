use crate::board::Board;
use crate::ttt::judge::{calculate_game_drawn, calculate_game_won, move_fair};
use crate::ttt::piece::TTTPiece;
use crate::ttt::player::TTTPlayer;

// Pre: game not over yet
pub fn pick_move(board: &mut Board, to_play: TTTPlayer) -> (usize, usize) {
    const MAX_LOOKAHEAD: u32 = 8;
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
        m.2 = eval(board, TTTPlayer::other(to_play), MAX_LOOKAHEAD - 1);
        board.undo(*row, *col);
    }

    // println!("Moves: {:#?}", valid_moves);
    // Unwrap safe due to precondition.
    let (best_row, best_col, _) = valid_moves.iter().max_by_key(|(_, _, val)| val).unwrap();
    (*best_row, *best_col)
}

fn eval(board: &mut Board, to_play: TTTPlayer, max_lookahead: u32) -> i32 {
    if calculate_game_won(board) {
        // The to_play player has lost.
        return match to_play {
            TTTPlayer::O => -1000,
            TTTPlayer::X => 1000,
        };
    }

    if calculate_game_drawn(board) || max_lookahead == 0 {
        return 0;
    }

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
        m.2 = eval(board, TTTPlayer::other(to_play), max_lookahead - 1);
        board.undo(*row, *col);
    }

    // O (arro) is Row Player, X (human) is Column Player.
    let (_, _, best_val) = match to_play {
        TTTPlayer::O => valid_moves.iter().max_by_key(|(_, _, val)| val).unwrap(),
        TTTPlayer::X => valid_moves.iter().min_by_key(|(_, _, val)| val).unwrap(),
    };

    *best_val
}
