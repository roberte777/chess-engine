use crate::{
    board::Board,
    move_generator::MoveGenerator,
};

pub fn perft(depth: u32, board: &mut Board, is_top_level: bool) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = MoveGenerator::generate_legal_moves(board);

    (0..moves.len()).for_each(|i| {
        let current_move = moves[i];
        board.make_move(current_move);
        let moves_after_move = perft(depth - 1, board, false);
        nodes += moves_after_move;
        if is_top_level {
            println!(
                "{} {}",
                current_move.to_standard_notation(),
                moves_after_move
            );
        }
        board.unmake();
    });

    if is_top_level {
        println!("\n{}", nodes);
    }

    nodes
}
