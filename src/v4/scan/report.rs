use crate::v4::piece::{
  Color,
  ChessPieceKind,
};
use super::{
  Pin,
  TileVector,
  Capturable,
};


#[derive(Debug)]
pub struct ScanReport {
  pub origin: usize,
  pub origin_kind: ChessPieceKind,
  pub origin_color: Color,
  pub available_tiles: Vec<usize>,
  pub capturables: Vec<Capturable>,
  pub pins: Vec<Pin>,
}

impl ScanReport {
  pub fn new(
    origin: usize,
    origin_kind: ChessPieceKind,
    origin_color: Color,
    tile_vectors: Vec<TileVector>
  ) -> ScanReport {
    let mut rv = ScanReport {
      origin,
      origin_kind,
      origin_color,
      available_tiles: vec![],
      capturables: vec![],
      pins: vec![],
    };

    for mut v in tile_vectors {
      rv.available_tiles.append(&mut v.available_tiles);
      if v.capturable.is_some() {
        rv.capturables.push(v.capturable.unwrap());
      }
      if v.pin.is_some() {
        rv.pins.push(v.pin.unwrap());
      }
    }

    rv
  }

  pub fn is_fork(&self) -> bool {
    self.capturables.len() > 1
  }
}

