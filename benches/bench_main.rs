#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;

extern crate algos;
extern crate walkdir;

mod benchmarks;

criterion_main! {
    benchmarks::string_benchmark::benches,
    benchmarks::tsp_benchmark::benches,
}
