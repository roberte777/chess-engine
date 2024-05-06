use chess_engine::perft::perft;
use chess_engine::score::minimax_ab;
use chess_engine::{board::Board, score::minimax};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn midgame_perft(c: &mut Criterion) {
    c.bench_function("perft 5 5", |b| {
        b.iter(|| {
            let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
            let board = Board::from_fen(fen);
            if board.is_err() {
                eprintln!("Invalid FEN string: {}", board.err().unwrap());
                return;
            }
            let mut board = board.unwrap();
            let depth = 5;
            perft(depth, &mut board, true);
        })
    });
}

criterion_group!(benches, midgame_perft);
criterion_main!(benches);
