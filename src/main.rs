mod board;
use crate::board::{Player};

mod movegen;

fn main() {
   match board::Board::from_fen("8/8/1r6/P7/8/8/8 w Kkq - 0 1"){
     Ok(g_board) => {
       println!("Here is the board");
       g_board.print_board();

       let moves = movegen::MoveGen::pseudo_legal(&g_board, Player::White);

       for x in moves {
         x.print_move();
       } 
     },
     Err(x) => println!("Error creating fen string: {}", x),
    } 
}
