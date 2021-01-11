use super::ScanCtx;
use super::TileVector;
use super::Direction;

pub type Directions = Vec<Direction>;

pub trait Scan {
  fn tile_vectors<'a>(ctx: &ScanCtx) -> Vec<TileVector<'a>>;
}

