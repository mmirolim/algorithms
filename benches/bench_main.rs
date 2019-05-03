#[macro_use]
extern crate criterion;
extern crate algos;
extern crate walkdir;

mod benchmarks;

criterion_main! {
    benchmarks::string_benchmark::benches,
    benchmarks::tsp_benchmark::benches,
}
