use std::collections::HashMap;

use super::board::Board;
use super::piece::ChessPiece;
use super::piece::Color::Black;
use super::piece::Color::White;

#[derive(Debug, Clone)]
struct GameMove(String, String);

#[derive(Debug, Clone)]
pub struct Game {
  board: Board,
  pieces: HashMap<usize, ChessPiece>,
  history: Vec<GameMove>,
}

// Public
impl Game {
  pub fn new() -> Game {
    let mut game = Game {
      board: Board::new(),
      pieces: HashMap::new(),
      history: Vec::new()
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

  pub fn peek_tile(&self, pos: &str) -> Option<&ChessPiece> {
    match self.board.index_of(pos) {
      Some(idx) => self.pieces.get(&idx),
      None => None
    }
  }

  pub fn move_piece(&mut self, from: &str, to: &str) -> Result<(), String> {
    match self.board.index_of(from) {
      Some(idx) => match self.board.index_of(to) {
        Some(_) => match self.pieces.get_mut(&idx) {
          Some(piece) => {
            let is_valid_move = true;
            if is_valid_move {
              self.history.push(GameMove(from.to_owned(), to.to_owned()));
              piece.update_position(to);
            }
            else {
              return Err("Invalid move".to_owned());
            }
          }
          None => return Err(format!("There is no piece on {}", from))
        }
        None => return Err("Cannot move to tile that does not exist".to_owned())
      }
      None => return Err("Cannot move from tile that does not exist".to_owned())
    };
    let from_idx = self.board.index_of(from).unwrap();
    let moved_piece = self.pieces.remove(&from_idx).unwrap();
    self.add_piece(moved_piece)
  }

  pub fn undo_move(&mut self) -> Result<(), String> {
    if let Some(GameMove(from, to)) = self.history.pop() {
      return self.move_piece(&to, &from);
    }
    Err("Move history is empty".to_owned())
  }

  pub fn render_board(&self) -> String {
    let mut rv = String::new();
    let horizotal_divider = || {
      let mut rv = String::from("");
      rv.push_str(&format!("\n |"));
      for _ in 0..8 {
        rv.push_str("-----|");
      }
      rv.push('\n');
      rv
    };
    for (idx, tile) in self.board.tiles().iter().enumerate() {
      if idx % 8 == 0 {
        rv.push_str(&horizotal_divider());
        rv.push_str(&tile.chars().nth(1).unwrap().to_string());
      }
      match self.pieces.get(&idx) {
        Some(piece) => rv.push_str(&format!("| {} ", piece)),
        None =>        rv.push_str(&format!("|     ")),
      };
      if (idx + 1) % 8 == 0 {
        rv.push('|');
      }
    }
    rv.push_str(&horizotal_divider());
    rv.push_str(
      "    a     b     c     d     e     f     g     h"
    );
    rv
  }
}

// Private
impl Game {
  fn add_piece(&mut self, piece: ChessPiece) -> Result<(), String> {
    if let Some(idx) = self.board.index_of(&piece.position()) {
      self.pieces.insert(idx, piece);
    } else {
      return Err("Cannot add chess piece to non-existant tile".to_owned());
    }
    Ok(())
  }
}