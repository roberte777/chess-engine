use chess::{
    board::Board,
    chess_move::ChessMove,
    move_generator::MoveGenerator,
    piece::{Color, Piece, PieceType},
};

pub fn score(board: &mut Board) -> i32 {
    let mut score = 0;
    score += score_piece_value_diff(board);
    score += score_piece_square(board);
    score
}

fn score_piece_value_diff(board: &Board) -> i32 {
    let piece_values = [
        (PieceType::Pawn, 100),
        (PieceType::Knight, 320),
        (PieceType::Bishop, 330),
        (PieceType::Rook, 500),
        (PieceType::Queen, 900),
        (PieceType::King, 20000),
    ];

    let mut score = 0;
    for (piece_type, value) in &piece_values {
        let white_pieces = board.bitboards[Color::White as usize][*piece_type as usize].popcnt();
        let black_pieces = board.bitboards[Color::Black as usize][*piece_type as usize].popcnt();
        score += value * (white_pieces as i32 - black_pieces as i32);
    }
    score
}

fn score_piece_square(board: &Board) -> i32 {
    let mut score = 0;

    let piece_types = [
        (PieceType::Pawn, &PAWN_PIECE_TABLE),
        (PieceType::Knight, &KNIGHT_PIECE_TABLE),
        (PieceType::Bishop, &BISHOP_PIECE_TABLE),
        (PieceType::Rook, &ROOK_PIECE_TABLE),
        (PieceType::Queen, &QUEEN_PIECE_TABLE),
        (PieceType::King, &KING_PIECE_TABLE),
    ];

    for (piece_type, table) in &piece_types {
        for color in [Color::White, Color::Black].iter() {
            let color_index = *color as usize;
            let bitboard = board.bitboards[color_index][*piece_type as usize];
            let color_multiplier = if *color == Color::White { 1 } else { -1 };

            // Iterate over all pieces of this type and color
            let mut pieces = bitboard;
            while pieces.0 != 0 {
                let square = pieces.to_square(); // Get the square number of the least significant bit
                score += table[square as usize] * color_multiplier;
                pieces.0 ^= 1 << square; // Clear the bit for the current piece
            }
        }
    }
    score
}

pub fn minimax(board: &mut Board, depth: u32) -> (i32, Option<ChessMove>) {
    let maximizing = board.side_to_move == Color::White;
    if depth == 0 {
        return (score(board), None); // No move to return when depth is 0
    }
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    for mv in MoveGenerator::generate_legal_moves(board) {
        board.make_move(mv);
        let (score, _) = minimax(board, depth - 1);
        board.unmake();
        if (maximizing && score > best_score) || (!maximizing && score < best_score) {
            best_score = score;
            best_move = Some(mv);
        }
    }
    (best_score, best_move)
}

const MATE_SCORE: i32 = 100_000;

pub fn minimax_ab(
    board: &mut Board,
    depth: u32,
    ply: u32,
    mut alpha: i32,
    mut beta: i32,
) -> (i32, Option<ChessMove>) {
    let maximizing = board.side_to_move == Color::White;
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    if depth == 0 {
        return (score(board), None); // No move to return when depth is 0
    }

    let mut moves = MoveGenerator::generate_legal_moves(board);
    if moves.is_empty() {
        if board.is_king_in_check(board.side_to_move) {
            return (
                if maximizing {
                    -MATE_SCORE + ply as i32
                } else {
                    MATE_SCORE - ply as i32
                },
                None,
            );
        }
        return (0, None);
    }
    if board.is_draw() {
        return (0, None);
    }

    order_moves(&mut moves);

    for mv in moves {
        board.make_move(mv);
        let (score, _) = minimax_ab(board, depth - 1, ply + 1, alpha, beta);
        board.unmake();
        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
            alpha = std::cmp::max(alpha, score);
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
            beta = std::cmp::min(beta, score);
        }
        if alpha >= beta {
            break;
        }
    }
    (best_score, best_move)
}
const PAWN_PIECE_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

const KNIGHT_PIECE_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_PIECE_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_PIECE_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

const QUEEN_PIECE_TABLE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const KING_PIECE_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

fn order_moves(moves: &mut Vec<ChessMove>) {
    moves.sort_by(|a, b| {
        // Order captures first
        let a_captures = a.captured_piece.is_some();
        let b_captures = b.captured_piece.is_some();
        b_captures.cmp(&a_captures)
    });
}
