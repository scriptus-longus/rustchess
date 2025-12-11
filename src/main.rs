mod board;
mod game;

use crate::board::{Player, Board};
use crate::game::{GameState};

mod movegen;

fn main() {
  let mut board = Board::from_fen("rnbqkb1r/pp2pppp/5n2/1PppP3/8/8/P1PP1PPP/RNBQKBNR w KQkq d6 0 1").unwrap();
  board.print_board();
  board.flip();
  println!("\nFlipped: \n");
  board.print_board();
  board.flip();
  println!("Original: \n");
  board.print_board();
}
