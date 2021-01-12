pub mod prelude;

use crate::v4::piece::{
  Color,
  Pieces,
  ChessPieceKind,
};
use crate::v4::board::Board;

pub use self::prelude::*;

pub enum Direction { Up, Right, Down, Left }

#[derive(Debug, Clone)]
pub struct ScanCtx<'a> {
  pub origin: usize,
  pub origin_color: &'a Color,
  pub board: &'a Board,
}

impl<'a> ScanCtx<'a> {
  pub fn new(origin: usize, board: &'a Board) -> Result<ScanCtx<'a>, String> {
    match board.pieces.get(&origin) {
      Some(piece) => Ok(
        ScanCtx {
          board,
          origin,
          origin_color: piece.color(),
        }
      ),
      None => Err(format!("No piece at origin {}", origin))
    }
  }
}

#[derive(Debug, Clone)]
pub struct TileVector<'a> {
  ctx: &'a ScanCtx<'a>,
  tiles: Vec<usize>
}


impl<'a> TileVector<'a> {
  pub fn new(ctx: &'a ScanCtx, directions: &Directions, count: Option<usize>) -> TileVector<'a> {
    TileVector {
      ctx,
      tiles: util::get_tile_vector_tiles(ctx.origin, directions, count)
    }
  }
  pub fn iter(&self) -> TileVectorIterator<'a> {
    TileVectorIterator { ctx: self.ctx, tiles: self.tiles.clone(), cursor: 0 }
  }
}


#[derive(Debug, Clone)]
pub struct TileVectorIterator<'a> {
  ctx: &'a ScanCtx<'a>,
  cursor: usize,
  tiles: Vec<usize>
}

impl<'a> Iterator for TileVectorIterator<'a>{
  type Item = (usize, Option<(Color, ChessPieceKind)>);

  fn next(&mut self) -> Option<Self::Item> {
    let cursor = self.cursor;
    self.cursor += 1;
    match self.tiles.get(cursor) {
      Some(tile) => match self.ctx.board.pieces.get(tile) {
        Some(piece) => {
          let color = piece.color().clone();
          let kind = piece.kind().clone();
          Some((*tile, Some((color, kind))))
        }
        None => {
          Some((*tile, None))
        }
      }
      None => None
    }
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