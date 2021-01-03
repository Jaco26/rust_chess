

#[derive(Debug, Clone)]
pub struct Board {
  tiles: Vec<String>
}

impl Board {
  pub fn new() -> Board {
    let mut tiles = vec![];
    for number in vec!["8", "7", "6", "5", "4", "3", "2", "1"] {
      for letter in vec!["a", "b", "c", "d", "e", "f", "g", "h"] {
        tiles.push(format!("{}{}", letter, number));
      }
    }
    Board { tiles }
  }
  pub fn tiles(&self) -> Vec<String> {
    self.tiles.clone()
  }
  pub fn tile_at(&self, idx: usize) -> Option<&String> {
    self.tiles.get(idx)
  }
  pub fn row_at(&self, idx: usize) -> Option<usize> {
    if let Some(tile) = self.tile_at(idx) {
      return Some(tile.chars().nth(1).unwrap().to_digit(10).unwrap() as usize)
    }
    None
  }
  pub fn index_of(&self, pos: &str) -> Option<usize> {
    self.tiles.iter().position(|x| x == pos)
  }
}