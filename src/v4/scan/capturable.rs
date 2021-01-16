use crate::v4::piece::ChessPieceKind;

#[derive(Debug, Clone)]
pub struct Capturable {
  pub origin: usize,
  pub kind: ChessPieceKind
}

impl Capturable {
  pub fn new(origin: usize, kind: ChessPieceKind) -> Capturable {
    Capturable { origin, kind }
  }
}

