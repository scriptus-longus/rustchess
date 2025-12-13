use rustyline::{DefaultEditor};
use rustyline;
use rustyline::error::ReadlineError;
use itertools::Itertools;
use rand::prelude::*;

//use crate::board::{Player, Board};
use crate::game::{Game};
use crate::movegen::{Move, MoveGen};

mod board;
mod game;

mod movegen;

fn parse_position<'a, I >(tokens: &mut I, game: &mut Game) -> Result<(), &'static str>
where
  I: Iterator<Item = &'a str>,
{
  // load position
  match tokens.next() {
    Some("startpos") => {
      game.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    },
    Some("fen") => {
      let fen_str = tokens.take(6).join(" "); //&tokens[1..8].join(' ');
      
      game.load_fen(&fen_str)?;
    },
    _ => {
      return Err("Position not recognized");
    }
  }

  // make moves
  match tokens.next() {
    Some("move") => {
      while let Some(m_fen) = tokens.next() {
        let m: Move = Move::from_lan(m_fen, &game.state).unwrap();
        
        match game.do_move(&m) {
          None => return Err("Could not make move"),
          Some(_) => (),
        }
      }
    },
    None => return Ok(()),
    _ => println!("Not passed correctly"),
  }

  Ok(())
}

fn uci(s: &str, game: &mut Game) {
  let mut tokens = s.split_whitespace();

  match tokens.next() {
    Some("uci") => {
      println!("info name rustchess");
      println!("info author scriptus_longus");
      println!("uciok");
    },
    Some("isready") => {
      println!("readyok!"); 
    },
    Some("ucinewgame") => (),
    Some("position") => {
      match parse_position(&mut tokens, game) {
        Ok(_) => {
          println!("info string success");
          game.state.print_state();
        },
        Err(x) => println!("info string Error: {}", x),
      }
    },
    Some("go") => {
      let mut moves = MoveGen::pseudo_legal(&game.state);

      let m = loop {
        let (i, m_candidate) = moves.iter().enumerate().choose(&mut rand::rng()).unwrap(); 
        if let Some(_) = game.do_move(&m_candidate) {
          game.undo_move();
          break m_candidate;
        }

        moves.remove(i);
      };

      println!("Move: {:?}", m);

      let lan = Move::to_lan(&m, &game.state).unwrap();

      println!("bestmove {}", lan);
    },
    Some("makemove") => {
      let m = match tokens.next() {
        Some(x) => {

          match Move::from_lan(x, &game.state) {
            Ok(x) => x,
            Err(x) => {println!("info string Not a valid move {}", x); return},
          }

        }
        None => {
          println!("info string No move was given");
          return;
        }
      };

      if MoveGen::pseudo_legal(&game.state).iter().contains(&m) {
        match game.do_move(&m) {
          Some(_) => {
            game.state.print_state();
          }
          None => {
            println!("info string invalid move")
          }
        }
      } else {
        println!("info string Move is invalid {:?}", m);
        let moves = MoveGen::pseudo_legal(&game.state);

        for i in 0..moves.len() {
          println!("Move: {:?}", moves.get(i));
        }
      }
    },
    Some("print") => {
      game.state.print_state();
    },
    _ => (),
  }
}

fn main() -> rustyline::Result<()> {  
  let mut rl = DefaultEditor::new()?;

  let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

  loop {
    let readline = rl.readline("> ");

    match readline {
      Ok(l) => {
        rl.add_history_entry(l.as_str())?;

        if l == "exit" {
          println!("Exiting...");
          break;
        } else {
          uci(&l, &mut game);
        }
      },
      Err(ReadlineError::Interrupted) => {
        println!("Exiting...");
        break;
      }
      _ => {
        println!("Error reading line");
      }
    }
  }

  Ok(())
}
