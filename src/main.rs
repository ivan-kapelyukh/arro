use std::fmt;

fn main() {
  let board = Board {
    width: 10,
    height: 12,
  };
  println!("{}", board);
}

struct Board {
  width: u32,
  height: u32,
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} by {}", self.width, self.height)
  }
}
