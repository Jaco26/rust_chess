mod ctx;
mod tile_vector;

pub use ctx::ScanCtx;
pub use tile_vector::TileVector;


pub enum Direction { Up, Right, Down, Left }


pub type Directions = Vec<Direction>;


pub struct Pin;
pub struct Fork;


pub struct ScanReport {
  pub origin: usize,
  pub available_tiles: Vec<usize>,
  pub capturable_tiles: Vec<usize>,
  pub pins: Vec<Pin>,
  pub forks: Vec<Fork>
}

impl ScanReport {
  pub fn new(origin: usize) -> ScanReport {
    ScanReport {
      origin,
      available_tiles: vec![],
      capturable_tiles: vec![],
      pins: vec![],
      forks: vec![],
    }
  } 
}


pub trait Scan {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String>;
}

