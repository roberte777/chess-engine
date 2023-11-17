use crate::{
    board::{Board, DIRECTION_OFFSETS, NUM_SQUARES_TO_EDGE},
    piece::Piece,
};

pub struct Move {
    pub start_square: u32,
    pub target_square: u32,
}

pub fn generate_moves(board: Board) -> Vec<Move> {
    let mut moves = Vec::new();
    for square in 0..64 {
        let piece = board.squares[square];
        if Piece::is_color(piece, board.color_to_move) {
            if Piece::is_sliding_piece(piece) {
                generate_sliding_piece_moves(square, piece, &board, &mut moves);
            }
        }
    }
    moves
}
pub fn generate_sliding_piece_moves(
    square: usize,
    piece: u32,
    board: &Board,
    moves: &mut Vec<Move>,
) {
    let start_dir_index = if piece == Piece::BISHOP { 4 } else { 0 };
    let end_dir_index = if piece == Piece::ROOK { 4 } else { 8 };
    (start_dir_index..end_dir_index).for_each(|direction| {
        for n in 0..NUM_SQUARES_TO_EDGE[square][direction] {
            let target_square = square as i32 + DIRECTION_OFFSETS[direction] * (n + 1) as i32;

            let target_piece = board.squares[target_square as usize];
            if target_piece == Piece::NONE {
                // free to move
                moves.push(Move {
                    start_square: square as u32,
                    target_square: target_square as u32,
                });
            } else if Piece::is_color(target_piece, board.color_to_move) {
                // blocked by friendly piece
                break;
            } else {
                // capture
                moves.push(Move {
                    start_square: square as u32,
                    target_square: target_square as u32,
                });
                break;
            }
        }
    });
}
