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
    for (i, row) in self.cells.iter().enumerate() {
      for (j, cell) in row.iter().enumerate() {
        write!(f, "{}", cell)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}
