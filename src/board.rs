use crate::ttt::piece::TTTPiece;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Board {
    cells: Vec<Vec<TTTPiece>>,
}

// TODO: polymorphism through composition - should be easy to have
// this refer to Piece rather than specifically TTTPiece
impl Board {
    pub fn empty(n: usize) -> Self {
        Board {
            cells: vec![vec![TTTPiece::default(); n]; n],
        }
    }

    // In case you want to use iterators on the board.
    pub fn cells(&self) -> &Vec<Vec<TTTPiece>> {
        &self.cells
    }

    pub fn get(&self, row: usize, col: usize) -> TTTPiece {
        self.cells[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, new_piece: TTTPiece) {
        self.cells[row][col] = new_piece;
    }

    // For more complicated games, may need to store extra history info.
    pub fn undo(&mut self, row: usize, col: usize) {
        self.cells[row][col] = TTTPiece::Empty;
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    pub fn square_to_pos(&self, square: &str) -> Option<(usize, usize)> {
        let mut square = square.chars();
        let col = square.next()? as usize;
        if col < 'a' as usize {
            return None;
        }
        let col = col - 'a' as usize;

        let row = square.next()?.to_digit(10)? as usize - 1;
        if row >= self.height() || col >= self.width() {
            return None;
        }

        Some((row, col))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
