use std::cmp::Ordering;
use crate::v4::piece::{
  Color,
  ChessPieceKind,
};
use super::{
  Pin,
  TileVector,
  Capturable,
  CapturableValue::{
    Check,
    Points,
  },
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

  /// A somewhat arbitrary quantification as means of determining relative value
  /// of moving to a tile at self.origin
  pub fn score(&self) -> usize {
    let variety = self.available_tiles.len();
    
    let fork_factor = match self.is_fork() {
      true => self.capturables.len() * 2,
      false => 0
    };

    let capturables_points = self.capturables
      .iter()
      .fold(0, |mut acc, x| acc + match x.value() {
        Points(n) => n,
        Check => 50
      });

    let pin_points = self.pins
      .iter()
      .fold(0, |mut acc, pin| {
        acc += match pin.pinned.value() {
          Points(n) => n,
          Check => 50,
        };
        acc += match pin.shielded.value() {
          Points(n) => n,
          Check => 50,
        };
        acc
      });

    variety + fork_factor + capturables_points + pin_points
  }
}

impl Eq for ScanReport { }

impl PartialEq for ScanReport {
  fn eq(&self, other: &Self) -> bool {
    self.score() == other.score()
  }
}

impl PartialOrd for ScanReport {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ScanReport {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score().cmp(&other.score())
  }
}
