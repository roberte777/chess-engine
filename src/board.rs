use crate::{chess_move::Move, piece::Piece};
const STATING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
pub const DIRECTION_OFFSETS: [i32; 8] = [8, -8, -1, 1, 7, -7, 9, -9];

lazy_static! {
    pub static ref NUM_SQUARES_TO_EDGE: [[usize; 8]; 64] = precomputed_move_data();
}

#[derive(Debug)]
pub struct Board {
    pub squares: [u32; 64],
    pub color_to_move: u32,
}
impl Board {
    pub fn new() -> Board {
        Board {
            squares: [
                Piece::ROOK | Piece::WHITE,
                Piece::KNIGHT | Piece::WHITE,
                Piece::BISHOP | Piece::WHITE,
                Piece::QUEEN | Piece::WHITE,
                Piece::KING | Piece::WHITE,
                Piece::BISHOP | Piece::WHITE,
                Piece::KNIGHT | Piece::WHITE,
                Piece::ROOK | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::PAWN | Piece::WHITE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::NONE,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::PAWN | Piece::BLACK,
                Piece::ROOK | Piece::BLACK,
                Piece::KNIGHT | Piece::BLACK,
                Piece::BISHOP | Piece::BLACK,
                Piece::QUEEN | Piece::BLACK,
                Piece::KING | Piece::BLACK,
                Piece::BISHOP | Piece::BLACK,
                Piece::KNIGHT | Piece::BLACK,
                Piece::ROOK | Piece::BLACK,
            ],
            color_to_move: Piece::WHITE,
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();
        let mut rank: usize = 7;
        let mut file: usize = 0;
        let piece_type_from_symbol = |c| match c {
            'r' => Piece::ROOK,
            'n' => Piece::KNIGHT,
            'b' => Piece::BISHOP,
            'q' => Piece::QUEEN,
            'k' => Piece::KING,
            'p' => Piece::PAWN,
            'R' => Piece::ROOK,
            'N' => Piece::KNIGHT,
            'B' => Piece::BISHOP,
            'Q' => Piece::QUEEN,
            'K' => Piece::KING,
            'P' => Piece::PAWN,
            _ => Piece::NONE,
        };
        for c in fen.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let piece_type = piece_type_from_symbol(c);
                let piece_color = if c.is_uppercase() {
                    Piece::WHITE
                } else {
                    Piece::BLACK
                };
                if piece_type != Piece::NONE {
                    board.squares[rank * 8 + file] = piece_type | piece_color;
                    file += 1;
                }
            }
        }
        board
    }
    pub fn make(&mut self, move_to_make: Move) -> bool {
        if move_to_make.start_square == move_to_make.target_square {
            return false;
        }
        if self.squares[move_to_make.start_square as usize] == Piece::NONE {
            return false;
        }
        if !Piece::is_color(
            self.squares[move_to_make.start_square as usize],
            self.color_to_move,
        ) {
            return false;
        }
        let start_square = move_to_make.start_square as usize;
        let target_square = move_to_make.target_square as usize;
        let piece = self.squares[start_square];
        self.squares[start_square] = Piece::NONE;
        self.squares[target_square] = piece;

        // update color to move
        self.color_to_move ^= Piece::BLACK;
        true
    }
}

fn precomputed_move_data() -> [[usize; 8]; 64] {
    let mut num_squares_to_edge: [[usize; 8]; 64] = [[0; 8]; 64];
    for file in 0..8 {
        for rank in 0..8 {
            let num_north = 7 - rank;
            let num_south = rank;
            let num_east = 7 - file;
            let num_west = file;
            let square_index = rank * 8 + file;

            num_squares_to_edge[square_index] = [
                num_north,
                num_south,
                num_west,
                num_east,
                std::cmp::min(num_north, num_west),
                std::cmp::min(num_south, num_east),
                std::cmp::min(num_north, num_east),
                std::cmp::min(num_south, num_west),
            ];
        }
    }
    num_squares_to_edge
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board_from_fen() {
        let board = Board::from_fen(STATING_FEN);
        let expected_board = Board::new();
        assert_eq!(board.squares, expected_board.squares);
    }
}
