

#[derive(Debug)]
pub struct Board {
  tiles: Vec<String>
}

impl Board {
  pub fn new() -> Board {
    let mut tiles = vec![];
    for letter in vec!["a", "b", "c", "d", "e", "f", "g", "h"] {
      for number in vec!["1", "2", "3", "4", "5", "6", "7", "8"] {
        tiles.push(format!("{}{}", letter, number));
      }
    }
    Board { tiles }
  }
  pub fn index_of(&self, pos: &str) -> Option<usize> {
    self.tiles.iter().position(|x| x == pos)
  }
}