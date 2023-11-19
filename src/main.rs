use std::io;

use chess_engine::{board::Board, piece::Piece};

fn main() {
    let mut board = Board::new();
    loop {
        let turn = if board.color_to_move == Piece::WHITE {
            "White"
        } else {
            "Black"
        };
        println!("Turn: {}", turn);
        println!("{}", board);
        // take input for piece you want
        // take input for square you want to move to
        // check if move is valid
        // if move is valid, make move
        println!("What piece do you want to move?");
        let start_piece = ask_for_piece();
        let start_piece = match start_piece {
            Command::Piece(piece) => piece,
            Command::Undo => {
                board.human_undo();
                continue;
            }
        };
        println!("Where move to?");
        let end_piece = ask_for_piece();
        let end_piece = match end_piece {
            Command::Piece(piece) => piece,
            Command::Undo => {
                board.human_undo();
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
    let string_piece: String = piece.parse().expect("Not a number");
    let piece = convert_standard_chess_notation_to_index(&string_piece);
    Command::Piece(piece)
}

fn convert_standard_chess_notation_to_index(square: &str) -> usize {
    let mut square = square.chars();
    let file = square.next().unwrap();
    let rank = square.next().unwrap();
    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid file"),
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
        _ => panic!("Invalid rank"),
    };
    rank * 8 + file
}
