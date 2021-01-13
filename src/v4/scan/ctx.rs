use crate::v4::piece::Color;
use crate::v4::board::Board;

#[derive(Debug, Clone)]
pub struct ScanCtx<'a> {
  pub origin: usize,
  pub origin_color: &'a Color,
  pub board: &'a Board,
}

impl<'a> ScanCtx<'a> {
  pub fn new(origin: usize, board: &'a Board) -> Result<ScanCtx<'a>, String> {
    match board.pieces.get(&origin) {
      Some(piece) => Ok(
        ScanCtx {
          board,
          origin,
          origin_color: piece.color(),
        }
      ),
      None => Err(format!("No piece at origin {}", origin))
    }
  }
}