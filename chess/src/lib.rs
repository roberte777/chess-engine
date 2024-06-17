extern crate lazy_static;
mod bitboard;
pub mod board;
pub mod chess_move;
pub mod move_generator;
pub mod perft;
pub mod piece;
pub use board::{Board, STARTING_FEN};
pub use piece::{Color, PieceType};
