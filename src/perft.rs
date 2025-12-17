use crate::game::{Game, GameResult};
use crate::board::{Player, Pieces};
use crate::movegen::{Move};

pub fn perft(game: &mut Game, depth: usize) -> u32 {
  if depth == 0 {
    //game.state.print_state();
    return 1;
  }

  let moves = game.legal_moves();

  let mut nodes = 0;   

  for m in moves.iter() {
    nodes += match game.makemove(m) {
      Some(GameResult::NotDone) => {
        let r = perft(game, depth - 1);
        r
      },
      Some(GameResult::Win(_)) => 1,
      Some(GameResult::Remis) => 0,
      None => {
        println!("Warning: None returned by makemove (probably illegal move)");
        0
      },
    };
     
    //game.makemove(m);

    //nodes += perft(game, depth - 1);
    game.undo_move();
  }

  nodes
}

pub fn debug_perft(game: &mut Game, depth: usize, print: bool) -> u32 {
  if depth == 0 {
    //game.state.print_state();
    return 1;
  }

  let moves = game.legal_moves();

  let mut nodes = 0;   

  for m in moves.iter() {
    let n = match game.makemove(m) {
      Some(GameResult::NotDone) => {
        let r = debug_perft(game, depth - 1, false);
        r
      },
      Some(GameResult::Win(_)) => 1,
      Some(GameResult::Remis) => 0,
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
     
    //game.makemove(m);

    //nodes += perft(game, depth - 1);
  }

  nodes
}

