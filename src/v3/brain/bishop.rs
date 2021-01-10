use crate::v3::board::Board;
use crate::v3::piece::Color;
use crate::v3::history::GameHistory;
use super::prelude::{
  Pieces,
  TheThinkyBitsV2,
};
use super::scanner::{
  TileVector,
  Direction::{self, Right, Left},
};

pub struct BishopBrain;


pub struct ScanCtx<'a> {
  origin: usize,
  origin_color: &'a Color,
  pieces: &'a Pieces,
}



fn available_tiles(tile_vectors: Vec<TileVector>) -> Vec<usize> {
  tile_vectors
    .into_iter()
    .fold(Vec::new(), |mut acc, mut tile_vector| {
      acc.append(&mut tile_vector.reachable_tiles());
      acc
    })
}



impl TheThinkyBitsV2 for BishopBrain {
  fn scan_board(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    match pieces.get(&origin) {
      Some(piece) => {
        let forward = || Direction::forward(piece);
        let back = || Direction::back(piece);

        let get_tile_vectors = |directions: Vec<Vec<Direction>>| -> Result<Vec<TileVector>, String> {
          directions
            .into_iter()
            .map(|d| TileVector::new(pieces, origin, &d, None))
            .collect()
        };

        let tile_vectors = get_tile_vectors(vec![
          vec![forward(), Right],
          vec![forward(), Left],
          vec![back(), Right],
          vec![back(), Left]
        ])?;

        Ok(available_tiles(tile_vectors))
      }
      None => Err(format!("No piece found at: {}", board.tile_at(origin).unwrap()))
    }
  }
  fn prioritize_moves() -> () {

  }
}