use crate::game::{Game, GameResult};
use crate::board::{Player, Pieces};
use crate::movegen::{Move};

pub fn perft(game: &mut Game, depth: usize) -> u32 {
  if depth == 0 {
    return 1;
  }

  let moves = game.legal_moves();

  let mut nodes = 0;   

  for m in moves.iter() {
    let n = match game.makemove(m) {
      Some(_) => {
        let r = debug_perft(game, depth-1, false);
        r
      }
      None  => {
        println!("Warning: None returned by makemove (probably illegal move)");
        0
      },
    };
    
    nodes += n;
    game.undo_move();
  }

  nodes
}

pub fn debug_perft(game: &mut Game, depth: usize, print: bool) -> u32 {
  if depth == 0 {
    return 1;
  }

  let moves = game.legal_moves();

  let mut nodes = 0;   

  for m in moves.iter() {
    let n = match game.makemove(m) {
      Some(_) => {
        let r = debug_perft(game, depth-1, false);
        r
      }
      None => {
        println!("Warning: None returned by makemove (probably illegal move)");
        0
      },
    };

    nodes += n;
    game.undo_move();

    let lan = match Move::to_lan(m, &game.state) {
      Ok(x) => x,
      _ => String::from("Error decoding lan"),
    };
  
    if print {
      println!("{}: {}", lan, n);
    }
  }

  nodes
}

