use std::fmt::Display;
use thiserror::Error;

use crate::{
    chess_move::{generate_legal_moves, Move},
    piece::Piece,
};
pub const STATING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
// first 8 are offsets for north, south, west, east, north-west, south-east, north-east, south-west
// second 8 are offsets for knight moves
pub const DIRECTION_OFFSETS: [i32; 16] =
    [8, -8, -1, 1, 7, -7, 9, -9, 6, -6, 15, -15, 17, -17, 10, -10];

lazy_static! {
    pub static ref NUM_SQUARES_TO_EDGE: [[usize; 8]; 64] = precomputed_move_data();
}

#[derive(Error, Debug)]
pub enum FenParseError {
    #[error("Invalid piece")]
    InvalidPiece,
    #[error("Invalid color")]
    InvalidColor,
    #[error("Invalid en passant")]
    InvalidEnPassant,
    #[error("Invalid castle rights")]
    InvalidCastleRights,
    #[error("Invalid half move clock")]
    InvalidHalfMoveClock,
    #[error("Invalid full move number")]
    InvalidFullMoveNumber,
}

#[derive(Debug)]
pub struct Board {
    pub squares: [u32; 64],
    pub color_to_move: u32,
    pub en_passant_square: Option<usize>,
    pub en_pasasnt_stack: Vec<Option<usize>>,
    pub all_moves: Vec<Move>,
    pub castle_rights: CastleRights,
}
impl Board {
    pub fn new() -> Board {
        Board {
            squares: [Piece::NONE; 64],
            color_to_move: Piece::WHITE,
            en_passant_square: None,
            en_pasasnt_stack: vec![None],
            all_moves: Vec::new(),
            castle_rights: CastleRights {
                white_king_side: true,
                white_queen_side: true,
                black_king_side: true,
                black_queen_side: true,
            },
        }
    }

    pub fn from_fen(fen: &str) -> Result<Board, FenParseError> {
        let mut board = Board::new();
        let mut rank: usize = 7;
        let mut file: usize = 0;
        let fen = fen.trim().to_string();
        let fen_parts: Vec<&str> = fen.split(' ').collect();

        // parse piece placement
        let pieces_placement = fen_parts.first();
        if pieces_placement.is_none() {
            return Err(FenParseError::InvalidPiece);
        }

        let piece_placement = pieces_placement.unwrap().to_string();
        for c in piece_placement.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = match c {
                    'p' => Piece::PAWN | Piece::BLACK,
                    'n' => Piece::KNIGHT | Piece::BLACK,
                    'b' => Piece::BISHOP | Piece::BLACK,
                    'r' => Piece::ROOK | Piece::BLACK,
                    'q' => Piece::QUEEN | Piece::BLACK,
                    'k' => Piece::KING | Piece::BLACK,
                    'P' => Piece::PAWN | Piece::WHITE,
                    'N' => Piece::KNIGHT | Piece::WHITE,
                    'B' => Piece::BISHOP | Piece::WHITE,
                    'R' => Piece::ROOK | Piece::WHITE,
                    'Q' => Piece::QUEEN | Piece::WHITE,
                    'K' => Piece::KING | Piece::WHITE,
                    _ => return Err(FenParseError::InvalidPiece),
                };
                board.squares[rank * 8 + file] = piece;
                file += 1;
            }
        }

        // parse color to move
        let color_string = fen_parts.get(1);
        if color_string.is_none() {
            return Err(FenParseError::InvalidColor);
        }
        let color_to_move = match *color_string.unwrap() {
            "w" => Piece::WHITE,
            "b" => Piece::BLACK,
            _ => return Err(FenParseError::InvalidColor),
        };

        // parse castling rights
        let mut castle_rights = CastleRights {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        };

        if fen_parts.get(2).is_none() {
            return Err(FenParseError::InvalidCastleRights);
        }
        if fen_parts[2] != "-" {
            for c in fen_parts[2].chars() {
                match c {
                    'K' => castle_rights.white_king_side = true,
                    'Q' => castle_rights.white_queen_side = true,
                    'k' => castle_rights.black_king_side = true,
                    'q' => castle_rights.black_queen_side = true,
                    _ => return Err(FenParseError::InvalidCastleRights),
                }
            }
        }

        // parse en passant square
        let en_passant_string = fen_parts.get(3);
        if en_passant_string.is_none() {
            return Err(FenParseError::InvalidEnPassant);
        }
        let en_passant_square = match *en_passant_string.unwrap() {
            "-" => None,
            _ => {
                let en_passant_square =
                    Piece::standard_notation_to_index(en_passant_string.unwrap()) as usize;
                if en_passant_square > 63 {
                    return Err(FenParseError::InvalidEnPassant);
                }
                Some(en_passant_square)
            }
        };

        // parse half move clock
        let _half_move_clock = match fen_parts.get(4).unwrap_or(&"0").parse::<u32>() {
            Ok(half_move_clock) => half_move_clock,
            Err(_) => return Err(FenParseError::InvalidHalfMoveClock),
        };

        // parse full move number
        let _full_move_number = match fen_parts.get(5).unwrap_or(&"0").parse::<u32>() {
            Ok(full_move_number) => full_move_number,
            Err(_) => return Err(FenParseError::InvalidFullMoveNumber),
        };

        board.color_to_move = color_to_move;
        board.en_passant_square = en_passant_square;
        board.castle_rights = castle_rights;
        Ok(board)
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
            println!("{}", self);
            for i in 0..self.all_moves.len() {
                let promotion_piece = self.all_moves[i].promoted_piece;
                let promotion_print = match promotion_piece {
                    Some(piece) => match piece {
                        Piece::QUEEN => "q",
                        Piece::ROOK => "r",
                        Piece::BISHOP => "b",
                        Piece::KNIGHT => "n",
                        _ => "",
                    },
                    None => "",
                };
                println!(
                    "{}{}{}",
                    Piece::index_to_standard_notation(self.all_moves[i].start_square),
                    Piece::index_to_standard_notation(self.all_moves[i].target_square),
                    promotion_print
                );
                panic!("Illegal move");
            }
            return false;
        }
        let start_square = move_to_make.start_square as usize;
        let target_square = move_to_make.target_square as usize;
        let piece = self.squares[start_square];

        // check if we need to update castling rights
        if Piece::is_type(piece, Piece::KING) {
            if self.color_to_move == Piece::WHITE {
                self.castle_rights.white_king_side = false;
                self.castle_rights.white_queen_side = false;
            } else {
                self.castle_rights.black_king_side = false;
                self.castle_rights.black_queen_side = false;
            }
        }
        if Piece::is_type(piece, Piece::ROOK) {
            if self.color_to_move == Piece::WHITE {
                if move_to_make.start_square == 0 {
                    self.castle_rights.white_queen_side = false;
                } else if move_to_make.start_square == 7 {
                    self.castle_rights.white_king_side = false;
                }
            }
            //check if black rook is moving
            else if move_to_make.start_square == 56 {
                self.castle_rights.black_queen_side = false;
            } else if move_to_make.start_square == 63 {
                self.castle_rights.black_king_side = false;
            }
        }

        // check if pawn is moving two squares
        if Piece::is_type(piece, Piece::PAWN)
            && (target_square as i32 - start_square as i32).abs() == 16
        {
            self.en_passant_square = Some((start_square + target_square) / 2);
        } else {
            self.en_passant_square = None;
        }
        self.en_pasasnt_stack.push(self.en_passant_square);
        if move_to_make.is_castle {
            // move king
            self.squares[start_square] = Piece::NONE;
            self.squares[target_square] = piece;

            // move rook
            let rook_start_square = match target_square {
                2 => 0,
                6 => 7,
                58 => 56,
                62 => 63,
                _ => panic!("Invalid castle move"),
            };
            let rook_target_square = match target_square {
                2 => 3,
                6 => 5,
                58 => 59,
                62 => 61,
                _ => panic!("Invalid castle move"),
            };
            let rook = match self.color_to_move {
                Piece::WHITE => Piece::ROOK | Piece::WHITE,
                Piece::BLACK => Piece::ROOK | Piece::BLACK,
                _ => panic!("Invalid color"),
            };
            self.squares[rook_start_square] = Piece::NONE;
            self.squares[rook_target_square] = rook;
        } else {
            // check if captured piece is rook.
            // if so, update castle rights
            if move_to_make.captured_piece.is_some() {
                let captured_piece = move_to_make.captured_piece.unwrap();
                if Piece::is_type(captured_piece, Piece::ROOK) {
                    if self.color_to_move == Piece::WHITE {
                        if move_to_make.target_square == 56 {
                            self.castle_rights.black_queen_side = false;
                        } else if move_to_make.target_square == 63 {
                            self.castle_rights.black_king_side = false;
                        }
                    }
                    //check if black rook is moving
                    else if move_to_make.target_square == 0 {
                        self.castle_rights.white_queen_side = false;
                    } else if move_to_make.target_square == 7 {
                        self.castle_rights.white_king_side = false;
                    }
                }
            }

            // remove captured piece
            // this is imoprtant for moves like en passant, where the moving piece
            // will not overwrite the captured piece
            if let Some(captured_piece) = move_to_make.captured_piece_square {
                self.squares[captured_piece] = Piece::NONE;
            }
            self.squares[start_square] = Piece::NONE;
            self.squares[target_square] = piece;
            if move_to_make.promoted_piece.is_some() {
                self.squares[target_square] =
                    move_to_make.promoted_piece.unwrap() | self.color_to_move;
            }
        }
        // update color to move
        self.all_moves.push(*move_to_make);
        self.swap_turn();
        true
    }
    pub fn undo(&mut self, move_to_undo: &Move) {
        // update color to move
        self.swap_turn();
        // self.all_moves.pop();
        self.castle_rights = move_to_undo.prev_castle_rights;
        let start_square = move_to_undo.start_square as usize;
        let target_square = move_to_undo.target_square as usize;
        let moved_piece = self.squares[target_square];

        if Piece::is_type(moved_piece, Piece::ROOK) {
            if self.color_to_move == Piece::WHITE {
                if move_to_undo.start_square == 0 {
                    self.castle_rights.white_queen_side = true;
                } else if move_to_undo.start_square == 7 {
                    self.castle_rights.white_king_side = true;
                }
            }
            //check if black rook is moving
            else if move_to_undo.start_square == 56 {
                self.castle_rights.black_queen_side = true;
            } else if move_to_undo.start_square == 63 {
                self.castle_rights.black_king_side = true;
            }
        }

        if move_to_undo.is_en_passant {
            // Move the capturing pawn back to its start square
            self.squares[start_square] = moved_piece;
            self.squares[target_square] = Piece::NONE;

            // Restore the captured pawn to its original square
            let captured_pawn_square = move_to_undo.captured_piece_square.unwrap();
            self.squares[captured_pawn_square] = move_to_undo.captured_piece.unwrap();
        } else if move_to_undo.is_castle {
            // Move the king back to its start square
            self.squares[start_square] = moved_piece;
            self.squares[target_square] = Piece::NONE;

            // Move the rook back to its start square
            let rook_start_square = match target_square {
                2 => 0,
                6 => 7,
                58 => 56,
                62 => 63,
                _ => panic!("Invalid castle move"),
            };
            let rook_target_square = match target_square {
                2 => 3,
                6 => 5,
                58 => 59,
                62 => 61,
                _ => panic!("Invalid castle move"),
            };
            let rook = match self.color_to_move {
                Piece::WHITE => Piece::ROOK | Piece::WHITE,
                Piece::BLACK => Piece::ROOK | Piece::BLACK,
                _ => panic!("Invalid color"),
            };
            self.squares[rook_start_square] = rook;
            self.squares[rook_target_square] = Piece::NONE;
        } else {
            // For regular moves and captures
            self.squares[start_square] = moved_piece;
            self.squares[target_square] = match move_to_undo.captured_piece {
                Some(piece) => piece,
                None => Piece::NONE,
            };
            if move_to_undo.promoted_piece.is_some() {
                self.squares[start_square] = Piece::PAWN | self.color_to_move;
            }
        }
        self.en_pasasnt_stack.pop();
        self.en_passant_square = *self.en_pasasnt_stack.last().unwrap();
    }

    pub fn swap_turn(&mut self) {
        if self.color_to_move == Piece::WHITE {
            self.color_to_move = Piece::BLACK;
        } else {
            self.color_to_move = Piece::WHITE;
        }
    }

    pub fn can_castle_kingside(&self) -> bool {
        if self.color_to_move == Piece::WHITE {
            return self.castle_rights.white_king_side;
        }
        self.castle_rights.black_king_side
    }

    pub fn can_castle_queenside(&self) -> bool {
        if self.color_to_move == Piece::WHITE {
            return self.castle_rights.white_queen_side;
        }
        self.castle_rights.black_queen_side
    }

    pub fn human_move(&mut self, start: usize, end: usize) -> bool {
        let moves: Vec<Move> = generate_legal_moves(self);
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
        board_string += " +---+---+---+---+---+---+---+---+\n";
        for rank in (0..8).rev() {
            for file in 0..8 {
                if file == 0 {
                    board_string += " | "
                }
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
                    board_string.push(piece_char.to_ascii_uppercase());
                } else {
                    board_string.push(piece_char);
                }
                board_string += " | ";
                if file == 7 {
                    board_string += &format!("{}", rank + 1);
                }
            }
            board_string.push('\n');
            board_string += " +---+---+---+---+---+---+---+---+";
            board_string.push('\n');
            if rank == 0 {
                board_string += "   a   b   c   d   e   f   g   h\n";
            }
        }
        write!(f, "{}", board_string)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CastleRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}
impl CastleRights {
    pub fn new() -> CastleRights {
        CastleRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

impl Default for CastleRights {
    fn default() -> Self {
        Self::new()
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
        let board = Board::from_fen(STATING_FEN).expect("Invalid FEN");
        let squares = [
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
        ];
        assert_eq!(board.squares, squares);
    }
    #[test]
    fn test_fen_castling() {
        let board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
            .expect("Invalid FEN");
        // assert that white can castle kingside and queenside
        assert!(board.castle_rights.white_king_side);
        assert!(board.castle_rights.white_queen_side);
        assert!(!board.castle_rights.black_king_side);
        assert!(!board.castle_rights.black_queen_side);
        let board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w - - 1 8")
            .expect("Invalid FEN");
        assert!(!board.castle_rights.white_king_side);
        assert!(!board.castle_rights.white_queen_side);
        assert!(!board.castle_rights.black_king_side);
        assert!(!board.castle_rights.black_queen_side);
        let board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w k - 1 8")
            .expect("Invalid FEN");
        assert!(!board.castle_rights.white_king_side);
        assert!(!board.castle_rights.white_queen_side);
        assert!(board.castle_rights.black_king_side);
        assert!(!board.castle_rights.black_queen_side);
    }
    #[test]
    fn test_fen_en_passant() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - e3 1 2")
            .expect("Invalid FEN");
        assert_eq!(board.en_passant_square, Some(20));
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - e6 1 2")
            .expect("Invalid FEN");
        assert_eq!(board.en_passant_square, Some(44));
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 1 2")
            .expect("Invalid FEN");
        assert_eq!(board.en_passant_square, None);
    }
    #[test]
    fn test_fen_color_to_move() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 0")
            .expect("Invalid FEN");
        assert_eq!(board.color_to_move, Piece::BLACK);
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0")
            .expect("Invalid FEN");
        assert_eq!(board.color_to_move, Piece::WHITE);
    }
}
