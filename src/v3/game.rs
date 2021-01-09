use std::collections::HashMap;

use super::board::Board;

use super::history::GameMove;
use super::history::GameHistory;

use super::piece::ChessPiece;
use super::piece::ChessPieceKind;
use super::piece::Color::Black;
use super::piece::Color::White;

use super::scanner::Direction;
use super::scanner::recursive_tile_vector;

use super::brain::PawnBrain;
use super::brain::KnightBrain;
use super::brain::prelude::TheThinkyBits;



#[derive(Debug, Clone)]
pub struct Game {
  board: Board,
  active_pieces: HashMap<usize, ChessPiece>,
  history: GameHistory,
}

// Public
impl Game {
  pub fn new() -> Game {
    let mut game = Game {
      board: Board::new(),
      active_pieces: HashMap::new(),
      history: GameHistory::new(),
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
      Some(idx) => self.active_pieces.get(&idx),
      None => None
    }
  }

  pub fn move_piece(&mut self, from: &str, to: &str) -> Result<(), String> {
    let pieces = self.active_pieces.clone();
    match self.board.index_of(from) {
      Some(from_idx) => match self.board.index_of(to) {
        Some(to_idx) => match self.active_pieces.get_mut(&from_idx) {
          Some(piece) => {
            let is_valid_move = match piece.kind() {
              ChessPieceKind::Pawn => {
                let available_moves = PawnBrain::available_tiles(from_idx, &self.board, &pieces, Some(&self.history))?;
                if !available_moves.contains(&to_idx) {
                  return Err(format!("Invalid move: Cannot move Pawn from {} to {}", from, to));
                }
                true
              }
              ChessPieceKind::Knight => {
                let available_moves = KnightBrain::available_tiles(from_idx, &self.board, &pieces, None)?;
                if !available_moves.contains(&to_idx) {
                  return Err(format!("Invalid move: Cannot move Knight from {} to {}", from, to));
                }
                true
              }
              some_other_kind => {
                println!("[warning] move checking is not implmeneted for {:?}", some_other_kind);
                true
              }
            };
            if is_valid_move {
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
    let to_idx = self.board.index_of(to).unwrap();
    let moved_piece = self.active_pieces.remove(&from_idx).unwrap();
    let maybe_capture = self.add_piece(moved_piece).unwrap();
    self.history.push(from_idx, to_idx, maybe_capture);
    Ok(())
  }

  pub fn undo_move(&mut self) -> Result<(), String> {
    if let Some(GameMove { from, to, capture }) = self.history.pop() {
      if let Some(mut piece) = self.active_pieces.remove(&to) {
        piece.update_position(&self.board.tiles()[from]);
        if let Some(capture) = capture {
          self.add_piece(capture).map(|_| ())?;
        }
        return self.add_piece(piece).map(|_| ());
      }
      return Err("Could not undo move".to_owned());
    }
    Err("Move history is empty".to_owned())
  }

  pub fn render_board(&self) -> String {
    let mut rv = String::new();
    let horizotal_divider = || {
      "\n |-----|-----|-----|-----|-----|-----|-----|-----|\n"
    };
    for (idx, tile) in self.board.tiles().iter().enumerate() {
      if idx % 8 == 0 {
        rv.push_str(&horizotal_divider());
        rv.push_str(&tile.chars().nth(1).unwrap().to_string());
      }
      match self.active_pieces.get(&idx) {
        Some(piece) => rv.push_str(&format!("| {} ", piece)),
        None =>        rv.push_str(&format!("|     ")),
      };
      if (idx + 1) % 8 == 0 {
        rv.push('|');
      }
    }
    rv.push_str(&horizotal_divider());
    rv.push_str(
      "    a     b     c     d     e     f     g     h    "
    );
    rv
  }

  pub fn history(&self, range: Option<std::ops::Range<usize>>) -> String {
    self.history
      .slice(range)
      .iter()
      .fold(String::new(), |mut acc, game_move| {
        let from_pos = &self.board.tiles()[game_move.from];
        let to_pos = &self.board.tiles()[game_move.to];
        if game_move.capture.is_some() {
          acc.push_str(
            &format!("({} -> {} took {}), ", from_pos, to_pos, game_move.capture.as_ref().unwrap())
          );
        }
        else {
          acc.push_str(&format!("({} -> {}), ", from_pos, to_pos));
        }
        acc
      })
  }

  pub fn peek_available_moves(&self, pos: &str) -> Result<String, String> {
    let mut rv = String::new();
    match self.board.index_of(pos) {
      Some(idx) => {
        match self.active_pieces.get(&idx) {
          Some(piece) => {
            let moves = match piece.kind() {
              ChessPieceKind::Pawn => {
                PawnBrain::available_tiles(idx, &self.board, &self.active_pieces, Some(&self.history))?
              }
              ChessPieceKind::Knight => {
                KnightBrain::available_tiles(idx, &self.board, &self.active_pieces, None)?
              }
              _ => return Err("NotImplemented: Game.peek_available_moves is only implemented for pawns".to_owned())
            };

            for idx in moves {
              rv.push_str(&format!("{}, ", self.board.tile_at(idx).unwrap()));
            }
          }
          None => return Err(format!("There is no piece on tile {}", pos)),
        }
      }
      None => return Err("You provided a non-existant position".to_owned()),
    }
    Ok(rv)
  }

  pub fn generate_vector(&self, pos: &str, pattern: Vec<&str>) -> Result<String, String> {
    let pattern = pattern
      .iter()
      .map(|x| {
        match *x {
          "w" | "up" => Ok(Direction::Up),
          "d" | "right" => Ok(Direction::Right),
          "s" | "down" => Ok(Direction::Down),
          "a" | "left" => Ok(Direction::Left),
          _ => Err(format!("Invalid direction pattern: {}", x))
        }
      })
      .collect::<Result<Vec<_>, String>>()?;
    
    if let Some(idx) = self.board.index_of(pos) {
      return Ok(recursive_tile_vector(idx, &pattern, None)
        .iter()  
        .fold(String::new(), |mut acc, idx| {
          acc.push_str(&format!("{}, ", self.board.tile_at(*idx).unwrap()));
          acc
        }));
    }
    Err(format!("Cannot search from non-existent starting point: {}", pos))
  }
}

// Private
impl Game {
  fn add_piece(&mut self, piece: ChessPiece) -> Result<Option<ChessPiece>, String> {
    if let Some(idx) = self.board.index_of(&piece.position()) {
      return Ok(self.active_pieces.insert(idx, piece));
    } else {
      return Err("Cannot add chess piece to non-existant tile".to_owned());
    }
  }
}