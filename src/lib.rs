mod v3;

use std::env;

pub fn game() {
  let mut game = v3::game::Game::new();

  println!("");

  let mut args = env::args();
  let _program = args.next();
  let command = args.next();

  match command {
    Some(x) => {
      if &x == "peek" {
        let pos = args.next().unwrap();
        println!("Peek tile at {}: {:?}", pos, game.peek_tile(&pos));
      } else if &x == "move" {
        let from = args.next().unwrap();
        let to = args.next().unwrap();
        match game.move_piece(&from, &to) {
          Ok(_) => println!("Moved from {} to {}", from, to),
          Err(msg) => eprintln!("{}", msg),
        };
      } else {
        println!("Command: '{}' not recognized", x);
      }
    },
    None => {
      println!("No command provided")
    }
  };
}