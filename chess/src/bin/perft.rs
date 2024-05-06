use chess::{board::Board, chess_move::Move, perft::perft};
use std::env;

fn main() {
    // Accept depth, fen, moves
    // $depth is the maximum depth of the evaluation,
    // $fen is the Forsyth-Edwards Notation string of some base position,
    // $moves is an optional space-separated list of moves from the base position to the position to be evaluated, where each move is formatted as $source$target$promotion, e.g. e2e4 or a7b8Q.
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are passed
    if args.len() < 3 {
        eprintln!("Usage: program_name depth fen [moves]");
        return;
    }

    // Parse depth as an integer
    let depth: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid depth. Please provide an integer.");
            return;
        }
    };

    // The FEN string
    let fen = &args[2];

    // Optional moves argument
    let moves = if args.len() > 3 {
        Some(args[3..].to_vec())
    } else {
        None
    };

    // Your logic here
    let board = Board::from_fen(fen);
    if board.is_err() {
        eprintln!("Invalid FEN string: {}", board.err().unwrap());
        return;
    }
    let mut board = board.unwrap();

    // perform each move on the board
    if let Some(moves) = moves {
        for m in moves {
            let move_to_make = Move::from_standard_notation(&m);
            if !board.make(&move_to_make) {
                eprintln!("Invalid move: {}", m);
                return;
            }
        }
    }

    let start = std::time::Instant::now();
    perft(depth, &mut board, true);
    println!("Time taken: {:?}", start.elapsed());
}
