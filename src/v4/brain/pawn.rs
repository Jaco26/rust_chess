use crate::v4::piece::{
  Color,
  ChessPieceKind,
};
use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
  Capturable,
  TileVector,
  Direction::{self, *}
};

pub struct PawnBrain;

impl<'a> Scan<'a> for PawnBrain {
  fn scan(ctx: &ScanCtx) -> Result<ScanReport, String> {

    let forward = || match ctx.origin_color {
      Color::Black => Down,
      Color::White => Up,
    };

    let rank = ctx.board.rank_of_index(ctx.origin, &ctx.origin_color).unwrap();

    let forward_count: usize = match rank {
      2 => 2,
      _ => 1,
    };

    let mut rv = ScanReport::new(
      ctx.origin, 
      vec![
        TileVector::new(ctx, &vec![forward()], Some(forward_count)),
        TileVector::new(ctx, &vec![forward(), Left], Some(1)),
        TileVector::new(ctx, &vec![forward(), Right], Some(1)),
      ]
    );

    if rank == 5 {
      if let Some(prev_move) = ctx.history.last() {
        if let Some(moved_piece) = ctx.board.pieces.get(&prev_move.to) {
          let is_pawn = moved_piece.kind() == &ChessPieceKind::Pawn;
          let is_same_color = moved_piece.color() == ctx.origin_color;
          let moved_to_rank_4 = {
            ctx.board.rank_of(moved_piece).unwrap() == 4
          };
          let moved_from_rank_2 = {
            ctx.board.rank_of_index(prev_move.from, moved_piece.color()).unwrap() == 2
          };
          let en_passant_move_option: Option<usize> = {
            match transform_origin(ctx.origin, Left) {
              Some(idx_to_left) if idx_to_left == prev_move.to => {
                transform_origin(idx_to_left, forward())
              }
              _ => match transform_origin(ctx.origin, Right) {
                Some(idx_to_right) if idx_to_right == prev_move.to => {
                  transform_origin(idx_to_right, forward())
                }
                _ => None
              }
            }
          };
          if is_pawn &&
            !is_same_color &&
            moved_from_rank_2 &&
            moved_to_rank_4 &&
            en_passant_move_option.is_some()
          {
            let capturable_tile = en_passant_move_option.unwrap();
            rv.available_tiles.push(capturable_tile);
            rv.capturables.push(Capturable::new(capturable_tile, ChessPieceKind::Pawn));
          }
        }
      }
    }

    Ok(rv)
  }
}


fn transform_origin(origin: usize, direction: Direction) -> Option<usize> {
  let origin = origin as i32;

  let proposed = match direction {
    Direction::Up =>    origin - 8,
    Direction::Right => origin + 1,
    Direction::Down =>  origin + 8,
    Direction::Left =>  origin - 1,
  };

  match direction {
    Direction::Left if (proposed + 1) % 8 == 0 => return None,
    Direction::Right if proposed % 8 == 0 => return None,
    _ if proposed < 0 || proposed > 63 => return None,
    _ => { }
  };

  Some(proposed as usize)
}