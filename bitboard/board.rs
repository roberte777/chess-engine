use crate::piece::{Color, Piece};

#[derive(Debug, Default)]
pub struct Board {
    pub piece_bb: [u64; 14],
    pub empty_bb: u64,
    pub occupied_bb: u64,
}

impl Board {
    pub fn new(piece_bb: [u64; 14], empty_bb: u64, occupied_bb: u64) -> Board {
        Board {
            piece_bb,
            empty_bb,
            occupied_bb,
        }
    }
    /**
     * Creates a new board from the given FEN string.
     * @param fen The FEN string to create the board from.
     * @return A new board created from the given FEN string.
     */
    pub fn from_fen(fen: String) -> Self {
        let mut board = Board::default();
        let mut rank = 7;
        let mut file = 0;
        for c in fen.chars() {
            match c {
                'P' => {
                    board.piece_bb[Piece::WhitePawn as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::White as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'p' => {
                    board.piece_bb[Piece::BlackPawn as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::Black as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'R' => {
                    board.piece_bb[Piece::WhiteRook as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::White as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'r' => {
                    board.piece_bb[Piece::BlackRook as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::Black as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'N' => {
                    board.piece_bb[Piece::WhiteKnight as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::White as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'n' => {
                    board.piece_bb[Piece::BlackKnight as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::Black as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'B' => {
                    board.piece_bb[Piece::WhiteBishop as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::White as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'b' => {
                    board.piece_bb[Piece::BlackBishop as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::Black as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'Q' => {
                    board.piece_bb[Piece::WhiteQueen as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::White as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'q' => {
                    board.piece_bb[Piece::BlackQueen as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::Black as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'K' => {
                    board.piece_bb[Piece::WhiteKing as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::White as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                'k' => {
                    board.piece_bb[Piece::BlackKing as usize] |= 1 << (rank * 8 + file);
                    board.piece_bb[Piece::Black as usize] |= 1 << (rank * 8 + file);
                    board.occupied_bb |= 1 << (rank * 8 + file);
                    file += 1;
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1' => {
                    file += 1;
                }
                '2' => {
                    file += 2;
                }
                '3' => {
                    file += 3;
                }
                '4' => {
                    file += 4;
                }
                '5' => {
                    file += 5;
                }
                '6' => {
                    file += 6;
                }
                '7' => {
                    file += 7;
                }
                '8' => {
                    file += 8;
                }
                _ => {}
            }
        }
        board.empty_bb = !board.occupied_bb;
        board
    }
    pub fn get_piece_set(&self, piece: &Piece) -> &u64 {
        &self.piece_bb[*piece as usize]
    }
    pub fn get_empty_set(&self) -> &u64 {
        &self.empty_bb
    }
    pub fn get_occupied_set(&self) -> &u64 {
        &self.occupied_bb
    }
    pub fn get_white_pawns(&self) -> &u64 {
        &self.piece_bb[Piece::WhitePawn as usize]
    }
    pub fn get_black_pawns(&self) -> &u64 {
        &self.piece_bb[Piece::BlackPawn as usize]
    }
    pub fn get_pawns(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::WhitePawn as usize + color as usize]
    }
    pub fn get_white_rooks(&self) -> &u64 {
        &self.piece_bb[Piece::WhiteRook as usize]
    }
    pub fn get_black_rooks(&self) -> &u64 {
        &self.piece_bb[Piece::BlackRook as usize]
    }
    pub fn get_rooks(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::WhiteRook as usize + color as usize]
    }
    pub fn get_white_knights(&self) -> &u64 {
        &self.piece_bb[Piece::WhiteKnight as usize]
    }
    pub fn get_black_knights(&self) -> &u64 {
        &self.piece_bb[Piece::BlackKnight as usize]
    }
    pub fn get_knights(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::WhiteKnight as usize + color as usize]
    }
    pub fn get_white_bishops(&self) -> &u64 {
        &self.piece_bb[Piece::WhiteBishop as usize]
    }
    pub fn get_black_bishops(&self) -> &u64 {
        &self.piece_bb[Piece::BlackBishop as usize]
    }
    pub fn get_bishops(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::WhiteBishop as usize + color as usize]
    }
    pub fn get_white_queens(&self) -> &u64 {
        &self.piece_bb[Piece::WhiteQueen as usize]
    }
    pub fn get_black_queens(&self) -> &u64 {
        &self.piece_bb[Piece::BlackQueen as usize]
    }
    pub fn get_queens(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::WhiteQueen as usize + color as usize]
    }
    pub fn get_white_kings(&self) -> &u64 {
        &self.piece_bb[Piece::WhiteKing as usize]
    }
    pub fn get_black_kings(&self) -> &u64 {
        &self.piece_bb[Piece::BlackKing as usize]
    }
    pub fn get_kings(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::WhiteKing as usize + color as usize]
    }

    /**
     * Returns the set of white pieces. The returned set will be the location of
     * all white pieces. A one indicates white has a piece in that location, a
     * zero indicates white does not.
     */
    pub fn get_white_pieces(&self) -> &u64 {
        &self.piece_bb[Piece::White as usize]
    }

    /**
     * Returns the set of black pieces. The returned set will be the location of
     * all black pieces. A one indicates black has a piece in that location, a
     * zero indicates black does not.
     */
    pub fn get_black_pieces(&self) -> &u64 {
        &self.piece_bb[Piece::Black as usize]
    }

    /**
     * Returns the set of pieces of the given color. The returned set will be
     * the location of all pieces for the given color. A once indicates the
     * color has a piece in that location, a zero indicates the color does not
     * @param color The color of the pieces to return.
     */
    pub fn get_pieces(&self, color: Color) -> &u64 {
        &self.piece_bb[Piece::White as usize + color as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen() {
        let board = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
        assert_eq!(
            board.piece_bb[Piece::WhitePawn as usize],
            0x000000000000FF00
        );
        assert_eq!(
            board.piece_bb[Piece::WhiteRook as usize],
            0x0000000000000081
        );
        assert_eq!(
            board.piece_bb[Piece::WhiteKnight as usize],
            0x0000000000000042
        );
        assert_eq!(
            board.piece_bb[Piece::WhiteBishop as usize],
            0x0000000000000024
        );
        assert_eq!(
            board.piece_bb[Piece::WhiteQueen as usize],
            0x0000000000000008
        );
        assert_eq!(
            board.piece_bb[Piece::WhiteKing as usize],
            0x0000000000000010
        );
        assert_eq!(board.piece_bb[Piece::White as usize], 0x000000000000FFFF);
        assert_eq!(
            board.piece_bb[Piece::BlackPawn as usize],
            0x00FF000000000000
        );
        assert_eq!(
            board.piece_bb[Piece::BlackRook as usize],
            0x8100000000000000
        );
        assert_eq!(
            board.piece_bb[Piece::BlackKnight as usize],
            0x4200000000000000
        );
        assert_eq!(
            board.piece_bb[Piece::BlackBishop as usize],
            0x2400000000000000
        );
        assert_eq!(
            board.piece_bb[Piece::BlackQueen as usize],
            0x0800000000000000
        );
        assert_eq!(
            board.piece_bb[Piece::BlackKing as usize],
            0x1000000000000000
        );
        assert_eq!(board.piece_bb[Piece::Black as usize], 0xFFFF000000000000);
        assert_eq!(board.empty_bb, 0x0000FFFFFFFF0000);
        assert_eq!(board.occupied_bb, 0xFFFF00000000FFFF);
    }
}
