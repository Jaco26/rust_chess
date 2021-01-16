mod ctx;
mod tile_vector;

use crate::v4::piece::ChessPieceKind;

pub use ctx::ScanCtx;
pub use tile_vector::TileVector;

#[derive(PartialEq)]
pub enum Direction { Up, Right, Down, Left }


pub type Directions = Vec<Direction>;


pub struct Pin;
pub struct Fork;

#[derive(Debug, Clone)]
pub struct Capturable {
  pub tile: usize,
  pub kind: ChessPieceKind
}

impl Capturable {
  pub fn new(tile: usize, kind: ChessPieceKind) -> Capturable {
    Capturable { tile, kind }
  }
}


pub struct ScanReport {
  pub origin: usize,
  pub available_tiles: Vec<usize>,
  pub capturables: Vec<Capturable>,
  pub pinned: Vec<Capturable>,
  pub forks: Vec<Fork>
}

impl ScanReport {
  pub fn new(origin: usize, tile_vectors: Vec<TileVector>) -> ScanReport {
    let mut rv = ScanReport {
      origin,
      available_tiles: vec![],
      capturables: vec![],
      pinned: vec![],
      forks: vec![],
    };

    for mut v in tile_vectors {
      rv.available_tiles.append(&mut v.available_tiles);
      if v.capturable.is_some() {
        rv.capturables.push(v.capturable.unwrap());
      }
      if v.pinned.is_some() {
        rv.pinned.push(v.pinned.unwrap());
      }
    }

    rv
  } 
}


pub trait Scan<'a> {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String>;
}

