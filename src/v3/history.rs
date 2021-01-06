use std::ops::Range;
use super::piece::ChessPiece;

#[derive(Debug, Clone)]
pub struct GameMove {
  pub from: usize,
  pub to: usize,
  pub capture: Option<ChessPiece>,
}

#[derive(Debug, Clone)]
pub struct GameHistory {
  turns: Vec<GameMove>,
}

// Constructor
impl GameHistory {
  pub fn new() -> GameHistory {
    GameHistory { turns: Vec::new() }
  }
}

impl GameHistory {
  pub fn push(&mut self, from: usize, to: usize, capture: Option<ChessPiece>) {
    self.turns.push(GameMove { from, to, capture });
  }
  pub fn pop(&mut self) -> Option<GameMove> {
    self.turns.pop()
  }
  pub fn slice(&self, range: Option<Range<usize>>) -> &[GameMove] {
    match range {
      Some(range) => &self.turns[range],
      None => &self.turns[..]
    }
  }
  pub fn last(&self) -> Option<&GameMove> {
    self.turns.last()
  }
}