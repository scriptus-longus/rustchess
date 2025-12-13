use crate::board::{Board, Player, Pieces, BitBoard};
use crate::movegen::{Move, MoveGen};

pub const CASTLE_WHITE_KINGSIDE: u8 = 0b1 << 3;
pub const CASTLE_WHITE_QUEENSIDE: u8 = 0b1 << 2;
pub const CASTLE_BLACK_KINGSIDE: u8 = 0b1 << 1;
pub const CASTLE_BLACK_QUEENSIDE: u8 = 0b1 << 0;

pub enum GameResult {
  Win(Player),
  Remis,
  NotDone,
}

pub fn algebraic_to_shift(pos: &str) -> Option<u32> {
  if pos.len() != 0 {return None; }

  let mut chars = pos.chars();

  let file = chars.next()?;
  let rank = chars.next()?;

  if !('a'..='h').contains(&file) { return None;} 
  
  if !('1'..='8').contains(&rank) { return None;}

  let file_idx = ((file as u8) - b'a') as u32;
  let rank_idx = 7 - ((rank as u8) - b'1') as u32;

  Some(file_idx * 8 + rank_idx)
}

/*pub fn shift_to_algebraic(shift: u32) -> String {
  let file = (shift / 8) as usize;
  let rank = 7 - (shift % 8) as usize;

  let file_c = ('a'..='h').into_iter().nth(file).unwrap();
  let rank_c = ('1'..='8').into_iter().nth(rank).unwrap();
    
  let mut ret_str = String::with_capacity(2);
  ret_str.push(file_c);
  ret_str.push(rank_c);
  
  ret_str
}*/

#[derive(Copy, Clone)]
pub struct GameState {
  //history: Vec<BoardHistoryEntry>,
  pub relative_board: Board,
  //board: Board,
  player: Player,
  castling: u8,
  ep_square: Option<u32>,
  halfmove_clock: u32,
  fullmove_clock: u32,
  king_moved: bool,
  queenside_rook_moved: bool,
  kingside_rook_moved: bool,
}

pub struct History {
  history: Vec<GameState>,
  idx: i32,
}

pub struct Game {
  history: History,
  pub state: GameState,
}

impl Game {
  pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
    let state = GameState::from_fen(fen);
    match state {
      Ok(state) => {
        let history = History::new(state);
        Ok(Game { state:state, history: history})
      },
      Err(x) => Err(x),
    }
  }

  pub fn load_fen(&mut self, fen: &str) -> Result<(), &'static str> {
    self.history.clear(); 

    self.state = GameState::from_fen(fen)?;
    self.history.push(self.state);

    Ok(())
  }

  pub fn is_check(&self, player: Player) -> bool {
    self.state.is_check(player)
  }

  pub fn do_move(&mut self, m: &Move) -> Option<GameResult> {
    let player = self.state.get_player();
    self.state.make_move(m);

    self.history.push(self.state);

    if self.state.is_check(player) {
      self.undo_move();
      return None;
    }
    
    Some(GameResult::NotDone)
  }

  pub fn moves(&self) -> Vec<Move> {
    MoveGen:: pseudo_legal(&self.state)
  }

  pub fn undo_move(&mut self) {
    self.history.pop();

    self.state = match self.history.peek() {
      Some(x) => x,
      None => return,
    }
  }
}

impl History {
  pub fn new(game: GameState) -> Self {
    let history = vec![game];
    History {history: history, idx: 0}
  }

  pub fn clear(&mut self) {
    self.history.clear();
    self.idx = 0;
  }

  pub fn push(&mut self, game: GameState) {
    self.history.push(game);
    self.idx += 1;
  }

  pub fn pop(&mut self) -> Option<GameState> {
    if self.idx < 0 {
      return None;
    }

    let ret = Some(self.history[self.idx as usize]);

    self.history.remove(self.idx as usize);
    self.idx -= 1;

    ret
  }

  pub fn peek(&self) -> Option<GameState> {
    if self.idx < 0 {
      return None;
    }
    
    let ret = Some(self.history[self.idx as usize]);
    ret
  }
}

impl GameState {
  pub fn algebraic_to_shift(&self, pos: &str) -> Option<u32> {
    if pos.len() < 2 {return None; }

    let mut chars = pos.chars();

    let file = chars.next()?;
    let rank = chars.next()?;

    if !('a'..='h').contains(&file) { return None;} 
    
    if !('1'..='8').contains(&rank) { return None;}

    /*let file_idx = if self.player == Player::White {
      let x = ((file as u8) - b'a') as u32;
      x
    } else {
      let x = 7 - ((file as u8) - b'a') as u32;
      x
    };

    let rank_idx = 7 - ((rank as u8) - b'1') as u32;*/

    let file_idx = 7 - ((file as u8) - b'a') as u32;
    let mut rank_idx = ((rank as u8) - b'1') as u32;
    
    if self.player == Player::Black {
      rank_idx = 7 - rank_idx;
    }


    Some(file_idx + rank_idx * 8)
  }

  pub fn shift_to_algebraic(&self, shift: u32) -> Option<String> {
    //let mut file = (shift / 8) as usize;
    //let mut rank = 7 - (shift % 8) as usize;
    let file = 7 - (shift % 8) as usize;
    let mut rank = (shift / 8) as usize;

    if file > 7 || rank > 7 {
      return None;
    } 

    if self.player == Player::Black {
      rank = 7 - rank;
    }

    let file_c = ('a'..='h').into_iter().nth(file).unwrap();
    let rank_c = ('1'..='8').into_iter().nth(rank).unwrap();
      
    let mut ret_str = String::with_capacity(2);
    ret_str.push(file_c);
    ret_str.push(rank_c);
    
    Some(ret_str)
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
      Some(x) => algebraic_to_shift(x),
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
 
 
    let mut rel_board = board.clone();
    if active == Player::Black {
      rel_board.flip();
    }
  
    Ok(GameState {relative_board: rel_board,
                 //board: board,
                 player: active,
                 castling: castling,
                 ep_square: ep_target,
                  halfmove_clock: half_moves,
                  fullmove_clock: full_moves,
                  king_moved: false,
                  kingside_rook_moved: false,
                  queenside_rook_moved: false,
              })
  }


  pub fn is_check(&self, player: Player) -> bool {
    let opp = match player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let attacked = MoveGen::get_all_attacks(&self.relative_board, opp);

    (attacked & self.relative_board.get_pieceboard(player, Pieces::King).bitboard) != 0 

  }

  pub fn is_checkmate(&self, player: Player) -> bool {
    let opp = match self.player {
      Player::White => Player::Black,
      Player::Black => Player::White,
    };

    let attacks = MoveGen::get_all_attacks(&self.relative_board, opp);
    let king = self.relative_board.get_pieceboard(player, Pieces::King).bitboard;

    if (king & attacks)  != 0 {
      // check if the king can get away
      let mut king_moves = MoveGen::get_king_attacks(king);
      king_moves &= !(self.relative_board.get_player_mask(player));

      let outs = (king_moves & attacks) ^ king_moves;
      if outs == 0 {
        return true;
      }
    }

    false
  }

  pub fn make_move(&mut self, m: &Move) {
    let piece = m.piece;
    let next_player = match self.player {
      Player::White => {
        self.halfmove_clock += 1;
        Player::Black
      },
      Player::Black => {
        self.halfmove_clock += 1;
        self.fullmove_clock += 1;
        Player::White
      }
    };

    // capture
    if let Some((x, y)) = self.relative_board.get_piece(m.to as i32){
      self.relative_board.flip_piece(x, y, m.to as i32).unwrap();
    }

    // move piece
    self.relative_board.flip_piece(self.player, piece, m.from as i32).unwrap();
    self.relative_board.flip_piece(self.player, piece, m.to as i32).unwrap();

    self.ep_square = None;

    // extra handling
    match piece {
      Pieces::Pawn => {
        if let Some(_) = m.promotion {
          self.relative_board.flip_piece(self.player, piece, m.to as i32).unwrap();
        } else if m.ep == true {
          self.relative_board.flip_piece(next_player, piece, (m.to - 8) as i32).unwrap();
        } else if m.to - m.from == 16 {
          let row = m.to / 8;
          let col = m.to % 8;

          self.ep_square = Some((7 - row) + col);
        }
      },
      Pieces::King => {
        if m.from == 3 && m.to == 1 {
          self.relative_board.flip_piece(self.player, Pieces::Rook, 0).unwrap();
          self.relative_board.flip_piece(self.player, Pieces::Rook, 2).unwrap();
        } else if m.from == 3 && m.to == 5 {
          self.relative_board.flip_piece(self.player, Pieces::Rook, 7).unwrap();
          self.relative_board.flip_piece(self.player, Pieces::Rook, 4).unwrap();
        }

        if self.player == Player::White { 
          self.castling &= !(CASTLE_WHITE_KINGSIDE | CASTLE_WHITE_QUEENSIDE);
        } else {
          self.castling &= !(CASTLE_BLACK_KINGSIDE |  CASTLE_BLACK_QUEENSIDE);
        }
      },
      Pieces::Rook => {
        if self.kingside_rook_moved == false && m.from == 0 {
          self.kingside_rook_moved = true;

          match self.player {
            Player::White => self.castling &= !(CASTLE_WHITE_KINGSIDE),
            Player::Black => self.castling &= !(CASTLE_BLACK_KINGSIDE),
          };
        }
        if self.queenside_rook_moved == false && m.from == 7 {
          self.queenside_rook_moved = true;

          match self.player {
            Player::White => self.castling &= !(CASTLE_WHITE_QUEENSIDE),
            Player::Black => self.castling &= !(CASTLE_BLACK_QUEENSIDE),
          };
        }
        

        /*let (kingside, queenside) = match self.player {
          Player::White => (self.castling & CASTLE_WHITE_KINGSIDE != 0, self.castling & CASTLE_WHITE_QUEENSIDE != 0),
          Player::Black => (self.castling & CASTLE_BLACK_KINGSIDE != 0, self.castling & CASTLE_BLACK_QUEENSIDE != 0),
        };

        if kingside && m.from == 0 {
          match self.player {
            Player::White => self.castling &= !(CASTLE_WHITE_KINGSIDE),
            Player::Black => self.castling &= !(CASTLE_BLACK_KINGSIDE),
          };
        } else if queenside && m.from == 7 {
          match self.player {
            Player::White => self.castling &= !(CASTLE_WHITE_QUEENSIDE),
            Player::Black => self.castling &= !(CASTLE_BLACK_QUEENSIDE),
          };
        }*/
      }
      _ => (),
    }

    self.relative_board.flip();
    self.player = next_player;
  }

  /*pub fn get_board(&self) -> Board {
    self.board
  }*/

  pub fn get_relative_board(&self) -> Board {
    self.relative_board
  }

  pub fn get_player(&self) -> Player {
    self.player
  }

  pub fn get_ep(&self) -> Option<u32> {
    self.ep_square
  }

  pub fn get_castling(&self) -> u8 {
    self.castling
  }

  pub fn print_state(&mut self) {
    match self.player {
      Player::White => println!("Player is white"),
      Player::Black => {
        self.relative_board.flip();
        println!("Player is black");
      }
    }

    self.relative_board.print_board();

    if self.player == Player::Black {
      self.relative_board.flip();
    }
  }
}



