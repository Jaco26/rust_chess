use crate::v3::board::Board;
use crate::v3::history::GameHistory;
use crate::v3::brain::prelude::*;
use crate::v3::piece::Color;
use crate::v3::brain::util;
use crate::v3::scanner::Direction;
use crate::v3::scanner::recursive_tile_vector;

pub struct PawnBrain;

impl TheThinkyBits for PawnBrain {
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    if let Some(piece) = pieces.get(&origin) {
      use Direction::*;

      let has_moved_from_row = |start_row: usize| match board.row_at(origin) {
        Some(row) if row == start_row => false,
        _ => true
      };

      let (forward_dir, forward_count) = match piece.color() {
        Color::Black => (
          Down,
          match has_moved_from_row(7) {
            true => 1,
            false => 2
          }
        ),
        Color::White => (
          Up,
          match has_moved_from_row(2) {
            true => 1,
            false => 2,
          }
        ),
      };

      let mut attacks = vec![];
      attacks.append(&mut recursive_tile_vector(origin, &vec![forward_dir.clone(), Left], Some(1)));
      attacks.append(&mut recursive_tile_vector(origin, &vec![forward_dir.clone(), Right], Some(1)));

      let mut attacks: Vec<usize> = attacks
        .iter()
        .fold(vec![], |mut acc, x| {
          if let Some(other_piece) = util::peek_tile(*x, pieces) {
            if !util::is_same_color(piece, other_piece) {
              acc.push(*x);
            }
          }
          acc
        });

      let mut forward_moves = vec![];
      for idx in recursive_tile_vector(origin, &vec![forward_dir.clone()], Some(forward_count)) {
        if let Some(_) = util::peek_tile(idx, pieces) {
          break;
        }
        forward_moves.push(idx);
      }

      let mut rv = vec![];

      rv.append(&mut attacks);
      rv.append(&mut forward_moves);

      return Ok(rv)
    }
    Err(format!("No piece found at: {}", board.tile_at(origin).unwrap()))
  }
  fn pins(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin> {
    let rv = vec![];

    rv
  }
  fn forks(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork> {
    let rv = vec![];

    rv
  }
}