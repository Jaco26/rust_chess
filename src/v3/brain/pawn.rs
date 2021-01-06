use crate::v3::board::Board;
use crate::v3::history::GameHistory;
use crate::v3::history::GameMove;
use crate::v3::brain::prelude::*;
use crate::v3::piece::Color;
use crate::v3::brain::util;
use crate::v3::scanner::Direction;
use crate::v3::scanner::recursive_tile_vector;
use crate::v3::scanner::transform_origin;
use crate::v3::piece::ChessPieceKind;

pub struct PawnBrain;

impl TheThinkyBits for PawnBrain {
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    
    if history.is_none() {
      return Err("ProgrammingError: You must provide game history for PawnBrain::available_tiles".to_owned());
    }

    let history = history.unwrap();

    if let Some(piece) = pieces.get(&origin) {
      use Direction::*;

      let mut attacks = vec![];
      attacks.append(
        &mut recursive_tile_vector(origin, &vec![piece.forward_direction(), Left], Some(1))
      );
      attacks.append(
        &mut recursive_tile_vector(origin, &vec![piece.forward_direction(), Right], Some(1))
      );

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

      // check "en passant"
      if util::is_on_rank(5, piece, board) {
        if let Some(last_move) = history.last() {
          if let Some(moved_piece) = pieces.get(&last_move.to) {
            let is_same_color = util::is_same_color(piece, moved_piece);
            let is_pawn = moved_piece.kind() == &ChessPieceKind::Pawn;
            let moved_to_rank_4 = {
              board.rank_of(moved_piece).unwrap() == 4
            };
            let moved_from_rank_2 = {
              board.rank_of_index(last_move.from, moved_piece.color()).unwrap() == 2
            };
            let en_passant_move_option: Option<usize> = {
              let piece_idx = board.index_of(&piece.position()).unwrap();
              let idx_to_left = transform_origin(piece_idx, &Direction::Left).unwrap();
              let idx_to_right = transform_origin(piece_idx, &Direction::Right).unwrap();
              let mut rv = None;
              if idx_to_left == last_move.to {
                rv = transform_origin(idx_to_left, &piece.forward_direction());
              }
              else if idx_to_right == last_move.to {
                rv = transform_origin(idx_to_right, &piece.forward_direction());
              }
              rv
            };
            if is_pawn && 
              !is_same_color && 
              moved_from_rank_2 && 
              moved_to_rank_4 &&
              en_passant_move_option.is_some()
            {
              attacks.push(en_passant_move_option.unwrap());
            }
          }
        }
      }

      let n_steps = match util::is_on_rank(2, piece, board) {
        true => 2,
        false => 1
      };

      let mut forward_moves = vec![];
      for idx in recursive_tile_vector(origin, &vec![piece.forward_direction()], Some(n_steps)) {
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