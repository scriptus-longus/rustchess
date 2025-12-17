use rustyline::{DefaultEditor};
use rustyline;
use rustyline::error::ReadlineError;
use itertools::Itertools;
use rand::prelude::*;
use std::io::{self, BufRead};

use crate::game::{Game, GameResult};
use crate::board::{Player, BitBoard};
use crate::movegen::{Move, MoveGen};
//use crate::perft;

mod board;
mod game;
mod movegen;
mod perft;

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
      let fen_str = tokens.take(6).join(" "); 
      
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
        
        match game.makemove(&m) {
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
      println!("id name rustchess");
      println!("id author scriptus_longus");
      println!("uciok");
    },
    Some("isready") => {
      println!("readyok");
    },
    Some("ucinewgame") => {
      println!("");
    },
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
      match tokens.next() {
        Some("perft") => {
          let depth = match tokens.next() {
            Some(x) => match x.chars().next().unwrap().to_digit(10) {
              Some(d) => d as usize,
              _ => 0,
            },
            _ => 0,
          };

          let n = perft::debug_perft(game, depth, true);
          println!();
          println!("{}", n);
        },
        _ => {
          let moves = game.legal_moves();

          if moves.len() == 0 {
            println!("println string No moves possible");
            println!("bestmove 0000");
            return;
          }

          let m = moves.iter().choose(&mut rand::rng()).unwrap(); 
          let lan = Move::to_lan(&m, &game.state).unwrap();

          println!("bestmove {}", lan);
        },
      }

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

      if game.legal_moves().iter().contains(&m) {
        match game.makemove(&m) {
          Some(x) => {
            game.state.print_state();
            match x {
              GameResult::Win(Player::White) => println!("info string White has won! CHECKMATE for black"),
              GameResult::Win(Player::Black) => println!("info string Black has won! CHECKMATE for white"),
              GameResult::Remis => println!("info string Remis"),
              _ => (),
            }
          }
          None => {
            println!("info string invalid move")
          }
        }
      } else {
        println!("info string Move is invalid {:?}", m);
        println!("info string Available moves:");
        let moves = MoveGen::pseudo_legal(&game.state);

        for i in 0..moves.len() {
          println!("info string Move: {:?}", moves.get(i));
        }
      }
    },
    Some("attacks") => {
      let bb = BitBoard{bitboard: game.state.get_attacks()};
      println!("info string Attacks");
      bb.print_bitboard()
    }
    Some("unmakemove") => {
      game.undo_move();
    },
    Some("print") => {
      game.state.print_state();
    },
    _ => (),
  }
}

fn main() -> rustyline::Result<()> {  
//fn main() {
  println!("   ___ _               _            ___ ");
  println!("  / __\\ |__   ___  ___| | __ /\\/\\  ( _ )");
  println!(" / /  | '_ \\ / _ \\/ __| |/ //    \\ / _ \\");
  println!("/ /___| | | |  __/ (__|   </ /\\/\\ \\ (_) |");
  println!("\\____/|_| |_|\\___|\\___|_|\\_\\/    \\/\\___/");
  println!();                                         

  
  let mut game = Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

  let mut rl = DefaultEditor::new()?;
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

  /*let stdin = io::stdin();
  let mut stdin = stdin.lock();

  let mut line = String::new();

  loop {
      line.clear();

      // Read one line from stdin
      if stdin.read_line(&mut line).unwrap() == 0 {
          break; // EOF
      }

      let cmd = line.trim();
      if cmd.is_empty() {
          continue;
      }

      if cmd == "quit" {
          break;
      }

      uci(cmd, &mut game);
  }*/

  Ok(())
}

