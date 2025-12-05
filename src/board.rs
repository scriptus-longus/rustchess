use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Pieces {
  Pawn,
  Rook,
  Knight,
  Bishop,
  Queen,
  King
}

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum Player {
  White,
  Black
}

#[derive(Copy, Clone)]
pub struct BitBoard {
  pub bitboard: u64,
}

impl BitBoard {
  pub fn new() -> Self {
    BitBoard {bitboard: 0u64 }
  }
  
  pub fn flip_bit(&mut self, file: usize, rank: usize) -> Result<(), &'static str>{
    if (file >= 8) || (rank >= 8) {
      return Err("file and rank must be between 0 and 8");
    }

    let shift = rank * 8 + file; //64 - ((rank+1) * 8) + file;
    let mask = 1u64 << shift;

    self.bitboard ^= mask;
    Ok(())
  }

  pub fn get_bit(&self, file: usize, rank: usize) ->  Result<bool, &'static str> {
    if (file >= 8) || (rank >= 8) {
      return Err("file and rank must be between 0 and 8");
    }

    let shift = rank * 8 + file;
    let mask = 1u64 << shift;
  
    if mask & self.bitboard != 0 {
      Ok(true)
    } else {
      Ok(false)
    }
  }
 

  #[allow(dead_code)]
  pub fn print_bitboard(&self) {
    for i in 0..8 {
      for j in 0..8 {
        let shift = 64 - ((i+1) * 8) + j;

        let mask: u64 = 1u64 << shift; 
        let bit = if mask & self.bitboard != 0 {'1'} else {'0'};
        print!("{bit}");
      }
      println!(" ");
    }
  }
}

#[derive(Copy, Clone)]
pub struct Board {
  //bitboard: [[u8; 6]; 2],
  bb_board: [[BitBoard; 6]; 2]
}

impl Board {
  pub fn get_piece(&self, file: usize, rank: usize) -> Option<(Player, Pieces)> {
    for player in Player::iter() {
      for piece in Pieces::iter() {
        if self.bb_board[player.clone() as usize][piece.clone() as usize].get_bit(file, rank) == Ok(true) {
          return Some((player, piece));
        } 
      }
    }

    None
  }


  pub fn get_pieceboard(&self, player: Player, piece: Pieces) -> BitBoard {
    self.bb_board[player as usize][piece as usize].clone()
  }
  
  pub fn get_freesq_mask(&self) -> u64 {
    let mut ret = 0u64;

    for player in Player::iter() {
      for piece in Pieces::iter() {
        ret |= self.bb_board[player.clone() as usize][piece.clone() as usize].bitboard;
      }
    }

    !ret
  }

  pub fn get_player_mask(&self, player: Player) -> u64 {
    let ret: u64 = self.bb_board[player.clone() as usize].iter().fold(0u64, |acc, &bb| bb.bitboard | acc);
    ret
  }


  pub fn flip_piece(&mut self,  player: Player, piece: Pieces, file: usize, rank: usize) -> Result<(), &'static str> {

    self.bb_board[player as usize][piece as usize].flip_bit(file, rank)
  }
  
  pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
    let mut fields = fen.split(" ");

    let mut board_fen = match fields.next() {
      Some(x) => x,
      None => "8/8/8/8/8/8/8/8",
    }.chars();

    let mut ret = Board {bb_board: [[BitBoard::new(); 6]; 2]};

    let mut rank = 7;
    let mut file = 0;

    while let Some(fen_sym) = board_fen.next() {
      let res = match fen_sym {
        'r' => ret.flip_piece(Player::Black, Pieces::Rook, file, rank),
        'b' => ret.flip_piece(Player::Black, Pieces::Bishop, file, rank),
        'p' => ret.flip_piece(Player::Black, Pieces::Pawn, file, rank),
        'q' => ret.flip_piece(Player::Black, Pieces::Queen, file, rank),
        'k' => ret.flip_piece(Player::Black, Pieces::King, file, rank),
        'n' => ret.flip_piece(Player::Black, Pieces::Knight, file, rank),
        'R' => ret.flip_piece(Player::White, Pieces::Rook, file, rank),
        'B' => ret.flip_piece(Player::White, Pieces::Bishop, file, rank),
        'P' => ret.flip_piece(Player::White, Pieces::Pawn, file, rank),
        'Q' => ret.flip_piece(Player::White, Pieces::Queen, file, rank),
        'K' => ret.flip_piece(Player::White, Pieces::King, file, rank),
        'N' => ret.flip_piece(Player::White, Pieces::Knight, file, rank),
        '1'..='8' => {

          if let Some(x) = fen_sym.to_digit(10) {
            file += (x as usize) - 1;
            Ok(())
          } else {
            Err("Could not empty squares to number")
          }
        },
        '/' => {

          if file == 8 {
            rank -= 1;
            file = 0;
            Ok(())
          } else {
            Err("Could not convert empty square to number")
          }
        },
        _ => Err("Symbol not recognized"),
      };

      match res {
        Ok(_) => {
          if fen_sym != '/' {
            file += 1;
          }
        },
        Err(x) => return Err(x),
      }

    }

    Ok(ret)
  }

  #[allow(dead_code)]
  pub fn print_board(&self) {
    for rank in (0..8).rev() {
      for file in 0..8 {
        let x = self.get_piece(file, rank);

        match x {
          Some((Player::Black, Pieces::Rook)) => print!("r"),
          Some((Player::Black, Pieces::Bishop)) => print!("b"),
          Some((Player::Black, Pieces::Pawn)) => print!("p"),
          Some((Player::Black, Pieces::Queen)) => print!("q"),
          Some((Player::Black, Pieces::King)) => print!("k"),
          Some((Player::Black, Pieces::Knight)) => print!("n"),
          Some((Player::White, Pieces::Rook)) => print!("R"),
          Some((Player::White, Pieces::Bishop)) => print!("B"),
          Some((Player::White, Pieces::Pawn)) => print!("P"),
          Some((Player::White, Pieces::Queen)) => print!("Q"),
          Some((Player::White, Pieces::King)) => print!("K"),
          Some((Player::White, Pieces::Knight)) => print!("N"),
          None => print!("."),
        };
      }
      println!();
    }
  }
}




