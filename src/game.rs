use crate::board::{Board, Player, BitBoard};


pub const CASTLE_WHITE_KINGSIDE: u8 = 0b1 << 3;
pub const CASTLE_WHITE_QUEENSIDE: u8 = 0b1 << 2;
pub const CASTLE_BLACK_KINGSIDE: u8 = 0b1 << 1;
pub const CASTLE_BLACK_QUEENSIDE: u8 = 0b1 << 0;

pub struct BoardHistoryEntry {
  board: Board,
  player: Player,
}

#[allow(dead_code)]
pub struct GameState {
  //history: Vec<BoardHistoryEntry>,
  board: Board,
  player: Player,
  castling: u8,
  ep_square: Option<i32>,
  halfmove_clock: u32,
  fullmove_clock: u32,
}

impl GameState {
  pub fn algebraic_to_shift(pos: &str) -> Option<i32> {
    if pos.len() != 0 {return None; }

    let mut chars = pos.chars();

    let file = chars.next()?;
    let rank = chars.next()?;

    if !('a'..='h').contains(&file) { return None;} 
    
    if !('1'..='8').contains(&rank) { return None;}

    let file_idx = ((file as u8) - b'a') as i32;
    let rank_idx = ((rank as u8) - b'1') as i32;

    Some(file_idx * 8 + rank_idx)
  }

  pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
    let mut fields = fen.split(" ");


    let board = match fields.next() {
      Some(x) => Board::from_fen(x)?,
      None => return Err("Invalid FEN String. No board configuration found."),
    };

    let active = match fields.next() {
      Some(x) => match x {
        "w" => Player::White,
        "b" => Player::Black,
        _ => return Err("Invalid FEN String. Player must be 'w' or 'b'."),
      },
      None => return Err("Invalid FEN String. Player must be provided")
    };

    // extract castling mask
    let castling = match fields.next() {
      Some(x) => {
        let mut c: u8 = 0b0;

        for ch in x.chars() {
          match ch {
            'Q' => c |= CASTLE_WHITE_QUEENSIDE,
            'K' => c |= CASTLE_WHITE_KINGSIDE,
            'q' => c |= CASTLE_BLACK_QUEENSIDE,
            'k' => c |= CASTLE_BLACK_KINGSIDE,
            '-' => break,
            _ => break,
          }
        }
        c
      }
      None => return Err("Invalid FEN String. Castling rights not specified")
    };

    let ep_target =  match fields.next() {
      Some(x) => GameState::algebraic_to_shift(x),
      None => return Err("Invalid FEN String. No en-passant targets provided")
    };

    let half_moves: u32 = match fields.next() {
      Some(x) => x.parse().expect("Invalid FEN String. halfmove clock is not a nubmer"),
      None => return Err("Invalid FEN String. Halfmove Clokc not specified."),
    }
    ;
    let full_moves: u32 = match fields.next() {
      Some(x) => x.parse().expect("Invalid FEN String. Move clock is not a nubmer"),
      None => return Err("Invalid FEN String. Move Clokc not specified."),
    };
  

    Ok(GameState {board: board,
                 player: active,
                 castling: castling,
                 ep_square: ep_target,
                  halfmove_clock: half_moves,
                  fullmove_clock: full_moves,
              })
  }

  pub fn get_board(&self) -> Board {
    self.board
  }

  pub fn get_player(&self) -> Player {
    self.player
  }

  pub fn get_ep(&self) -> Option<i32> {
    self.ep_square
  }

  pub fn print_state(&self) {
    self.board.print_board();
  }
}



