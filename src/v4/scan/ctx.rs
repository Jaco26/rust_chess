use crate::v4::piece::Color;
use crate::v4::board::Board;
use crate::v4::history::GameHistory;

#[derive(Debug, Clone)]
pub struct ScanCtx<'a> {
  pub origin: usize,
  pub origin_color: &'a Color,
  pub board: &'a Board,
  pub history: &'a GameHistory,
}

impl<'a> ScanCtx<'a> {
  pub fn new(origin: usize, board: &'a Board, history: &'a GameHistory) -> Result<ScanCtx<'a>, String> {
    match board.pieces.get(&origin) {
      Some(piece) => Ok(
        ScanCtx {
          board,
          history,
          origin,
          origin_color: piece.color(),
        }
      ),
      None => Err(format!("No piece at origin {}", origin))
    }
  }
}