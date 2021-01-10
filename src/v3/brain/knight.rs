use crate::v3::board::Board;
use crate::v3::history::GameHistory;
use super::prelude::{
  Pin,
  Fork,
  Pieces,
  TheThinkyBits,
  TheThinkyBitsV2,
};
use super::scanner::{
  self,
  Direction::{self, Right, Left},
  TileVector,
};

pub struct KnightBrain;

impl TheThinkyBitsV2 for KnightBrain {
  fn scan_board(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    match pieces.get(&origin) {
      Some(piece) => {

        let forward = || Direction::forward(piece);
        let back = || Direction::back(piece);

        let directions = vec![
          vec![forward(), forward(), Right],
          vec![forward(), Right, Right],
          vec![back(), Right, Right],
          vec![back(), back(), Right],
          vec![back(), back(), Left],
          vec![back(), Left, Left],
          vec![forward(), Left, Left],
          vec![forward(), forward(), Left],
        ];

        let rv = directions
          .iter()
          .fold(Ok(vec![]), |acc, directions| -> Result<Vec<usize>, String> {
            let mut tile_vector = TileVector::new(pieces, origin, directions, Some(1))?;
            let mut acc = acc?;
            acc.append(&mut tile_vector.reachable_tiles());
            Ok(acc)
          })?;
        
        Ok(rv)
      }
      None => Err(format!("No piece found at: {}", board.tile_at(origin).unwrap()))
    }
  }
  fn prioritize_moves() -> () {

  }
}



impl TheThinkyBits for KnightBrain {
  fn available_tiles(origin: usize, board: &Board, pieces: &Pieces, history: Option<&GameHistory>) -> Result<Vec<usize>, String> {
    match pieces.get(&origin) {
      Some(piece) => {

        let forward = || Direction::forward(piece);
        let back = || Direction::back(piece);

        let directions = vec![
          vec![forward(), forward(), Right],
          vec![forward(), Right, Right],
          vec![back(), Right, Right],
          vec![back(), back(), Right],
          vec![back(), back(), Left],
          vec![back(), Left, Left],
          vec![forward(), Left, Left],
          vec![forward(), forward(), Left],
        ];

        let mut scans = directions
          .iter()
          .fold(vec![], |mut acc, x| {
            acc.push(
              scanner::scan_tiles(
                origin, piece.color(), pieces, x, Some(1)
              )
            );
            acc
          });

        println!("Knight::available_tiles -> \n{:#?}", scans);

        Ok(
          scans.iter().fold(vec![], |mut acc, scan| {
            acc.append(&mut scan.reachable_tiles.clone());
            acc
          })
        )
      }
      None => Err(format!("No piece found at: {}", board.tile_at(origin).unwrap()))
    }
  }
  fn pins(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Pin> {
    let mut rv = vec![];

    rv
  }
  fn forks(tiles: Vec<usize>, board: &Board, pieces: &Pieces) -> Vec<Fork> {
    let mut rv = vec![];

    rv
  }
}
