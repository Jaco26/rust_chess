use std::collections::HashMap;
use std::thread;
use std::sync::{
  mpsc,
  Arc,
  MutexGuard,
  Mutex,
};
use std::time::Duration;

use crate::v4::game::Game;
use crate::v4::scan::{
  Scan,
  ScanReport,
};
use crate::v4::piece::{
  ChessPiece,
  Color::{
    White, Black
  },
  ChessPieceKind
};

pub fn do_concurrent_scan() -> Result<(), String> {

  let mut game = Game::new();

  let scans: Arc<Mutex<HashMap<String, ScanReport>>> = Arc::new(Mutex::new(HashMap::new()));

  game.setup(vec![
    ChessPiece::rook(White, "d4"),
    ChessPiece::pawn(White, "c4"),
    ChessPiece::bishop(Black, "d7"),
    ChessPiece::queen(Black, "d8"),
  ]);

  let mut handles = vec![];

  for (_, piece) in game.board.pieces.clone() {
    let scans = Arc::clone(&scans);
    let game = game.clone();
    let handle = thread::spawn(move || {
      let mut lock = scans.lock().unwrap();
      let pos = piece.position();
      let current_scan = game.do_scan(&pos).unwrap();

      lock.insert(pos.clone(), current_scan);
    }); 
    handles.push(handle);
  }

  for h in handles {
    h.join().unwrap();
  }

  println!("{:#?}", scans.lock().unwrap());


  // learning_zone::mutex();

  // let (tx, rx) = mpsc::channel();

  // let mut handles = vec![];

  // for (_, piece) in game.board.pieces.iter() {
  //   let mut game = game.clone();
  //   let pos = piece.position();
  //   let tx = tx.clone();
  //   let handle = thread::spawn(move || {
  //     let scan = game.do_scan(&pos).unwrap();
  //     tx.send(scan).unwrap();
  //   });
  //   handles.push(handle);
  // }

  // println!("{:?}", rx.recv().unwrap());
  // println!("{:?}", rx.recv().unwrap());
  // println!("{:?}", rx.recv().unwrap());
  // println!("{:?}", rx.recv().unwrap());
  // println!("{:?}", rx.recv().unwrap());

  // let mut rv = vec![];
  // for x in rx {
  //   rv.push(x);
  // }

  // for h in handles {
  //   h.join().unwrap();
  // }

  // println!("{:?}", rx.into_iter().collect::<Vec<ScanReport>>());

  // let scan_1 = game.do_scan("d4").unwrap();
  // let scan_2 = game.do_scan("c4").unwrap();
  // let scan_3 = game.do_scan("d7").unwrap();
  // let scan_4 = game.do_scan("d8").unwrap();

  // println!("{:#?}", vec![scan_1, scan_2, scan_3, scan_4]);

  Ok(())
}

mod learning_zone {
  use super::*;

  pub fn mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
      let counter = counter.clone();
      let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
      });
      handles.push(handle);
    }
    for handle in handles {
      handle.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap())
  }

  pub fn channels() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

  }


  pub fn thread_joins() {
    let my_vec = vec![1,2,3];

    let handle = std::thread::spawn(move || {
      println!("{:?}", my_vec.clone());
      for i in 1..10 {
        println!("Hi number {} from spawned thread!", i);
        thread::sleep(Duration::from_millis(5));
      }
    }); 
  
    for i in 1..10 {
      println!("Hi number {} from the main thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  
    // Calling join on the handle blocks the thread currently running 
    // until the thread represented by the handle terminates
    handle.join().unwrap();
  }
}
