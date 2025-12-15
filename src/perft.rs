use crate::game::{Game, GameResult};
use crate::board::{Player, Pieces};

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
      None => {
        println!("Warning: None returned by makemove (probably illegal move)");
        0
      },
      _ => 1,
    };
     
    //game.makemove(m);

    //nodes += perft(game, depth - 1);
    game.undo_move();
  }

  nodes
}

