use crate::{
    attack::generate_attack_board_for_color,
    board::{Board, CastleRights, DIRECTION_OFFSETS, NUM_SQUARES_TO_EDGE},
    piece::Piece,
};

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub start_square: u32,
    pub target_square: u32,
    pub captured_piece: Option<u32>,
    pub captured_piece_square: Option<usize>,
    pub promoted_piece: Option<u32>,
    pub is_en_passant: bool,
    pub is_castle: bool,
    pub prev_castle_rights: CastleRights,
}
impl Move {
    pub fn to_standard_notation(&self) -> String {
        let start_square = Piece::index_to_standard_notation(self.start_square);
        let target_square = Piece::index_to_standard_notation(self.target_square);
        let mut notation = format!("{}{}", start_square, target_square);
        if let Some(promoted_piece) = self.promoted_piece {
            notation.push_str(&Piece::get_type(promoted_piece).to_string());
        }
        notation
    }
    pub fn from_standard_notation(notation: &str) -> Move {
        let start_square = Piece::standard_notation_to_index(&notation[0..2]);
        let target_square = Piece::standard_notation_to_index(&notation[2..4]);
        let promoted_piece = if notation.len() == 5 {
            let piece_string = notation.chars().nth(4).unwrap().to_string();
            match {
                match piece_string.as_str() {
                    "q" => Some(Piece::QUEEN),
                    "r" => Some(Piece::ROOK),
                    "b" => Some(Piece::BISHOP),
                    "n" => Some(Piece::KNIGHT),
                    _ => None,
                }
            } {
                Some(piece) => Some(piece),
                None => {
                    println!("Invalid promotion piece: {}", piece_string);
                    None
                }
            }
        } else {
            None
        };
        Move {
            start_square,
            target_square,
            captured_piece: None,
            captured_piece_square: None,
            promoted_piece,
            is_en_passant: false,
            is_castle: false,
            prev_castle_rights: CastleRights::new(),
        }
    }
}

pub fn generate_legal_moves(board: &mut Board) -> Vec<Move> {
    // TODO: Track this on the board struct instead
    let mut legal_moves = Vec::new();
    let pseudo_legal_moves = generate_moves(board);
    for m in pseudo_legal_moves.iter() {
        if !board.make(m) {
            println!("Problem!!!")
        }
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

pub fn generate_moves(board: &mut Board) -> Vec<Move> {
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

    // if we have castling rights, we know this rook hasn't moved before.
    let first_move: bool = if Piece::is_type(piece, Piece::ROOK) {
        if (square == 0 || square == 56) && board.can_castle_queenside() {
            true
        } else {
            (square == 63 || square == 7) && board.can_castle_kingside()
        }
    } else {
        false
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
                    promoted_piece: None,
                    is_castle: false,
                    prev_castle_rights: board.castle_rights,
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
                    promoted_piece: None,
                    is_castle: false,
                    prev_castle_rights: board.castle_rights,
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
    let promotion_rank = if direction == 0 { 7 } else { 0 };
    let start_square = square as i32;
    let target_square = start_square + 8 * rank_offset;
    if target_square > 63 || target_square < 0 {
        println!("pawn target square out of bounds");
        println!("start square: {}", start_square);
        println!("board:\n{}", board);
        return;
    }

    let add_move = |moves: &mut Vec<Move>,
                    start_square: u32,
                    target_square: u32,
                    captured_piece: Option<u32>,
                    captured_piece_square: Option<usize>,
                    is_en_passant: bool| {
        let is_promotion = target_square / 8 == promotion_rank;
        if is_promotion {
            for &promoted_piece in &[Piece::QUEEN, Piece::ROOK, Piece::BISHOP, Piece::KNIGHT] {
                moves.push(Move {
                    start_square,
                    target_square,
                    promoted_piece: Some(promoted_piece),
                    captured_piece,
                    captured_piece_square,
                    is_en_passant,
                    is_castle: false,
                    prev_castle_rights: board.castle_rights,
                });
            }
        } else {
            moves.push(Move {
                start_square,
                target_square,
                promoted_piece: None,
                captured_piece,
                captured_piece_square,
                is_en_passant,
                is_castle: false,
                prev_castle_rights: board.castle_rights,
            });
        }
    };

    if Piece::is_type(board.squares[target_square as usize], Piece::NONE) {
        // free to move
        add_move(
            moves,
            square as u32,
            target_square as u32,
            None,
            None,
            false,
        );
        if square / 8 == start_rank
            && Piece::is_type(
                board.squares[(target_square + 8 * rank_offset) as usize],
                Piece::NONE,
            )
        {
            add_move(
                moves,
                square as u32,
                (target_square + 8 * rank_offset) as u32,
                None,
                None,
                false,
            );
        }
    }
    let left_target_square = start_square + 8 * rank_offset - 1;
    if left_target_square % 8 != 7
        && (0..=63).contains(&left_target_square)
        && !(Piece::is_type(board.squares[left_target_square as usize], Piece::NONE))
        && !Piece::is_color(
            board.squares[left_target_square as usize],
            board.color_to_move,
        )
    {
        add_move(
            moves,
            square as u32,
            left_target_square as u32,
            Some(board.squares[left_target_square as usize]),
            Some(left_target_square as usize),
            false,
        );
    }
    let right_target_square = start_square + 8 * rank_offset + 1;
    if right_target_square % 8 != 0
        && (0..=63).contains(&right_target_square)
        && !(Piece::is_type(board.squares[right_target_square as usize], Piece::NONE))
        && !Piece::is_color(
            board.squares[right_target_square as usize],
            board.color_to_move,
        )
    {
        add_move(
            moves,
            square as u32,
            right_target_square as u32,
            Some(board.squares[right_target_square as usize]),
            Some(right_target_square as usize),
            false,
        );
    }

    // en passant
    if board.en_passant_square.is_some() {
        let en_passant_square = board.en_passant_square.unwrap();
        // make sure we don't flip to the other side of the board
        if en_passant_square as i32 == left_target_square && en_passant_square % 8 == 7 {
            return;
        }
        // make sure we don't flip to the other side of the board
        if en_passant_square as i32 == right_target_square && en_passant_square % 8 == 0 {
            return;
        }
        if en_passant_square as i32 == left_target_square
            || en_passant_square as i32 == right_target_square
        {
            add_move(
                moves,
                square as u32,
                en_passant_square as u32,
                Some(
                    // set the captured piece to the pawn that was captured
                    board.squares[(en_passant_square as i32 - 8 * rank_offset) as usize],
                ),
                Some((en_passant_square as i32 - 8 * rank_offset) as usize),
                true,
            );
        }
    }
}
pub fn generate_king_moves(square: usize, _piece: u32, board: &mut Board, moves: &mut Vec<Move>) {
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
                promoted_piece: None,
                is_castle: false,
                prev_castle_rights: board.castle_rights,
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
                promoted_piece: None,
                is_castle: false,
                prev_castle_rights: board.castle_rights,
            });
            return;
        }
    });
    // check if king in check
    let opposing_color = if board.color_to_move == Piece::WHITE {
        Piece::BLACK
    } else {
        Piece::WHITE
    };
    let opposing_attack_board = generate_attack_board_for_color(opposing_color, board);
    let mut in_check = false;
    if opposing_attack_board[square] == 1 {
        in_check = true;
    };

    if !in_check && board.can_castle_kingside() {
        let target_square = square as i32 + 2;
        if board.squares[target_square as usize] == Piece::NONE
            && board.squares[(target_square - 1) as usize] == Piece::NONE
        {
            //check if king will move through check
            let mut will_move_through_check = false;
            let mut current_square = square as i32;
            while current_square <= target_square {
                if opposing_attack_board[current_square as usize] == 1 {
                    will_move_through_check = true;
                    break;
                }
                current_square += 1;
            }
            if !will_move_through_check {
                moves.push(Move {
                    start_square: square as u32,
                    target_square: target_square as u32,
                    captured_piece: None,
                    captured_piece_square: None,
                    is_en_passant: false,
                    promoted_piece: None,
                    is_castle: true,
                    prev_castle_rights: board.castle_rights,
                });
            }
        }
    }
    if !in_check && board.can_castle_queenside() {
        let target_square = square as i32 - 2;
        if board.squares[target_square as usize] == Piece::NONE
            && board.squares[(target_square + 1) as usize] == Piece::NONE
            && board.squares[(target_square - 1) as usize] == Piece::NONE
        {
            let mut will_move_through_check = false;
            let mut current_square = square as i32;
            while current_square >= target_square {
                if opposing_attack_board[current_square as usize] == 1 {
                    will_move_through_check = true;
                    break;
                }
                current_square -= 1;
            }
            if !will_move_through_check {
                moves.push(Move {
                    start_square: square as u32,
                    target_square: target_square as u32,
                    captured_piece: None,
                    captured_piece_square: None,
                    is_en_passant: false,
                    promoted_piece: None,
                    is_castle: true,
                    prev_castle_rights: board.castle_rights,
                });
            }
        }
    }
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
                promoted_piece: None,
                is_castle: false,
                prev_castle_rights: board.castle_rights,
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
                promoted_piece: None,
                is_castle: false,
                prev_castle_rights: board.castle_rights,
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
        let mut board = Board::from_fen(STATING_FEN).unwrap();
        let perft_1 = perft(1, &mut board, true);
        assert_eq!(perft_1, 20);
        let perft_2 = perft(2, &mut board, true);
        assert_eq!(perft_2, 400);
        let perft_3 = perft(3, &mut board, true);
        assert_eq!(perft_3, 8902);
        let perft_4 = perft(4, &mut board, true);
        assert_eq!(perft_4, 197_281);
        let perft_5 = perft(5, &mut board, true);
        assert_eq!(perft_5, 4_865_609);
        // let perft_6 = perft(6, &mut board);
        // assert_eq!(perft_6, 119_060_324);
        // let perft_7 = perft(7, &mut board);
        // assert_eq!(perft_7, 3_195_901_860);
        // let perft_8 = perft(8, &mut board);
        // assert_eq!(perft_8, 84_998_978_956);
        // let perft_9 = perft(9, &mut board);
        // assert_eq!(perft_9, 2_439_530_234_167);
        // let perft_10 = perft(10, &mut board);
        // assert_eq!(perft_10, 69_352_859_712_417);
    }
    #[test]
    fn test_perft_2() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
        let mut board = Board::from_fen(fen).unwrap();
        let perft_1 = perft(1, &mut board, true);
        assert_eq!(perft_1, 48);
        let perft_2 = perft(2, &mut board, true);
        assert_eq!(perft_2, 2_039);
        let perft_3 = perft(3, &mut board, true);
        assert_eq!(perft_3, 97_862);
        let perft_4 = perft(4, &mut board, true);
        assert_eq!(perft_4, 4_085_603);
        let perft_5 = perft(5, &mut board, true);
        assert_eq!(perft_5, 193_690_690);
    }
    #[test]
    fn test_perft_5() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let mut board = Board::from_fen(fen).unwrap();
        let perft_1 = perft(1, &mut board, true);
        assert_eq!(perft_1, 44);
        let perft_2 = perft(2, &mut board, true);
        assert_eq!(perft_2, 1_486);
        let perft_3 = perft(3, &mut board, true);
        assert_eq!(perft_3, 62_379);
    }
}
