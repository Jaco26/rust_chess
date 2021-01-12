use crate::v4::board::Board;
use crate::v4::piece::{
  Color::*,
  ChessPiece,
};
use crate::v4::history::{
  GameMove,
  GameHistory,
};

#[derive(Debug)]
pub struct Game {
  pub board: Board,
  pub history: GameHistory,
}


// Constructor
impl Game {
  pub fn new() -> Game {
    let mut game = Game {
      board: Board::new(),
      history: GameHistory::new()
    };

    game.board.add_piece(ChessPiece::rook(White, "a1")).unwrap();
    game.board.add_piece(ChessPiece::knight(White, "b1")).unwrap();
    game.board.add_piece(ChessPiece::bishop(White, "c1")).unwrap();
    game.board.add_piece(ChessPiece::queen(White, "d1")).unwrap();
    game.board.add_piece(ChessPiece::king(White, "e1")).unwrap();
    game.board.add_piece(ChessPiece::bishop(White, "f1")).unwrap();
    game.board.add_piece(ChessPiece::knight(White, "g1")).unwrap();
    game.board.add_piece(ChessPiece::rook(White, "h1")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "a2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "b2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "c2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "d2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "e2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "f2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "g2")).unwrap();
    game.board.add_piece(ChessPiece::pawn(White, "h2")).unwrap();

    game.board.add_piece(ChessPiece::rook(Black, "a8")).unwrap();
    game.board.add_piece(ChessPiece::knight(Black, "b8")).unwrap();
    game.board.add_piece(ChessPiece::bishop(Black, "c8")).unwrap();
    game.board.add_piece(ChessPiece::queen(Black, "d8")).unwrap();
    game.board.add_piece(ChessPiece::king(Black, "e8")).unwrap();
    game.board.add_piece(ChessPiece::bishop(Black, "f8")).unwrap();
    game.board.add_piece(ChessPiece::knight(Black, "g8")).unwrap();
    game.board.add_piece(ChessPiece::rook(Black, "h8")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "a7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "b7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "c7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "d7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "e7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "f7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "g7")).unwrap();
    game.board.add_piece(ChessPiece::pawn(Black, "h7")).unwrap();

    game
  }

  pub fn render_board(&self) -> String {
    let mut rv = String::new();
    let horizontal_divider = || {
      "\n |-----|-----|-----|-----|-----|-----|-----|-----|\n"
    };
    for (idx, tile_name) in self.board.tile_names.iter().enumerate() {
      if idx % 8 == 0 {
        rv.push_str(&horizontal_divider());
        rv.push_str(&tile_name.chars().nth(1).unwrap().to_string());
      }
      match self.board.pieces.get(&idx) {
        Some(piece) => rv.push_str(&format!("| {} ", piece)),
        None        => rv.push_str("|     "),
      };
      if (idx + 1) % 8 == 0 {
        rv.push('|');
      }
    }
    rv.push_str(&horizontal_divider());
    rv.push_str(
      "    a     b     c     d     e     f     g     h    "
    );
    rv
  }

  pub fn move_piece(&mut self, from: &str, to: &str) -> Result<(), String> {
    match self.board.index_of(from) {
      Some(from_idx) => match self.board.index_of(to) {
        Some(to_idx) => match self.board.pieces.get_mut(&from_idx) {
          Some(piece) => {
            let is_valid_move = {
              true
            };
            if is_valid_move {
              piece.update_position(to);
            }
          }
          None => return Err(format!("There is no piece on {}", from))
        }
        None => return Err("Cannot move to tile that does not exist".to_owned())
      }
      None => return Err("Cannot move from tile that does not exist".to_owned())
    }
    let from_idx = self.board.index_of(from).unwrap();
    let to_idx = self.board.index_of(to).unwrap();
    let moved_piece = self.board.pieces.remove(&from_idx).unwrap();
    let maybe_capture = self.board.add_piece(moved_piece).unwrap();
    self.history.push(from_idx, to_idx, maybe_capture);
    Ok(())
  }

  pub fn undo_move(&mut self) -> Result<(), String> {
    if let Some(GameMove { from, to, capture }) = self.history.pop() {
      if let Some(mut piece) = self.board.pieces.remove(&to) {
        piece.update_position(&self.board.tile_names()[from]);
        if let Some(capture) = capture {
          self.board.add_piece(capture).map(|_| ())?;
        }
        return self.board.add_piece(piece).map(|_| ());
      }
      return Err("Could not undo move".to_owned());
    }
    Err("Move history is empty".to_owned())
  }

  pub fn history(&self, range: Option<std::ops::Range<usize>>) -> String {
    self.history
      .slice(range)
      .iter()
      .fold(String::new(), |mut acc, game_move| {
        let from_pos = &self.board.tile_names()[game_move.from];
        let to_pos = &self.board.tile_names()[game_move.to];
        if game_move.capture.is_some() {
          acc.push_str(
            &format!("({} -> {} captured {}), ", from_pos, to_pos, game_move.capture.as_ref().unwrap())
          );
        }
        else {
          acc.push_str(&format!("({} -> {}), ", from_pos, to_pos));
        }
        acc
      })
  }


}
