use chess_engine::perft::perft;
use chess_engine::score::minimax_ab;
use chess_engine::{board::Board, score::minimax};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_minimaxes(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimaxes prune comparison");
    group.sample_size(10);
    group.bench_function("no prune", |b| {
        b.iter(|| {
            let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
            let mut board = Board::from_fen(fen);
            if board.is_err() {
                eprintln!("Invalid FEN string: {}", board.err().unwrap());
                return;
            }
            let mut board = board.unwrap();
            let depth = 5;
            minimax(&mut board, depth);
        })
    });

    group.bench_function("prune", |b| {
        b.iter(|| {
            let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
            let mut board = Board::from_fen(fen);
            if board.is_err() {
                eprintln!("Invalid FEN string: {}", board.err().unwrap());
                return;
            }
            let mut board = board.unwrap();
            let depth = 5;
            minimax_ab(&mut board, depth, i32::MIN, i32::MAX);
        })
    });
}

criterion_group!(benches, bench_minimaxes);
criterion_main!(benches);
