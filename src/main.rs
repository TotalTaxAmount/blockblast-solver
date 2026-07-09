use std::{
    io::{self, Write},
    process::exit,
};

use crate::{board::Board, engine::Engine, piece::Piece};

mod board;
mod engine;
mod piece;

fn main() {
    let mut board = Board::from(0);
    let mut pieces: Vec<Piece> = Vec::new();

    loop {
        let input = get_input();
        let args: Vec<&str> = input.split(" ").collect();
        let command: &str = args.first().unwrap_or(&"");

        match command {
            "board" => {
                board = match args.get(1).unwrap().parse::<u64>() {
                    Ok(n) => Board::from(n),
                    Err(_) => {
                        println!("Error parsing board");
                        continue;
                    }
                };
            }
            "piece" => {
                let piece = match args.get(1).unwrap().parse::<u32>() {
                    Ok(n) => Piece::from(n),
                    Err(_) => {
                        println!("Error parsing piece");
                        break;
                    }
                };

                pieces.push(piece);
            }
            "reset" => {
                board = Board::from(0);
                pieces = Vec::new();
            }
            "solve" => {
                let line = Engine::slove(board, pieces.clone());
                let mut b = board;

                println!("state {}", b.0);
                // println!("{:?}", line);

                for m in line.moves {
                    println!("move {} {} {}", m.piece.0, m.pos.0, m.pos.1);

                    b.apply_move(&m);

                    println!("state {}", b.0);
                }

                println!("done");
            }
            "quit" => exit(0),
            _ => println!("Unknown command"),
        }
    }
}

fn get_input() -> String {
    print!("> ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
