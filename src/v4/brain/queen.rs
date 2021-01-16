use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  TileVector,
  Direction::*,
};

pub struct QueenBrain;

impl<'a> Scan<'a> for QueenBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {
    Ok(
      ScanReport::new(
        ctx.origin, 
        ctx.origin_kind.clone(),
        ctx.origin_color.clone(),
        vec![
          TileVector::new(ctx, &vec![Up, Left], None),
          TileVector::new(ctx, &vec![Up, Right], None),
          TileVector::new(ctx, &vec![Down, Left], None),
          TileVector::new(ctx, &vec![Down, Right], None),
          TileVector::new(ctx, &vec![Up], None),
          TileVector::new(ctx, &vec![Right], None),
          TileVector::new(ctx, &vec![Down], None),
          TileVector::new(ctx, &vec![Left], None),
        ]
      )
    )
  }
}