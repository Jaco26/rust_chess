
#[derive(Clone)]
pub enum Direction { Up, Right, Down, Left }

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

