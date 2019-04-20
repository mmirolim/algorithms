#[macro_use]
extern crate criterion;

use algos::strings::*;
use criterion::*;

fn search_min_window_benchmark(c: &mut Criterion) {
    let bytes: &[u8] = &[0; 10024];
    c.bench(
        "search_min_window",
        Benchmark::new("search", |b| {
            b.iter(|| search_min_window("tist", "this is a test string"))
        })
        .throughput(Throughput::Bytes(bytes.len() as u32)),
    );
}

fn the_8_queen_puzzle_nw_solution_benchmark(c: &mut Criterion) {
    let bytes: &[u8] = &[0; 10024];
    c.bench(
        "8-queen-puzzle-nw-solution",
        Benchmark::new("8-queen", |b| b.iter(|| the_8_queen_puzzle_nw_solution()))
            .throughput(Throughput::Bytes(bytes.len() as u32)),
    );
}

criterion_group!(
    benches,
    search_min_window_benchmark,
    the_8_queen_puzzle_nw_solution_benchmark,
);

criterion_main!(benches);
