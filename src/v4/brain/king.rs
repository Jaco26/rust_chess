use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  TileVector,
  Direction::*,
};

pub struct KingBrain;

impl<'a> Scan<'a> for KingBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {
    Ok(
      ScanReport::new(ctx.origin, vec![
        TileVector::new(ctx, &vec![Up, Left], Some(1)),
        TileVector::new(ctx, &vec![Up, Right], Some(1)),
        TileVector::new(ctx, &vec![Down, Left], Some(1)),
        TileVector::new(ctx, &vec![Down, Right], Some(1)),
        TileVector::new(ctx, &vec![Up], Some(1)),
        TileVector::new(ctx, &vec![Right], Some(1)),
        TileVector::new(ctx, &vec![Down], Some(1)),
        TileVector::new(ctx, &vec![Left], Some(1)),
      ])
    )
  }
}