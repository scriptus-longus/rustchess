use rustchess::game::{Game, GameResult};
use rustchess::perft;

#[cfg(test)]
mod perft_test {
  use super::*;


  #[test]
  pub fn perf_test_depth_0() {
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let perft_res = perft::perft(&mut game, 0);
    assert_eq!(perft_res, 1);
  }

  #[test]
  pub fn perf_test_depth_1() {
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let perft_res = perft::perft(&mut game, 1);
    assert_eq!(perft_res, 20);
  }
  
  #[test]
  pub fn perf_test_depth_2() {
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let perft_res = perft::perft(&mut game, 2);
    assert_eq!(perft_res, 400);
  }

  #[test]
  pub fn perf_test_depth_3() {
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let perft_res = perft::perft(&mut game, 3);
    assert_eq!(perft_res, 8902);
  }

  #[test]
  pub fn perf_test_depth_4() {
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let perft_res = perft::perft(&mut game, 4);
    assert_eq!(perft_res, 197_281);
  }
}
