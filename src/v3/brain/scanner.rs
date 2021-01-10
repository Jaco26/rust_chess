use crate::v3::brain::{
  util,
  prelude::{
    Scan,
    Pieces,
    Capturable,
    Fork,
    Pin,
  }
};
use crate::v3::piece::{
  ChessPiece,
  Color,
};



#[derive(Clone)]
pub enum Direction { Up, Right, Down, Left }

impl Direction {
  pub fn forward(piece: &ChessPiece) -> Direction {
    match piece.color() {
      Color::Black => Direction::Down,
      Color::White => Direction::Up,
    }
  }
  pub fn back(piece: &ChessPiece) -> Direction {
    match piece.color() {
      Color::Black => Direction::Up,
      Color::White => Direction::Down,
    }
  }
}

type Directions = Vec<Direction>;

pub fn transform_origin(origin: usize, direction: &Direction) -> Option<usize> {
  let origin = origin as i32;

  let proposed = match direction {
    Direction::Up => origin - 8,
    Direction::Right => origin + 1,
    Direction::Down => origin + 8,
    Direction::Left => origin - 1,
  };

  match direction {
    Direction::Left if (proposed + 1) % 8 == 0 => return None,
    Direction::Right if proposed % 8 == 0 => return None,
    _ if proposed < 0 || proposed > 63 => return None,
    _ => { }
  };

  Some(proposed as usize)
}

/// Recursively generate a vector of tile indices based on the provided pattern
pub fn get_tile_vector(origin: usize, directions: &Directions, count: Option<usize>) -> Vec<usize> {
  let mut rv = vec![];
  let mut origin = origin;
  for step in directions {
    match transform_origin(origin, &step) {
      Some(idx) => {
        origin = idx;
      }
      None => return rv
    };
  }
  match count {
    Some(count) if count > 0 => {
      rv.push(origin);
      rv.append(&mut get_tile_vector(origin, directions, Some(count - 1)));
      rv
    }
    Some(_) => { // can assume count is 0
      rv
    }
    None => {
      rv.push(origin);
      rv.append(&mut get_tile_vector(origin, directions, None));
      rv
    }
  }
}


pub fn get_reachable_tiles(origin_color: &Color, path: &Vec<usize>, pieces: &Pieces) -> (Vec<usize>, Option<Capturable>) {
  let mut tiles = vec![];
  let mut capturable = None;
  for idx in path {
    if let Some(other_piece) = util::peek_tile(*idx, pieces) {
      if other_piece.color() != origin_color {
        capturable = Some(
          Capturable { idx: *idx, kind: other_piece.kind().clone() }
        );
        tiles.push(*idx);
      }
      break;
    }
    tiles.push(*idx);
  }
  (tiles, capturable)
}



pub fn scan_tiles(origin: usize, origin_color: &Color, pieces: &Pieces, directions: &Directions, count: Option<usize>) -> Scan {
  let tile_vector = get_tile_vector(origin, directions, count);

  let (reachable_tiles, capturable) = get_reachable_tiles(
    origin_color,
    &tile_vector,
    pieces,
  );

  Scan::new(origin, tile_vector, reachable_tiles, capturable)
}

pub struct TileVectorCtx<'a> {
  pieces: &'a Pieces,
  origin: usize,
  origin_color: &'a Color,
}

impl<'a> TileVectorCtx<'a> {
  pub fn new(pieces: &'a Pieces, origin: usize, origin_color: &'a Color) -> TileVectorCtx<'a> {
    TileVectorCtx { pieces, origin, origin_color }
  }
}


pub struct TileVector<'a> {
  ctx: TileVectorCtx<'a>,
  tiles: Vec<usize>,
  cursor: usize,
}

impl<'a> TileVector<'a> {
  pub fn new(pieces: &'a Pieces, origin: usize, directions: &Directions, count: Option<usize>) -> Result<TileVector<'a>, String> {
    match pieces.get(&origin) {
      Some(piece) => {
        Ok(
          TileVector {
            ctx: TileVectorCtx::new(pieces, origin, piece.color()),
            tiles: get_tile_vector(origin, directions, count),
            cursor: 0
          }
        )
      }
      None => Err(format!("[TileVector- OriginError]: No piece found at origin {}", origin))
    }
  }
  pub fn next_piece(&mut self) -> Option<&ChessPiece> {
    while self.cursor < self.tiles.len() {
      if let Some(piece) = self.ctx.pieces.get(&self.tiles[self.cursor]) {
        self.cursor += 1;
        return Some(piece)
      }
      self.cursor += 1
    }
    self.cursor = 0;
    None
  }
  pub fn reachable_tiles(&mut self) -> Vec<usize> {
    let origin_color = self.ctx.origin_color;
    let rv = match self.next_piece() {
      Some(piece) => match piece.color() == origin_color {
        true => self.tiles[0..(self.cursor - 1)].to_vec(),
        false => self.tiles[0..(self.cursor)].to_vec(),
      }
      None => self.tiles.clone()
    };
    self.cursor = 0;
    rv
  }
}