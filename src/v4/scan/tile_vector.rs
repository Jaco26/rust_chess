use crate::v4::piece::ChessPieceKind;

use super::Direction;
use super::Directions;
use super::Capturable;
use super::ctx::ScanCtx;


#[derive(Debug, Clone)]
pub struct TileVector<'a, 'b> {
  ctx: &'a ScanCtx<'a>,
  pub available_tiles: Vec<usize>,
  pub capturable: Option<Capturable<'b>>,
  pub pin: Option<Capturable<'b>>,
}


impl<'a, 'b> TileVector<'a, 'b> {
  pub fn new(ctx: &'a ScanCtx, directions: &Directions, count: Option<usize>) -> TileVector<'a, 'b> {
    let mut rv = TileVector { ctx, available_tiles: vec![], capturable: None, pin: None };

    for tile_idx in util::get_tile_vector_tiles(ctx.origin, directions, count) {
      match ctx.board.pieces.get(&tile_idx) {
        Some(piece) => {
          if piece.color() == ctx.origin_color {
            break
          } else {
            match rv.capturable {
              Some(_) => match rv.pin {
                Some(_) => {
                  break
                }
                None => {
                  rv.pin = Some(Capturable::new(tile_idx, piece.kind()))
                }
              }
              None => {
                rv.available_tiles.push(tile_idx);
                rv.capturable = Some(Capturable::new(tile_idx, piece.kind()));
              }
            }
          }
        }
        None => {
          match rv.capturable {
            Some(_) => { }
            None => {
              rv.available_tiles.push(tile_idx);
            }
          }
        }
      }
    }

    rv
  }
}


mod util {
  use super::{Direction, Directions};

  fn transform_origin(origin: usize, direction: &Direction) -> Option<usize> {
    let origin = origin as i32;

    let proposed = match direction {
      Direction::Up =>    origin - 8,
      Direction::Right => origin + 1,
      Direction::Down =>  origin + 8,
      Direction::Left =>  origin - 1,
    };

    match direction {
      Direction::Left if (proposed + 1) % 8 == 0 => return None,
      Direction::Right if proposed % 8 == 0 => return None,
      _ if proposed < 0 || proposed > 63 => return None,
      _ => { }
    };
  
    Some(proposed as usize)
  }

  pub fn get_tile_vector_tiles(origin: usize, directions: &Directions, count: Option<usize>) -> Vec<usize> {
    let mut rv = vec![];
    let mut origin = origin;
    for step in directions {
      match transform_origin(origin, step) {
        Some(idx) => {
          origin = idx
        }
        None => return rv
      }
    }
    match count {
      Some(count) if count > 0 => {
        rv.push(origin);
        rv.append(&mut get_tile_vector_tiles(origin, directions, Some(count - 1)));
        rv
      }
      Some(_) => { // can assume count is 0
        rv
      }
      None => {
        rv.push(origin);
        rv.append(&mut get_tile_vector_tiles(origin, directions, None));
        rv
      }
    }
  }
}