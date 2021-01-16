use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  TileVector,
  Direction::*,
};

pub struct KnightBrain;

impl<'a> Scan<'a> for KnightBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {
    Ok(
      ScanReport::new(
        ctx.origin, 
        ctx.origin_kind.clone(),
        ctx.origin_color.clone(),
        vec![
          TileVector::new(ctx, &vec![Up, Up, Left], Some(1)),
          TileVector::new(ctx, &vec![Up, Up, Right], Some(1)),
          TileVector::new(ctx, &vec![Right, Right, Up], Some(1)),
          TileVector::new(ctx, &vec![Right, Right, Down], Some(1)),
          TileVector::new(ctx, &vec![Down, Down, Right], Some(1)),
          TileVector::new(ctx, &vec![Down, Down, Left], Some(1)),
          TileVector::new(ctx, &vec![Left, Left, Down], Some(1)),
          TileVector::new(ctx, &vec![Left, Left, Up], Some(1)),
        ]
      )
    )
  }
}