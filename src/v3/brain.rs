use std::collections::HashMap;
use super::board::Board;
use super::piece::ChessPiece;
use super::piece::ChessPieceKind;

type Pieces = HashMap<usize, ChessPiece>;

pub struct Pin {
  origin: usize,
  pinned: (usize, ChessPieceKind),
  pinned_against: (usize, ChessPieceKind),
}

pub struct Fork {
  origin: usize,
  fork_1: (usize, ChessPieceKind),
  fork_2: (usize, ChessPieceKind),
}

pub trait TheThinkyBits {
  /// Find all tiles a piece can validly move to
  fn available_tiles(&self, origin: usize, board: &Board, pieces: &Pieces) -> Vec<usize>;
  /// Find all cases where moving to a given tile whould result in a pin
  fn pins(&self, tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin>;
  /// Find all cases where moving to a given tile would result in a fork
  fn forks(&self, tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork>;
}