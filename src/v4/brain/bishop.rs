use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  Pin,
  Fork,
  TileVector,
  Direction::*,
};

pub struct BishopBrain;

impl<'a> Scan<'a> for BishopBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport<'a>, String> {
    let tile_vectors = vec![
      TileVector::new(ctx, &vec![Up, Left], None),
      TileVector::new(ctx, &vec![Up, Right], None),
      TileVector::new(ctx, &vec![Down, Left], None),
      TileVector::new(ctx, &vec![Down, Right], None),
    ];

    let mut rv: ScanReport<'a> = ScanReport::new(ctx.origin);

    for mut v in tile_vectors {
      rv.available_tiles.append(&mut v.available_tiles);
      if v.capturable.is_some() {
        rv.pins.push(v.capturable.unwrap());
      }
      
    }

    Ok(rv)
  }
}