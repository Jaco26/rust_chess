use crate::v4::board::Board;
use crate::v4::scan::{
  Scan,
  ScanCtx,
  ScanReport,
};
use crate::v4::piece::{
  Color::*,
  ChessPiece,
  ChessPieceKind,
};
use crate::v4::history::{
  GameMove,
  GameHistory,
};
use crate::v4::brain::{
  PawnBrain,
  KnightBrain,
  BishopBrain,
  RookBrain,
  QueenBrain,
  KingBrain,
};


#[derive(Debug, Clone)]
pub struct Game {
  pub board: Board,
  pub history: GameHistory,
}


// Constructor
impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
      history: GameHistory::new()
    }
  }

  pub fn setup(&mut self, pieces: Vec<ChessPiece>) {
    for p in pieces {
      self.board.add_piece(p).unwrap();
    }
  }

  pub fn do_scan(&self, pos: &str) -> Result<ScanReport, String> {
    match self.board.index_of(pos) {
      Some(idx) => match self.board.pieces.get(&idx) {
        Some(piece) => {
          let scan_ctx = ScanCtx::new(idx, &self.board, &self.history)?;
          Ok(
            match piece.kind() {
              ChessPieceKind::Pawn => PawnBrain::scan(&scan_ctx)?,
              ChessPieceKind::Knight => KnightBrain::scan(&scan_ctx)?,
              ChessPieceKind::Bishop => BishopBrain::scan(&scan_ctx)?,
              ChessPieceKind::Rook => RookBrain::scan(&scan_ctx)?,
              ChessPieceKind::Queen => QueenBrain::scan(&scan_ctx)?,
              ChessPieceKind::King => KingBrain::scan(&scan_ctx)?,
            }
          )
        }
        None => Err(format!("No piece on tile '{}'", pos))
      }
      None => Err(format!("Tile '{}' does not exist", pos))
    }
  }

  pub fn available_moves(&self, pos: &str) -> Result<String, String> {
    let scan_result = self.do_scan(pos)?; 

    if scan_result.available_tiles.len() == 0 {
      return Err(format!("No moves available for piece at {}", pos))
    }

    let available_moves: Vec<String> = scan_result.available_tiles
      .iter()
      .map(|idx| self.board.tile_name_at(*idx).unwrap().to_owned())
      .collect();

    let capturables: Vec<String> = scan_result.capturables
      .iter()
      .map(|capturable| {
        format!(
          "{:?} at {}",
          capturable.kind,
          self.board.tile_name_at(capturable.tile).unwrap(),
        )
      })
      .collect();

    let pinned: Vec<String> = scan_result.pinned
      .iter()
      .map(|capturable| {
        format!(
          "{:?} at {}",
          capturable.kind,
          self.board.tile_name_at(capturable.tile).unwrap(),
        )
      })
      .collect();

    let piece = self.board.pieces.get(&scan_result.origin).unwrap();

    let pad_rule = || match piece.kind() {
      ChessPieceKind::Pawn |
      ChessPieceKind::Rook |
      ChessPieceKind::King => "----",
      ChessPieceKind::Queen => "-----",
      ChessPieceKind::Bishop |
      ChessPieceKind::Knight => "------",
    };
  
    let mut rv = format!(
      "\nFor {:?} at {}\n----------{}\n", piece.kind(), pos, pad_rule()
    );
    rv.push_str(&format!("Moves      : {:?}\n", available_moves));
    rv.push_str(&format!("Capturable : {:?}\n", capturables));
    rv.push_str(&format!("Pinned     : {:?}\n", pinned));

    Ok(rv)
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

  pub fn move_piece(&mut self, from: &str, to: &str, force: bool) -> Result<(), String> {
    let (from_idx, to_idx) = match self.board.index_of(from) {
      Some(from_idx) => match self.board.index_of(to) {
        Some(to_idx) => match self.board.pieces.contains_key(&from_idx) {
          true => (from_idx, to_idx),
          false => return Err(format!("There is no piece on {}", from))
        }
        None => return Err("Cannot move to tile that does not exist".to_owned())
      }
      None => return Err("Cannot move from tile that does not exist".to_owned())
    };

    let scan_report = self.do_scan(from)?;

    let piece = self.board.pieces.get(&from_idx).unwrap();

    let is_valid_move = {
      scan_report.available_tiles.contains(&to_idx)
    };

    if is_valid_move || force {
      let mut piece = self.board.pieces.remove(&from_idx).unwrap();
      piece.update_position(to);
      self.history.push(
        from_idx, to_idx, self.board.add_piece(piece).unwrap()
      );
      return Ok(())
    }
    Err(format!("Cannot move {:?} from '{}' to '{}'", piece.kind(), from, to))
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
