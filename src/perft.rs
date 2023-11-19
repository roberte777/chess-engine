use crate::{board::Board, chess_move::generate_legal_moves, piece::Piece};

pub fn perft(depth: u32, board: &mut Board, is_top_level: bool) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = generate_legal_moves(board);

    (0..moves.len()).for_each(|i| {
        let current_move = moves[i];
        if !board.make(&current_move) {
            println!("illegal move: {:?}", current_move);
            return;
        }
        let moves_after_move = perft(depth - 1, board, false);
        nodes += moves_after_move;
        if is_top_level {
            let promotion_piece = current_move.promoted_piece;
            let promotion_print = match promotion_piece {
                Some(piece) => match piece {
                    Piece::QUEEN => "q",
                    Piece::ROOK => "r",
                    Piece::BISHOP => "b",
                    Piece::KNIGHT => "n",
                    _ => "",
                },
                None => "",
            };
            println!(
                "{}{}{} {}",
                Piece::index_to_standard_notation(moves[i].start_square),
                Piece::index_to_standard_notation(moves[i].target_square),
                promotion_print,
                moves_after_move
            );
        }
        board.undo(&current_move);
    });

    if is_top_level {
        println!("\n{}", nodes);
    }

    nodes
}
