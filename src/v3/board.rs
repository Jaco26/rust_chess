use crate::v3::piece::Color;
use crate::v3::piece::ChessPiece;

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
  pub fn rank_of(&self, piece: &ChessPiece) -> Option<usize> {
    match self.index_of(&piece.position()) {
      Some(idx) => self.rank_of_index(idx, piece.color()),
      None => None
    }
  }
  pub fn rank_of_index(&self, idx: usize, color: &Color) -> Option<usize> {
    if let Some(row) = self.row_at(idx) {
      return Some(match color {
        Color::Black => 9 - row,
        Color::White => row
      })
    }
    None
  }
}