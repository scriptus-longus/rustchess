use crate::board::{Board, Player, Pieces};
//use crate::board::BitBoard;

pub static KNIGHT_MOVES_LOOKUP: [u64; 64] = [
132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656
];

#[derive(Debug, PartialEq)]
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
  fn pawn_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let pawns = board.get_pieceboard(player, Pieces::Pawn);
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
    // TODO: For now we will ignore en passant moves

    moves
  }

  pub fn knight_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];
    let knights_bb = board.get_pieceboard(player, Pieces::Knight);
    let mut knights = knights_bb.bitboard; 

    while knights != 0 {
      let from_sq = knights.trailing_zeros();
      let mut targets = KNIGHT_MOVES_LOOKUP[from_sq as usize];


      knights ^= 1u64 << from_sq; 

      // check for piece collision with player pieces
      targets = targets & !board.get_player_mask(player);

      //let test = BitBoard{bitboard: board.get_player_mask(player)};

      while targets != 0 {
        let to_sq = targets.trailing_zeros();
        targets ^= 1u64 << to_sq;

        moves.push(Move{from: from_sq, to: to_sq});
      }
    }

    moves
  }

  pub fn pseudo_legal(board: &Board, player: Player) -> Vec<Move>{
    let mut moves = vec![];

    moves.extend(MoveGen::pawn_moves(board, player));
    moves.extend(MoveGen::knight_moves(board, player));

    moves
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pawn_moves() {
    let board = Board::from_fen("8/8/8/1r6/8/1r6/P7/8 w - -");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::pawn_moves(&x, Player::White);
        assert_eq!(moves, vec![Move{from: 8, to: 16}, Move{from: 8, to: 24}, Move{from: 8, to: 17}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
    }
  }

  #[test]
  fn test_knight_moves() {
    let board = Board::from_fen("8/2r5/3P4/1N6/3R4/Q1K5/8/8 w HAha - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::knight_moves(&x, Player::White);
        assert_eq!(moves, vec![Move{from: 4*8 + 1, to: 48}, Move{from: 4*8 + 1, to: 50}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }
}

