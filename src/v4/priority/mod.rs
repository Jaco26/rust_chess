
use std::collections::BinaryHeap;
use crate::v4::game::Game;
use crate::v4::scan::{
  ScanCtx,
  ScanReport
};



pub fn prioritize_possible_moves(game: Game, origin: usize) -> (ScanReport, BinaryHeap<ScanReport>) {
  let from_pos = game.board.tile_name_at(origin).unwrap();

  let primary_scan = game.do_scan(from_pos).unwrap();

  let available_tiles: Vec<(usize, String)> = primary_scan.available_tiles
    .iter()
    .map(|idx| (*idx, game.board.tile_name_at(*idx).unwrap().to_owned()))
    .collect();

  let mut pq = BinaryHeap::new();

  for (idx, tile_name) in available_tiles {
    let mut game = game.clone();
    game.move_piece(from_pos, &tile_name, false).unwrap();
    let scan = game.do_scan(&tile_name).unwrap();
    pq.push(scan);
  }

  (primary_scan, pq)
}
