
pub struct ChessBoard {
  pieces: std::collections::HashMap<String, Box<dyn ChessPiece>>
}

pub trait ChessPiece {
  fn move_to(&mut self, board: &ChessBoard, position: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Knight {
  position: String
}