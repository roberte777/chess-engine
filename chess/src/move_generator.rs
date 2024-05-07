use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::chess_move::{ChessMove, FLAG_CASTLE, FLAG_EN_PASSANT, FLAG_PROMOTION};
use crate::piece::{Color, PieceType};

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn generate_legal_moves(board: &mut Board) -> Vec<ChessMove> {
        let moves = Self::generate_moves(board);
        let mut legal_moves = Vec::new();

        for m in moves.into_iter() {
            board.make_move(m);
            // board.print_board();
            if !board.is_king_in_check(board.side_to_move.opposite()) {
                legal_moves.push(m);
            }
            board.unmake();
        }

        legal_moves
    }
    /// Generates all possible moves for the given board state.
    pub fn generate_moves(board: &Board) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        // Generate moves based on the current side to move
        if board.side_to_move == Color::White {
            Self::generate_moves_for_color(board, Color::White, &mut moves);
        } else {
            Self::generate_moves_for_color(board, Color::Black, &mut moves);
        }

        moves
    }

    /// Generates moves for a specific color.
    fn generate_moves_for_color(board: &Board, color: Color, moves: &mut Vec<ChessMove>) {
        // Iterate over all pieces of the given color and generate moves
        for piece_type in 0..6 {
            let bitboard = board.bitboards[color as usize][piece_type];
            if bitboard.0 != 0 {
                Self::generate_moves_for_piece(board, piece_type, bitboard, color, moves);
            }
        }
    }

    /// Generates moves for a specific piece type.
    fn generate_moves_for_piece(
        board: &Board,
        piece_type: usize,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        // Example: handle move generation for each piece type
        match PieceType::from(piece_type) {
            PieceType::Pawn => Self::generate_pawn_moves(board, bitboard, color, moves),
            PieceType::Knight => Self::generate_knight_moves(board, bitboard, color, moves),
            PieceType::Bishop => Self::generate_bishop_moves(board, bitboard, color, moves),
            PieceType::Rook => Self::generate_rook_moves(board, bitboard, color, moves),
            PieceType::Queen => Self::generate_queen_moves(board, bitboard, color, moves),
            PieceType::King => Self::generate_king_moves(board, bitboard, color, moves),
        }
    }
    fn generate_pawn_moves(
        board: &Board,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let not_occupied = !board.combined.0; // Not occupied squares
        let rank_mask = 0xFF; // Mask for a single rank
        let promotion_rank_mask = match color {
            Color::White => rank_mask << (7 * 8),
            Color::Black => rank_mask,
        };
        let initial_rank_mask = match color {
            Color::White => rank_mask << (1 * 8),
            Color::Black => rank_mask << (6 * 8),
        };

        let forward_one_step = match color {
            Color::White => 8,
            Color::Black => -8,
        };
        let forward_two_steps = forward_one_step * 2;

        // Single step forward moves
        let mut single_moves = (match color {
            Color::White => bitboard.0 << forward_one_step,
            Color::Black => bitboard.0 >> -forward_one_step,
        }) & not_occupied;

        // Double step forward moves, correctly applying initial rank mask to original pawns bitboard
        // Now we first ensure the square immediately in front of the pawn is clear before considering the double move
        let intermediate_single_moves = match color {
            Color::White => (bitboard.0 & initial_rank_mask) << forward_one_step,
            Color::Black => (bitboard.0 & initial_rank_mask) >> -forward_one_step,
        } & not_occupied; // Check if the immediate square is free

        let double_moves = match color {
            Color::White => intermediate_single_moves << forward_one_step,
            Color::Black => intermediate_single_moves >> -forward_one_step,
        } & not_occupied; // Now check if the second square is free too
                          //
                          // Captures
        let left_captures = match color {
            Color::White => (bitboard.0 & !0x0101010101010101) << 7,
            Color::Black => (bitboard.0 & !0x0101010101010101) >> 9,
        } & board.occupied[color.opposite() as usize].0;

        let right_captures = match color {
            Color::White => (bitboard.0 & !0x8080808080808080) << 9,
            Color::Black => (bitboard.0 & !0x8080808080808080) >> 7,
        } & board.occupied[color.opposite() as usize].0;

        // println!("bitboard: {}", bitboard);
        // println!("bitboard: {}", board.occupied[color.opposite() as usize]);
        // println!("left_captures: {}", left_captures);
        // println!("right_captures: {}", right_captures);

        // Generate moves for single and double advances
        Self::generate_pawn_move_list(
            single_moves,
            forward_one_step,
            promotion_rank_mask,
            board,
            moves,
            color,
            false,
            false,
        );
        Self::generate_pawn_move_list(
            double_moves,
            forward_two_steps,
            promotion_rank_mask,
            board,
            moves,
            color,
            false,
            false,
        );

        // Generate capture moves
        Self::generate_pawn_move_list(
            left_captures,
            forward_one_step - 1,
            promotion_rank_mask,
            board,
            moves,
            color,
            true,
            false,
        );
        Self::generate_pawn_move_list(
            right_captures,
            forward_one_step + 1,
            promotion_rank_mask,
            board,
            moves,
            color,
            true,
            false,
        );

        // if let Some(en_passant_square) = board.en_passant {
        //     // Calculate the square behind the en passant square based on the color of the pawn that made the two-square move.
        //     let passed_square = if color == Color::White {
        //         en_passant_square - 8 // The passed square for White is one rank below the landing square
        //     } else {
        //         en_passant_square + 8 // The passed square for Black is one rank above the landing square
        //     };
        //
        //     let passed_target = 1u64 << en_passant_square;
        //     let file = en_passant_square % 8;
        //
        //     // Mask out the edges if the passed square is on a or h file
        //     let valid_attackers = match file {
        //         0 => en_passant_square << 1, // Passed square on a-file, attack can only come from b-file
        //         7 => en_passant_square >> 1, // Passed square on h-file, attack can only come from g-file
        //         _ => (en_passant_square >> 1 | en_passant_square << 1), // Normal cases
        //     };
        //
        //     let potential_en_passant_attackers = valid_attackers & bitboard.0;
        //     board.print_board();
        //     println!("{}", BitBoard::new(potential_en_passant_attackers));
        //
        //     // If potential attackers can capture the passed square
        //     if potential_en_passant_attackers != 0 {
        //         Self::generate_pawn_move_list(
        //             potential_en_passant_attackers,
        //             forward_one_step,
        //             promotion_rank_mask,
        //             board,
        //             moves,
        //             color,
        //             true,
        //         );
        //     }
        // }

        // // Handle en passant captures
        if let Some(en_passant_square) = board.en_passant {
            let file = en_passant_square % 8; // 0 is a-file, 7 is h-file
                                              // Calculate the square behind the en passant square based on the color of the pawn that made the two-square move.
            let passed_square = if color == Color::White {
                en_passant_square - 8 // The passed square for White is one rank below the landing square
            } else {
                en_passant_square + 8 // The passed square for Black is one rank above the landing square
            };
            let en_passant_target = 1u64 << passed_square;

            // Mask out the edges if the en passant square is on a or h file
            let valid_attackers = match file {
                0 => en_passant_target << 1, // En passant square on a-file, attack can only come from b-file
                7 => en_passant_target >> 1, // En passant square on h-file, attack can only come from g-file
                _ => en_passant_target >> 1 | en_passant_target << 1, // Normal cases
            };

            let mut potential_en_passant_attackers = valid_attackers & bitboard.0;

            // If potential attackers can capture the en passant target
            if potential_en_passant_attackers != 0 {
                // iterate over all potential en passant attackers
                // generate moves
                while potential_en_passant_attackers != 0 {
                    let from = potential_en_passant_attackers.trailing_zeros() as u8;
                    let to = en_passant_square;
                    let captured_piece = PieceType::Pawn;
                    let flags = FLAG_EN_PASSANT;
                    let promoted_piece = None;

                    moves.push(ChessMove {
                        from,
                        to,
                        captured_piece: Some(captured_piece),
                        promoted_piece,
                        flags,
                        old_castling_rights: board.castling_rights,
                        old_en_passant_square: board.en_passant,
                        old_halfmove_clock: board.half_move_clock,
                    });

                    potential_en_passant_attackers &= potential_en_passant_attackers - 1;
                }
                // Self::generate_pawn_move_list(
                //     potential_en_passant_attackers,
                //     forward_one_step,
                //     promotion_rank_mask,
                //     board,
                //     moves,
                //     color,
                //     true,
                //     true,
                // );
            }
        }
    }

    /// Do not use this method for en passant
    // TODO: Remove en passant from this method
    fn generate_pawn_move_list(
        moves_bitboard: u64,
        step: i8,
        promotion_rank_mask: u64,
        board: &Board,
        moves: &mut Vec<ChessMove>,
        color: Color,
        is_capture: bool,
        is_en_passant: bool,
    ) {
        let mut moves_bits = moves_bitboard;
        while moves_bits != 0 {
            let to = moves_bits.trailing_zeros() as u8;
            let from = to as i8 - step;
            moves_bits &= moves_bits - 1; // Clear the least significant bit

            if (1u64 << to) & promotion_rank_mask != 0 {
                // Handle promotions
                for &promo_type in &[
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                ] {
                    moves.push(ChessMove {
                        from: from as u8,
                        to,
                        promoted_piece: Some(promo_type),
                        captured_piece: if is_capture {
                            Some(board.piece_at(to, color.opposite()).unwrap())
                        } else {
                            None
                        },
                        flags: FLAG_PROMOTION,
                        old_castling_rights: board.castling_rights,
                        old_en_passant_square: board.en_passant,
                        old_halfmove_clock: board.half_move_clock,
                    });
                }
            } else {
                // Regular move or capture
                moves.push(ChessMove {
                    from: from as u8,
                    to,
                    promoted_piece: None,
                    captured_piece: if is_capture {
                        if is_en_passant {
                            Some(PieceType::Pawn)
                        } else {
                            Some(board.piece_at(to, color.opposite()).unwrap())
                        }
                    } else {
                        None
                    },
                    flags: if is_en_passant { FLAG_EN_PASSANT } else { 0 },
                    old_castling_rights: board.castling_rights,
                    old_en_passant_square: board.en_passant,
                    old_halfmove_clock: board.half_move_clock,
                });
            }
        }
    }
    /// Generates pawn attacks from a given square
    pub fn pawn_attacks(square: u8, color: Color) -> BitBoard {
        let mask = 1u64 << square; // Position the pawn on the square.

        let left_attacks = match color {
            Color::White => (mask & !0x0101010101010101) << 7, // Mask prevents wrapping from h-file to a-file
            Color::Black => (mask & !0x0101010101010101) >> 9, // Same mask for Black pawns
        };

        let right_attacks = match color {
            Color::White => (mask & !0x8080808080808080) << 9, // Mask prevents wrapping from a-file to h-file
            Color::Black => (mask & !0x8080808080808080) >> 7, // Same mask for Black pawns
        };

        // Combine left and right attacks into one BitBoard
        BitBoard(left_attacks | right_attacks)
    }

    /// Generates all knight moves for a given knight bitboard.
    fn generate_knight_moves(
        board: &Board,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let mut knights = bitboard.0;
        let own_pieces = board.occupied[color as usize];
        let opponent_pieces = board.occupied[color.opposite() as usize].0;

        while knights != 0 {
            let from = knights.trailing_zeros() as u8;
            let knight_moves = Self::knight_attacks(from) & !own_pieces.0;

            let possible_moves = knight_moves & !opponent_pieces; // Normal moves
            Self::generate_move_list(board, from, possible_moves, moves, color, None);

            let possible_captures = knight_moves & opponent_pieces; // Capture moves
            Self::generate_move_list(board, from, possible_captures, moves, color, Some(0));

            knights &= knights - 1; // Remove this knight from the set
        }
    }

    /// Helper to generate a list of moves from a set of move possibilities.
    fn generate_move_list(
        board: &Board,
        from: u8,
        move_bitboard: u64,
        moves: &mut Vec<ChessMove>,
        color: Color,
        capture_flag: Option<u8>,
    ) {
        let mut bits = move_bitboard;
        while bits != 0 {
            let to = bits.trailing_zeros() as u8;
            bits &= bits - 1; // Clear the least significant bit

            moves.push(ChessMove {
                from,
                to,
                promoted_piece: None,
                captured_piece: board.piece_at(to, color.opposite()),
                flags: capture_flag.unwrap_or(0),
                old_castling_rights: board.castling_rights,
                old_en_passant_square: board.en_passant,
                old_halfmove_clock: board.half_move_clock,
            });
        }
    }

    /// Calculates all possible knight moves from a given position using bitboards.
    pub fn knight_attacks(square: u8) -> u64 {
        let mut attacks = 0u64;
        let bit = 1u64 << square;

        // Generate all possible knight moves with explicit boundary checks
        // Left 2, Up 1
        if square % 8 > 1 && square / 8 < 7 {
            attacks |= bit << 6;
        }
        // Right 2, Up 1
        if square % 8 < 6 && square / 8 < 7 {
            attacks |= bit << 10;
        }
        // Left 2, Down 1
        if square % 8 > 1 && square / 8 > 0 {
            attacks |= bit >> 10;
        }
        // Right 2, Down 1
        if square % 8 < 6 && square / 8 > 0 {
            attacks |= bit >> 6;
        }
        // Up 2, Left 1
        if square % 8 > 0 && square / 8 < 6 {
            attacks |= bit << 15;
        }
        // Up 2, Right 1
        if square % 8 < 7 && square / 8 < 6 {
            attacks |= bit << 17;
        }
        // Down 2, Left 1
        if square % 8 > 0 && square / 8 > 1 {
            attacks |= bit >> 17;
        }
        // Down 2, Right 1
        if square % 8 < 7 && square / 8 > 1 {
            attacks |= bit >> 15;
        }

        attacks
    }

    /// Generates all bishop moves for a given bishop bitboard.
    fn generate_bishop_moves(
        board: &Board,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let mut bishops = bitboard.0;
        let own_pieces = board.occupied[color as usize].0;
        let opponent_pieces = board.occupied[1 - color as usize].0;
        let all_pieces = board.combined.0;

        while bishops != 0 {
            let from = bishops.trailing_zeros() as u8;
            let bishop_moves = Self::bishop_attacks(from, all_pieces);

            let possible_moves = bishop_moves & !own_pieces & !opponent_pieces; // Normal moves
            Self::generate_move_list(board, from, possible_moves, moves, color, None);

            let possible_captures = bishop_moves & opponent_pieces; // Capture moves
            Self::generate_move_list(board, from, possible_captures, moves, color, Some(0));

            bishops &= bishops - 1; // Remove this bishop from the set
        }
    }

    /// Generates bishop attacks from a given square, considering current blockages.
    pub fn bishop_attacks(square: u8, all_pieces: u64) -> u64 {
        let mut attacks = 0u64;
        let directions = [7, 9, -7, -9]; // Diagonal movements: NW, NE, SE, SW

        for &direction in &directions {
            let mut position = square as i8;

            while {
                position += direction;
                position >= 0 && position < 64 // Stay within board limits
                    && !((direction == 7 && position % 8 == 7) // Wraparounds for each direction
                        || (direction == -7 && position % 8 == 0)
                        || (direction == 9 && position % 8 == 0)
                        || (direction == -9 && position % 8 == 7))
            } {
                let mask = 1u64 << position;

                attacks |= mask; // Add this square to the attacks
                if mask & all_pieces != 0 {
                    break; // Blocked by another piece
                }
            }
        }

        attacks
    }

    /// Generates all rook moves for a given rook bitboard.
    fn generate_rook_moves(
        board: &Board,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let mut rooks = bitboard.0;
        let own_pieces = board.occupied[color as usize].0;
        let opponent_pieces = board.occupied[1 - color as usize].0;
        let all_pieces = board.combined.0;

        while rooks != 0 {
            let from = rooks.trailing_zeros() as u8;
            let rook_moves = Self::rook_attacks(from, all_pieces);

            let possible_moves = rook_moves & !own_pieces & !opponent_pieces; // Normal moves
            Self::generate_move_list(board, from, possible_moves, moves, color, None);

            let possible_captures = rook_moves & opponent_pieces; // Capture moves
            Self::generate_move_list(board, from, possible_captures, moves, color, Some(0));

            rooks &= rooks - 1; // Remove this rook from the set
        }
    }

    /// Generates rook attacks from a given square, considering current blockages.
    pub fn rook_attacks(square: u8, all_pieces: u64) -> u64 {
        let mut attacks = 0u64;
        let directions = [8, -8, 1, -1]; // Vertical and horizontal movements

        for &direction in &directions {
            let mut position = square as i8;

            while {
                position += direction;
                position >= 0 && position < 64 // Stay within board limits
                    && !((direction == 1 && position % 8 == 0) // Avoid wrapping around from right to left
                        || (direction == -1 && position % 8 == 7)) // Avoid wrapping around from left to right
            } {
                let mask = 1u64 << position;

                attacks |= mask; // Add this square to the attacks
                if mask & all_pieces != 0 {
                    break; // Blocked by another piece
                }
            }
        }

        attacks
    }

    /// Generates all queen moves for a given queen bitboard.
    fn generate_queen_moves(
        board: &Board,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let mut queens = bitboard.0;
        let own_pieces = board.occupied[color as usize].0;
        let opponent_pieces = board.occupied[1 - color as usize].0;
        let all_pieces = board.combined.0;

        while queens != 0 {
            let from = queens.trailing_zeros() as u8;
            let queen_moves = Self::queen_attacks(from, all_pieces);

            let possible_moves = queen_moves & !own_pieces & !opponent_pieces; // Normal moves
            Self::generate_move_list(board, from, possible_moves, moves, color, None);

            let possible_captures = queen_moves & opponent_pieces; // Capture moves
            Self::generate_move_list(board, from, possible_captures, moves, color, Some(0));

            queens &= queens - 1; // Remove this queen from the set
        }
    }

    /// Generates queen attacks from a given square, considering current blockages.
    fn queen_attacks(square: u8, all_pieces: u64) -> u64 {
        // Combine rook and bishop attacks since queen can move as both
        Self::rook_attacks(square, all_pieces) | Self::bishop_attacks(square, all_pieces)
    }

    fn generate_king_moves(
        board: &Board,
        bitboard: BitBoard,
        color: Color,
        moves: &mut Vec<ChessMove>,
    ) {
        let mut kings = bitboard.0;
        let own_pieces = board.occupied[color as usize].0;
        let opponent_pieces = board.occupied[1 - color as usize].0;

        while kings != 0 {
            let from = kings.trailing_zeros() as u8;
            // println!("own pieces: {}", own_pieces);
            let king_moves = Self::king_attacks(from) & !own_pieces;
            // println!("king_moves: {}", king_moves);

            let possible_moves = king_moves & !opponent_pieces; // Normal moves
            Self::generate_move_list(board, from, possible_moves, moves, color, None);

            let possible_captures = king_moves & opponent_pieces; // Capture moves
            Self::generate_move_list(board, from, possible_captures, moves, color, Some(0));

            kings &= kings - 1; // Remove this king from the set

            // Generate castling moves if applicable
            if color == Color::White {
                // White kingside castling
                if board.castling_rights[0] && (board.combined.0 & 0x60) == 0 // Check if squares f1, g1 are clear
                && !board.is_square_attacked(4, Color::Black)
                && !board.is_square_attacked(5, Color::Black)
                && !board.is_square_attacked(6, Color::Black)
                {
                    moves.push(ChessMove {
                        from,
                        to: 6,
                        promoted_piece: None,
                        captured_piece: None,
                        flags: FLAG_CASTLE,
                        old_castling_rights: board.castling_rights,
                        old_en_passant_square: board.en_passant,
                        old_halfmove_clock: board.half_move_clock,
                    });
                }
                // White queenside castling
                if board.castling_rights[1] && (board.combined.0 & 0xE) == 0 // Check if squares b1, c1, d1 are clear
                && !board.is_square_attacked(4, Color::Black)
                && !board.is_square_attacked(3, Color::Black)
                && !board.is_square_attacked(2, Color::Black)
                {
                    moves.push(ChessMove {
                        from,
                        to: 2,
                        promoted_piece: None,
                        captured_piece: None,
                        flags: FLAG_CASTLE,
                        old_castling_rights: board.castling_rights,
                        old_en_passant_square: board.en_passant,
                        old_halfmove_clock: board.half_move_clock,
                    });
                }
            } else {
                // Black kingside castling
                if board.castling_rights[2] && (board.combined.0 & 0x6000000000000000) == 0 // Check if squares f8, g8 are clear
                && !board.is_square_attacked(60, Color::White)
                && !board.is_square_attacked(61, Color::White)
                && !board.is_square_attacked(62, Color::White)
                {
                    moves.push(ChessMove {
                        from,
                        to: 62,
                        promoted_piece: None,
                        captured_piece: None,
                        flags: FLAG_CASTLE,
                        old_castling_rights: board.castling_rights,
                        old_en_passant_square: board.en_passant,
                        old_halfmove_clock: board.half_move_clock,
                    });
                }
                // Black queenside castling
                if board.castling_rights[3] && (board.combined.0 & 0xE00000000000000) == 0 // Check if squares b8, c8, d8 are clear
                && !board.is_square_attacked(60, Color::White)
                && !board.is_square_attacked(59, Color::White)
                && !board.is_square_attacked(58, Color::White)
                {
                    moves.push(ChessMove {
                        from,
                        to: 58,
                        promoted_piece: None,
                        captured_piece: None,
                        flags: FLAG_CASTLE,
                        old_castling_rights: board.castling_rights,
                        old_en_passant_square: board.en_passant,
                        old_halfmove_clock: board.half_move_clock,
                    });
                }
            }
        }
    }

    pub fn king_attacks(square: u8) -> u64 {
        let mut attacks = 0u64;
        let mut bit = 1u64 << square;

        // Positions around the king
        let not_a_file = 0xfefefefefefefefe; // ~0x0101010101010101
        let not_h_file = 0x7f7f7f7f7f7f7f7f; // ~0x8080808080808080

        // King can move one square in any direction
        // Horizontal + vertical
        // attacks |= (bit >> 1) & not_h_file; // Move right west one
        // attacks |= (bit << 1) & not_a_file; // Move left east one
        // attacks |= bit << 8; // Move up
        // attacks |= bit >> 8; // Move down
        //
        // // Diagonal
        // attacks |= (bit << 7) & not_h_file; // Move up-right
        // attacks |= (bit << 9) & not_a_file; // Move up-left
        // attacks |= (bit >> 9) & not_h_file; // Move down-right
        // attacks |= (bit >> 7) & not_a_file; // Move down-left

        attacks = ((bit << 1) & not_a_file) | ((bit >> 1) & not_h_file);
        bit |= attacks;
        attacks |= (bit << 8) | (bit >> 8);
        // println!("king_attacks: {}", attacks);

        attacks
    }
    // /// Calculates all possible king moves from a given position using bitboards.
    // pub fn king_attacks(square: u8) -> u64 {
    //     let mut attacks = 0u64;
    //     let bit = 1u64 << square;
    //
    //     // Generate all possible king moves
    //     if bit & 0xfe != 0 {
    //         attacks |= (bit << 1) | (bit >> 1) | (bit << 9) | (bit >> 7);
    //     }
    //     if bit & 0x7f != 0 {
    //         attacks |= (bit << 7) | (bit >> 9) | (bit << 8) | (bit >> 8);
    //     }
    //     if bit & 0xff00 != 0 {
    //         attacks |= (bit << 8) | (bit >> 8);
    //     }
    //     if bit & 0x01ff != 0 {
    //         attacks |= (bit << 9) | (bit >> 9);
    //     }
    //
    //     attacks
    // }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, STARTING_FEN},
        chess_move::ChessMove,
        perft::perft,
    };

    use super::MoveGenerator;
    // use crate::perft::peft;

    fn run_perft_test(fen: &str, expected_nodes: Vec<u64>) {
        let mut board = Board::from_fen(fen).unwrap();
        for (depth, &nodes) in expected_nodes.iter().enumerate() {
            let result = perft(depth as u32 + 1, &mut board, true);
            assert_eq!(result, nodes);
        }
    }

    #[test]
    fn test_perft_initial_position() {
        run_perft_test(STARTING_FEN, vec![20, 400, 8_902, 197_281, 4_865_609])
    }

    #[test]
    fn test_perft_position_2() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        run_perft_test(fen, vec![48, 2_039, 97_862, 4_085_603])
    }

    // #[test]
    // fn test_broken() {
    //     let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    //     let mut board = Board::from_fen(fen).unwrap();
    //     let moves = vec![
    //         ChessMove {
    //             from: 8,
    //             to: 16,
    //             captured_piece: None,
    //             flags: 0,
    //             old_castling_rights: [true; 4],
    //             old_en_passant_square: None,
    //             old_halfmove_clock: 0,
    //             promoted_piece: None,
    //         },
    //         ChessMove {
    //             from: 23,
    //             to: 14,
    //             captured_piece: Some(crate::piece::PieceType::Pawn),
    //             flags: 0,
    //             old_castling_rights: [true; 4],
    //             old_en_passant_square: None,
    //             old_halfmove_clock: 0,
    //             promoted_piece: None,
    //         },
    //         ChessMove {
    //             from: 15,
    //             to: 23,
    //             captured_piece: None,
    //             flags: 0,
    //             old_castling_rights: [true; 4],
    //             old_en_passant_square: None,
    //             old_halfmove_clock: 0,
    //             promoted_piece: None,
    //         },
    //     ];
    //     for mv in moves.clone() {
    //         board.make_move(mv);
    //         board.print_board();
    //     }
    //     for mv in moves {
    //         board.unmake();
    //         board.print_board();
    //     }
    //     assert!(false);
    // }

    #[test]
    fn test_not_working() {
        let fen = "rnbqkbnr/1ppppppp/8/p7/8/N7/PPPPPPPP/R1BQKBNR w KQkq a6 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 20);
    }
    #[test]
    fn test_not_working_2() {
        let fen = "rnbqkbnr/1ppppppp/8/p7/8/4P3/PPPP1PPP/RNBQKBNR w KQkq a6 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 30);
    }

    #[test]
    fn test_not_working_3() {
        let fen = "rnbqkbnr/pppppp1p/8/6p1/8/3P4/PPP1PPPP/RNBQKBNR w KQkq g6 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 26);
    }

    #[test]
    fn test_enemy_king_check() {
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPP1PPP/RNBQKBNR w KQkq d6 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 30);
    }

    #[test]
    fn test_non_duplicating_moves() {
        let fen = "rnbqkbnr/ppppppp1/8/7p/8/4P3/PPPP1PPP/RNBQKBNR w KQkq h6 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 30);
    }
    #[test]
    fn test_not_working_4() {
        let fen = "rnbqkbnr/ppppp1pp/5p2/8/6P1/7P/PPPPPP2/RNBQKBNR b KQkq - 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 19);
    }
    #[test]
    fn test_not_working_5() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/5P2/6P1/PPPPP2P/RNBQKBNR b KQkq f3 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 22);
    }

    #[test]
    fn test_not_working_6() {
        let fen = "rnbqkbnr/1ppppppp/8/p7/1P6/P7/2PPPPPP/RNBQKBNR b KQkq b3 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 22);

        let nodes = perft(1, &mut board, true);
        assert_eq!(nodes, 22);
    }

    #[test]
    fn test_not_working_7() {
        let fen = "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 22);

        let nodes = perft(1, &mut board, true);
        assert_eq!(nodes, 22);
    }
    #[test]
    fn test_not_working_8() {
        let fen = "rnbqkbnr/p1pppppp/8/8/1p6/3P4/PPPKPPPP/RNBQ1BNR w kq - 0 3";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 21);

        let nodes = perft(1, &mut board, true);
        assert_eq!(nodes, 21);
    }
    #[test]
    fn test_black_pawn_king_check() {
        let fen = "rnbqkbnr/ppppppp1/8/8/7p/5P2/PPPPPKPP/RNBQ1BNR w kq - 0 3";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        for m in &moves {
            println!("{}", m.to_standard_notation());
        }
        assert_eq!(moves.len(), 20);

        let nodes = perft(1, &mut board, true);
        assert_eq!(nodes, 20);
    }

    #[test]
    fn test_king_move_after_castle() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/4P3/1pN2Q1p/PPPBBPPP/R4RK1 w kq - 0 2";
        let mut board = Board::from_fen(fen).unwrap();
        let moves = MoveGenerator::generate_legal_moves(&mut board);
        assert_eq!(moves.len(), 49)
    }

    #[test]
    fn test_broken_2() {
        let fen = "r3k2r/p1ppqpb1/1n2pnN1/1b1P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 2";
        let mut board = Board::from_fen(fen).unwrap();
        assert_eq!(perft(2, &mut board, true), 2034);
    }
}
