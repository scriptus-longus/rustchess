use rustchess::game::{Game};
use rustchess::perft;

#[cfg(test)]
mod perft_test {
  use super::*;

  #[test]
  pub fn pertf_test_fen_1() {
    for (depth, res) in [1, 20, 400, 8902, 197_281].into_iter().enumerate() {
      let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
      let perft_res = perft::perft(&mut game, depth);

      assert_eq!(perft_res, res, "FEN1: Failed at depth {}: expected {} but got {}", depth, res, perft_res);
    }
  }

  #[test]
  pub fn pertf_test_fen_2() {
    for (depth, res) in [1, 48, 2039, 97_862].into_iter().enumerate() {
      let mut game = Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
      let perft_res = perft::perft(&mut game, depth);

      assert_eq!(perft_res, res, "FEN1: Failed at depth {}: expected {} but got {}", depth, res, perft_res);
    }
  }

  #[test]
  pub fn pertf_test_fen_3() {
    for (depth, res) in [1, 14, 191, 2812].into_iter().enumerate() {
      let mut game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
      let perft_res = perft::perft(&mut game, depth);

      assert_eq!(perft_res, res, "FEN1: Failed at depth {}: expected {} but got {}", depth, res, perft_res);
    }
  }
}
