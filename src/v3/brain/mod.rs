
mod pawn;
mod knight;
mod bishop;

pub mod util;
pub mod prelude;
pub mod scanner;

pub use pawn::PawnBrain;
pub use knight::KnightBrain;
pub use bishop::BishopBrain;

/// Encapsulate higher level decision making about best moves weighing each piece's
/// possibile moves against the context of all possible moves of all other pieces
/// on the board
pub struct Brain { }