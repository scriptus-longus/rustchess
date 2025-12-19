use crate::game::{Game, GameState};
use crate::movegen::{Move};
use crate::board::{Player, Pieces};

const PAWN_V: f64 = 100.0;
const KNIGHT_V: f64 = 320.0;
const BISHOP_V: f64 = 330.0;
const ROOK_V: f64 = 500.0;
const QUEEN_V: f64 = 900.0;
const KING_V: f64 = 1000000.0;

#[inline]
pub fn max(a: f64, b: f64) -> f64 {
  if a > b {
    a
  } else {
    b
  }
}

#[inline]
pub fn min(a: f64, b: f64) -> f64 {
  if a < b {
    a
  } else {
    b
  }
}

pub fn eval(s: &GameState) -> f64 {
  let v: f64 = PAWN_V * ((s.count_pieces(Player::White, Pieces::Pawn) as f64) - (s.count_pieces(Player::Black, Pieces::Pawn) as f64)) +
          KNIGHT_V * ((s.count_pieces(Player::White, Pieces::Knight) as f64) - (s.count_pieces(Player::Black, Pieces::Knight) as f64)) +
          BISHOP_V * ((s.count_pieces(Player::White, Pieces::Bishop) as f64) - (s.count_pieces(Player::Black, Pieces::Bishop) as f64))+
          ROOK_V * ((s.count_pieces(Player::White, Pieces::Rook) as f64) - (s.count_pieces(Player::Black, Pieces::Rook) as f64)) +
          QUEEN_V * ((s.count_pieces(Player::White, Pieces::Queen) as f64) - (s.count_pieces(Player::Black, Pieces::Queen) as f64)) +
          KING_V * ((s.count_pieces(Player::White, Pieces::King) as f64) - (s.count_pieces(Player::Black, Pieces::King) as f64));
  v

}

pub fn alphabeta(game: &mut Game, depth: u32, mut alpha: f64, beta:f64, color: i32) -> f64 {
  if depth == 0 || game.is_checkmate(game.get_player()) || game.is_remis() {
    return eval(&game.state) * (color as f64);
  }
 
  let moves = game.legal_moves();

  
  let mut best_v = -std::f64::INFINITY;


  for m in moves.iter() {
    game.makemove(m);
    let v = -1.0 * alphabeta(game, depth-1, -beta, -alpha, -color);
    game.undo_move();

    if v > best_v {
      best_v = v;
    }

    alpha = max(alpha, v);
    if alpha >= beta {
      break;
    }


  }

  best_v
}

pub fn root_search(game: &mut Game, depth: u32) -> (Option<Move>, f64) {
  let moves = game.legal_moves();
  
  let mut best_move = None;
  let mut best_v = match game.state.get_player() {
    Player::White => -std::f64::INFINITY,
    Player::Black => std::f64::INFINITY,
  };

  if game.state.get_player() == Player::White {
    for m in moves.iter() {
      game.makemove(m);
      let v = alphabeta(game, depth-1, -std::f64::INFINITY, std::f64::INFINITY, -1);
      game.undo_move();

      if v > best_v {
        best_v = v;
        best_move = Some(*m);
      }
    }
  } else {
    for m in moves.iter() {
      game.makemove(m);
      let v = alphabeta(game, depth-1, -std::f64::INFINITY, std::f64::INFINITY, 1);
      game.undo_move();

      if v < best_v {
        best_v = v;
        best_move = Some(*m);
      }
    }
  }

  (best_move, best_v)
}
