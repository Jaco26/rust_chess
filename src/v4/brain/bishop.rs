use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  TileVector,
  Direction::*,
};

pub struct BishopBrain;

impl<'a> Scan<'a> for BishopBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {
    Ok(
      ScanReport::new(ctx.origin, vec![
        TileVector::new(ctx, &vec![Up, Left], None),
        TileVector::new(ctx, &vec![Up, Right], None),
        TileVector::new(ctx, &vec![Down, Left], None),
        TileVector::new(ctx, &vec![Down, Right], None),
      ])
    )
  }
}