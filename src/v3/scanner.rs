
pub enum Direction { Up, Right, Down, Left }

/// Calculate an index given a starting point an vertor of steps to follow
pub fn plot_move_from(origin: usize, steps: Vec<Direction>) -> Option<usize> {
  let mut rv = origin as i32;

  for step in steps {
    let proposed = match step {
      Direction::Up => rv - 8,
      Direction::Right => rv + 1,
      Direction::Down => rv + 8,
      Direction::Left => rv - 1,
    };
    if proposed < 0 || proposed > 63 {
      return None
    }
    rv = proposed;
  }

  Some(rv as usize)
}


fn transform_origin(origin: usize, direction: &Direction) -> Option<usize> {
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
pub fn generate_search_vector(origin: usize, pattern: &Vec<Direction>) -> Vec<usize> {
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
  rv.push(origin);
  rv.append(&mut generate_search_vector(origin, pattern));
  rv
}

