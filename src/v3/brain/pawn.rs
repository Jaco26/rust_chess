use crate::v3::board::Board;
use crate::v3::history::GameHistory;
use crate::v3::brain::prelude::*;
use crate::v3::brain::util;
use crate::v3::scanner;


pub struct PawnBrain;

impl TheThinkyBits for PawnBrain {
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    let rv = vec![];
    
    Ok(rv)
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