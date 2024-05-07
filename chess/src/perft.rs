use crate::{
    board::Board,
    move_generator::MoveGenerator,
    piece::{Color, Piece, PieceType},
};

pub fn perft(depth: u32, board: &mut Board, is_top_level: bool) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = MoveGenerator::generate_legal_moves(board);

    (0..moves.len()).for_each(|i| {
        let current_move = moves[i];
        // println!("{} {}", current_move.from, current_move.to);
        // if board.piece_at(14, Color::Black).is_some()
        //     && board.piece_at(14, Color::Black).unwrap() == PieceType::Knight
        // {
        //     println!("here")
        // }
        // board.print_board();
        board.make_move(current_move);
        let moves_after_move = perft(depth - 1, board, false);
        nodes += moves_after_move;
        if is_top_level {
            // let promotion_piece = current_move.promoted_piece;
            // let promotion_print = match promotion_piece {
            //     Some(piece) => match piece {
            //         PieceType::Queen => "q",
            //         PieceType::Rook => "r",
            //         PieceType::Bishop => "b",
            //         PieceType::Knight => "n",
            //         _ => "",
            //     },
            //     None => "",
            // };
            println!(
                "{} {}",
                current_move.to_standard_notation(),
                moves_after_move
            );
        }
        // if board.piece_at(14, Color::Black).is_some()
        //     && board.piece_at(14, Color::Black).unwrap() == PieceType::Knight
        // {
        //     println!("here")
        // }
        board.unmake();
        // if board.piece_at(14, Color::Black).is_some()
        //     && board.piece_at(14, Color::Black).unwrap() == PieceType::Knight
        // {
        //     println!("here")
        // }
    });

    if is_top_level {
        println!("\n{}", nodes);
    }

    nodes
}
