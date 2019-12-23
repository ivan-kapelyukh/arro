mod board;
mod piece;
mod ttt;

use board::Board;
use std::io::stdin;
use ttt::judge::{calculate_game_drawn, calculate_game_won, move_fair};
use ttt::piece::TTTPiece;
use ttt::player::TTTPlayer;

fn main() {
    // Clear terminal.
    print!("{}[2J", 27 as char);

    let size = input_board_size();
    let mut board = Board::empty(size);
    println!("{}", board);

    let mut to_play = TTTPlayer::X;
    let mut game_won = false;

    while !game_won && !calculate_game_drawn(&board) {
        let (row, col) = input_move_index(&board, to_play);
        board.set(row, col, TTTPiece::Mark(to_play));
        println!("{}", board);
        game_won = calculate_game_won(&board);
        to_play = TTTPlayer::other(to_play);
    }

    if game_won {
        println!("You won, Player {}!", TTTPlayer::other(to_play));
    } else {
        println!("Draw");
    }
}

fn input_board_size() -> usize {
    println!("Hello there! How many rows should the game board have?");

    loop {
        let mut size = String::new();
        stdin().read_line(&mut size).expect("Error reading input");
        let size = size.trim().parse();
        if let Ok(n) = size {
            // At time of writing, these lines can't be merged yet!
            if n > 0 {
                return n;
            }
        }
        println!("Please enter a suitable number of rows");
    }
}

fn input_move_index(board: &Board, player: TTTPlayer) -> (usize, usize) {
    println!("Your move, Player {}!", player);

    loop {
        let mut player_move = String::new();
        stdin()
            .read_line(&mut player_move)
            .expect("Error reading input");

        let parsed_pos = board.square_to_pos(&player_move);
        if let Some((row, col)) = parsed_pos {
            if move_fair(board, row, col) {
                return (row, col);
            } else {
                println!("You can only choose empty squares");
            }
        } else {
            println!(
                "Square should be in the form: a1, up to {}{}",
                (b'a' + (board.width() - 1) as u8) as char,
                board.height()
            );
        }
    }
}
