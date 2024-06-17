use crate::{
    board::Board,
    piece::{Color, PieceType},
};

#[derive(Clone, Copy, Debug)]
pub struct ChessMove {
    pub from: u8,
    pub to: u8,
    pub promoted_piece: Option<PieceType>,
    pub captured_piece: Option<PieceType>,
    pub flags: u8,
    pub old_castling_rights: [bool; 4], // Store the castling rights before the move
    pub old_en_passant_square: Option<u8>, // Store the en passant square before the move
    pub old_halfmove_clock: u32,        // Store the halfmove clock before the move
}

impl ChessMove {
    // Convert board index to algebraic chess notation, e.g., 0 -> "a1"
    fn index_to_algebraic(index: u8) -> String {
        let file = (index % 8) as char; // file, from 'a' to 'h'
        let rank = (index / 8) + 1; // rank, from 1 to 8
        format!("{}{}", (file as u8 + b'a') as char, rank)
    }

    pub fn to_standard_notation(&self) -> String {
        let mut move_string = format!(
            "{}{}",
            Self::index_to_algebraic(self.from),
            Self::index_to_algebraic(self.to)
        );

        // Check for promotion
        if let Some(promoted) = self.promoted_piece {
            move_string.push(match promoted {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => unreachable!(), // Only these types are valid for promotion
            });
        }

        // Check for en passant
        if self.flags & FLAG_EN_PASSANT != 0 {
            move_string += " e.p.";
        }

        move_string
    }
    fn algebraic_to_index(algebraic: &str) -> u8 {
        let bytes = algebraic.as_bytes();
        let file = bytes[0] - b'a'; // 'a' to 'h' -> 0 to 7
        let rank = bytes[1] - b'1'; // '1' to '8' -> 0 to 7
        rank * 8 + file
    }

    pub fn from_standard_notation(s: &str, board: &Board) -> Option<ChessMove> {
        // Castling
        if s == "O-O" || s == "O-O-O" {
            let from = match board.side_to_move {
                Color::White => 4,
                Color::Black => 60,
            };
            let to = match board.side_to_move {
                Color::White => {
                    if s == "O-O" {
                        6
                    } else {
                        2
                    }
                }
                Color::Black => {
                    if s == "O-O" {
                        62
                    } else {
                        58
                    }
                }
            };
            return Some(ChessMove {
                from,
                to,
                promoted_piece: None,
                captured_piece: None,
                flags: FLAG_CASTLE,
                old_castling_rights: board.castling_rights,
                old_en_passant_square: board.en_passant,
                old_halfmove_clock: board.half_move_clock,
            });
        }

        let mut chars = s.chars();

        let from_file = chars.next().unwrap();
        let from_rank = chars.next().unwrap();
        let to_file = chars.next().unwrap();
        let to_rank = chars.next().unwrap();

        let from = Self::algebraic_to_index(&format!("{}{}", from_file, from_rank));
        let to = Self::algebraic_to_index(&format!("{}{}", to_file, to_rank));

        let mut promoted_piece = None;
        let mut flags = 0;

        // if from piece is a king and the distance is 2, then it is a castle move
        if board.piece_at(from, board.side_to_move).unwrap() == PieceType::King
            && (from as i8 - to as i8).abs() == 2
        {
            flags |= FLAG_CASTLE;
        }

        // Check for promotion
        if let Some(promotion) = chars.next() {
            promoted_piece = Some(match promotion {
                'q' => PieceType::Queen,
                'r' => PieceType::Rook,
                'b' => PieceType::Bishop,
                'n' => PieceType::Knight,
                _ => return None,
            });
            flags |= FLAG_PROMOTION;
        }

        let captured_piece = board.piece_at(to, board.side_to_move.opposite());
        if board.piece_at(from, board.side_to_move).unwrap() == PieceType::Pawn
            && board.en_passant.is_some()
            && to == board.en_passant.unwrap()
        {
            flags |= FLAG_EN_PASSANT;
        }

        Some(ChessMove {
            from,
            to,
            promoted_piece,
            captured_piece,
            flags,
            old_castling_rights: board.castling_rights,
            old_en_passant_square: board.en_passant,
            old_halfmove_clock: board.half_move_clock,
        })
    }
}

pub const FLAG_CASTLE: u8 = 0b0001;
pub const FLAG_EN_PASSANT: u8 = 0b0010;
pub const FLAG_PROMOTION: u8 = 0b0100;
