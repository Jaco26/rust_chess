mod v3;
mod v4;

use std::io;
use std::io::Write;
use std::process;


pub fn sandbox_v4() {
  use v4::game::Game;
  use v4::piece::{
    Color,
    ChessPieceKind
  };
  use v4::scan::{
    ScanCtx,
    Direction,
    TileVector
  };

  let game = Game::new();

  let origin = game.board.index_of("d2").unwrap();

  let scan_ctx = ScanCtx::new(origin, &game.board).unwrap();

  let tile_vec = TileVector::new(&scan_ctx, &vec![Direction::Up], None);

  for x in tile_vec.iter() {
    println!("{:?}", x);
  }

  let tile_vec_pieces: Vec<(usize, Option<(Color, ChessPieceKind)>)> = tile_vec
    .iter()
    .filter(|x| match x {
      (_, Some(_)) => true,
      _ => false
    })
    .collect();
  
  println!("{:?}", tile_vec_pieces);

}

pub fn game_v4() {
  let mut game = v4::game::Game::new();

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
              Some(to) => match game.move_piece(from, to) {
                Ok(_) => println!("Moved from {} to {}\n{}", from, to, game.render_board()),
                Err(msg) => eprintln!("{}", msg),
              },
              None => eprintln!("You must provide a tile to move to"),
            },
            None => eprintln!("You must provide a tile to move from"),
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