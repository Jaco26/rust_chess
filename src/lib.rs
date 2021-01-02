mod v3;

use std::io;
use std::io::Write;
use std::process;

pub fn game() {
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
