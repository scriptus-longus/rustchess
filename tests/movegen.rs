use rustchess::board::{Pieces, Player, Board};
use rustchess::movegen::{MoveGen, Move};

#[cfg(test)]
mod movegen_test {
  use super::*;

  #[test]
  fn test_pawn_moves() {
    let board = Board::from_fen("8/8/8/1r6/8/1r6/P7/8 w - -");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::pawn_moves(&x, Player::White, None);
        assert_eq!(moves, vec![Move{piece: Pieces::Pawn, from: 15, to: 15+8, promotion: None, ep: false}, 
                               Move{piece: Pieces::Pawn, from: 15, to: 15 + 16, promotion: None, ep: false}, 
                               Move{piece: Pieces::Pawn, from: 15, to: 15 + 8 - 1, promotion: None, ep: false}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
    }
  }

  #[test]
  fn test_pawn_promotions() {
    let board = Board::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::pawn_moves(&x, Player::White, None);
        assert_eq!(moves, vec![Move{piece: Pieces::Pawn, from: 63-8, to: 63, promotion: Some(Pieces::Knight), ep: false}, 
                               Move{piece: Pieces::Pawn, from: 63-8, to: 63, promotion: Some(Pieces::Bishop), ep: false}, 
                               Move{piece: Pieces::Pawn, from: 63-8, to: 63, promotion: Some(Pieces::Rook), ep: false}, 
                               Move{piece: Pieces::Pawn, from: 63-8, to: 63, promotion: Some(Pieces::Queen), ep: false}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
    }
  }

  #[test]
  fn test_pawn_not_a_file() {
    let board = Board::from_fen("p6p/p6p/p6p/p6p/p6p/p6p/p6p/p6P w - - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::pawn_moves(&x, Player::White, None);
        assert_eq!(moves, vec![])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
    }
  }

  #[test]
  fn test_pawn_not_h_file() {
    let board = Board::from_fen("p6p/p6p/p6p/p6p/p6p/p6p/p6p/P6p w - - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::pawn_moves(&x, Player::White, None);
        assert_eq!(moves, vec![])
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
        assert_eq!(moves, vec![Move{piece: Pieces::Knight, from: 4*8 + 6, to: 6*8 + 5, promotion: None, ep: false},
                               Move{piece: Pieces::Knight, from: 4*8 + 6, to: 6*8 + 7, promotion: None, ep: false}])
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
        assert_eq!(moves, vec![Move{piece: Pieces::Rook, from: 7*8 + 7, to: 5*8 + 7, promotion: None, ep: false}, 
                               Move{piece: Pieces::Rook, from: 7*8 + 7, to: 6*8 + 7, promotion: None, ep: false}])
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
        assert_eq!(moves, vec![Move{piece: Pieces::Queen, from: 7, to: 5, promotion: None, ep: false}, 
                               Move{piece: Pieces::Queen, from: 7, to: 6, promotion: None, ep: false}, 
                               Move{piece: Pieces::Queen, from: 7, to: 8 + 6, promotion: None, ep: false}, 
                               Move{piece: Pieces::Queen, from:7, to: 8*2 + 5, promotion: None, ep: false}])
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
        let moves = MoveGen::king_moves(&x, Player::White, false, false);
        assert_eq!(moves, vec![Move{piece: Pieces::King, from: 8 + 7, to: 8 + 6, promotion: None, ep: false}, 
                               Move{piece: Pieces::King, from: 8 + 7, to: 8*2 + 6, promotion: None, ep: false}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }

  #[test]
  fn test_king_castle_kingside() {
    let board = Board::from_fen("8/8/8/4B3/8/8/2PPPP2/R1BQK2R w KQ - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::king_moves(&x, Player::White, true, true);
        assert_eq!(moves, vec![Move{piece: Pieces::King, from: 3, to: 2, promotion: None, ep: false}, 
                               Move{piece: Pieces::King, from: 3, to: 1, promotion: None, ep: false}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }

  #[test]
  fn test_king_castle_queenside() {
    let board = Board::from_fen("8/8/8/4B3/8/8/1QPPPP2/R3KB1R w KQ - 0 1");
    match board {
      Ok(x) => {
        x.print_board();
        let moves = MoveGen::king_moves(&x, Player::White, true, true);
        assert_eq!(moves, vec![Move{piece: Pieces::King, from: 3, to: 4, promotion: None, ep: false}, 
                               Move{piece: Pieces::King, from: 3, to: 5, promotion: None, ep: false}])
      },
      Err(_) => {println!("could not create board"); assert_eq!(1,0)},
      
    }
  }
}
