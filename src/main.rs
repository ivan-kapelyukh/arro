mod board;
mod piece;
mod player;
mod ttt;

use board::Board;
use player::Player;
use std::io::stdin;
use ttt::judge::{calculate_game_drawn, calculate_game_won};
use ttt::piece::TTTPiece;

fn main() {
    // Clear terminal.
    print!("{}[2J", 27 as char);

    let size = input_board_size();
    let mut board = Board::empty(size);
    println!("{}", board);

    let mut game_won = false;

    while !game_won && !calculate_game_drawn(&board) {
        let (row, col) = input_move_index(&board);
        board.set(row, col, TTTPiece::Mark(Player::One));
        println!("{}", board);
        game_won = calculate_game_won(&board);
    }

    if game_won {
        println!("You won!");
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

fn input_move_index(board: &Board) -> (usize, usize) {
    println!("Your move!");
    loop {
        let mut player_move = String::new();
        stdin()
            .read_line(&mut player_move)
            .expect("Error reading input");

        let parsed_pos = board.square_to_pos(&player_move);
        if let Some(pos) = parsed_pos {
            return pos;
        }

        println!(
            "Square should be in the form: a1, up to {}{}",
            (b'a' + (board.width() - 1) as u8) as char,
            board.height()
        );
    }
}
