
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

