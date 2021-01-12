use std::fmt;
use std::collections::HashMap;

pub type Pieces = HashMap<usize, ChessPiece>;

#[derive(Debug, Clone)]
pub enum Color { Black, White }


#[derive(Debug, Clone)]
pub enum ChessPieceKind {
  Pawn, Knight, Bishop,
  Rook, Queen, King
}


#[derive(Debug, Clone)]
pub struct ChessPiece {
  kind: ChessPieceKind,
  color: Color,
  position: String,
}

// Constructors
impl ChessPiece {
  pub fn pawn(color: Color, pos: &str) -> ChessPiece {
    ChessPiece { color, kind: ChessPieceKind::Pawn, position: pos.to_owned() }
  }
  pub fn knight(color: Color, pos: &str) -> ChessPiece {
    ChessPiece { color, kind: ChessPieceKind::Knight, position: pos.to_owned() }
  }
  pub fn bishop(color: Color, pos: &str) -> ChessPiece {
    ChessPiece { color, kind: ChessPieceKind::Bishop, position: pos.to_owned() }
  }
  pub fn rook(color: Color, pos: &str) -> ChessPiece {
    ChessPiece { color, kind: ChessPieceKind::Rook, position: pos.to_owned() }
  }
  pub fn queen(color: Color, pos: &str) -> ChessPiece {
    ChessPiece { color, kind: ChessPieceKind::Queen, position: pos.to_owned() }
  }
  pub fn king(color: Color, pos: &str) -> ChessPiece {
    ChessPiece { color, kind: ChessPieceKind::King, position: pos.to_owned() }
  }
}

// Methods
impl ChessPiece {
  pub fn points(&self) -> Option<u8> {
    use ChessPieceKind::*;
    match self.kind {
      Pawn => Some(1),
      Knight |
      Bishop => Some(3),
      Rook => Some(5),
      Queen => Some(9),
      _ => None,
    }
  }
  pub fn color(&self) -> &Color {
    &self.color
  }
  pub fn kind(&self) -> &ChessPieceKind {
    &self.kind
  }
  pub fn position(&self) -> String {
    self.position.to_string()
  }
  pub fn update_position(&mut self, pos: &str) {
    self.position = pos.to_owned();
  }
}

impl fmt::Display for ChessPiece {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let color = match self.color {
      Color::White => "w",
      Color::Black => "b",
    };
    let kind = match self.kind {
      ChessPieceKind::Pawn => "P",
      ChessPieceKind::Knight => "N",
      ChessPieceKind::Bishop => "B",
      ChessPieceKind::Rook => "R",
      ChessPieceKind::Queen => "Q",
      ChessPieceKind::King => "K",
    };
    write!(f, "{}_{}", color, kind)
  }
}