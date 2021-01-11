use std::collections::HashMap;

use crate::v4::board::Board;
use crate::v4::piece::{
  Color::*,
  ChessPiece,
};
use crate::v4::history::GameHistory;

#[derive(Debug)]
pub struct Game {
  pub board: Board,
  pub pieces: HashMap<usize, ChessPiece>,
  pub history: GameHistory,
}


// Constructor
impl Game {
  pub fn new() -> Game {
    let mut game = Game {
      board: Board::new(),
      pieces: HashMap::new(),
      history: GameHistory::new()
    };

    game.add_piece(ChessPiece::rook(White, "a1")).unwrap();
    game.add_piece(ChessPiece::knight(White, "b1")).unwrap();
    game.add_piece(ChessPiece::bishop(White, "c1")).unwrap();
    game.add_piece(ChessPiece::queen(White, "d1")).unwrap();
    game.add_piece(ChessPiece::king(White, "e1")).unwrap();
    game.add_piece(ChessPiece::bishop(White, "f1")).unwrap();
    game.add_piece(ChessPiece::knight(White, "g1")).unwrap();
    game.add_piece(ChessPiece::rook(White, "h1")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "a2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "b2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "c2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "d2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "e2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "f2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "g2")).unwrap();
    game.add_piece(ChessPiece::pawn(White, "h2")).unwrap();

    game.add_piece(ChessPiece::rook(Black, "a8")).unwrap();
    game.add_piece(ChessPiece::knight(Black, "b8")).unwrap();
    game.add_piece(ChessPiece::bishop(Black, "c8")).unwrap();
    game.add_piece(ChessPiece::queen(Black, "d8")).unwrap();
    game.add_piece(ChessPiece::king(Black, "e8")).unwrap();
    game.add_piece(ChessPiece::bishop(Black, "f8")).unwrap();
    game.add_piece(ChessPiece::knight(Black, "g8")).unwrap();
    game.add_piece(ChessPiece::rook(Black, "h8")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "a7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "b7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "c7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "d7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "e7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "f7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "g7")).unwrap();
    game.add_piece(ChessPiece::pawn(Black, "h7")).unwrap();

    game
  }
  fn add_piece(&mut self, piece: ChessPiece) -> Result<Option<ChessPiece>, String> {
    match self.board.index_of(&piece.position()) {
      Some(idx) => Ok(self.pieces.insert(idx, piece)),
      None      => Err("Cannot add piece to non-existant tile".to_owned())
    }
  }
}
