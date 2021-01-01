use crate::v1::board::Board;
use crate::v1::piece::{
  Piece,
  PieceKind,
  PieceColor::{
    Black,
    White,
  },
};

#[derive(Debug)]
pub struct App {
  pub board: Board,
}

impl App {
  pub fn new() -> App {
    App {
      board: Board::new()
    }
  }

  pub fn setup_board(&mut self) {
    self.board.add_piece(Piece::pawn(White, "a2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "b2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "c2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "d2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "e2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "f2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "g2")).unwrap();
    self.board.add_piece(Piece::pawn(White, "h2")).unwrap();
    self.board.add_piece(Piece::rook(White, "a1")).unwrap();
    self.board.add_piece(Piece::knight(White, "b1")).unwrap();
    self.board.add_piece(Piece::bishop(White, "c1")).unwrap();
    self.board.add_piece(Piece::queen(White, "d1")).unwrap();
    self.board.add_piece(Piece::king(White, "e1")).unwrap();
    self.board.add_piece(Piece::bishop(White, "f1")).unwrap();
    self.board.add_piece(Piece::knight(White, "g1")).unwrap();
    self.board.add_piece(Piece::rook(White, "h1")).unwrap();

    self.board.add_piece(Piece::pawn(Black, "a7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "b7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "c7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "d7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "e7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "f7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "g7")).unwrap();
    self.board.add_piece(Piece::pawn(Black, "h7")).unwrap();
    self.board.add_piece(Piece::rook(Black, "a8")).unwrap();
    self.board.add_piece(Piece::knight(Black, "b8")).unwrap();
    self.board.add_piece(Piece::bishop(Black, "c8")).unwrap();
    self.board.add_piece(Piece::queen(Black, "d8")).unwrap();
    self.board.add_piece(Piece::king(Black, "e8")).unwrap();
    self.board.add_piece(Piece::bishop(Black, "f8")).unwrap();
    self.board.add_piece(Piece::knight(Black, "g8")).unwrap();
    self.board.add_piece(Piece::rook(Black, "h8")).unwrap();
  }

}