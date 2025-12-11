mod board;
mod game;

use crate::board::{Player, Board};
use crate::game::{GameState};

mod movegen;

fn main() {
  

  /*match GameState::from_fen("rnbqkb1r/pp2pppp/5n2/1PppP3/8/8/P1PP1PPP/RNBQKBNR w KQkq d6 0 1") {
    Ok(state) => {
      state.print_state()}
    Err(x) => println!("Error creating fen string: {}", x),
  }*/

  let mut board = Board::from_fen("rnbqkb1r/pp2pppp/5n2/1PppP3/8/8/P1PP1PPP/RNBQKBNR w KQkq d6 0 1").unwrap();
  board.print_board();
  board.flip();
  println!("\nFlipped: \n");
  board.print_board();
  board.flip();
  println!("Original: \n");
  board.print_board();





   /*match board::Board::from_fen("8/8/1r6/P7/8/8/8 w Kkq - 0 1"){
     Ok(g_board) => {
       println!("Here is the board");
       g_board.print_board();

       let moves = movegen::MoveGen::pseudo_legal(&g_board, Player::White);

       for x in moves {
         x.print_move();
       } 
     },
     Err(x) => println!("Error creating fen string: {}", x),
    }*/ 
}
