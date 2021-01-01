use std::fmt;

pub enum ChessMoveError {
  InvalidMovePattern,
  KingPin,
  TerminatedPath,
  OffBoard,
}

impl fmt::Display for ChessMoveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      ChessMoveError::InvalidMovePattern =>
        write!(f, "Tile not in available tiles as derived from this piece's move pattern (e.g. A rook cannot move diagonally)"),
      ChessMoveError::KingPin =>
        write!(f, "Piece is protecting the king from check"),
      ChessMoveError::TerminatedPath =>
        write!(f, "Piece cannot move to this tile because its path has been terminated by another peice on the board"),
      ChessMoveError::OffBoard =>
        write!(f, "Piece cannot move off of the board")
    }
  }
}
