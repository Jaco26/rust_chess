use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  TileVector,
  Direction::*
};

pub struct PawnBrain;

impl<'a> Scan<'a> for PawnBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {

    // let history

    let mut rv = ScanReport::new(ctx.origin, vec![]);


    Ok(rv)
  }
}