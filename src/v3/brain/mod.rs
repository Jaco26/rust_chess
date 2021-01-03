#![allow(dead_code, unused_variables, unused_imports)]

pub mod util;

use std::collections::HashMap;

use super::board::Board;
use super::scanner::{
  plot_move_from,
  Direction,
};
use super::piece::{
  Color,
  ChessPiece,
  ChessPieceKind
};
use super::history::GameHistory;

pub type Pieces = HashMap<usize, ChessPiece>;

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
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String>;
  /// Find all cases where moving to a given tile whould result in a pin
  fn pins(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin>;
  /// Find all cases where moving to a given tile would result in a fork
  fn forks(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork>;
}


