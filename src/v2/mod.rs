
mod error;

use std::collections::HashMap;

type ChessPieces = HashMap<String, Box<dyn ChessPiece>>;

pub struct ChessBoard {
  pieces: ChessPieces,
}

impl ChessBoard {
  pub fn new() -> ChessBoard {
    ChessBoard { pieces: HashMap::new() }
  }

  pub fn peek_tile(&self, key: &str) -> Option<&Box<dyn ChessPiece>> {
    self.pieces.get(key)
  }

  pub fn add_piece<P: ChessPiece + 'static>(&mut self, piece: P) {
    self.pieces.insert(piece.position(), Box::new(piece)).unwrap();
  }

  pub fn move_piece(&mut self, old: &str, new: &str) -> ChessMoveResult {
    let pieces = self.get_piece(old);
    if let Some(piece) = &pieces {
      piece.update_position(&pieces, new)?;
    }
    Ok(())
  }

  fn get_piece(&mut self, key: &str) -> Option<&mut Box<dyn ChessPiece>> {
    self.pieces.get_mut(key)
  }
}



type ChessMoveResult = Result<(), error::ChessMoveError>;

pub trait ChessPiece {
  /// Update the piece's current position
  fn update_position(&mut self, pieces: &ChessPieces, pos: &str) -> ChessMoveResult;
  /// Get the piece's current position
  fn position(&self) -> String;
}

pub struct Knight {
  pos: String,
}

impl Knight {
  pub fn new(pos: &str) -> Knight {
    Knight { pos: pos.to_string() }
  }
}

impl ChessPiece for Knight {
  fn update_position(&mut self, pieces: &ChessPieces, pos: &str) -> ChessMoveResult {

    Ok(())
  }

  fn position(&self) -> String {
    self.pos.clone()
  }
}