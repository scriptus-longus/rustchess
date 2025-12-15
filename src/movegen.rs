use crate::board::{Board, Player, Pieces};
use crate::game::{GameState};
use crate::game::{CASTLE_WHITE_KINGSIDE, CASTLE_WHITE_QUEENSIDE, CASTLE_BLACK_KINGSIDE, CASTLE_BLACK_QUEENSIDE};

//use crate::game;

pub static KNIGHT_MOVES_LOOKUP: [u64; 64] = [
132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656
];

pub static KING_MOVES_LOOKUP: [u64; 64] = [
770, 1797, 3594, 7188, 14376, 28752, 57504, 49216, 197123, 460039, 920078, 1840156, 3680312, 7360624, 14721248, 12599488, 50463488, 117769984, 235539968, 471079936, 942159872, 1884319744, 3768639488, 3225468928, 12918652928, 30149115904, 60298231808, 120596463616, 241192927232, 482385854464, 964771708928, 825720045568, 3307175149568, 7718173671424, 15436347342848, 30872694685696, 61745389371392, 123490778742784, 246981557485568, 211384331665408, 846636838289408, 1975852459884544, 3951704919769088, 7903409839538176, 15806819679076352, 31613639358152704, 63227278716305408, 54114388906344448, 216739030602088448, 505818229730443264, 1011636459460886528, 2023272918921773056, 4046545837843546112, 8093091675687092224, 16186183351374184448, 13853283560024178688, 144959613005987840, 362258295026614272, 724516590053228544, 1449033180106457088, 2898066360212914176, 5796132720425828352, 11592265440851656704, 4665729213955833856
];


#[derive(Debug, PartialEq)]
pub struct Move {
  pub piece: Pieces,
  pub from: u32,
  pub to: u32,
  pub promotion: Option<Pieces>,
  pub ep: bool,
}

impl Move {
  pub fn from_lan(lan: &str, state: &GameState) -> Result<Self, &'static str> {

    if lan.len() < 4 {
      return Err("Not a valid move");
    }

    let from = match state.algebraic_to_shift(&lan[0..2]) {
      Some(x) => x,
      None => return Err("Could not convert start to LAN"),
    };

    let to = match state.algebraic_to_shift(&lan[2..4]) {
      Some(x) => x,
      None => return Err("Could not convert target to LAN"),
    };

    let mut promotion = None;
    if lan.len() == 5 {
      promotion = match lan.chars().nth(4).unwrap() {
        'q' => Some(Pieces::Queen),
        'r' => Some(Pieces::Rook),
        'n' => Some(Pieces::Knight),
        'b' => Some(Pieces::Bishop),
        _ => None
      }
    }

    // get piece
    let piece = match state.relative_board.get_piece(from as i32) {
      Some((_, x)) => x,
      None => return Err("Square is not occupied"),
    };

    //check ep
    let mut ep = false;
    if let Some(x) = state.get_ep() {
      if piece == Pieces::Pawn && to - 8 == x {
        ep = true;
      }
    }

    Ok(Move {piece: piece, from: from, to: to, promotion: promotion, ep: ep})
  }

  pub fn to_lan(m: &Move, state: &GameState) -> Result<String, &'static str> {
    let mut from = match state.shift_to_algebraic(m.from) {
      Some(x) => x,
      None => return Err("Not a valid move"),
    };

    let to = match state.shift_to_algebraic(m.to) {
      Some(x) => x,
      None => return Err("Not a valid move"),
    };



    from.push_str(&to);
    Ok(from)
  }
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


pub struct MoveGen;

impl MoveGen {
  fn ray_mask(from: u32, blocker_mask: u64, direction: Dir) -> u64 {
    let mut ret = 0;
    let st_size: i32 = match direction {
      Dir::U => 8,
      Dir::D => -8,
      Dir::L => -1,
      Dir::R => 1,
    };

    let start_rank: i32 = (from as i32) / 8;
    let start_file: i32 = (from as i32) % 8;

    let mut sq = (from as i32) + st_size;
    while sq < 64 && sq >= 0 && (sq >> 3 == start_rank || sq & 7 == start_file)  {
      let to_mask = 1u64 << sq;

      //if to_mask & blocker_mask != 0 {break;}

      ret |= to_mask;
      sq += st_size;

      if to_mask & blocker_mask != 0 {
        //ret |= to_mask;
        break;
      }
    }

    ret
  }

  fn diag_ray_mask(from: u32, blocker_mask: u64, direction: DiagDir) -> u64 {
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

      //if to_mask & blocker_mask != 0 {break;}

      ret |= to_mask;
      sq += st_size;

      if to_mask & blocker_mask != 0 {
        //ret |= to_mask;
        break;
      }
    }

    ret
  }

  // these functions will also be used for checking if check or checkmate has occured 
  pub fn get_all_attacks(board: &Board, player: Player) -> u64 {
    let free_mask = board.get_freesq_mask();


    let pawns = board.get_pieceboard(player, Pieces::Pawn).bitboard;
    let knights = board.get_pieceboard(player, Pieces::Knight).bitboard;
    let rooks = board.get_pieceboard(player, Pieces::Rook).bitboard;
    let bishops = board.get_pieceboard(player, Pieces::Bishop).bitboard;
    let queens = board.get_pieceboard(player, Pieces::Queen).bitboard;
    let king = board.get_pieceboard(player, Pieces::King).bitboard;

    

    let attacked = MoveGen::get_pawn_attacks(pawns) | 
                  MoveGen::get_knight_attacks(knights) |
                  MoveGen::get_rook_attacks(rooks, free_mask) |
                  MoveGen::get_bishop_attacks(bishops, free_mask) |
                  MoveGen::get_queen_attacks(queens, free_mask) |
                  MoveGen::get_king_attacks(king);
  
    attacked 
  }

  pub fn get_pawn_attacks(pawns: u64) -> u64 {
    let not_h_file = 0xfefefefefefefefeu64;
    let not_a_file = 0x7f7f7f7f7f7f7f7fu64;

    let mut targets = (pawns & not_h_file) << 7;
    targets        |= (pawns & not_a_file) << 9;
    targets
  }

  pub fn get_knight_attacks(mut knights: u64) -> u64 {
    let mut targets = 0u64;

    while knights != 0 {
      let from_sq = knights.trailing_zeros();
      targets |= KNIGHT_MOVES_LOOKUP[from_sq as usize];
      knights ^= 1u64 << from_sq;
      //targets |= targets;
    }

    targets
  }

  pub fn get_rook_attacks(mut rooks: u64, free_mask: u64) -> u64 {
    let mut targets = 0u64;

    while rooks != 0 {
      let from_sq = rooks.trailing_zeros();

      let ray_u = MoveGen::ray_mask(from_sq, !free_mask, Dir::U);
      let ray_d = MoveGen::ray_mask(from_sq, !free_mask, Dir::D);
      let ray_l = MoveGen::ray_mask(from_sq, !free_mask, Dir::L);
      let ray_r = MoveGen::ray_mask(from_sq, !free_mask, Dir::R);

      targets |= ray_u | ray_d | ray_l | ray_r; 


      rooks ^= 1u64 << from_sq;
    }

    targets
  }

  pub fn get_bishop_attacks(mut bishops: u64, free_mask: u64) -> u64 {
    let mut targets = 0u64;

    while bishops != 0 {
      let from_sq = bishops.trailing_zeros();

      let ray_ne = MoveGen::diag_ray_mask(from_sq, !free_mask, DiagDir::NE);
      let ray_nw = MoveGen::diag_ray_mask(from_sq, !free_mask, DiagDir::NW);
      let ray_se = MoveGen::diag_ray_mask(from_sq, !free_mask, DiagDir::SE);
      let ray_sw = MoveGen::diag_ray_mask(from_sq, !free_mask, DiagDir::SW);


      targets |= ray_ne | ray_nw | ray_se | ray_sw; 


      bishops ^= 1u64 << from_sq;
    }

    targets
  }

  pub fn get_queen_attacks(mut queens: u64, free_mask: u64) -> u64 {
    let mut targets = 0u64;

    while queens != 0 {
      let from_sq = queens.trailing_zeros();
      let queen = 1u64 << from_sq;

      targets |= MoveGen::get_rook_attacks(queen, free_mask) | MoveGen::get_bishop_attacks(queen, free_mask);

      queens ^= 1u64 << from_sq;
    }

    targets

  }

  pub fn get_king_attacks(king: u64) -> u64 {
    let from_sq = king.trailing_zeros();

    KING_MOVES_LOOKUP[from_sq as usize]
  }



  // generate moves
  pub fn pawn_moves(board: &Board, player: Player, ep: Option<u32>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let pawns = board.get_pieceboard(player, Pieces::Pawn);
    let free_mask = board.get_freesq_mask();
  

    let opp = match player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let opp_piece_mask = board.get_player_mask(opp);


    // single pushes
    let single_push = (pawns.bitboard << 8) & free_mask;
    moves.extend(MoveGen::collect_pawn_moves(single_push, 8));


    // double pushes
    let double_push = ((pawns.bitboard & 0xFF00u64) << 16) & free_mask & (free_mask << 8);
    moves.extend(MoveGen::collect_pawn_moves(double_push, 16));


    // capture
    let not_h_file = 0xfefefefefefefefeu64;
    let not_a_file = 0x7f7f7f7f7f7f7f7fu64;

    let mut capture = ((pawns.bitboard & not_h_file) << 7) & opp_piece_mask;
    moves.extend(MoveGen::collect_pawn_moves(capture, 7));

    capture = ((pawns.bitboard & not_a_file) << 9) & opp_piece_mask;
    moves.extend(MoveGen::collect_pawn_moves(capture, 9));

    // en passant
    if let Some(ep_target) = ep {
      let mut mask = ((pawns.bitboard & not_a_file) << 1) & (1u64 << ep_target);
      if mask != 0 {
        let to_sq = mask.trailing_zeros(); 
        moves.push(Move{piece: Pieces::Pawn, from: to_sq - 1 , to: to_sq + 8, promotion: None, ep: true});
      }
      mask = ((pawns.bitboard & not_h_file) >> 1) & (1u64 << ep_target);

      if mask != 0 {
        let to_sq = mask.trailing_zeros(); 
        moves.push(Move{piece: Pieces::Pawn, from: to_sq + 1 , to: to_sq + 8, promotion: None, ep: true});
      }
    }

    moves
  }

  pub fn knight_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];
    let mut knights = board.get_pieceboard(player, Pieces::Knight).bitboard;

    while knights != 0 {
      // get knight
      let from_sq = knights.trailing_zeros();
      let mask = 1u64 << from_sq;

      // extract only one knight

      knights ^= mask;

      // compute targets
      let mut targets = MoveGen::get_knight_attacks(mask);
      targets = targets & !board.get_player_mask(player);

      moves.extend(MoveGen::collect_moves(from_sq, targets, Pieces::Knight));
    }

    moves
  }



  pub fn rook_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];

    let mut rooks = board.get_pieceboard(player, Pieces::Rook).bitboard;
    while rooks != 0 {
      let from_sq = rooks.trailing_zeros();
      let mask = 1u64 << from_sq;
        
      let targets = MoveGen::get_rook_attacks(mask, board.get_freesq_mask()) & !(board.get_player_mask(player));

      moves.extend(MoveGen::collect_moves(from_sq, targets, Pieces::Rook));

      rooks ^= 1u64 << from_sq;
    }
  
    moves 
  }

  pub fn bishop_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];

    let mut bishops = board.get_pieceboard(player, Pieces::Bishop).bitboard;
    while bishops != 0 {
      let from_sq = bishops.trailing_zeros();
      let mask = 1u64 << from_sq;

      let targets = MoveGen::get_bishop_attacks(mask, board.get_freesq_mask()) & !(board.get_player_mask(player));
      
      moves.extend(MoveGen::collect_moves(from_sq, targets, Pieces::Bishop));

      bishops ^= 1u64 << from_sq;
    }

    moves
  }


  pub fn queen_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut moves = vec![];

    let mut queens = board.get_pieceboard(player, Pieces::Queen).bitboard;
    while queens != 0 {
      let from_sq = queens.trailing_zeros();
      let mask = 1u64 << from_sq;

      let targets = MoveGen::get_queen_attacks(mask, board.get_freesq_mask()) & !(board.get_player_mask(player));


      moves.extend(MoveGen::collect_moves(from_sq, targets, Pieces::Queen));

      queens ^= 1u64 << from_sq;
    }

    moves
  }

  pub fn king_moves(board: &Board, player: Player, can_castle_kingside: bool, can_castle_queenside: bool) -> Vec<Move> {
    let mut moves = vec![];
    let king = board.get_pieceboard(player, Pieces::King).bitboard;
    let from_sq = king.trailing_zeros();

    let king_move_mask = KING_MOVES_LOOKUP[from_sq as usize];
    let block_mask = board.get_player_mask(player);

    let targets = king_move_mask & !block_mask;

    moves.extend(MoveGen::collect_moves(from_sq, targets, Pieces::King));

    // castling
    let c_kingside = 1u64 << 2 | 1u64 << 1;
    let c_queenside = 1u64 << 4 | 1u64 << 5 | 1u64 << 6;
    let free_mask = !board.get_freesq_mask();
   
    if (c_kingside & free_mask == 0) && can_castle_kingside {
      moves.push(Move{piece: Pieces::King, from: 3, to: 1, promotion: None, ep: false});
    }
    if (c_queenside & free_mask == 0) && can_castle_queenside {
      moves.push(Move{piece: Pieces::King, from: 3, to: 5, promotion: None, ep: false});
    }

    moves
  }

  fn collect_moves(from: u32, mut targets: u64, piece: Pieces) -> Vec<Move>{
    let mut moves = vec![];
    while targets != 0 {
      let to_sq = targets.trailing_zeros();
      targets ^= 1u64 << to_sq;

      moves.push(Move{piece: piece, from: from, to: to_sq, promotion: None, ep: false});
    }

    moves
  }

  fn collect_pawn_moves(mut targets: u64, shift: u32) -> Vec<Move> {
    let mut moves = vec![];
    while targets != 0 {
      let to_sq = targets.trailing_zeros();
      targets ^= 1u64 << to_sq;
      let from_sq = to_sq - shift;

      if to_sq <= 63 && to_sq >= 63 - 7 {
        for p in [Pieces::Knight, Pieces::Bishop, Pieces::Rook, Pieces::Queen] {
          moves.push(Move{piece: Pieces::Pawn, from: from_sq, to: to_sq, promotion: Some(p), ep: false});
        } 
      } else {
        moves.push(Move{piece: Pieces::Pawn, from: from_sq, to: to_sq, promotion: None, ep: false});
      }
    }

    moves
  }

  pub fn pseudo_legal(game: &GameState) -> Vec<Move> {
    let mut moves = vec![];

    let board = game.get_relative_board();
    let player = game.get_player();
    let ep_square = game.get_ep();
    let castling_r = game.get_castling();

    let can_castle_kingside;
    let can_castle_queenside;

    match player {
      Player::White => {
        can_castle_kingside = castling_r & (CASTLE_WHITE_KINGSIDE) != 0;
        can_castle_queenside = castling_r & (CASTLE_WHITE_QUEENSIDE) != 0;
      },
      Player::Black => {
        can_castle_kingside = castling_r & (CASTLE_BLACK_KINGSIDE) != 0;
        can_castle_queenside = castling_r & (CASTLE_BLACK_QUEENSIDE) != 0;
        //board.flip();
      },
    }


    moves.extend(MoveGen::pawn_moves(&board, player, ep_square));
    moves.extend(MoveGen::knight_moves(&board, player));
    moves.extend(MoveGen::rook_moves(&board, player));
    moves.extend(MoveGen::bishop_moves(&board, player));
    moves.extend(MoveGen::queen_moves(&board, player));
    moves.extend(MoveGen::king_moves(&board, player, can_castle_kingside, can_castle_queenside));


    moves
  }
}
