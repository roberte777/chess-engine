pub struct Piece {}
impl Piece {
    pub const NONE: u32 = 0;
    pub const KING: u32 = 1;
    pub const PAWN: u32 = 2;
    pub const KNIGHT: u32 = 3;
    pub const BISHOP: u32 = 4;
    pub const ROOK: u32 = 5;
    pub const QUEEN: u32 = 6;
    pub const WHITE: u32 = 8;
    pub const BLACK: u32 = 16;

    pub fn is_color(piece: u32, color: u32) -> bool {
        (piece & (Self::WHITE | Self::BLACK)) == color
    }
    pub fn is_type(piece: u32, piece_type: u32) -> bool {
        (piece & 0b00111) == piece_type
    }
    pub fn is_sliding_piece(piece: u32) -> bool {
        Piece::is_type(piece, Piece::BISHOP)
            || Piece::is_type(piece, Piece::ROOK)
            || Piece::is_type(piece, Piece::QUEEN)
    }
    pub fn get_type(piece: u32) -> u32 {
        piece & 0b00111
    }
}
