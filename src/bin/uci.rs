use std::io::{self, BufRead, Write};

use chess_engine::{
    board::{Board, STARTING_FEN},
    chess_move::Move,
    score::{minimax, minimax_ab},
};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut output = io::stdout();

    let mut board = Board::from_fen(STARTING_FEN).unwrap();

    for line in input.lines() {
        let line = line.expect("Could not read line from standard input");
        if line == "uci" {
            handle_uci(&mut output);
        } else if line.starts_with("position") {
            handle_position(&mut board, &line);
        } else if line.starts_with("go") {
            handle_go(&mut board, &mut output);
        } else if line == "quit" {
            break;
        }
    }
}

fn handle_uci(output: &mut impl Write) {
    writeln!(output, "id name MyChessEngine").expect("Error writing output");
    writeln!(output, "id author Your Name").expect("Error writing output");
    writeln!(output, "uciok").expect("Error writing output");
}

fn handle_position(board: &mut Board, line: &str) {
    // Example: position startpos moves e2e4 e7e5
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts[1] == "startpos" {
        *board = Board::from_fen(STARTING_FEN).unwrap();
        if parts.len() > 3 && parts[2] == "moves" {
            for move_notation in parts[3..].iter() {
                let mv = parse_move(move_notation);
                board.make(&mv);
            }
        }
    } else if parts[1] == "fen" {
        let fen: String = parts[2..].join(" ");
        *board = Board::from_fen(&fen).unwrap();
    }
}

fn handle_go(board: &mut Board, output: &mut impl Write) {
    let (_, mv) = minimax_ab(board, 6, i32::MIN, i32::MAX);
    if let Some(mv) = mv {
        writeln!(output, "bestmove {}", mv.to_standard_notation()).expect("Error writing output");
    } else {
        writeln!(output, "bestmove none").expect("Error writing output");
    }
}

fn parse_move(move_notation: &str) -> Move {
    Move::from_standard_notation(move_notation)
}
