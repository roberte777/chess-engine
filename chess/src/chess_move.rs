use crate::piece::PieceType;

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
        format!("{}{}", (file as u8 + 'a' as u8) as char, rank)
    }

    pub fn to_standard_notation(&self) -> String {
        let mut move_string = format!(
            "{}{}",
            Self::index_to_algebraic(self.from),
            Self::index_to_algebraic(self.to)
        );

        // Check for promotion
        if let Some(promoted) = self.promoted_piece {
            move_string.push('=');
            move_string.push(match promoted {
                PieceType::Queen => 'Q',
                PieceType::Rook => 'R',
                PieceType::Bishop => 'B',
                PieceType::Knight => 'N',
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
        let file = bytes[0] as u8 - b'a'; // 'a' to 'h' -> 0 to 7
        let rank = bytes[1] as u8 - b'1'; // '1' to '8' -> 0 to 7
        rank * 8 + file
    }

    pub fn from_standard_notation(s: &str) -> Option<ChessMove> {
        // Castling
        if s == "O-O" || s == "O-O-O" {
            return Some(ChessMove {
                from: if s == "O-O" { 4 } else { 4 }, // E.g., e1 for white
                to: if s == "O-O" { 6 } else { 2 },   // g1 or c1 for white
                promoted_piece: None,
                captured_piece: None,
                flags: FLAG_CASTLE,
                old_castling_rights: [false; 4],
                old_en_passant_square: None,
                old_halfmove_clock: 0,
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

        // Check for promotion
        if let Some('=') = chars.next() {
            let piece_type = chars.next();
            promoted_piece = Some(match piece_type {
                Some('Q') => PieceType::Queen,
                Some('R') => PieceType::Rook,
                Some('B') => PieceType::Bishop,
                Some('N') => PieceType::Knight,
                _ => return None,
            });
            flags |= FLAG_PROMOTION;
        }

        Some(ChessMove {
            from,
            to,
            promoted_piece,
            captured_piece: None, // This needs to be set based on the board state.
            flags,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        })
    }
}

pub const FLAG_CASTLE: u8 = 0b0001;
pub const FLAG_EN_PASSANT: u8 = 0b0010;
pub const FLAG_PROMOTION: u8 = 0b0100;
