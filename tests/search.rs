use rustchess::game::{Game};
use rustchess::search::{eval};
use rustchess::board::{Player};
use rustchess::movegen::{Move};

use rustchess::search;

fn minimax_eval(game: &mut Game, depth: u32)  -> f64 {
  if depth == 0 {
    return eval(&mut game.state);
  }

  let moves = game.legal_moves();

  
  let mut best_v = match game.state.get_player() {
    Player::White => -std::f64::INFINITY,
    Player::Black => std::f64::INFINITY,
  };

  if game.state.get_player() == Player::White {
    for m in moves.iter() {
      game.makemove(m);
      let v = minimax_eval(game, depth-1);
      game.undo_move();

      if v > best_v {
        best_v = v;
      }
    }
  } else {
    for m in moves.iter() {
      game.makemove(m);
      let v = minimax_eval(game, depth-1);
      game.undo_move();

      if v < best_v {
        best_v = v;
      }
    }
  }

  best_v
}


fn root_search(game: &mut Game, depth: u32) -> (Option<Move>, f64) {
  let moves = game.legal_moves();
  
  let mut best_move = None;
  let mut best_v = match game.state.get_player() {
    Player::White => -std::f64::INFINITY,
    Player::Black => std::f64::INFINITY,
  };

  if game.state.get_player() == Player::White {
    for m in moves.iter() {
      game.makemove(m);
      let v = minimax_eval(game, depth);
      game.undo_move();

      println!("Move: {} Value: {}", Move::to_lan(m, &game.state).unwrap(), v);

      if v > best_v {
        best_v = v;
        best_move = Some(*m);
      }
    }
  } else {
    for m in moves.iter() {
      game.makemove(m);
      let v = minimax_eval(game, depth);
      game.undo_move();

      if v < best_v {
        best_v = v;
        best_move = Some(*m);
      }
    }
  }

  println!();
  (best_move, best_v)
}

#[cfg(test)]
mod test_search {
  use super::*;

  #[test]
  pub fn test_fen_1() {
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let (t_m, v_m) = root_search(&mut game, 3);
    let (t_s, v_s) = search::root_search(&mut game, 3);


    assert_eq!(t_m, t_s, "Moves not the same");
    assert_eq!(v_m, v_s, "Value returened not the same");
  }

  #[test]
  pub fn test_fen_2() {
    let mut game = Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let (t_m, v_m) = root_search(&mut game, 2);
    let (t_s, v_s) = search::root_search(&mut game, 2);


    assert_eq!(t_m, t_s, "Failed at depth {}: Moves not the same", 3);
    assert_eq!(v_m, v_s, "Failed at depth {}: Value returened not the same", 3);
  }
}
