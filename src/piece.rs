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
        piece & color != 0
    }
    pub fn is_sliding_piece(piece: u32) -> bool {
        piece == Piece::BISHOP || piece == Piece::ROOK || piece == Piece::QUEEN
    }
}
