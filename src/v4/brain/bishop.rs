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

impl Scan for BishopBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {
    let tile_vectors = vec![
      TileVector::new(ctx, &vec![Up, Left], None),
      TileVector::new(ctx, &vec![Up, Right], None),
      TileVector::new(ctx, &vec![Down, Left], None),
      TileVector::new(ctx, &vec![Down, Right], None),
    ];

    let mut report = ScanReport::new(ctx.origin);

    report.available_tiles = tile_vectors
      .iter()
      .fold(Vec::new(), |mut acc, tv| {
        for (tile_idx, piece_opt) in tv.iter() {
          if let Some((color, _kind)) = piece_opt {
            if color == *ctx.origin_color {
              break
            }
            acc.push(tile_idx);
            break
          }
          acc.push(tile_idx);
        }
        acc
      });
    

    Ok(report)
  }
}