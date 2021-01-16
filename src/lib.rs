#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

mod v3;
mod v4;
mod concurrent_scan;

use std::io;
use std::io::Write;
use std::process;


pub fn sandbox_v4() {
  use v4::game::Game;
  use v4::piece::{
    ChessPiece,
    Color,
    ChessPieceKind
  };
  use v4::scan::{
    ScanCtx,
    Direction,
    TileVector
  };

  let mut game = Game::new();

  concurrent_scan::do_concurrent_scan().unwrap_or_else(|err| {
    eprintln!("[ConcurrentScanError]: {}", err);
  });


}

pub fn game_v4() {
  use v4::game::Game;
  use v4::piece::{
    ChessPiece,
    Color::{
      White, Black
    },
    ChessPieceKind
  };

  let mut game = Game::new();

  game.setup(vec![
    ChessPiece::rook(White, "a1"),
    ChessPiece::knight(White, "b1"),
    ChessPiece::bishop(White, "c1"),
    ChessPiece::queen(White, "d1"),
    ChessPiece::king(White, "e1"),
    ChessPiece::bishop(White, "f1"),
    ChessPiece::knight(White, "g1"),
    ChessPiece::rook(White, "h1"),
    ChessPiece::pawn(White, "a2"),
    ChessPiece::pawn(White, "b2"),
    ChessPiece::pawn(White, "c2"),
    ChessPiece::pawn(White, "d2"),
    ChessPiece::pawn(White, "e2"),
    ChessPiece::pawn(White, "f2"),
    ChessPiece::pawn(White, "g2"),
    ChessPiece::pawn(White, "h2"),
    
    ChessPiece::rook(Black, "a8"),
    ChessPiece::knight(Black, "b8"),
    ChessPiece::bishop(Black, "c8"),
    ChessPiece::queen(Black, "d8"),
    ChessPiece::king(Black, "e8"),
    ChessPiece::bishop(Black, "f8"),
    ChessPiece::knight(Black, "g8"),
    ChessPiece::rook(Black, "h8"),
    ChessPiece::pawn(Black, "a7"),
    ChessPiece::pawn(Black, "b7"),
    ChessPiece::pawn(Black, "c7"),
    ChessPiece::pawn(Black, "d7"),
    ChessPiece::pawn(Black, "e7"),
    ChessPiece::pawn(Black, "f7"),
    ChessPiece::pawn(Black, "g7"),
    ChessPiece::pawn(Black, "h7"),
  ]);

  println!("{}", game.render_board());

  loop {
    let args = user_input().expect("Couldn't read user input");

    let mut args = args.split_whitespace();

    match args.next() {
      Some(command) => {
        if command == "board" {
          println!("{}", game.render_board());
        }
        else if command == "move" {
          match args.next() {
            Some(from) => match args.next() {
              Some(to) => {
                let next_arg = args.next().unwrap_or("");
                let force_move = next_arg == "-f" || next_arg == "--force";
                match game.move_piece(from, to, force_move) {
                  Ok(_) => println!("Moved from {} to {}\n{}", from, to, game.render_board()),
                  Err(msg) => eprintln!("{}", msg),
                }
              }
              None => eprintln!("You must provide a tile to move to"),
            },
            None => eprintln!("You must provide a tile to move from"),
          }
        }
        else if command == "moves" {
          match args.next() {
            Some(from) => {
              match game.available_moves(from) {
                Ok(available_moves) => println!("{}", available_moves),
                Err(msg) => eprintln!("{}", msg),
              }
            }
            None => eprintln!("You must provide a tile")
          }
        }
        else if command == "undo" {
          match game.undo_move() {
            Ok(()) => println!("Success!\n{}", game.render_board()),
            Err(msg) => eprintln!("{}", msg),
          };
        }
        else if command == "history" {
          println!("{}", game.history(None));
        }
      }
      None => {
        println!("Plese provide a command")
      }
    }
  }
}




pub fn sandbox_v3() {
  use v3::scanner::Direction::*;
  use v3::scanner::recursive_tile_vector;

  println!("{:?}", recursive_tile_vector(61, &vec![Up], Some(4)));
}


pub fn game_v3() {
  let mut game = v3::game::Game::new();

  println!("{}", game.render_board());

  loop {
    let args = user_input().unwrap_or_else(|err| {
      eprintln!("Could not read user input: {:?}", err);
      process::exit(1);
    });

    let mut args = args.split_whitespace();

    match args.next() {
      Some(command) => {
        if command == "peek" {
          match args.next() {
            Some(pos) => {
              match game.peek_tile(pos) {
                Some(piece) => println!("{}", piece),
                None => println!("There is no piece on tile {}", pos),
              };
            },
            None => eprintln!("You must provide a tile to peek")
          };
        }

        else if command == "move" {
          match args.next() {
            Some(from) => match args.next() {
              Some(to) => match game.move_piece(from, to) {
                Ok(_) => println!("Moved from {} to {}\n{}", from, to, game.render_board()),
                Err(msg) => eprintln!("{}", msg),
              },
              None => eprintln!("You must provide a tile to move to"),
            },
            None => eprintln!("You must provide a tile to move from"),
          }
        }

        else if command == "board" {
          println!("{}", game.render_board());
        }

        else if command == "undo" {
          match game.undo_move() {
            Ok(()) => println!("Success!\n{}", game.render_board()),
            Err(msg) => eprintln!("{}", msg),
          };
        }

        else if command == "moves" {
          match args.next() {
            Some(pos) => match game.peek_available_moves(pos) {
              Ok(moves) if moves.is_empty() => {
                println!("No moves available for piece at {}", pos);
              }
              Ok(moves) => println!("{}", moves),
              Err(msg) => eprintln!("{}", msg),
            }
            _ => eprintln!("Subcommand not recognized")
          }
        }

        else if command == "history" {
          println!("{}", game.history(None));
        }

        else if command == "vector" {
          if let Some(origin) = args.next() {
            let pattern: Vec<&str> = args.collect();
            match game.generate_vector(origin, pattern) {
              Ok(txt) => println!("{}", txt),
              Err(msg) => eprintln!("{}", msg),
            };
          }
        }

        else if command == "help" {
          help();
        }

        else {
          println!("Command: '{}' not recognized", command);
        }
      },
      None => {
        println!("No command provided")
      }
    };
    println!("");
  }
}

fn user_input() -> Result<String, io::Error> {
  print!("> ");
  io::stdout().flush()?;
  let mut rv = String::new();
  io::stdin().read_line(&mut rv)?;
  rv.pop();
  Ok(rv.to_string())
}


fn help() {
  println!("
    peek <position>               Show the piece at the given position>
                                    > peek e2
                                    w_P

    board                         Renders the chess board.

    move <from> <to>              Move a piece on the board.
                                    > move e2 e4
        
    undo                          Undo the last move.


    moves <from>                  PARTIALLY IMPLEMENTED (only implemented 
                                  for pawns). Get list of tiles the 
                                  piece at the given position can move to.
                                    > moves e2

    history                       Get a list of all moves made so far.

    vector <from> ...direction    Get a list of tiles a piece would land on by 
                                  following the provided directions. This was 
                                  mostly implemented as a way to test internal 
                                  path finding functions...

                                  Direction argments can be:
                                  'up' or 'w'
                                  'right' or 'd'
                                  'down' or 's'
                                  'right' or 'a'
                                  
                                    > vector e2 up left
                                    d3, c4, b5, a6,

                                    > vector e2 up up left
                                    d4, c6, b8,

                                    > vector e2 up 
                                    e3, e4, e5, e6, e7, e8"
  );
}