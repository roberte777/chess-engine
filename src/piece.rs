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
    pub fn get_color(piece: u32) -> u32 {
        piece & (Self::WHITE | Self::BLACK)
    }
    pub fn standard_notation_to_index(square: &str) -> u32 {
        let file = square.chars().next().unwrap() as u32 - 'a' as u32;
        let rank = square.chars().nth(1).unwrap() as u32 - '1' as u32;
        rank * 8 + file
    }
    pub fn index_to_standard_notation(index: u32) -> String {
        let file = index % 8;
        let rank = index / 8;
        let file = (file as u8 + b'a') as char;
        let rank = (rank as u8 + b'1') as char;
        format!("{}{}", file, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_color() {
        assert!(Piece::is_color(Piece::WHITE, Piece::WHITE));
        assert!(Piece::is_color(Piece::BLACK, Piece::BLACK));
        assert!(!Piece::is_color(Piece::WHITE, Piece::BLACK));
        assert!(!Piece::is_color(Piece::BLACK, Piece::WHITE));
    }

    #[test]
    fn test_is_type() {
        assert!(Piece::is_type(Piece::KING, Piece::KING));
        assert!(Piece::is_type(Piece::PAWN, Piece::PAWN));
        assert!(Piece::is_type(Piece::KNIGHT, Piece::KNIGHT));
        assert!(Piece::is_type(Piece::BISHOP, Piece::BISHOP));
        assert!(Piece::is_type(Piece::ROOK, Piece::ROOK));
        assert!(Piece::is_type(Piece::QUEEN, Piece::QUEEN));
        assert!(!Piece::is_type(Piece::KING, Piece::PAWN));
        assert!(!Piece::is_type(Piece::PAWN, Piece::KING));
        assert!(!Piece::is_type(Piece::KNIGHT, Piece::PAWN));
        assert!(!Piece::is_type(Piece::BISHOP, Piece::PAWN));
        assert!(!Piece::is_type(Piece::ROOK, Piece::PAWN));
        assert!(!Piece::is_type(Piece::QUEEN, Piece::PAWN));
    }

    #[test]
    fn test_is_sliding_piece() {
        assert!(!Piece::is_sliding_piece(Piece::KING));
        assert!(!Piece::is_sliding_piece(Piece::PAWN));
        assert!(!Piece::is_sliding_piece(Piece::KNIGHT));
        assert!(Piece::is_sliding_piece(Piece::BISHOP));
        assert!(Piece::is_sliding_piece(Piece::ROOK));
        assert!(Piece::is_sliding_piece(Piece::QUEEN));
    }
    #[test]
    fn test_get_type() {
        assert_eq!(Piece::get_type(Piece::KING), Piece::KING);
        assert_eq!(Piece::get_type(Piece::PAWN), Piece::PAWN);
        assert_eq!(Piece::get_type(Piece::KNIGHT), Piece::KNIGHT);
        assert_eq!(Piece::get_type(Piece::BISHOP), Piece::BISHOP);
        assert_eq!(Piece::get_type(Piece::ROOK), Piece::ROOK);
        assert_eq!(Piece::get_type(Piece::QUEEN), Piece::QUEEN);
    }
    #[test]
    fn test_standard_notation_to_index() {
        assert_eq!(Piece::standard_notation_to_index("a1"), 0);
        assert_eq!(Piece::standard_notation_to_index("a2"), 8);
        assert_eq!(Piece::standard_notation_to_index("a3"), 16);
        assert_eq!(Piece::standard_notation_to_index("a4"), 24);
        assert_eq!(Piece::standard_notation_to_index("a5"), 32);
        assert_eq!(Piece::standard_notation_to_index("a6"), 40);
        assert_eq!(Piece::standard_notation_to_index("a7"), 48);
        assert_eq!(Piece::standard_notation_to_index("a8"), 56);
        assert_eq!(Piece::standard_notation_to_index("b1"), 1);
        assert_eq!(Piece::standard_notation_to_index("b2"), 9);
        assert_eq!(Piece::standard_notation_to_index("b3"), 17);
        assert_eq!(Piece::standard_notation_to_index("b4"), 25);
        assert_eq!(Piece::standard_notation_to_index("b5"), 33);
        assert_eq!(Piece::standard_notation_to_index("b6"), 41);
        assert_eq!(Piece::standard_notation_to_index("b7"), 49);
        assert_eq!(Piece::standard_notation_to_index("b8"), 57);
        assert_eq!(Piece::standard_notation_to_index("c1"), 2);
        assert_eq!(Piece::standard_notation_to_index("c2"), 10);
        assert_eq!(Piece::standard_notation_to_index("c3"), 18);
        assert_eq!(Piece::standard_notation_to_index("c4"), 26);
        assert_eq!(Piece::standard_notation_to_index("c5"), 34);
        assert_eq!(Piece::standard_notation_to_index("c6"), 42);
        assert_eq!(Piece::standard_notation_to_index("c7"), 50);
        assert_eq!(Piece::standard_notation_to_index("c8"), 58);
        assert_eq!(Piece::standard_notation_to_index("d1"), 3);
        assert_eq!(Piece::standard_notation_to_index("d2"), 11);
        assert_eq!(Piece::standard_notation_to_index("d3"), 19);
        assert_eq!(Piece::standard_notation_to_index("d4"), 27);
        assert_eq!(Piece::standard_notation_to_index("d5"), 35);
        assert_eq!(Piece::standard_notation_to_index("d6"), 43);
        assert_eq!(Piece::standard_notation_to_index("d7"), 51);
        assert_eq!(Piece::standard_notation_to_index("d8"), 59);
        assert_eq!(Piece::standard_notation_to_index("e1"), 4);
        assert_eq!(Piece::standard_notation_to_index("e2"), 12);
        assert_eq!(Piece::standard_notation_to_index("e3"), 20);
        assert_eq!(Piece::standard_notation_to_index("e4"), 28);
        assert_eq!(Piece::standard_notation_to_index("e5"), 36);
        assert_eq!(Piece::standard_notation_to_index("e6"), 44);
        assert_eq!(Piece::standard_notation_to_index("e7"), 52);
        assert_eq!(Piece::standard_notation_to_index("e8"), 60);
        assert_eq!(Piece::standard_notation_to_index("f1"), 5);
        assert_eq!(Piece::standard_notation_to_index("f2"), 13);
        assert_eq!(Piece::standard_notation_to_index("f3"), 21);
        assert_eq!(Piece::standard_notation_to_index("f4"), 29);
        assert_eq!(Piece::standard_notation_to_index("f5"), 37);
        assert_eq!(Piece::standard_notation_to_index("f6"), 45);
        assert_eq!(Piece::standard_notation_to_index("f7"), 53);
        assert_eq!(Piece::standard_notation_to_index("f8"), 61);
        assert_eq!(Piece::standard_notation_to_index("g1"), 6);
        assert_eq!(Piece::standard_notation_to_index("g2"), 14);
        assert_eq!(Piece::standard_notation_to_index("g3"), 22);
        assert_eq!(Piece::standard_notation_to_index("g4"), 30);
        assert_eq!(Piece::standard_notation_to_index("g5"), 38);
        assert_eq!(Piece::standard_notation_to_index("g6"), 46);
        assert_eq!(Piece::standard_notation_to_index("g7"), 54);
        assert_eq!(Piece::standard_notation_to_index("g8"), 62);
        assert_eq!(Piece::standard_notation_to_index("h1"), 7);
        assert_eq!(Piece::standard_notation_to_index("h2"), 15);
        assert_eq!(Piece::standard_notation_to_index("h3"), 23);
        assert_eq!(Piece::standard_notation_to_index("h4"), 31);
        assert_eq!(Piece::standard_notation_to_index("h5"), 39);
        assert_eq!(Piece::standard_notation_to_index("h6"), 47);
        assert_eq!(Piece::standard_notation_to_index("h7"), 55);
        assert_eq!(Piece::standard_notation_to_index("h8"), 63);
    }
    #[test]
    fn test_index_to_standard_notation() {
        assert_eq!(Piece::index_to_standard_notation(0), "a1");
        assert_eq!(Piece::index_to_standard_notation(8), "a2");
        assert_eq!(Piece::index_to_standard_notation(16), "a3");
        assert_eq!(Piece::index_to_standard_notation(24), "a4");
        assert_eq!(Piece::index_to_standard_notation(32), "a5");
        assert_eq!(Piece::index_to_standard_notation(40), "a6");
        assert_eq!(Piece::index_to_standard_notation(48), "a7");
        assert_eq!(Piece::index_to_standard_notation(56), "a8");
        assert_eq!(Piece::index_to_standard_notation(1), "b1");
        assert_eq!(Piece::index_to_standard_notation(9), "b2");
        assert_eq!(Piece::index_to_standard_notation(17), "b3");
        assert_eq!(Piece::index_to_standard_notation(25), "b4");
        assert_eq!(Piece::index_to_standard_notation(33), "b5");
        assert_eq!(Piece::index_to_standard_notation(41), "b6");
        assert_eq!(Piece::index_to_standard_notation(49), "b7");
        assert_eq!(Piece::index_to_standard_notation(57), "b8");
        assert_eq!(Piece::index_to_standard_notation(2), "c1");
        assert_eq!(Piece::index_to_standard_notation(10), "c2");
        assert_eq!(Piece::index_to_standard_notation(18), "c3");
        assert_eq!(Piece::index_to_standard_notation(26), "c4");
        assert_eq!(Piece::index_to_standard_notation(34), "c5");
        assert_eq!(Piece::index_to_standard_notation(42), "c6");
        assert_eq!(Piece::index_to_standard_notation(50), "c7");
        assert_eq!(Piece::index_to_standard_notation(58), "c8");
        assert_eq!(Piece::index_to_standard_notation(3), "d1");
        assert_eq!(Piece::index_to_standard_notation(11), "d2");
        assert_eq!(Piece::index_to_standard_notation(19), "d3");
    }
}
