use crate::v3::brain::{
  util,
  prelude::{
    Pieces,
    Fork,
    Pin,
  }
};


#[derive(Clone)]
pub enum Direction { Up, Right, Down, Left }

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
pub fn recursive_tile_vector(origin: usize, pattern: &Vec<Direction>, count: Option<usize>) -> Vec<usize> {
  let mut rv = vec![];
  let mut origin = origin;
  for step in pattern {
    match transform_origin(origin, step) {
      Some(idx) => {
        origin = idx;
      }
      None => return rv
    };
  }
  match count {
    Some(count) if count > 0 => {
      rv.push(origin);
      rv.append(&mut recursive_tile_vector(origin, pattern, Some(count - 1)));
      rv
    }
    Some(_) => { // can assume count is 0
      rv
    }
    None => {
      rv.push(origin);
      rv.append(&mut recursive_tile_vector(origin, pattern, None));
      rv
    }
  }
}

pub fn tile_vector(origin: usize, directions: &Directions) -> Vec<usize> {
  let mut rv = vec![];
  let mut origin = origin;
  for direction in directions {
    match transform_origin(origin, direction) {
      Some(idx) => {
        origin = idx
      }
      None => return rv
    }
  }
  rv.push(origin);
  rv.append(&mut tile_vector(origin, directions));
  rv
}


pub fn finite_tile_vector(origin: usize, directions: &Directions, count: usize) -> Vec<usize> {
  let mut rv = vec![];
  let mut origin = origin;
  for direction in directions {
    match transform_origin(origin, direction) {
      Some(idx) => {
        origin = idx
      }
      None => return rv
    }
  }
  if count > 0 {
    rv.push(origin);
    rv.append(&mut finite_tile_vector(origin, directions, count - 1));
  }
  rv
}


pub fn some_pin() -> Option<usize> {
  
  None
}

pub fn some_fork() {

}

pub fn stop_at_piece(origin: usize, tiles: &Vec<usize>, pieces: &Pieces) -> Vec<usize> {
  let mut rv = vec![];
  for idx in tiles {
    if util::peek_tile(*idx, pieces).is_some() {
      break;
    }
    rv.push(*idx)
  }
  rv
}