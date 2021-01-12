use std::collections::HashMap;
use crate::v4::piece::{
  Color,
  Pieces,
  ChessPiece,
};

#[derive(Debug, Clone)]
pub struct Board {
  pub tile_names: Vec<String>,
  pub pieces: Pieces,
}

impl Board {
  pub fn new() -> Board {
    let mut board = Board { tile_names: vec![], pieces: HashMap::new() };
    
    for number in vec!["8", "7", "6", "5", "4", "3", "2", "1"] {
      for letter in vec!["a", "b", "c", "d", "e", "f", "g", "h"] {
        board.tile_names.push(format!("{}{}", letter, number));
      }
    }

    board
  }
  pub fn add_piece(&mut self, piece: ChessPiece) -> Result<Option<ChessPiece>, String> {
    match self.index_of(&piece.position()) {
      Some(idx) => Ok(self.pieces.insert(idx, piece)),
      None      => Err("Cannot add piece to non-existant tile".to_owned())
    }
  }
  pub fn tile_names(&self) -> Vec<String> {
    self.tile_names.clone()
  }
  pub fn tile_name_at(&self, idx: usize) -> Option<&String> {
    self.tile_names.get(idx)
  }
  pub fn row_at(&self, idx: usize) -> Option<usize> {
    if let Some(tile) = self.tile_name_at(idx) {
      return Some(tile.chars().nth(1).unwrap().to_digit(10).unwrap() as usize)
    }
    None
  }
  pub fn index_of(&self, pos: &str) -> Option<usize> {
    self.tile_names.iter().position(|x| x == pos)
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