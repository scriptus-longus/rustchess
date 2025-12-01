use crate::board::{Board, Player, Pieces};

pub static KNIGHT_MOVES_LOOKUP: [u64; 64] = [
132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656
];

pub static KING_MOVES_LOOKUP: [u64; 64] = [
770, 1797, 3594, 7188, 14376, 28752, 57504, 49216, 197123, 460039, 920078, 1840156, 3680312, 7360624, 14721248, 12599488, 50463488, 117769984, 235539968, 471079936, 942159872, 1884319744, 3768639488, 3225468928, 12918652928, 30149115904, 60298231808, 120596463616, 241192927232, 482385854464, 964771708928, 825720045568, 3307175149568, 7718173671424, 15436347342848, 30872694685696, 61745389371392, 123490778742784, 246981557485568, 211384331665408, 846636838289408, 1975852459884544, 3951704919769088, 7903409839538176, 15806819679076352, 31613639358152704, 63227278716305408, 54114388906344448, 216739030602088448, 505818229730443264, 1011636459460886528, 2023272918921773056, 4046545837843546112, 8093091675687092224, 16186183351374184448, 13853283560024178688, 144959613005987840, 362258295026614272, 724516590053228544, 1449033180106457088, 2898066360212914176, 5796132720425828352, 11592265440851656704, 4665729213955833856
];

#[derive(Debug, PartialEq)]
pub struct Move {
  from: u32,
  to: u32,
}

pub enum Dir {
  U,
  D,
  R,
  L
}

#[derive(PartialEq)]
pub enum DiagDir {
  NE,
  NW,
  SE,
  SW
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
  fn ray_mask(from: u32, blocker_mask: u64, capture_mask: u64, direction: Dir) -> u64 {
    let mut ret = 0;
    let st_size: i32 = match direction {
      Dir::U => 8,
      Dir::D => -8,
      Dir::L => -1,
      Dir::R => 1,
    };

    //let mut p_bb = 1u64 << from;
    let start_rank: i32 = (from as i32) / 8;
    let start_file: i32 = (from as i32) % 8;

    let mut sq = (from as i32) + st_size;
    while sq < 64 && sq >= 0 && (sq >> 3 == start_rank || sq & 7 == start_file)  {
      let to_mask = 1u64 << sq;

      if to_mask & blocker_mask != 0 {break;}
      if to_mask & capture_mask != 0 {
        ret |= to_mask;
        break;
      }

      ret |= to_mask;
      sq += st_size;
    }

    ret
  }

  fn diag_ray_mask(from: u32, blocker_mask: u64, capture_mask: u64, direction: DiagDir) -> u64 {
    let mut ret = 0;

    let st_size: i32 = match direction {
      DiagDir::NE => 9,
      DiagDir::NW => 7,
      DiagDir::SE => -7,
      DiagDir::SW => -9,
    };

    let mut sq:i32 = from as i32 + st_size;


    // ugly condition but it works
    while sq < 64 && sq >= 0 && ((sq as u32 & 7) > (from & 7) && (direction == DiagDir::NE || direction == DiagDir::SE) ||
                                 (sq as u32 & 7) < (from & 7) && (direction == DiagDir::NW || direction == DiagDir::SW) )  {
      let to_mask = 1u64 << sq;

      if to_mask & blocker_mask != 0 {break;}
      if to_mask & capture_mask != 0 {
        ret |= to_mask;
        break;
      }

      ret |= to_mask;
      sq += st_size;
    }

    ret
  }

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

      moves.extend(MoveGen::collect_moves(from_sq, targets));
    }

    moves
  }



  pub fn rook_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];

    let opp =  match player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let block_mask = board.get_player_mask(player);
    let capture_mask = board.get_player_mask(opp);

    let mut rook_mask = board.get_pieceboard(player, Pieces::Rook).bitboard;
    while rook_mask != 0 {
      let from_sq = rook_mask.trailing_zeros();

      let ray_u = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::U);
      let ray_d = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::D);
      let ray_l = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::L);
      let ray_r = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::R);

      let targets = ray_u | ray_d | ray_l | ray_r; 

      moves.extend(MoveGen::collect_moves(from_sq, targets));

      rook_mask ^= 1u64 << from_sq;
    }
  
    moves 
  }

  pub fn bishop_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];
    let opp =  match player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let block_mask = board.get_player_mask(player);
    let capture_mask = board.get_player_mask(opp);

    let mut rook_mask = board.get_pieceboard(player, Pieces::Bishop).bitboard;

    while rook_mask != 0 {
      let from_sq = rook_mask.trailing_zeros();

      let ray_ne = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::NE);
      let ray_nw = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::NW);
      let ray_se = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::SE);
      let ray_sw = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::SW);

      let targets = ray_ne | ray_nw | ray_se | ray_sw;

      moves.extend(MoveGen::collect_moves(from_sq, targets));

      rook_mask ^= 1u64 << from_sq;
    }
  


    moves
  }


  pub fn queen_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];
    let opp =  match player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let block_mask = board.get_player_mask(player);
    let capture_mask = board.get_player_mask(opp);

    let mut rook_mask = board.get_pieceboard(player, Pieces::Queen).bitboard;

    while rook_mask != 0 {
      let from_sq = rook_mask.trailing_zeros();

      let ray_u = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::U);
      let ray_d = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::D);
      let ray_l = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::L);
      let ray_r = MoveGen::ray_mask(from_sq, block_mask, capture_mask, Dir::R);

      let ray_ne = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::NE);
      let ray_nw = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::NW);
      let ray_se = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::SE);
      let ray_sw = MoveGen::diag_ray_mask(from_sq, block_mask, capture_mask, DiagDir::SW);

      let targets = ray_ne | ray_nw | ray_se | ray_sw | ray_u | ray_d | ray_l | ray_r;

      moves.extend(MoveGen::collect_moves(from_sq, targets));

      rook_mask ^= 1u64 << from_sq;
    }

    moves

  }

  pub fn king_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];
    let king = board.get_pieceboard(player, Pieces::King).bitboard;
    let from_sq = king.trailing_zeros();

    let king_move_mask = KING_MOVES_LOOKUP[from_sq as usize];
    let block_mask = board.get_player_mask(player);

    let targets = king_move_mask & !block_mask;

    moves.extend(MoveGen::collect_moves(from_sq, targets));

    // TODO: implement castling

    moves

  }

  fn collect_moves(from: u32, mut targets: u64) -> Vec<Move>{
    let mut moves = vec![];
    while targets != 0 {
      let to_sq = targets.trailing_zeros();
      targets ^= 1u64 << to_sq;

      moves.push(Move{from: from, to: to_sq});
    }

    moves
  }

  pub fn pseudo_legal(board: &Board, player: Player) -> Vec<Move>{
    let mut moves = vec![];

    moves.extend(MoveGen::pawn_moves(board, player));
    moves.extend(MoveGen::knight_moves(board, player));
    moves.extend(MoveGen::rook_moves(board, player));
    moves.extend(MoveGen::bishop_moves(board, player));
    moves.extend(MoveGen::queen_moves(board, player));
    moves.extend(MoveGen::king_moves(board, player));

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

  #[test]
  fn test_rook_moves() {
    let board = Board::from_fen("RP6/8/p7/8/8/8/8/8 w HAha - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::rook_moves(&x, Player::White);
        assert_eq!(moves, vec![Move{from: 7*8, to: 5*8}, Move{from: 7*8, to: 6*8}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }

  #[test]
  fn test_queen_moves() {
    let board = Board::from_fen("8/8/8/8/8/2r5/P7/Q1p5 w HAha - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::queen_moves(&x, Player::White);
        assert_eq!(moves, vec![Move{from: 0, to: 1}, Move{from: 0, to: 2}, Move{from: 0, to: 8*1 + 1}, Move{from:0, to: 8*2 + 2}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }

  #[test]
  fn test_king_moves() {
    let board = Board::from_fen("8/8/8/8/8/Pp6/K7/BN6 w HAha - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::king_moves(&x, Player::White);
        assert_eq!(moves, vec![Move{from: 8, to: 9}, Move{from: 8, to: 8*2 + 1}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }
}

