use crate::v4::piece::ChessPieceKind;

#[derive(Debug, Clone)]
pub enum CapturableValue {
  Check,
  Points(usize)
}

#[derive(Debug, Clone)]
pub struct Capturable {
  pub origin: usize,
  pub kind: ChessPieceKind
}

// Constructor
impl Capturable {
  pub fn new(origin: usize, kind: ChessPieceKind) -> Capturable {
    Capturable { origin, kind }
  }
}

// Methods
impl Capturable {
  pub fn value(&self) -> CapturableValue {
    use CapturableValue::{Check, Points};
    match self.kind {
      ChessPieceKind::Pawn => Points(1),
      ChessPieceKind::Knight |
      ChessPieceKind::Bishop => Points(3),
      ChessPieceKind::Rook => Points(5),
      ChessPieceKind::Queen => Points(9),
      ChessPieceKind::King => Check
    }
  }
}