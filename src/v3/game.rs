use std::collections::HashMap;

use super::board::Board;
use super::piece::ChessPiece;
use super::piece::Color::Black;
use super::piece::Color::White;

#[derive(Debug)]
pub struct Game {
  board: Board,
  pieces: HashMap<usize, ChessPiece>
}

// Public
impl Game {
  pub fn new() -> Game {
    let mut game = Game { board: Board::new(), pieces: HashMap::new() };
    game.add_piece(ChessPiece::rook(White, "a8"));
    game.add_piece(ChessPiece::knight(White, "b1"));
    game.add_piece(ChessPiece::bishop(White, "c1"));
    game.add_piece(ChessPiece::queen(White, "d1"));
    game.add_piece(ChessPiece::king(White, "e1"));
    game.add_piece(ChessPiece::bishop(White, "f1"));
    game.add_piece(ChessPiece::knight(White, "g1"));
    game.add_piece(ChessPiece::rook(White, "h1"));
    game.add_piece(ChessPiece::pawn(White, "a2"));
    game.add_piece(ChessPiece::pawn(White, "b2"));
    game.add_piece(ChessPiece::pawn(White, "c2"));
    game.add_piece(ChessPiece::pawn(White, "d2"));
    game.add_piece(ChessPiece::pawn(White, "e2"));
    game.add_piece(ChessPiece::pawn(White, "f2"));
    game.add_piece(ChessPiece::pawn(White, "g2"));
    game.add_piece(ChessPiece::pawn(White, "h2"));

    game.add_piece(ChessPiece::rook(Black, "a8"));
    game.add_piece(ChessPiece::knight(Black, "b8"));
    game.add_piece(ChessPiece::bishop(Black, "c8"));
    game.add_piece(ChessPiece::queen(Black, "d8"));
    game.add_piece(ChessPiece::king(Black, "e8"));
    game.add_piece(ChessPiece::bishop(Black, "f8"));
    game.add_piece(ChessPiece::knight(Black, "g8"));
    game.add_piece(ChessPiece::rook(Black, "h8"));
    game.add_piece(ChessPiece::pawn(Black, "a7"));
    game.add_piece(ChessPiece::pawn(Black, "b7"));
    game.add_piece(ChessPiece::pawn(Black, "c7"));
    game.add_piece(ChessPiece::pawn(Black, "d7"));
    game.add_piece(ChessPiece::pawn(Black, "e7"));
    game.add_piece(ChessPiece::pawn(Black, "f7"));
    game.add_piece(ChessPiece::pawn(Black, "g7"));
    game.add_piece(ChessPiece::pawn(Black, "h7"));

    game
  }

  pub fn peek_tile(&self, pos: &str) -> Option<&ChessPiece> {
    match self.board.index_of(pos) {
      Some(idx) => self.pieces.get(&idx),
      None => None
    }
  }

  pub fn move_piece(&mut self, from: &str, to: &str) -> Result<(), &'static str> {
    if let Some(idx) = self.board.index_of(from) {
      if let Some(p) = self.pieces.get_mut(&idx) {
        let is_valid_move = true;
        // if valid move...
        if is_valid_move {
          p.update_position(to);
        } else {
          return Err("Invalid move");
        }
      } else {

        return Err("There is no piece on this tile");
      }
    } else {
      return Err("This tile does not exist");
    }
    let from_idx = self.board.index_of(from).unwrap();
    let moved_piece = self.pieces.remove(&from_idx).unwrap();
    self.add_piece(moved_piece)
  }
}

// Private
impl Game {
  fn add_piece(&mut self, piece: ChessPiece) -> Result<(), &'static str> {
    if let Some(idx) = self.board.index_of(&piece.position()) {
      self.pieces.insert(idx, piece);
    } else {
      return Err("Cannot add chess piece to non-existant tile");
    }
    Ok(())
  }
}