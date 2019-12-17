use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

fn main() {
    let board = Board::empty(3);
    println!("{}", board);
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
