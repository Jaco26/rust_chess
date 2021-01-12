use super::ScanCtx;
use super::TileVector;
use super::Direction;

pub type Directions = Vec<Direction>;

pub trait Scan {
  fn scan<'a>(ctx: &ScanCtx) -> Vec<TileVector<'a>>;
}

