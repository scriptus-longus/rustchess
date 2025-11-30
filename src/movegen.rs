use crate::board::{Board, Player, Pieces};
use crate::board::BitBoard;

#[derive(Debug)]
pub struct Move {
  from: u32,
  to: u32,
}

impl Move {
  pub fn print_move(&self) {
    let from_rank:u32 = self.from % 8;
    let from_file:u32 = self.from / 8;

    let to_rank:u32 = self.to % 8;
    let to_file: u32 = self.to / 8;

    println!("({}, {}) -> ({}, {})", from_file, from_rank, to_file, to_rank);
  }
}

pub struct MoveGen;

impl MoveGen {
  fn gen_pawn_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let pawns = board.get_pieceboard(player.clone(), Pieces::Pawn);
    let free_mask = board.get_freesq_mask();
  

    let opp = match player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let opp_piece_mask = board.get_player_mask(opp);

    // single pushes
    let mut single_push = (pawns.bitboard << 8) & free_mask;
    while single_push != 0 {
      let to_sq = single_push.trailing_zeros();
      single_push ^= 1u64 << to_sq;
      let from_sq = to_sq - 8;

      moves.push(Move{from: from_sq, to: to_sq});
    }

    // double pushes
    let mut double_push = ((pawns.bitboard & 0xFF00u64) << 16) & free_mask & (free_mask << 8);
    while double_push != 0 {
      let to_sq = double_push.trailing_zeros();
      double_push ^= 1u64 << to_sq;
      let from_sq = to_sq - 16;

      moves.push(Move{from: from_sq, to: to_sq});
    }

    // capture
    let not_a_file = 0xfefefefefefefefeu64;
    let not_h_file = 0x7f7f7f7f7f7f7f7fu64;

    let mut capture = ((pawns.bitboard & not_a_file) << 7) & opp_piece_mask;
    while capture != 0 {
      let to_sq = capture.trailing_zeros();
      capture ^= 1u64 << to_sq;

      let from_sq = to_sq - 7;
      
      moves.push(Move{from: from_sq, to: to_sq});
    }

    capture = ((pawns.bitboard & not_h_file) << 9) & opp_piece_mask;
    while capture != 0 {
      let to_sq = capture.trailing_zeros();
      capture ^= 1u64 << to_sq;

      let from_sq = to_sq - 9;
      
      moves.push(Move{from: from_sq, to: to_sq});
    }

    // en passant
    // TODO:

    moves

  }

  pub fn pseudo_legal(board: &Board, player: Player) -> Vec<Move>{
    let mut moves = vec![];

    moves.extend(MoveGen::gen_pawn_moves(board, player));

    moves
  }
}
