#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}
#[derive(Copy, Clone)]
pub enum Piece {
    White,
    Black,
    WhitePawn,
    BlackPawn,
    WhiteRook,
    BlackRook,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}
// direction offsets
const NORTH: i8 = 8;
const SOUTH: i8 = -8;
const EAST: i8 = 1;
const WEST: i8 = -1;
const NORTH_EAST: i8 = 9;
const NORTH_WEST: i8 = 7;
const SOUTH_EAST: i8 = -7;
const SOUTH_WEST: i8 = -9;
