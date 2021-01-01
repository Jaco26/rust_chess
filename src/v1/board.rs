use std::collections::HashMap;
use crate::v1::piece::Piece;

#[derive(Debug)]
pub struct Board {
  pub pieces: HashMap<String, Piece>,
}

impl Board {
  pub fn new() -> Board {
    Board { pieces: HashMap::new() }
  }

  pub fn get_piece(&mut self, name: &str) -> Option<&mut Piece> {
    self.pieces.get_mut(name)
  }

  pub fn add_piece(&mut self, piece: Piece) -> Result<(), &'static str> {
    if self.position_valid(&piece.position) {
      self.pieces.insert(piece.position.to_owned(), piece);
      return Ok(())
    }
    Err("Game piece must have valid position")
  }

  fn position_valid(&self, pos: &str) -> bool {
    let mut chars = pos.chars();
    let col = chars.next().unwrap();
    let row = chars.next().unwrap();
    if col < 'a' || col > 'h' || row < '1' || row > '8' {
      return false
    }
    !self.pieces.contains_key(pos)
  }
}