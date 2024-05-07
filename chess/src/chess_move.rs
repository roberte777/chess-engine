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

pub const FLAG_CASTLE: u8 = 0b0001;
pub const FLAG_EN_PASSANT: u8 = 0b0010;
pub const FLAG_PROMOTION: u8 = 0b0100;
