#![allow(dead_code, unused_variables)]
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
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String>;
  /// Find all cases where moving to a given tile whould result in a pin
  fn pins(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin>;
  /// Find all cases where moving to a given tile would result in a fork
  fn forks(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork>;
}

pub struct PawnBrain;
pub struct KnightBrain;
pub struct BishopBrain;
pub struct RookBrain;
pub struct QueenBrain;
pub struct KingBrain;


mod utils {
  use super::{
    ChessPiece,
    Pieces,
  };

  pub fn is_tile_empty(test_idx: usize, pieces: &Pieces) -> bool {
    match pieces.get(&test_idx) {
      Some(piece) => false,
      None => true
    }
  }

  pub fn is_tile_opponent(test_idx: usize, origin_piece: &ChessPiece, pieces: &Pieces) -> bool {
    if !is_tile_empty(test_idx, pieces) {
      if let Some(piece) = pieces.get(&test_idx) {
        return piece.color() != origin_piece.color()
      }
    }
    false
  }

  pub fn only_somes(items: Vec<Option<usize>>) -> Vec<usize> {
    items.iter().fold(Vec::new(), |mut acc, x| {
      if x.is_some() {
        acc.push(x.unwrap())
      }
      acc
    })
  }
}


impl TheThinkyBits for PawnBrain {
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    use Direction::*;
    use utils::*;
    if let Some(origin_piece) = pieces.get(&origin) {
      let test_attacks = match *origin_piece.color() {
        Color::Black => only_somes(vec![
          plot_move_from(origin, vec![Left, Down]),
          plot_move_from(origin, vec![Right, Down]),  
        ]),
        Color::White => only_somes(vec![
          plot_move_from(origin, vec![Left, Up]),
          plot_move_from(origin, vec![Right, Up]), 
        ]),
      };
      let test_moves = match *origin_piece.color() {
        Color::Black => only_somes(vec![
          plot_move_from(origin, vec![Down]),
          plot_move_from(origin, vec![Down, Down]),
        ]),
        Color::White => only_somes(vec![
          plot_move_from(origin, vec![Up]),
          plot_move_from(origin, vec![Up, Up]),
        ]),
      };
      let mut rv = vec![];
      for idx in test_attacks {
        if is_tile_opponent(idx, origin_piece, pieces) {
          rv.push(idx);
        }
      }
      for idx in test_moves {
        if is_tile_empty(idx, pieces) {
          rv.push(idx);
        }
      }
      return Ok(rv)
    }
    Err(format!("Tile {} is empty", board.tiles()[origin]))
  }
  fn pins(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin> {
    let rv = Vec::new();

    rv
  }
  fn forks(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork> {
    let rv = Vec::new();

    rv
  }
}
