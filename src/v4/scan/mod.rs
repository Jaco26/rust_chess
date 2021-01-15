mod ctx;
mod tile_vector;

use crate::v4::piece::ChessPieceKind;

pub use ctx::ScanCtx;
pub use tile_vector::TileVector;


pub enum Direction { Up, Right, Down, Left }


pub type Directions = Vec<Direction>;


pub struct Pin;
pub struct Fork;

#[derive(Debug, Clone)]
pub struct Capturable<'b> {
  tile: usize,
  kind: &'b ChessPieceKind
}

impl<'b> Capturable<'b> {
  pub fn new(tile: usize, kind: &'b ChessPieceKind) -> Capturable<'b> {
    Capturable { tile, kind }
  }
}


pub struct ScanReport<'a> {
  pub origin: usize,
  pub available_tiles: Vec<usize>,
  pub capturables: Vec<Capturable<'a>>,
  pub pins: Vec<Capturable<'a>>,
  pub forks: Vec<Fork>
}

impl<'a> ScanReport<'a> {
  pub fn new(origin: usize) -> ScanReport<'a> {
    ScanReport {
      origin,
      available_tiles: vec![],
      capturables: vec![],
      pins: vec![],
      forks: vec![],
    }
  } 
}


pub trait Scan<'a> {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport<'a>, String>;
}

