use std::fmt;

fn main() {
  let board = Board {
    cells: [[0, 1, 2], [3, 4, 5], [6, 7, 8]],
  };
  println!("{}", board);
}

struct Board {
  cells: [[u32; 3]; 3],
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write col headers.
    write!(f, " ")?;
    for c in (0..self.cells.len()).map(|i| (b'a' + (i as u8)) as char) {
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
