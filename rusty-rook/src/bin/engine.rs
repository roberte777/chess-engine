use std::io;

use chess::{
    board::{Board, STARTING_FEN},
    piece::Piece,
};
use rusty_rook::score::minimax_ab;

fn main() {
    let mut board = Board::from_fen(STARTING_FEN).unwrap();
    loop {
        let turn = if board.color_to_move() == Piece::WHITE {
            "White"
        } else {
            "Black"
        };
        println!("Turn: {}", turn);
        println!("{}", board);
        // engine to move
        if board.color_to_move() == Piece::BLACK {
            let (score, mv) = minimax_ab(&mut board, 6, i32::MIN, i32::MAX);
            // let (score, mv) = minimax(&mut board, 4);
            println!("Score: {}", score);
            if let Some(mv) = mv {
                board.make(&mv);
            }
            continue;
        }
        // take input for piece you want
        // take input for square you want to move to
        // check if move is valid
        // if move is valid, make move
        println!("What piece do you want to move?");
        let start_piece = ask_for_piece();
        let start_piece = match start_piece {
            Command::Piece(piece) => piece,
            Command::Undo => {
                let success = board.human_undo();
                if !success {
                    println!("Nothing to undo");
                }
                continue;
            }
            Command::Unknown => {
                println!("Invalid command");
                continue;
            }
        };
        println!("Where move to?");
        let end_piece = ask_for_piece();
        let end_piece = match end_piece {
            Command::Piece(piece) => piece,
            Command::Undo => {
                let success = board.human_undo();
                if !success {
                    println!("Nothing to undo");
                }
                continue;
            }
            Command::Unknown => {
                println!("Invalid command");
                continue;
            }
        };
        let result = board.human_move(start_piece, end_piece);
        if !result {
            println!("Invalid move");
        }
    }
}

enum Command {
    Piece(usize),
    Undo,
    Unknown,
}
fn ask_for_piece() -> Command {
    let mut piece = String::new();
    io::stdin()
        .read_line(&mut piece)
        .expect("Failed to read line");
    let piece = piece.trim();
    // check if piece is a chess location or "u" for undo
    if piece == "u" {
        return Command::Undo;
    }
    let piece = convert_standard_chess_notation_to_index(piece);
    if piece.is_err() {
        return Command::Unknown;
    }
    Command::Piece(piece.unwrap())
}

fn convert_standard_chess_notation_to_index(square: &str) -> Result<usize, io::Error> {
    let mut square = square.chars();
    let file = square
        .next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file"))?;
    let rank = square
        .next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid rank"))?;
    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file")),
    };
    let rank = match rank {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid rank")),
    };
    Ok(rank * 8 + file)
}
