use crate::v3::piece::ChessPiece;
use crate::v3::brain::prelude::Pieces;


pub fn peek_tile(idx: usize, pieces: &Pieces) -> Option<&ChessPiece> {
  pieces.get(&idx)
}

pub fn tile_is_own(piece: &ChessPiece, other_piece: &ChessPiece) -> bool {
  piece.color() == other_piece.color()
}

pub fn tile_is_opponent(piece: &ChessPiece, other_piece: &ChessPiece) -> bool {
  !tile_is_own(piece, other_piece)
}
