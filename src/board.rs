use std::fmt::Display;

use crate::{
    chess_move::{
        generate_king_moves, generate_knight_moves, generate_pawn_moves,
        generate_sliding_piece_moves, Move,
    },
    piece::Piece,
};
pub const STATING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
// first 8 are offsets for north, south, west, east, north-west, south-east, north-east, south-west
// second 8 are offsets for knight moves
pub const DIRECTION_OFFSETS: [i32; 16] =
    [8, -8, -1, 1, 7, -7, 9, -9, 6, -6, 15, -15, 17, -17, 10, -10];

lazy_static! {
    pub static ref NUM_SQUARES_TO_EDGE: [[usize; 8]; 64] = precomputed_move_data();
}

#[derive(Debug)]
pub struct Board {
    pub squares: [u32; 64],
    pub color_to_move: u32,
    pub en_passant_square: Option<usize>,
    pub en_pasasnt_stack: Vec<Option<usize>>,
    pub all_moves: Vec<Move>,
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
            en_passant_square: None,
            en_pasasnt_stack: vec![None],
            all_moves: Vec::new(),
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
    pub fn make(&mut self, move_to_make: &Move) -> bool {
        if move_to_make.start_square == move_to_make.target_square {
            println!("Illegal move: start square and target square are the same");
            return false;
        }
        if Piece::is_type(
            self.squares[move_to_make.start_square as usize],
            Piece::NONE,
        ) {
            println!("Illegal move: no piece on start square");
            println!("{:?}", self.to_string());
            return false;
        }
        if !Piece::is_color(
            self.squares[move_to_make.start_square as usize],
            self.color_to_move,
        ) {
            println!("Illegal move: piece on start square is not the color to move");
            return false;
        }
        let start_square = move_to_make.start_square as usize;
        let target_square = move_to_make.target_square as usize;
        let piece = self.squares[start_square];
        // check if pawn is moving two squares
        if Piece::is_type(piece, Piece::PAWN)
            && (target_square as i32 - start_square as i32).abs() == 16
        {
            self.en_passant_square = Some((start_square + target_square) / 2);
        } else {
            self.en_passant_square = None;
        }
        self.en_pasasnt_stack.push(self.en_passant_square);
        // remove captured piece
        // this is imoprtant for moves like en passant, where the moving piece
        // will not overwrite the captured piece
        if let Some(captured_piece) = move_to_make.captured_piece_square {
            self.squares[captured_piece] = Piece::NONE;
        }
        self.squares[start_square] = Piece::NONE;
        self.squares[target_square] = piece;

        // update color to move
        self.swap_turn();
        self.all_moves.push(*move_to_make);
        true
    }
    pub fn undo(&mut self, move_to_undo: &Move) {
        let start_square = move_to_undo.start_square as usize;
        let target_square = move_to_undo.target_square as usize;
        let moved_piece = self.squares[target_square];
        if move_to_undo.is_en_passant {
            // Move the capturing pawn back to its start square
            self.squares[start_square] = moved_piece;
            self.squares[target_square] = Piece::NONE;

            // Restore the captured pawn to its original square
            let captured_pawn_square = move_to_undo.captured_piece_square.unwrap();
            self.squares[captured_pawn_square] = move_to_undo.captured_piece.unwrap();
        } else {
            // For regular moves and captures
            self.squares[start_square] = moved_piece;
            self.squares[target_square] = match move_to_undo.captured_piece {
                Some(piece) => piece,
                None => Piece::NONE,
            };
        }
        self.en_pasasnt_stack.pop();
        self.en_passant_square = *self.en_pasasnt_stack.last().unwrap();
        // update color to move
        self.swap_turn();
    }
    pub fn swap_turn(&mut self) {
        if self.color_to_move == Piece::WHITE {
            self.color_to_move = Piece::BLACK;
        } else {
            self.color_to_move = Piece::WHITE;
        }
    }

    pub fn human_move(&mut self, start: usize, end: usize) -> bool {
        let start_piece = self.squares[start];
        let piece_type = Piece::get_type(start_piece);
        let mut moves: Vec<Move> = Vec::new();
        match piece_type {
            Piece::PAWN => {
                generate_pawn_moves(start, start_piece, self, &mut moves);
            }
            Piece::KING => {
                generate_king_moves(start, start_piece, self, &mut moves);
            }
            Piece::ROOK => {
                generate_sliding_piece_moves(start, start_piece, self, &mut moves);
            }
            Piece::QUEEN => {
                generate_sliding_piece_moves(start, start_piece, self, &mut moves);
            }
            Piece::BISHOP => {
                generate_sliding_piece_moves(start, start_piece, self, &mut moves);
            }
            Piece::KNIGHT => {
                generate_knight_moves(start, start_piece, self, &mut moves);
            }
            _ => {}
        }
        let move_to_make = moves.iter().find(|chess_move| {
            if chess_move.start_square as usize == start && chess_move.target_square as usize == end
            {
                return true;
            }
            false
        });
        if move_to_make.is_none() {
            return false;
        }
        self.make(move_to_make.unwrap())
    }
    pub fn human_undo(&mut self) {
        let last_move = self.all_moves.pop();
        if last_move.is_none() {
            return;
        }
        self.undo(&last_move.unwrap());
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = self.squares[rank * 8 + file];
                let piece_char = match Piece::get_type(square) {
                    Piece::PAWN => 'p',
                    Piece::KNIGHT => 'n',
                    Piece::BISHOP => 'b',
                    Piece::ROOK => 'r',
                    Piece::QUEEN => 'q',
                    Piece::KING => 'k',
                    _ => ' ',
                };
                if Piece::is_color(square, Piece::WHITE) {
                    board_string.push('w');
                } else {
                    board_string.push('b');
                }
                board_string.push(piece_char);
                board_string.push(' ');
            }
            board_string.push('\n');
        }
        write!(f, "{}", board_string)
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
