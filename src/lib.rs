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
    Color::*,
    ChessPieceKind,
  };
  use v4::scan::{
    ScanCtx,
    Direction,
    TileVector
  };

  let mut game = Game::new();

  game.setup(vec![
    ChessPiece::king(Black, "b8"),
    ChessPiece::rook(Black, "f8"),
    ChessPiece::pawn(Black, "a7"),
    ChessPiece::pawn(Black, "b7"),
    ChessPiece::pawn(Black, "g7"),
    ChessPiece::pawn(Black, "h7"),
    ChessPiece::pawn(Black, "c6"),
    ChessPiece::knight(Black, "e4"),
    ChessPiece::rook(Black, "e2"),

    ChessPiece::rook(White, "c1"),
    ChessPiece::rook(White, "f1"),
    ChessPiece::pawn(White, "a2"),
    ChessPiece::bishop(White, "f2"),
    ChessPiece::king(White, "g2"),
    ChessPiece::bishop(White, "b3"),
    ChessPiece::pawn(White, "g3"),
    ChessPiece::pawn(White, "h3"),
    ChessPiece::pawn(White, "c4")
  ]);
  
  let mut results = vec![];

  for (tile_idx, piece) in game.board.pieces.iter() {
    match piece.color() {
      White => {
        results.push(
          v4::priority::prioritize_possible_moves(
            game.clone(),
            *tile_idx
          )
        );
      }
      _ => {}
    }
  }

  let results: String = results
    .into_iter()
    .map(|(base_scan, mut pq)| {
      let from_pos = game.board.tile_name_at(base_scan.origin).unwrap();
      let mut rv = format!("For {:?} at {}\n", base_scan.origin_kind, from_pos);
      let mut count = 0;
      while let Some(scan) = pq.pop() {
        if count > 2 {
          break
        }
        let to_pos = game.board.tile_name_at(scan.origin).unwrap();
        rv.push_str(&format!("-> {}\n", to_pos));
        count += 1;
      }
      rv
    })
    .collect::<Vec<String>>()
    .join("\n");


  println!("{}", results);

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

  let mut game = Game::default();

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
        else if command == "help" {
          help_v4();
        }
      }
      None => {
        println!("Plese provide a command")
      }
    }
  }
}


fn help_v4() {
  println!("
  Commands for version 4
  ---------------------------------------------------------------------------------------------
  board                         Display the current state of the game board.

  move <from> <to> [opts]       Move a piece on the board.
                                  > move e2 e4
                                [opts]:
                                  -f --force      Move the piece <from> tile x <to> to tile b 
                                                  regardless of whether or not the move is legal

  moves <from>                  Display available moves for the piece at a given position <from>
                                  > moves e2

  undo                          Undo the last move. Reverts the game board to its state prior to 
                                the last executed 'move' command.

  history                       Display the move history.

  help                          Display this list of commands.
  ");
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
          help_v3();
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


fn help_v3() {
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