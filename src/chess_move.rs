use crate::{
    board::{Board, DIRECTION_OFFSETS, NUM_SQUARES_TO_EDGE},
    piece::Piece,
};

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub start_square: u32,
    pub target_square: u32,
    pub captured_piece: Option<u32>,
    pub captured_piece_square: Option<usize>,
    pub is_en_passant: bool,
}

pub fn generate_legal_moves(board: &mut Board) -> Vec<Move> {
    // TODO: Track this on the board struct instead
    let mut legal_moves = Vec::new();
    let pseudo_legal_moves = generate_moves(board);
    for m in pseudo_legal_moves.iter() {
        board.make(m);
        let king_square = board
            .squares
            .iter()
            .position(|&p| {
                Piece::is_type(p, Piece::KING) && !Piece::is_color(p, board.color_to_move)
            })
            .unwrap();
        let opponent_moves = generate_moves(board);
        if !opponent_moves
            .iter()
            .any(|om| om.target_square == king_square as u32)
        {
            legal_moves.push(*m);
        }
        board.undo(m);
    }
    legal_moves
}

pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    for square in 0..64 {
        let piece = board.squares[square];
        if Piece::is_color(piece, board.color_to_move) {
            if Piece::is_sliding_piece(piece) {
                generate_sliding_piece_moves(square, piece, board, &mut moves);
                // check if any start squares in the moves are None
            }
            if Piece::is_type(piece, Piece::PAWN) {
                generate_pawn_moves(square, piece, board, &mut moves);
            }
            if Piece::is_type(piece, Piece::KING) {
                generate_king_moves(square, piece, board, &mut moves);
            }
            if Piece::is_type(piece, Piece::KNIGHT) {
                generate_knight_moves(square, piece, board, &mut moves);
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
    (start_dir_index..end_dir_index).for_each(|direction| {
        for n in 0..NUM_SQUARES_TO_EDGE[square][direction] {
            let target_square = square as i32 + DIRECTION_OFFSETS[direction] * (n + 1) as i32;

            let target_piece = board.squares[target_square as usize];
            if Piece::is_type(target_piece, Piece::NONE) {
                // free to move
                moves.push(Move {
                    start_square: square as u32,
                    target_square: target_square as u32,
                    captured_piece: None,
                    captured_piece_square: None,
                    is_en_passant: false,
                });
            } else if Piece::is_color(target_piece, board.color_to_move) {
                // blocked by friendly piece
                break;
            } else {
                // capture
                moves.push(Move {
                    start_square: square as u32,
                    target_square: target_square as u32,
                    captured_piece: Some(target_piece),
                    captured_piece_square: Some(target_square as usize),
                    is_en_passant: false,
                });
                break;
            }
        }
    });
}

pub fn generate_pawn_moves(square: usize, piece: u32, board: &Board, moves: &mut Vec<Move>) {
    let direction = if Piece::is_color(piece, Piece::WHITE) {
        0
    } else {
        1
    };
    let start_rank = if direction == 0 { 1 } else { 6 };
    let rank_offset = if direction == 0 { 1 } else { -1 };
    let start_square = square as i32;
    let target_square = start_square + 8 * rank_offset;
    if Piece::is_type(board.squares[target_square as usize], Piece::NONE) {
        // free to move
        moves.push(Move {
            start_square: square as u32,
            target_square: target_square as u32,
            captured_piece: None,
            captured_piece_square: None,
            is_en_passant: false,
        });
        if square / 8 == start_rank
            && Piece::is_type(
                board.squares[(target_square + 8 * rank_offset) as usize],
                Piece::NONE,
            )
        {
            moves.push(Move {
                start_square: square as u32,
                target_square: (target_square + 8 * rank_offset) as u32,
                captured_piece: None,
                captured_piece_square: None,
                is_en_passant: false,
            });
        }
    }
    let left_target_square = start_square + 8 * rank_offset - 1;
    if left_target_square % 8 != 7
        && !(Piece::is_type(board.squares[left_target_square as usize], Piece::NONE))
        && !Piece::is_color(
            board.squares[left_target_square as usize],
            board.color_to_move,
        )
    {
        moves.push(Move {
            start_square: square as u32,
            target_square: left_target_square as u32,
            captured_piece: Some(board.squares[left_target_square as usize]),
            captured_piece_square: Some(left_target_square as usize),
            is_en_passant: false,
        });
    }
    let right_target_square = start_square + 8 * rank_offset + 1;
    if right_target_square % 8 != 0
        && !(Piece::is_type(board.squares[right_target_square as usize], Piece::NONE))
        && !Piece::is_color(
            board.squares[right_target_square as usize],
            board.color_to_move,
        )
    {
        moves.push(Move {
            start_square: square as u32,
            target_square: right_target_square as u32,
            captured_piece: Some(board.squares[right_target_square as usize]),
            captured_piece_square: Some(right_target_square as usize),
            is_en_passant: false,
        });
    }

    // en passant
    if board.en_passant_square.is_some() {
        let en_passant_square = board.en_passant_square.unwrap();
        if en_passant_square as i32 == left_target_square && en_passant_square % 8 == 7 {
            return;
        }
        if en_passant_square as i32 == right_target_square && en_passant_square % 8 == 0 {
            return;
        }
        if en_passant_square as i32 == left_target_square
            || en_passant_square as i32 == right_target_square
        {
            moves.push(Move {
                start_square: square as u32,
                target_square: en_passant_square as u32,
                captured_piece: Some(
                    // set the captured piece to the pawn that was captured
                    board.squares[(en_passant_square as i32 - 8 * rank_offset) as usize],
                ),
                captured_piece_square: Some((en_passant_square as i32 - 8 * rank_offset) as usize),
                is_en_passant: true,
            });
        }
    }
}
pub fn generate_king_moves(square: usize, _piece: u32, board: &Board, moves: &mut Vec<Move>) {
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
            moves.push(Move {
                start_square: square as u32,
                target_square: target_square as u32,
                captured_piece: None,
                captured_piece_square: None,
                is_en_passant: false,
            });
        } else if Piece::is_color(target_piece, board.color_to_move) {
            // blocked by friendly piece
            return;
        } else {
            // capture
            moves.push(Move {
                start_square: square as u32,
                target_square: target_square as u32,
                captured_piece: Some(target_piece),
                captured_piece_square: Some(target_square as usize),
                is_en_passant: false,
            });
            return;
        }
    });
}

pub fn generate_knight_moves(square: usize, _piece: u32, board: &Board, moves: &mut Vec<Move>) {
    let start_dir_index = 8;
    let end_dir_index = 16;
    (start_dir_index..end_dir_index).for_each(|direction| {
        let target_square = square as i32 + DIRECTION_OFFSETS[direction];
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
            moves.push(Move {
                start_square: square as u32,
                target_square: target_square as u32,
                captured_piece: None,
                captured_piece_square: None,
                is_en_passant: false,
            });
        } else if Piece::is_color(target_piece, board.color_to_move) {
            // blocked by friendly piece
            return;
        } else {
            // capture
            moves.push(Move {
                start_square: square as u32,
                target_square: target_square as u32,
                captured_piece: Some(target_piece),
                captured_piece_square: Some(target_square as usize),
                is_en_passant: false,
            });
            return;
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::board::STATING_FEN;
    use crate::perft::perft;

    use super::*;
    #[test]
    fn test_stock_perft() {
        let mut board = Board::from_fen(STATING_FEN);
        let perft_1 = perft(1, &mut board);
        assert_eq!(perft_1, 20);
        let perft_2 = perft(2, &mut board);
        assert_eq!(perft_2, 400);
        let perft_3 = perft(3, &mut board);
        assert_eq!(perft_3, 8902);
        let perft_4 = perft(4, &mut board);
        assert_eq!(perft_4, 197_281);
        let perft_5 = perft(5, &mut board);
        assert_eq!(perft_5, 4_865_609);
        let perft_6 = perft(6, &mut board);
        assert_eq!(perft_6, 119_060_324);
        // let perft_7 = perft(7, &mut board);
        // assert_eq!(perft_7, 3_195_901_860);
        // let perft_8 = perft(8, &mut board);
        // assert_eq!(perft_8, 84_998_978_956);
        // let perft_9 = perft(9, &mut board);
        // assert_eq!(perft_9, 2_439_530_234_167);
        // let perft_10 = perft(10, &mut board);
        // assert_eq!(perft_10, 69_352_859_712_417);
    }
}
