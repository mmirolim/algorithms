use algos::tsp::*;
use criterion::*;

fn permutations_benchmark(c: &mut Criterion) {
    for d in INPUT.iter() {
        c.bench(
            "tsp",
            Benchmark::new("permutations ".to_string() + d.0, move |b| {
                b.iter(|| permutations(&mut d.1.iter().collect()))
            }),
        );
    }
}

fn nearest_neighbour_solution_bench(c: &mut Criterion) {
    for d in INPUT.iter() {
        c.bench(
            "tsp-algorithms",
            Benchmark::new("nearest_neighbour_solution ".to_string() + d.0, move |b| {
                b.iter(|| nearest_neighbour_solution(&d.1))
            }),
        );
    }
}
fn closest_pair_brute_solution_bench(c: &mut Criterion) {
    for d in INPUT.iter() {
        c.bench(
            "tsp-algorithms",
            Benchmark::new("closest_pair_brute_solution ".to_string() + d.0, move |b| {
                b.iter(|| closest_pair_brute_solution(&d.1))
            }),
        );
    }
}

fn optimal_solution_bench(c: &mut Criterion) {
    for d in INPUT.iter() {
        c.bench(
            "tsp-algorithms",
            Benchmark::new("optimal_solution ".to_string() + d.0, move |b| {
                b.iter(|| optimal_solution(&d.1))
            }),
        );
    }
}

fn sa_solution_bench(c: &mut Criterion) {
    for d in INPUT.iter() {
        c.bench(
            "tsp-algorithms",
            Benchmark::new("sa_solution ".to_string() + d.0, move |b| {
                b.iter(|| sa_solution(&d.1))
            }),
        );
    }
}

fn closest_pair_brute_bench(c: &mut Criterion) {
    for d in INPUT.iter() {
        c.bench(
            "closest-pair-algorithms",
            Benchmark::new("brute_solution ".to_string() + d.0, move |b| {
                b.iter(|| closest_pair_brute(&d.1.iter().collect()))
            }),
        );
    }
}

fn closest_pair_dq_2d_bench(c: &mut Criterion) {
    for d in INPUT.iter() {
        let mut xs: Vec<&Point> = d.1.iter().collect();
        xs.sort_unstable_by(|&p1, &p2| p1.x.partial_cmp(&p2.x).unwrap());
        let mut ys: Vec<&Point> = d.1.iter().collect();
        ys.sort_unstable_by(|&p1, &p2| p1.y.partial_cmp(&p2.y).unwrap());

        c.bench(
            "closest-pair-algorithms",
            Benchmark::new("dq_2d_solution ".to_string() + d.0, move |b| {
                b.iter(|| closest_pair_dq_2d(&xs, &ys))
            }),
        );
    }
}

criterion_group!(
    benches,
    permutations_benchmark,
    nearest_neighbour_solution_bench,
    closest_pair_brute_solution_bench,
    optimal_solution_bench,
    sa_solution_bench,
    closest_pair_brute_bench,
    closest_pair_dq_2d_bench
);

lazy_static! {
    static ref INPUT: [(&'static str, Vec<Point>); 2] = [
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
    ];
}
