mod pin;
mod ctx;
mod report;
mod capturable;
mod tile_vector;

pub use pin::Pin;
pub use ctx::ScanCtx;
pub use report::ScanReport;
pub use capturable::Capturable;
pub use capturable::CapturableValue;
pub use tile_vector::TileVector;


#[derive(PartialEq)]
pub enum Direction { Up, Right, Down, Left }

pub type Directions = Vec<Direction>;

pub trait Scan<'a> {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String>;
}

