use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::stdin;

fn main() {
    // Clear terminal.
    print!("{}[2J", 27 as char);
    println!("Hello there! What size should the game board be?");

    let mut size = String::new();
    stdin().read_line(&mut size).expect("Error reading input");
    let size: usize = size.trim().parse().expect("Please enter a number");

    let mut board = Board::empty(size);
    println!("{}", board);

    let mut player_move = String::new();
    println!("Your move!");
    stdin()
        .read_line(&mut player_move)
        .expect("Error reading input");

    // TODO: a couple of error checks.
    let mut split = player_move.split_whitespace();
    let from = split.next().unwrap();
    let to = split.next().unwrap();
    println!("Moving from {} to {}", from, to);
}

struct Board {
    cells: Vec<Vec<Piece>>,
}

impl Board {
    fn empty(n: usize) -> Self {
        Board {
            cells: vec![vec![Piece::default(); n]; n],
        }
    }

    fn set(&mut self, row: usize, col: usize, new_piece: Piece) {
        self.cells[row][col] = new_piece;
    }
}

#[derive(Copy, Clone)]
enum Piece {
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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write col headers.
        write!(f, " ")?;
        for c in (0..self.cells[0].len()).map(|i| (b'a' + (i as u8)) as char) {
            write!(f, " {}", c)?;
        }
        writeln!(f)?;

        for (i, row) in self.cells.iter().enumerate() {
            write!(f, "{}", i + 1)?; // Write row headers.
            for cell in row.iter() {
                write!(f, " {}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
