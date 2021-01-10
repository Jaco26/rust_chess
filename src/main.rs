use std::env;
use chess;

fn main() {
  let mut args = env::args();
  args.next();
  match args.next() {
    Some(version) if version == "v3" => {
      chess::game_v3();
    }
    _ => eprintln!("Please specify a version")
  }
}
