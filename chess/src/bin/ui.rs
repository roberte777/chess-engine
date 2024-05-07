fn main() {}
// use std::io;
//
// use chess::{board::Board, piece::Piece};
//
// fn main() {
//     // let _pos5 = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
//     // let test = "8/2p5/3p4/KP5r/1R3p1k/4P3/6P1/8 b - - 1 8";
//     // let pos2 = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
//     let pos = "rn1qkb1r/pp2pppp/2p5/8/2pP4/4P3/PPP2PPP/RNBQK2R w KQkq - 0 9";
//     let mut board = Board::from_fen(pos).unwrap();
//     loop {
//         let turn = if board.color_to_move() == Piece::WHITE {
//             "White"
//         } else {
//             "Black"
//         };
//         println!("Turn: {}", turn);
//         println!("{}", board);
//         // take input for piece you want
//         // take input for square you want to move to
//         // check if move is valid
//         // if move is valid, make move
//         println!("What piece do you want to move?");
//         let start_piece = ask_for_piece();
//         let start_piece = match start_piece {
//             Command::Piece(piece) => piece,
//             Command::Undo => {
//                 let success = board.human_undo();
//                 if !success {
//                     println!("Nothing to undo");
//                 }
//                 continue;
//             }
//             Command::Unknown => {
//                 println!("Invalid command");
//                 continue;
//             }
//         };
//         println!("Where move to?");
//         let end_piece = ask_for_piece();
//         let end_piece = match end_piece {
//             Command::Piece(piece) => piece,
//             Command::Undo => {
//                 let success = board.human_undo();
//                 if !success {
//                     println!("Nothing to undo");
//                 }
//                 continue;
//             }
//             Command::Unknown => {
//                 println!("Invalid command");
//                 continue;
//             }
//         };
//         let result = board.human_move(start_piece, end_piece);
//         if !result {
//             println!("Invalid move");
//         }
//     }
// }
//
// enum Command {
//     Piece(usize),
//     Undo,
//     Unknown,
// }
// fn ask_for_piece() -> Command {
//     let mut piece = String::new();
//     io::stdin()
//         .read_line(&mut piece)
//         .expect("Failed to read line");
//     let piece = piece.trim();
//     // check if piece is a chess location or "u" for undo
//     if piece == "u" {
//         return Command::Undo;
//     }
//     let piece = convert_standard_chess_notation_to_index(piece);
//     if piece.is_err() {
//         return Command::Unknown;
//     }
//     Command::Piece(piece.unwrap())
// }
//
// fn convert_standard_chess_notation_to_index(square: &str) -> Result<usize, io::Error> {
//     let mut square = square.chars();
//     let file = square
//         .next()
//         .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file"))?;
//     let rank = square
//         .next()
//         .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid rank"))?;
//     let file = match file {
//         'a' => 0,
//         'b' => 1,
//         'c' => 2,
//         'd' => 3,
//         'e' => 4,
//         'f' => 5,
//         'g' => 6,
//         'h' => 7,
//         _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file")),
//     };
//     let rank = match rank {
//         '1' => 0,
//         '2' => 1,
//         '3' => 2,
//         '4' => 3,
//         '5' => 4,
//         '6' => 5,
//         '7' => 6,
//         '8' => 7,
//         _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid rank")),
//     };
//     Ok(rank * 8 + file)
// }
