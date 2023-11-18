use crate::{board::Board, chess_move::generate_legal_moves, piece::Piece};

pub fn perft(depth: u32, board: &mut Board) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = generate_legal_moves(board);
    for m in moves.iter() {
        if Piece::is_type(board.squares[m.start_square as usize], Piece::NONE) {
            println!("gotcha outer");
        }
    }
    println!("depth: {}, moves: {:?}\n", depth, moves);

    (0..moves.len()).for_each(|i| {
        if Piece::is_type(board.squares[moves[i].start_square as usize], Piece::NONE) {
            println!("gotcha inner");
        }
        if !board.make(&moves[i]) {
            println!("illegal move: {:?}", moves[i]);
            return;
        }
        nodes += perft(depth - 1, board);
        board.undo(&moves[i]);
    });

    nodes
}
