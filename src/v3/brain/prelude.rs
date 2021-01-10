use std::collections::HashMap;

use crate::v3::board::Board;
use crate::v3::history::GameHistory;
use crate::v3::piece::{
  ChessPiece,
  ChessPieceKind
};

pub type Pieces = HashMap<usize, ChessPiece>;

#[derive(Debug)]
pub struct Capturable {
  pub idx: usize,
  pub kind: ChessPieceKind
}

#[derive(Debug)]
pub struct Scan { 
  pub origin: usize,
  pub tile_vector: Vec<usize>,
  pub reachable_tiles: Vec<usize>,
  pub capturable: Option<Capturable>,
}

impl Scan {
  pub fn new(origin: usize, tile_vector: Vec<usize>, reachable_tiles: Vec<usize>, capturable: Option<Capturable>) -> Scan {
    Scan { origin, tile_vector, reachable_tiles, capturable }
  }
}

#[derive(Debug)]
pub enum PinKind {
  AbsolutePin,
  RelativePin,
  PartialPin,
  SituationalPin,
}

#[derive(Debug)]
pub struct Pin {
  origin: usize,
  kind: PinKind,
  pinned: Capturable,
  shielded: Capturable,
}

#[derive(Debug)]
pub struct Fork {
  origin: usize,
  fork_1: Capturable,
  fork_2: Capturable,
}

pub trait TheThinkyBitsV2 {
  /// Scan the board from the perspective of a piece's current position looking for
  /// all available moves as well as any pins and forks
  fn scan_board(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String>;
  /// Given the results of a board scan, rank a piece's available moves based on
  /// 1. Captures: which moves result in immediate captures
  /// 2. Pins: which moves result in pins. Prioritize pinning higher value pieces
  /// 3. Forks: which moves result in forks. Prioritize forking higher value pieces
  /// 4. Variety Created: which moves create the most opportunity for the next turn
  fn prioritize_moves() -> ();
}

pub trait TheThinkyBits {
  /// Find all tiles a piece can validly move to
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String>;
  /// Find all cases where moving to a given tile whould result in a pin
  fn pins(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin>;
  /// Find all cases where moving to a given tile would result in a fork
  fn forks(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork>;
}


