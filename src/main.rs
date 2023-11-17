use chess_engine::board::Board;
fn main() {
    let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());

    // print binary formatted to show 64 bits
    println!("{:#064b}", b.get_white_pieces());
    println!("{:#064b}", b.get_black_pieces());
}
