use log::warn;

use crate::{
    board::{Board, DIRECTION_OFFSETS, NUM_SQUARES_TO_EDGE},
    piece::Piece,
};
/**
* Generates a board with a 1 if the square is attacked by a sliding piece
*/
pub fn generate_sliding_attack(square: usize, board: &Board, color: u32) -> [usize; 64] {
    let piece = board.squares[square];
    if !Piece::is_sliding_piece(piece) {
        return [0; 64];
    }

    let start_dir_index = if Piece::is_type(piece, Piece::BISHOP) {
        4
    } else {
        0
    };
    let end_dir_index = if Piece::is_type(piece, Piece::ROOK) {
        4
    } else {
        8
    };

    let mut attack_squares = [0; 64];
    (start_dir_index..end_dir_index).for_each(|direction| {
        for n in 0..NUM_SQUARES_TO_EDGE[square][direction] {
            let target_square = square as i32 + DIRECTION_OFFSETS[direction] * (n + 1) as i32;

            let target_piece = board.squares[target_square as usize];
            if Piece::is_type(target_piece, Piece::NONE) {
                // free to move
                attack_squares[target_square as usize] = 1;
            } else if Piece::is_color(target_piece, color) {
                // blocked by friendly piece
                break;
            } else {
                // capture
                attack_squares[target_square as usize] = 1;
                break;
            }
        }
    });
    attack_squares
}
pub fn generate_pawn_attack(square: usize, board: &Board, _color: u32) -> [usize; 64] {
    let piece = board.squares[square];
    let mut attack_squares = [0; 64];
    if !Piece::is_type(piece, Piece::PAWN) {
        warn!("generate pawn attack called on a non pawn piece");
        return attack_squares;
    }
    let direction = if Piece::is_color(piece, Piece::WHITE) {
        0
    } else {
        1
    };
    let _start_rank = if direction == 0 { 1 } else { 6 };
    let rank_offset = if direction == 0 { 1 } else { -1 };
    let start_square = square as i32;
    let target_square = start_square + 8 * rank_offset;
    if !(0..=63).contains(&target_square) {
        warn!("pawn target square is out of bounds");
        return attack_squares;
    }

    let left_target_square = start_square + 8 * rank_offset - 1;

    if left_target_square.abs() % 8 != 7 && (0..64).contains(&left_target_square) {
        attack_squares[left_target_square as usize] = 1;
    }

    let right_target_square = start_square + 8 * rank_offset + 1;

    if right_target_square.abs() % 8 != 0 && (0..64).contains(&right_target_square) {
        attack_squares[right_target_square as usize] = 1;
    }

    // en passant
    if board.en_passant_square.is_some() {
        let en_passant_square = board.en_passant_square.unwrap();
        // make sure we don't flip to the other side of the board
        if en_passant_square as i32 == left_target_square && en_passant_square % 8 == 7 {
            return attack_squares;
        }
        // make sure we don't flip to the other side of the board
        if en_passant_square as i32 == right_target_square && en_passant_square % 8 == 0 {
            return attack_squares;
        }
        if en_passant_square as i32 == left_target_square
            || en_passant_square as i32 == right_target_square
        {
            attack_squares[(en_passant_square as i32 - 8 * rank_offset) as usize] = 1;
        }
    }

    attack_squares
}
pub fn generate_knight_attack(square: usize, board: &Board, color: u32) -> [usize; 64] {
    let mut attack_squares = [0; 64];
    let start_dir_index = 8;
    let end_dir_index = 16;
    (start_dir_index..end_dir_index).for_each(|direction| {
        let target_square = square as i32 + DIRECTION_OFFSETS[direction];

        // if target square is off the board
        if !(0..=63).contains(&target_square) {
            return;
        }
        // make sure the target does not wrap around the board
        let start_row = square / 8;
        let start_col = square % 8;
        let target_row = target_square / 8;
        let target_col = target_square % 8;
        let row_diff = (start_row as i32 - target_row).abs();
        let col_diff = (start_col as i32 - target_col).abs();
        if !((row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)) {
            return;
        }

        let target_piece = board.squares[target_square as usize];
        if target_piece == Piece::NONE {
            // free to move
            attack_squares[target_square as usize] = 1;
        } else if Piece::is_color(target_piece, color) {
            // blocked by friendly piece
            return;
        } else {
            // capture
            attack_squares[target_square as usize] = 1;
        }
    });

    attack_squares
}

pub fn generate_king_attack(square: usize, board: &Board, color: u32) -> [usize; 64] {
    let mut attack_squares = [0; 64];
    let start_dir_index = 0;
    let end_dir_index = 8;
    (start_dir_index..end_dir_index).for_each(|direction| {
        if NUM_SQUARES_TO_EDGE[square][direction] == 0 {
            return;
        }
        let target_square = square as i32 + DIRECTION_OFFSETS[direction];
        // make sure piece is in board
        if !(0..=63).contains(&target_square) {
            return;
        }

        let target_piece = board.squares[target_square as usize];
        if target_piece == Piece::NONE {
            // free to move
            attack_squares[target_square as usize] = 1;
        } else if Piece::is_color(target_piece, color) {
            // blocked by friendly piece
            return;
        } else {
            // capture
            attack_squares[target_square as usize] = 1;
        }
    });

    attack_squares
}

pub fn generate_attack_board_for_color(color: u32, board: &Board) -> [usize; 64] {
    let mut attack_board = [0; 64];
    (0..64).for_each(|square| {
        let piece = board.squares[square];
        if Piece::is_color(piece, color) {
            let attack_squares;
            if Piece::is_type(piece, Piece::PAWN) {
                attack_squares = generate_pawn_attack(square, board, color);
            } else if Piece::is_type(piece, Piece::KNIGHT) {
                attack_squares = generate_knight_attack(square, board, color);
            } else if Piece::is_type(piece, Piece::KING) {
                attack_squares = generate_king_attack(square, board, color);
            } else {
                attack_squares = generate_sliding_attack(square, board, color);
            }
            (0..64).for_each(|i| {
                if attack_squares[i] == 1 {
                    attack_board[i] = 1;
                }
            });
        }
    });
    attack_board
}
