use chess::{
    board::Board,
    chess_move::{generate_legal_moves, Move},
    piece::Piece,
};

pub fn score(board: &mut Board) -> i32 {
    let mut score = 0;
    score += score_piece_value_diff(board);
    // score += score_mobility(board);
    score += score_piece_square(board);
    score
}

fn score_piece_value_diff(board: &Board) -> i32 {
    let mut score = 0;
    for square in 0..64 {
        let piece = board.piece_at(square);
        if piece.is_none() {
            continue;
        }
        let piece = piece.unwrap();
        let piece_value = match Piece::get_type(piece) {
            Piece::PAWN => 100,
            Piece::KNIGHT => 320,
            Piece::BISHOP => 330,
            Piece::ROOK => 500,
            Piece::QUEEN => 900,
            Piece::KING => 20000,
            _ => 0,
        };
        let color_value = match Piece::get_color(piece) {
            Piece::WHITE => 1,
            Piece::BLACK => -1,
            _ => 0,
        };
        score += piece_value * color_value;
    }
    score
}
fn score_mobility(board: &mut Board) -> i32 {
    let mut score = 0;
    for square in 0..64 {
        let piece = board.piece_at(square);
        if piece.is_none() {
            continue;
        }
        let piece = piece.unwrap();
        let color = Piece::get_color(piece);
        if color == Piece::WHITE {
            score += generate_legal_moves(board).len() as i32;
        } else {
            score -= generate_legal_moves(board).len() as i32;
        }
    }
    score
}

fn score_piece_square(board: &Board) -> i32 {
    let mut score = 0;
    for square in 0..64 {
        let piece = board.piece_at(square);
        if piece.is_none() {
            continue;
        }
        let piece = piece.unwrap();
        let piece_value = match Piece::get_type(piece) {
            Piece::PAWN => PAWN_PIECE_TABLE[square],
            Piece::KNIGHT => KNIGHT_PIECE_TABLE[square],
            Piece::BISHOP => BISHOP_PIECE_TABLE[square],
            Piece::ROOK => ROOK_PIECE_TABLE[square],
            Piece::QUEEN => QUEEN_PIECE_TABLE[square],
            Piece::KING => KING_PIECE_TABLE[square],
            _ => 0,
        };
        let color_value = match Piece::get_color(piece) {
            Piece::WHITE => 1,
            Piece::BLACK => -1,
            _ => 0,
        };
        score += piece_value * color_value;
    }
    score
}

pub fn minimax(board: &mut Board, depth: u32) -> (i32, Option<Move>) {
    let maximizing = board.color_to_move() == Piece::WHITE;
    if depth == 0 {
        return (score(board), None); // No move to return when depth is 0
    }
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    for mv in generate_legal_moves(board) {
        // if Piece::get_color(mv.start_square) != board.color_to_move() {
        //     println!("Invalid move: {:?}", mv);
        //     continue;
        // }
        board.make(&mv);
        let (score, _) = minimax(board, depth - 1);
        board.undo(&mv);
        if (maximizing && score > best_score) || (!maximizing && score < best_score) {
            best_score = score;
            best_move = Some(mv);
        }
    }
    (best_score, best_move)
}

pub fn minimax_ab(
    board: &mut Board,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
) -> (i32, Option<Move>) {
    let maximizing = board.color_to_move() == Piece::WHITE;
    if depth == 0 {
        return (score(board), None); // No move to return when depth is 0
    }
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    let mut moves = generate_legal_moves(board);
    if moves.is_empty() {
        if board.is_check() {
            return (if maximizing { i32::MIN } else { i32::MAX }, None);
        }
        return (0, None);
    }
    if board.is_draw() {
        return (0, None);
    }
    order_moves(&mut moves);

    if best_move.is_none() {
        best_move = Some(moves[0]);
    }

    for mv in moves {
        board.make(&mv);
        let (score, _) = minimax_ab(board, depth - 1, alpha, beta);
        board.undo(&mv);
        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
            alpha = std::cmp::max(alpha, best_score);
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
            beta = std::cmp::min(beta, best_score);
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

fn order_moves(moves: &mut Vec<Move>) {
    moves.sort_by(|a, b| {
        // Order captures first
        let a_captures = a.captured_piece.is_some();
        let b_captures = b.captured_piece.is_some();
        b_captures.cmp(&a_captures)
    });
}
