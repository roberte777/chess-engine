use chess_engine::board::Board;
use chess_engine::perft::perft;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
