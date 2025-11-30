mod board;
use crate::board::{Player, Pieces};

mod movegen;

fn main() {
   match board::Board::from_fen("r7/2q5/8/8/1bNb4/1PP5/P2K4 w Kkq - 0 1"){
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
