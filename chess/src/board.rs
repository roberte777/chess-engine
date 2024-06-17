use crate::{
    bitboard::BitBoard,
    chess_move::{ChessMove, FLAG_CASTLE, FLAG_EN_PASSANT, FLAG_PROMOTION},
    move_generator::MoveGenerator,
    piece::{Color, PieceType},
};

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone)]
pub struct Board {
    pub bitboards: [[BitBoard; 6]; 2], // Indexed by [Color][PieceType]
    pub occupied: [BitBoard; 2],       // Occupied squares for each color
    pub en_passant: Option<u8>,        // En passant target square, if any
    pub castling_rights: [bool; 4], // Castling rights: [White Kingside, White Queenside, Black Kingside, Black Queenside]
    pub side_to_move: Color,        // Current player to move
    pub half_move_clock: u32,       // Half-move clock for the fifty-move rule
    pub full_move_number: u32,      // Full-move counter, incremented after Black's move
    moves: Vec<ChessMove>,
    pub combined: BitBoard,
    positions: Vec<BitBoard>,
}

impl Board {
    pub fn new() -> Self {
        let bitboards = [[BitBoard::default(); 6]; 2];
        let occupied = [BitBoard::default(), BitBoard::default()];
        let en_passant = None;
        let castling_rights = [false; 4];
        let side_to_move = Color::White;
        let half_move_clock = 0;
        let full_move_number = 1;
        let moves = Vec::new();
        let combined = BitBoard::default();
        let positions = Vec::new();

        Board {
            bitboards,
            occupied,
            en_passant,
            castling_rights,
            side_to_move,
            half_move_clock,
            full_move_number,
            moves,
            combined,
            positions,
        }
    }
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Invalid FEN: Wrong number of parts".to_owned());
        }

        let mut board = Board::new(); // Assuming `new` initializes an empty board

        // Parse pieces
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err("Invalid FEN: Incorrect number of ranks".to_owned());
        }

        for (i, rank) in ranks.iter().enumerate() {
            let mut file = 0;
            for ch in rank.chars() {
                if ch.is_ascii_digit() {
                    file += ch.to_digit(10).unwrap() as usize;
                } else {
                    let color = if ch.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let piece_type = match ch.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => return Err("Invalid FEN: Unknown piece type".to_owned()),
                    };
                    let index = (7 - i) * 8 + file;
                    board.set_piece(index, piece_type, color);
                    file += 1;
                }
            }
        }

        // Parse active color
        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid FEN: Invalid active color".to_owned()),
        };

        // Parse castling availability
        board.castling_rights = [false; 4];
        for ch in parts[2].chars() {
            match ch {
                'K' => board.castling_rights[0] = true,
                'Q' => board.castling_rights[1] = true,
                'k' => board.castling_rights[2] = true,
                'q' => board.castling_rights[3] = true,
                '-' => {}
                _ => return Err("Invalid FEN: Invalid castling flags".to_owned()),
            }
        }

        // En passant target
        board.en_passant = parts[3]
            .chars()
            .nth(0)
            .and_then(|_sq| square_to_index(parts[3]));

        // Half-move and full-move counters
        board.half_move_clock = parts[4]
            .parse::<u32>()
            .map_err(|_| "Invalid FEN: Invalid half-move count".to_owned())?;
        board.full_move_number = parts[5]
            .parse::<u32>()
            .map_err(|_| "Invalid FEN: Invalid full-move number".to_owned())?;
        board.update_attack_and_defense();

        Ok(board)
    }
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Piece placement
        for rank in (0..8).rev() {
            let mut empty_squares = 0;
            for file in 0..8 {
                let index = rank * 8 + file;
                let piece = self.find_piece_on_square(index);
                if piece != '.' {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    fen.push(piece);
                } else {
                    empty_squares += 1;
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // Active color
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling availability
        fen.push(' ');
        if self.castling_rights.iter().all(|&v| !v) {
            fen.push('-');
        } else {
            if self.castling_rights[0] {
                fen.push('K');
            }
            if self.castling_rights[1] {
                fen.push('Q');
            }
            if self.castling_rights[2] {
                fen.push('k');
            }
            if self.castling_rights[3] {
                fen.push('q');
            }
        }

        // En passant target square
        fen.push(' ');
        if let Some(square) = self.en_passant {
            let file = (square % 8) + b'a';
            let rank = (square / 8) + b'1';
            fen.push(file as char);
            fen.push(rank as char);
        } else {
            fen.push('-');
        }

        // Half-move clock
        fen.push(' ');
        fen.push_str(&self.half_move_clock.to_string());

        // Full-move number
        fen.push(' ');
        fen.push_str(&self.full_move_number.to_string());

        fen
    }

    fn set_piece(&mut self, index: usize, piece_type: PieceType, color: Color) {
        let mut bitboard_index = self.bitboards[color as usize][piece_type as usize];
        bitboard_index |= 1 << index;
        self.bitboards[color as usize][piece_type as usize] = bitboard_index;
    }
    /// Prints the board in a human-readable format.
    pub fn print_board(&self) {
        println!("  +------------------------+");
        for rank in (0..8).rev() {
            print!("{} |", rank + 1); // Print rank numbers on the left side
            for file in 0..8 {
                let index = rank * 8 + file;
                let square = self.find_piece_on_square(index);
                print!(" {} ", square);
            }
            println!("|");
        }
        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h  "); // Print file letters below the board
    }

    /// Helper method to find which piece is on a particular square by checking all bitboards.
    fn find_piece_on_square(&self, index: usize) -> char {
        let masks = [1u64 << index]; // Bit mask for the square

        for (color_idx, color) in [Color::White, Color::Black].iter().enumerate() {
            for (piece_idx, piece_type) in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ]
            .iter()
            .enumerate()
            {
                if self.bitboards[color_idx][piece_idx] & masks[0] != BitBoard(0) {
                    return match (piece_type, color) {
                        (PieceType::Pawn, Color::White) => 'P',
                        (PieceType::Pawn, Color::Black) => 'p',
                        (PieceType::Knight, Color::White) => 'N',
                        (PieceType::Knight, Color::Black) => 'n',
                        (PieceType::Bishop, Color::White) => 'B',
                        (PieceType::Bishop, Color::Black) => 'b',
                        (PieceType::Rook, Color::White) => 'R',
                        (PieceType::Rook, Color::Black) => 'r',
                        (PieceType::Queen, Color::White) => 'Q',
                        (PieceType::Queen, Color::Black) => 'q',
                        (PieceType::King, Color::White) => 'K',
                        (PieceType::King, Color::Black) => 'k',
                    };
                }
            }
        }
        '.'
    }
    pub fn make_move(&mut self, mut m: ChessMove) {
        // Store the castling rights before the move
        m.old_castling_rights = self.castling_rights;
        m.old_en_passant_square = self.en_passant;
        m.old_halfmove_clock = self.half_move_clock;
        self.en_passant = None;

        // Special move handling before the actual move (for castling)
        if m.flags & FLAG_CASTLE != 0 {
            self.handle_castling(m);
        }

        let piece = self.piece_at(m.from, self.side_to_move).unwrap();

        // Update pieces on the board: moving the piece
        self.move_piece(m.from, m.to, piece);

        // If there is a capture, remove the captured piece
        if let Some(captured) = self.piece_at(m.to, self.side_to_move.opposite()) {
            self.remove_piece(m.to, captured);
        }

        // update castling rights if the captured piece is a rook
        if let Some(captured) = m.captured_piece {
            if captured == PieceType::Rook {
                if self.side_to_move == Color::White {
                    if m.to == 56 {
                        self.castling_rights[3] = false;
                    } else if m.to == 63 {
                        self.castling_rights[2] = false;
                    }
                } else if m.to == 0 {
                    self.castling_rights[1] = false;
                }
                // Black queen side rook
                else if m.to == 7 {
                    self.castling_rights[0] = false;
                } // Black king side rook
            }
        }

        // Handle en passant
        if m.flags & FLAG_EN_PASSANT != 0 {
            self.handle_en_passant(m);
        }

        // Handle promotion
        if m.flags & FLAG_PROMOTION != 0 {
            self.promote_pawn(m.to, m.promoted_piece.unwrap()); // Assuming m.piece is the promoted piece type
        }

        // Update castling rights if the moved piece is a king or rook
        if piece == PieceType::King {
            if self.side_to_move == Color::White {
                self.castling_rights[0] = false; // White king side
                self.castling_rights[1] = false; // White queen side
            } else {
                self.castling_rights[2] = false; // Black king side
                self.castling_rights[3] = false; // Black queen side
            }
        } else if piece == PieceType::Rook {
            if self.side_to_move == Color::White {
                if m.from == 0 {
                    self.castling_rights[1] = false;
                }
                // White queen side rook
                else if m.from == 7 {
                    self.castling_rights[0] = false;
                } // White king side rook
            } else if m.from == 56 {
                self.castling_rights[3] = false;
            }
            // Black queen side rook
            else if m.from == 63 {
                self.castling_rights[2] = false;
            }
        }

        // Update en passant target
        self.update_en_passant_target(m, piece);
        // Increment the full move number
        if self.side_to_move == Color::Black {
            self.full_move_number += 1;
        }
        // Increment the half-move clock if the move is not a pawn move or a capture
        if piece != PieceType::Pawn && m.captured_piece.is_none() {
            self.half_move_clock += 1;
        } else {
            self.half_move_clock = 0;
        }

        // Add the move to the move list
        self.moves.push(m);
        self.update_attack_and_defense();
        self.positions.push(self.combined);
        // Update the side to move
        self.side_to_move = self.side_to_move.opposite();
    }

    fn move_piece(&mut self, from: u8, to: u8, piece: PieceType) {
        let from_mask = 1 << from;
        let to_mask = 1 << to;
        self.bitboards[self.side_to_move as usize][piece as usize] &= !from_mask;
        self.bitboards[self.side_to_move as usize][piece as usize] |= to_mask;
    }

    fn remove_piece(&mut self, position: u8, piece: PieceType) {
        let mask = 1 << position;
        self.bitboards[self.side_to_move.opposite() as usize][piece as usize] &= !mask;
    }

    fn handle_castling(&mut self, m: ChessMove) {
        if self.side_to_move == Color::White {
            if m.to == 6 {
                // e1 to g1 (White Kingside)
                self.move_piece(7, 5, PieceType::Rook); // Move the rook from h1 to f1
            } else if m.to == 2 {
                // e1 to c1 (White Queenside)
                self.move_piece(0, 3, PieceType::Rook); // Move the rook from a1 to d1
            }
        } else if m.to == 62 {
            // e8 to g8 (Black Kingside)
            self.move_piece(63, 61, PieceType::Rook); // Move the rook from h8 to f8
        } else if m.to == 58 {
            // e8 to c8 (Black Queenside)
            self.move_piece(56, 59, PieceType::Rook); // Move the rook from a8 to d8
        }
    }

    fn promote_pawn(&mut self, square: u8, new_piece: PieceType) {
        let mask = 1 << square;
        self.bitboards[self.side_to_move as usize][PieceType::Pawn as usize] &= !mask;
        self.bitboards[self.side_to_move as usize][new_piece as usize] |= mask;
    }
    fn handle_en_passant(&mut self, m: ChessMove) {
        // Assuming the pawn moves to 'm.to' and captures the pawn at 'm.from + 8' or 'm.from - 8'
        let captured_position = if self.side_to_move == Color::White {
            m.to - 8
        } else {
            m.to + 8
        };
        self.remove_piece(captured_position, PieceType::Pawn);
    }
    fn update_en_passant_target(&mut self, m: ChessMove, piece: PieceType) {
        // Reset en passant target at the start of each move
        self.en_passant = None;

        // Set the en passant target if a pawn moves two squares forward
        if piece == PieceType::Pawn && ((m.to as i8 - m.from as i8).abs() == 16) {
            self.en_passant = Some((m.from + m.to) / 2); // Midpoint between from and to
        }
    }

    pub fn piece_at(&self, square: u8, color: Color) -> Option<PieceType> {
        for piece_type in 0..6 {
            if self.bitboards[color as usize][piece_type] & (1 << square) != BitBoard(0) {
                return Some(PieceType::from(piece_type));
            }
        }
        None
    }

    pub fn unmake(&mut self) {
        self.side_to_move = self.side_to_move.opposite();
        let last_move = self.moves.pop().unwrap();
        let piece = self.piece_at(last_move.to, self.side_to_move).unwrap();
        self.move_piece(last_move.to, last_move.from, piece);
        // Handle special moves
        if last_move.flags & FLAG_CASTLE != 0 {
            self.unhandle_castling(last_move);
        }
        if last_move.flags & FLAG_PROMOTION != 0 {
            // Demote the piece back to a pawn and place it at the 'from' location
            self.demote_pawn(last_move.from, last_move.promoted_piece.unwrap());
        }
        if last_move.flags & FLAG_EN_PASSANT != 0 {
            self.unhandle_en_passant(last_move);
        } else if let Some(captured) = last_move.captured_piece {
            self.set_piece(last_move.to.into(), captured, self.side_to_move.opposite());
        }

        self.en_passant = last_move.old_en_passant_square;
        self.castling_rights = last_move.old_castling_rights;
        self.half_move_clock = last_move.old_halfmove_clock;
        if self.side_to_move == Color::Black {
            self.full_move_number -= 1;
        }
        self.update_attack_and_defense();
        self.positions.pop();
    }
    fn unhandle_castling(&mut self, m: ChessMove) {
        if self.side_to_move == Color::White {
            if m.to == 6 {
                // Move the rook back from f1 to h1
                self.move_piece(5, 7, PieceType::Rook);
            } else if m.to == 2 {
                // Move the rook back from d1 to a1
                self.move_piece(3, 0, PieceType::Rook);
            }
        } else if m.to == 62 {
            // Move the rook back from f8 to h8
            self.move_piece(61, 63, PieceType::Rook);
        } else if m.to == 58 {
            // Move the rook back from d8 to a8
            self.move_piece(59, 56, PieceType::Rook);
        }
    }

    fn unhandle_en_passant(&mut self, m: ChessMove) {
        // Re-add the captured pawn at its original position
        let captured_position = if self.side_to_move == Color::Black {
            m.to + 8
        } else {
            m.to - 8
        };
        self.set_piece(
            captured_position.into(),
            PieceType::Pawn,
            self.side_to_move.opposite(),
        );
    }
    fn demote_pawn(&mut self, square: u8, piece: PieceType) {
        // Replace the promoted piece back to a pawn
        let mask = 1 << square;
        self.bitboards[self.side_to_move as usize][piece as usize] &= !mask;
        self.bitboards[self.side_to_move as usize][PieceType::Pawn as usize] |= mask;
    }
    pub fn update_attack_and_defense(&mut self) {
        // Reset occupied bitboards
        self.occupied[Color::White as usize] = BitBoard::default();
        self.occupied[Color::Black as usize] = BitBoard::default();

        // Aggregate occupied squares for each color and compute the combined occupied bitboard
        for color in [Color::White, Color::Black].iter() {
            for piece in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ]
            .iter()
            {
                let bitboard = self.bitboards[*color as usize][*piece as usize];
                self.occupied[*color as usize] |= bitboard;
            }
        }

        // Calculate all occupied squares as the union of both color's occupied squares
        self.combined = self.occupied[Color::White as usize] | self.occupied[Color::Black as usize];

        // Reset and update pin and checker information
        // self.update_pinned_and_checkers();
    }
    /// Checks if a particular square is attacked by any piece of the specified color.
    pub fn is_square_attacked(&self, square: u8, attacker_color: Color) -> bool {
        let opponent_pieces = self.bitboards[attacker_color as usize];

        // check attacks from pawns
        if MoveGenerator::pawn_attacks(square, attacker_color.opposite())
            & opponent_pieces[PieceType::Pawn as usize]
            != BitBoard(0)
        {
            return true;
        }

        // Check attacks from knights
        if MoveGenerator::knight_attacks(square) & opponent_pieces[PieceType::Knight as usize] != 0
        {
            return true;
        }

        // Check attacks from kings
        if MoveGenerator::king_attacks(square) & opponent_pieces[PieceType::King as usize] != 0 {
            return true;
        }

        // Check attacks from rooks and queens (horizontal and vertical attacks)
        if (MoveGenerator::rook_attacks(square, self.combined)
            & (opponent_pieces[PieceType::Rook as usize]
                | opponent_pieces[PieceType::Queen as usize]))
            != 0
        {
            return true;
        }

        // Check attacks from bishops and queens (diagonal attacks)
        if (MoveGenerator::bishop_attacks(square, self.combined)
            & (opponent_pieces[PieceType::Bishop as usize]
                | opponent_pieces[PieceType::Queen as usize]))
            != 0
        {
            return true;
        }

        false
    }
    /// Checks if the current player's king is in check.
    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_position = self.bitboards[color as usize][PieceType::King as usize].to_square();
        self.is_square_attacked(king_position, color.opposite())
    }

    pub fn piece(&self, square: u8) -> Option<(Color, PieceType)> {
        for color in [Color::White, Color::Black].iter() {
            for piece in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ]
            .iter()
            {
                if self.bitboards[*color as usize][*piece as usize] & (1 << square) != 0 {
                    return Some((*color, *piece));
                }
            }
        }
        None
    }

    pub fn is_draw(&self) -> bool {
        self.is_insufficient_material() || self.is_50_move_rule() || self.is_threefold_repetition()
    }
    // Check for insufficient material on the board
    fn is_insufficient_material(&self) -> bool {
        let mut knight_count = [0, 0];
        let mut light_squared_bishops = [0, 0];
        let mut dark_squared_bishops = [0, 0];

        for color in [Color::White, Color::Black].iter() {
            for piece in [PieceType::Pawn, PieceType::Rook, PieceType::Queen].iter() {
                if self.bitboards[*color as usize][*piece as usize] != 0 {
                    return false; // Having a pawn, rook, or queen means sufficient material
                }
            }

            knight_count[*color as usize] =
                self.bitboards[*color as usize][PieceType::Knight as usize].popcnt();
            let bishops = self.bitboards[*color as usize][PieceType::Bishop as usize];

            // Process light and dark square bishops
            let light_squares = 0x55AA55AA55AA55AAu64; // Represents light colored squares on a chess board
            let dark_squares = !light_squares; // Represents dark colored squares

            // Use bitwise AND to filter bishops on light and dark squares
            light_squared_bishops[*color as usize] = (bishops & light_squares).popcnt();
            dark_squared_bishops[*color as usize] = (bishops & dark_squares).popcnt();
        }

        for i in 0..2 {
            if knight_count[i] > 1 {
                return false;
            }
            if knight_count[i] == 1 && (light_squared_bishops[i] > 0 || dark_squared_bishops[i] > 0)
            {
                return false;
            }
            if light_squared_bishops[i] > 1 || dark_squared_bishops[i] > 1 {
                return false;
            }
        }

        true
    }

    // Check for the fifty-move rule
    fn is_50_move_rule(&self) -> bool {
        self.half_move_clock >= 100
    }
    fn is_threefold_repetition(&self) -> bool {
        let mut count = 0;
        for position in self.positions.iter() {
            if *position == self.combined {
                count += 1;
            }
        }
        count >= 3
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

fn square_to_index(square: &str) -> Option<u8> {
    let bytes = square.as_bytes();
    if bytes.len() == 2 {
        let file = bytes[0] - b'a'; // Convert 'a'-'h' to 0-7
        let rank = bytes[1] - b'1'; // Convert '1'-'8' to 0-7

        if file < 8 && rank < 8 {
            Some(rank * 8 + file)
        } else {
            None
        }
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use crate::chess_move::FLAG_EN_PASSANT;

    use super::*;

    #[test]
    fn test_initial_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.castling_rights, [true, true, true, true]);
        assert_eq!(board.en_passant, None);
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_number, 1);
        board.print_board();
        // Additional checks for piece placement
    }

    #[test]
    fn test_position_with_en_passant() {
        let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPP1PPPP/RNBQKBNR b KQkq e6 0 1";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.en_passant, Some(square_to_index("e6").unwrap()));
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_number, 1);
        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(board.castling_rights, [true, true, true, true]);
        board.print_board();
        // Checks for the specific pawn structure
    }

    #[test]
    fn test_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1";
        let board = Board::from_fen(fen).unwrap();
        assert_eq!(board.castling_rights, [true, true, false, false]);
        board.print_board();
    }

    #[test]
    fn test_specific_position() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let board = Board::from_fen(fen).unwrap();
        // Assert specific pieces
        // Assert king positions for castling checks
        assert_eq!(board.castling_rights, [true, true, true, true]);
        assert_eq!(board.en_passant, None);
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_number, 1);
        board.print_board();
    }

    #[test]
    fn test_invalid_fen() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq e3 0"; // Missing move number
        assert!(Board::from_fen(fen).is_err());

        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq 0 1"; // Invalid en passant square
        assert!(Board::from_fen(fen).is_err());
    }
    #[test]
    fn test_standard_move() {
        let mut board = Board::from_fen("8/8/8/8/8/8/1P6/8 w - - 0 1").unwrap();
        let move_pawn = ChessMove {
            from: 9,
            to: 17,
            promoted_piece: None,
            captured_piece: None,
            flags: 0,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(move_pawn);
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 17
        );
    }
    #[test]
    fn test_capture_move() {
        let mut board = Board::from_fen("8/8/8/8/1p6/8/1P6/8 w - - 0 1").unwrap();
        let move_pawn_capture = ChessMove {
            from: 9,
            to: 25,
            promoted_piece: None,
            captured_piece: Some(PieceType::Pawn),
            flags: 0,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(move_pawn_capture);
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 25
        );
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::Pawn as usize],
            0
        );
    }
    #[test]
    fn test_castling_kingside_white() {
        let mut board = Board::from_fen("8/8/8/8/8/8/8/13K2R w KQ - 0 1").unwrap();
        let castle_kingside_white = ChessMove {
            from: 4,
            to: 6,
            promoted_piece: None,
            captured_piece: None,
            flags: FLAG_CASTLE,
            old_castling_rights: [true, true, false, false],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(castle_kingside_white);
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::King as usize],
            1 << 6
        );
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Rook as usize],
            1 << 5
        );
        assert!(!board.castling_rights[0]);
        assert!(!board.castling_rights[1]);
    }
    #[test]
    fn test_castling_queenside_white() {
        let mut board = Board::from_fen("8/8/8/8/8/8/8/R3K3 w kq - 0 1").unwrap();
        let castle_queenside_white = ChessMove {
            from: 4,
            to: 2,
            promoted_piece: None,
            captured_piece: None,
            flags: FLAG_CASTLE,
            old_castling_rights: [true, true, false, false],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(castle_queenside_white);
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::King as usize],
            1 << 2
        );
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Rook as usize],
            1 << 3
        );
        assert!(!board.castling_rights[0]);
        assert!(!board.castling_rights[1]);
    }
    #[test]
    fn test_castling_queenside_black() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/8 b kq - 0 1").unwrap();
        let castle_queenside_black = ChessMove {
            from: 60,
            to: 58,
            promoted_piece: None,
            captured_piece: None,
            flags: FLAG_CASTLE,
            old_castling_rights: [false, false, true, true],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(castle_queenside_black);
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::King as usize],
            1 << 58
        );
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::Rook as usize],
            (1 << 59) | (1 << 63)
        );
        assert!(!board.castling_rights[2]);
        assert!(!board.castling_rights[3]);
        assert!(!board.castling_rights[0]);
        assert!(!board.castling_rights[1]);
    }

    #[test]
    fn test_castling_kingside_black() {
        let mut board = Board::from_fen("r3k2r/8/8/8/8/8/8/8 b kq - 0 1").unwrap();
        let castle_kingside_black = ChessMove {
            from: 60,
            to: 62,
            promoted_piece: None,
            captured_piece: None,
            flags: FLAG_CASTLE,
            old_castling_rights: [false, false, true, true],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.print_board();
        board.make_move(castle_kingside_black);
        board.print_board();
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::King as usize],
            1 << 62
        );
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::Rook as usize],
            (1 << 61) | (1 << 56)
        );
        assert!(!board.castling_rights[2]);
        assert!(!board.castling_rights[3]);
    }
    #[test]
    fn test_removal_castling_rights() {
        let mut board = Board::from_fen("8/8/8/8/8/8/8/13K2R w KQ - 0 1").unwrap();
        let castle_kingside_white = ChessMove {
            from: 7,
            to: 15,
            promoted_piece: None,
            captured_piece: None,
            flags: 0,
            old_castling_rights: [true, true, false, false],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(castle_kingside_white);
        assert!(board.castling_rights[1]);
        assert!(!board.castling_rights[0]);
    }

    #[test]
    fn test_promotion_white() {
        let mut board = Board::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let promote_queen_white = ChessMove {
            from: 48,
            to: 56,
            promoted_piece: Some(PieceType::Queen),
            captured_piece: None,
            flags: FLAG_PROMOTION,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(promote_queen_white);
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Queen as usize],
            1 << 56
        );
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            0
        );
    }
    #[test]
    fn test_en_passant_black() {
        let mut board = Board::from_fen("8/8/8/3Pp3/8/8/8/8 w - d6 0 1").unwrap();
        board.en_passant = Some(27); // d6
        let en_passant_black = ChessMove {
            from: 35,
            to: 44,
            promoted_piece: None,
            captured_piece: Some(PieceType::Pawn),
            flags: FLAG_EN_PASSANT,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.print_board();
        board.make_move(en_passant_black);
        board.print_board();
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::Pawn as usize],
            0
        );
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 44
        );
    }
    #[test]
    fn test_unmake_standard_move() {
        let mut board = Board::from_fen("8/8/8/8/8/8/P7/8 w - - 0 1").unwrap();
        let move_pawn = ChessMove {
            from: 8,
            to: 16,
            promoted_piece: None,
            captured_piece: None,
            flags: 0,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(move_pawn);
        assert!(board.moves.len() == 1);
        board.unmake();
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 8
        );
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_number, 1);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.castling_rights, [false; 4]);
        assert_eq!(board.en_passant, None);
        assert!(board.moves.is_empty());
    }
    #[test]
    fn test_unmake_capture_move() {
        let mut board = Board::from_fen("8/8/8/8/8/1p6/P7/8 w - - 10 5").unwrap();
        let move_pawn_capture = ChessMove {
            from: 8,
            to: 17,
            promoted_piece: None,
            captured_piece: Some(PieceType::Pawn),
            flags: 0,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.make_move(move_pawn_capture);
        board.unmake();
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 8
        );
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::Pawn as usize],
            1 << 17
        );
        assert_eq!(board.half_move_clock, 10);
        assert_eq!(board.full_move_number, 5);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.castling_rights, [false; 4]);
        assert_eq!(board.en_passant, None);
        assert!(board.moves.is_empty());
    }
    #[test]
    fn test_unmake_castling_move() {
        let mut board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        let castle_kingside_white = ChessMove {
            from: 4,
            to: 6,
            promoted_piece: None,
            captured_piece: None,
            flags: FLAG_CASTLE,
            old_castling_rights: [true, true, false, false],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.print_board();
        board.make_move(castle_kingside_white);
        board.print_board();
        board.unmake();
        board.print_board();
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::King as usize],
            1 << 4
        );
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Rook as usize],
            (1 << 7) | (1 << 0)
        );
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_number, 1);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.castling_rights, [true, true, false, false]);
        assert_eq!(board.en_passant, None);
        assert!(board.moves.is_empty());
    }
    #[test]
    fn test_unmake_promotion_move() {
        let mut board = Board::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1").unwrap();
        let promote_queen_white = ChessMove {
            from: 48,
            to: 56,
            promoted_piece: Some(PieceType::Queen),
            captured_piece: None,
            flags: FLAG_PROMOTION,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.print_board();
        board.make_move(promote_queen_white);
        board.print_board();
        board.unmake();
        board.print_board();
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 48
        );
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Queen as usize],
            0
        );
        assert_eq!(board.half_move_clock, 0);
        assert_eq!(board.full_move_number, 1);
        assert_eq!(board.side_to_move, Color::White);
        assert_eq!(board.castling_rights, [false; 4]);
        assert_eq!(board.en_passant, None);
        assert!(board.moves.is_empty());
    }
    #[test]
    fn test_unmake_en_passant_move() {
        let mut board = Board::from_fen("8/8/8/3Pp3/8/8/8/8 w - e6 0 1").unwrap();
        board.en_passant = Some(27); // d6
        let en_passant_move = ChessMove {
            from: 35,
            to: 44,
            promoted_piece: None,
            captured_piece: Some(PieceType::Pawn),
            flags: FLAG_EN_PASSANT,
            old_castling_rights: [false; 4],
            old_en_passant_square: None,
            old_halfmove_clock: 0,
        };
        board.print_board();
        board.make_move(en_passant_move);
        board.print_board();
        board.unmake();
        board.print_board();
        assert_eq!(
            board.bitboards[Color::White as usize][PieceType::Pawn as usize],
            1 << 35
        );
        assert_eq!(
            board.bitboards[Color::Black as usize][PieceType::Pawn as usize],
            1 << 36
        );
    }
    #[test]
    fn test_bishop_take_unmake() {
        let fen = "rnbqkbnr/pppppp1p/8/6p1/8/3P3N/PPP1PPPP/RNBQKB1R w KQkq - 1 2";
        let mut board = Board::from_fen(fen).unwrap();
        let board_copy = board.clone();
        let chess_move = ChessMove {
            from: 2,
            to: 38,
            promoted_piece: None,
            captured_piece: Some(PieceType::Pawn),
            flags: 0,
            old_castling_rights: [true, true, true, true],
            old_en_passant_square: Some(46),
            old_halfmove_clock: 0,
        };
        board.make_move(chess_move);
        board.unmake();
        board.print_board();
        assert_eq!(board.combined, board_copy.combined);
    }
    #[test]
    fn insufficient_material_king_vs_king() {
        let mut board = Board::new();
        board.bitboards[Color::White as usize][PieceType::King as usize] = BitBoard::from_square(4);
        board.bitboards[Color::Black as usize][PieceType::King as usize] =
            BitBoard::from_square(60);
        assert!(
            board.is_insufficient_material(),
            "King vs King should be insufficient material for a draw."
        );
    }

    #[test]
    fn insufficient_material_king_and_bishop_vs_king() {
        let mut board = Board::new();
        board.bitboards[Color::White as usize][PieceType::King as usize] = BitBoard::from_square(4);
        board.bitboards[Color::White as usize][PieceType::Bishop as usize] =
            BitBoard::from_square(2); // Light square
        board.bitboards[Color::Black as usize][PieceType::King as usize] =
            BitBoard::from_square(60);
        assert!(
            board.is_insufficient_material(),
            "King and Bishop vs King should be insufficient material for a draw."
        );
    }

    #[test]
    fn insufficient_material_king_and_knight_vs_king() {
        let mut board = Board::new();
        board.bitboards[Color::White as usize][PieceType::King as usize] = BitBoard::from_square(4);
        board.bitboards[Color::White as usize][PieceType::Knight as usize] =
            BitBoard::from_square(57);
        board.bitboards[Color::Black as usize][PieceType::King as usize] =
            BitBoard::from_square(60);
        assert!(
            board.is_insufficient_material(),
            "King and Knight vs King should be insufficient material for a draw."
        );
    }

    #[test]
    fn insufficient_material_king_and_bishop_vs_king_and_bishop_same_color() {
        let mut board = Board::new();
        board.bitboards[Color::White as usize][PieceType::King as usize] = BitBoard::from_square(4);
        board.bitboards[Color::White as usize][PieceType::Bishop as usize] =
            BitBoard::from_square(2); // Light square
        board.bitboards[Color::Black as usize][PieceType::King as usize] =
            BitBoard::from_square(60);
        board.bitboards[Color::Black as usize][PieceType::Bishop as usize] =
            BitBoard::from_square(58); // Light square
        assert!(board.is_insufficient_material(), "King and Bishop vs King and Bishop on the same colored squares should be insufficient material for a draw.");
    }

    #[test]
    fn sufficient_material_king_and_rook_vs_king() {
        let mut board = Board::new();
        board.bitboards[Color::White as usize][PieceType::King as usize] = BitBoard::from_square(4);
        board.bitboards[Color::White as usize][PieceType::Rook as usize] = BitBoard::from_square(7);
        board.bitboards[Color::Black as usize][PieceType::King as usize] =
            BitBoard::from_square(60);
        assert!(
            !board.is_insufficient_material(),
            "King and Rook vs King should not be insufficient material for a draw."
        );
    }
    #[test]
    fn sufficient_material_multiple_knights_and_bishops() {
        let mut board = Board::new();
        // White pieces
        board.bitboards[Color::White as usize][PieceType::King as usize] = BitBoard::from_square(4);
        board.bitboards[Color::White as usize][PieceType::Knight as usize] =
            BitBoard::from_square(17) | BitBoard::from_square(32);
        board.bitboards[Color::White as usize][PieceType::Bishop as usize] =
            BitBoard::from_square(18) | BitBoard::from_square(33);

        // Black king
        board.bitboards[Color::Black as usize][PieceType::King as usize] =
            BitBoard::from_square(60);

        assert!(
            !board.is_insufficient_material(),
            "Multiple knights and bishops vs king should not be insufficient material for a draw."
        );
    }
}
