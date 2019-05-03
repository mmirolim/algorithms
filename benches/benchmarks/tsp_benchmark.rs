use algos::tsp::*;
use criterion::*;

fn permutations_benchmark(c: &mut Criterion) {
    let data = data();
    for d in data.iter() {
        let mut d = d.clone();
        c.bench(
            "tsp",
            Benchmark::new("permutations ".to_string() + d.0, move |b| {
                b.iter(|| permutations(&mut d.1))
            }),
        );
    }
}

fn nearest_neighbour_solution_bench(c: &mut Criterion) {
    let data = data();
    for d in data.iter() {
        let d = d.clone();
        c.bench(
            "tsp-algorithms",
            Benchmark::new("nearest_neighbour_solution ".to_string() + d.0, move |b| {
                b.iter(|| nearest_neighbour_solution(&d.1))
            }),
        );
    }
}
fn closest_pair_solution_bench(c: &mut Criterion) {
    let data = data();
    for d in data.iter() {
        let d = d.clone();
        c.bench(
            "tsp-algorithms",
            Benchmark::new("closest_pair_solution ".to_string() + d.0, move |b| {
                b.iter(|| closest_pair_solution(&d.1))
            }),
        );
    }
}

fn optimal_solution_bench(c: &mut Criterion) {
    let data = data();
    for d in data.iter() {
        let mut d = d.clone();
        c.bench(
            "tsp-algorithms",
            Benchmark::new("optimal_solution ".to_string() + d.0, move |b| {
                b.iter(|| optimal_solution(&mut d.1))
            }),
        );
    }
}

fn sa_solution_bench(c: &mut Criterion) {
    let data = data();
    for d in data.iter() {
        let d = d.clone();
        c.bench(
            "tsp-algorithms",
            Benchmark::new("sa_solution ".to_string() + d.0, move |b| {
                b.iter(|| sa_solution(&d.1))
            }),
        );
    }
}

criterion_group!(
    benches,
    permutations_benchmark,
    nearest_neighbour_solution_bench,
    closest_pair_solution_bench,
    optimal_solution_bench,
    sa_solution_bench
);

fn data() -> [(&'static str, Vec<Point>); 2] {
    [
        (
            "7 element INLINE_POS",
            vec![
                Point {
                    code: 1,
                    x: 5.0,
                    y: 11.0,
                },
                Point {
                    code: 2,
                    x: 5.0,
                    y: 10.0,
                },
                Point {
                    code: 3,
                    x: 5.0,
                    y: 12.0,
                },
                Point {
                    code: 4,
                    x: 5.0,
                    y: 8.0,
                },
                Point {
                    code: 5,
                    x: 5.0,
                    y: 16.0,
                },
                Point {
                    code: 6,
                    x: 5.0,
                    y: 0.0,
                },
                Point {
                    code: 7,
                    x: 5.0,
                    y: 33.0,
                },
            ],
        ),
        (
            "6 elements RECTANGLE_POS",
            vec![
                Point {
                    code: 1,
                    x: 2.0,
                    y: 2.0,
                },
                Point {
                    code: 2,
                    x: 2.0,
                    y: 6.9,
                },
                Point {
                    code: 3,
                    x: 7.1,
                    y: 6.9,
                },
                Point {
                    code: 4,
                    x: 12.2,
                    y: 6.9,
                },
                Point {
                    code: 5,
                    x: 12.2,
                    y: 2.0,
                },
                Point {
                    code: 6,
                    x: 7.1,
                    y: 2.0,
                },
            ],
        ),
    ]
}
