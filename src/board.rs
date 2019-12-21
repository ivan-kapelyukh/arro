use crate::piece::Piece;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Board {
    cells: Vec<Vec<Piece>>,
}

impl Board {
    pub fn empty(n: usize) -> Self {
        Board {
            cells: vec![vec![Piece::default(); n]; n],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, new_piece: Piece) {
        self.cells[row][col] = new_piece;
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    pub fn square_to_pos(&self, square: &str) -> Option<(usize, usize)> {
        let mut square = square.chars();
        let row = square.next()? as usize;
        if row < 'a' as usize {
            return None;
        }
        let row = row - 'a' as usize;

        let col = square.next()?.to_digit(10)? as usize - 1;
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
