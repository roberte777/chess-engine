use chess::board::{Board, STARTING_FEN};
use chess::perft::perft;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

pub fn perft_initial(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft_initial");
    group.sample_size(10);

    let expected_results = [
        (1, 20),
        (2, 400),
        (3, 8_902),
        (4, 197_281),
        (5, 4_865_609),
        (6, 119_060_324),
    ];

    for (depth, expected_nodes) in expected_results.iter() {
        group.bench_function(format!("Depth {}", depth), |b| {
            b.iter_batched(
                || Board::from_fen(STARTING_FEN).unwrap(),
                |mut board| {
                    let nodes = perft(*depth, &mut board, false);
                    assert_eq!(nodes, *expected_nodes); // Consider moving this assert out if it impacts performance metrics
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
    // save
}

pub fn perft_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft_2");
    group.sample_size(10);
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    let expected_results = [
        (1, 48),
        (2, 2_039),
        (3, 97_862),
        (4, 4_085_603),
        (5, 193_690_690),
    ];

    for (depth, expected_nodes) in expected_results.iter() {
        group.bench_function(format!("Depth {}", depth), |b| {
            b.iter_batched(
                || Board::from_fen(fen).unwrap(),
                |mut board| {
                    let nodes = perft(*depth, &mut board, false);
                    assert_eq!(nodes, *expected_nodes); // Consider moving this assert out if it impacts performance metrics
                },
                BatchSize::SmallInput,
            );
        });
    }
}

pub fn perft_3(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft_3");
    group.sample_size(10);
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

    let expected_results = [
        (1, 14),
        (2, 191),
        (3, 2_812),
        (4, 43_238),
        (5, 674_624),
        (6, 11_030_083),
    ];

    for (depth, expected_nodes) in expected_results.iter() {
        group.bench_function(format!("Depth {}", depth), |b| {
            b.iter_batched(
                || Board::from_fen(fen).unwrap(),
                |mut board| {
                    let nodes = perft(*depth, &mut board, false);
                    assert_eq!(nodes, *expected_nodes); // Consider moving this assert out if it impacts performance metrics
                },
                BatchSize::SmallInput,
            );
        });
    }
}

pub fn perft_initial_grouped(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft_initial_grouped");
    group.sample_size(10);

    let expected_results = [
        (1, 20),
        (2, 400),
        (3, 8_902),
        (4, 197_281),
        (5, 4_865_609),
        (6, 119_060_324),
    ];

    for &(depth, expected_nodes) in &expected_results {
        group.bench_with_input(BenchmarkId::new("Depth", depth), &depth, |b, &depth| {
            b.iter(|| {
                let mut board = Board::from_fen(STARTING_FEN).unwrap();
                let nodes = perft(depth, &mut board, false);
                assert_eq!(nodes, expected_nodes);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    perft_initial,
    perft_initial_grouped,
    perft_2,
    perft_3
);
criterion_main!(benches);
