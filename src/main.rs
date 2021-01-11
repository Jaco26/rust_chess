use std::env;
use chess;

fn main() {
  let mut args = env::args();
  args.next();
  match args.next() {
    Some(version) if version == "v3" => {
      match args.next() {
        Some(opt) if opt == "sandbox" => {
          chess::sandbox_v3();
        }
        Some(opt) => {
          eprintln!("Unknown option <{}>", opt);
        }
        None => {
          chess::game_v3();
        }
      }
    }
    Some(version) if version == "v4" => {
      match args.next() {
        Some(opt) if opt == "sandbox" => {
          chess::sandbox_v4();
        }
        Some(opt) => {
          eprintln!("Unknown option <{}>", opt);
        }
        None => {
          chess::game_v4();
        }
      }
    }
    _ => eprintln!("Please specify a version")
  }
}
