#[derive(Clone, Debug)]
pub enum PieceColor {
  Black,
  White,
}

#[derive(Clone, Debug)]
pub enum PieceKind {
  Pawn, Knight, Bishop,
  Rook, Queen, King,
}


#[derive(Clone, Debug)]
pub struct Piece {
  pub kind: PieceKind,
  pub color: PieceColor,
  pub position: String,
}

impl Piece {
  pub fn new(kind: PieceKind, color: PieceColor, position: &str) -> Piece {
    Piece { kind, color, position: position.to_owned() }
  }
  pub fn pawn(color: PieceColor, pos: &str) -> Piece {
    Piece::new(PieceKind::Pawn, color, pos)
  }
  pub fn knight(color: PieceColor, pos: &str) -> Piece {
    Piece::new(PieceKind::Knight, color, pos)
  }
  pub fn bishop(color: PieceColor, pos: &str) -> Piece {
    Piece::new(PieceKind::Bishop, color, pos)
  }
  pub fn rook(color: PieceColor, pos: &str) -> Piece {
    Piece::new(PieceKind::Rook, color, pos)
  }
  pub fn queen(color: PieceColor, pos: &str) -> Piece {
    Piece::new(PieceKind::Queen, color, pos)
  }
  pub fn king(color: PieceColor, pos: &str) -> Piece {
    Piece::new(PieceKind::King, color, pos)
  }

  pub fn name(&self) -> String {
    use PieceKind::{Pawn, Knight, Bishop, Rook, Queen, King};
    match self.kind {
      Pawn => format!("p_{}", self.position),
      Knight => format!("n_{}", self.position),
      Bishop => format!("b_{}", self.position),
      Rook => format!("r_{}", self.position),
      Queen => format!("q_{}", self.position),
      King => format!("k_{}", self.position),
    }
  }
  pub fn points(&self) -> Option<u8> {
    use PieceKind::{Pawn, Knight, Bishop, Rook, Queen};
    match self.kind {
      Pawn => Some(1),
      Knight => Some(3),
      Bishop => Some(3),
      Rook => Some(5),
      Queen => Some(9),
      _ => None
    }
  }
}
