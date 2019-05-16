use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Distanced {
    fn id(&self) -> usize;
    fn distance_to(&self, d: &Self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub code: usize,
    pub x: f32,
    pub y: f32,
}

impl Distanced for Point {
    fn id(&self) -> usize {
        self.code
    }
    fn distance_to(&self, d: &Self) -> f32 {
        ((d.x - self.x).powi(2) + (d.y - self.y).powi(2)).sqrt()
    }
}

pub fn dist(p1: &Point, p2: &Point) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

pub fn total_dist<T: Distanced>(path: &Vec<&T>) -> f32 {
    let mut d = 0.0;
    for i in 0..path.len() - 1 {
        d += &path[i].distance_to(&path[i + 1]);
    }
    d + &path[0].distance_to(&path[path.len() - 1])
}

// TODO use linked list as output
pub fn nearest_neighbour_solution<'a, T: Distanced>(input: &'a Vec<T>) -> Vec<&T> {
    let mut out = vec![];
    let choose_start_point = |ps: &'a [T]| -> &'a T { &ps[0] };
    let nearest_unvisited_neighbour =
        |p1: &T, unvisited: &HashMap<usize, &'a T>| -> Option<&'a T> {
            let mut min = (1 << 20) as f32;
            let mut p_nearest = None;
            for (_, p) in unvisited {
                let d = p1.distance_to(*p);
                if min > d {
                    min = d;
                    p_nearest = Some(*p);
                }
            }
            p_nearest
        };

    let mut p = choose_start_point(input);

    let mut unvisited_points = HashMap::new();
    for p in input {
        unvisited_points.insert(p.id(), p);
    }
    unvisited_points.remove(&p.id());
    // store start point
    out.push(p);
    while !&unvisited_points.is_empty() {
        p = nearest_unvisited_neighbour(p, &unvisited_points).unwrap();
        out.push(p);
        unvisited_points.remove(&p.id());
    }

    out
}

pub fn closest_pair_dq_solution(input: &Vec<Point>) -> Vec<Point> {
    let out = vec![];
    let mut xs = input.clone();
    xs.sort_unstable_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap());
    let mut ys = input.clone();
    ys.sort_unstable_by(|p1, p2| p1.y.partial_cmp(&p2.y).unwrap());

    let closest_pair = || -> (usize, usize) { (0, 0) };

    out
}

pub fn closest_pair_brute_solution<'a, T: Distanced>(input: &'a Vec<T>) -> Vec<&T> {
    let mut out = vec![];
    // TODO use random starting point
    out.push(&input[0]);

    let mut unjoined = HashMap::new();
    for i in 0..input.len() {
        unjoined.insert(input[i].id(), &input[i]);
    }
    unjoined.remove(&input[0].id());

    let mut ends = 1;
    while !&unjoined.is_empty() {
        let mut min = (1 << 20) as f32;
        let mut p_nearest: Option<&T> = None;
        let mut index = 0;
        for i in 0..ends {
            let p1 = if i == 0 { out[0] } else { out[out.len() - 1] };
            for (_, p2) in &unjoined {
                let d = p1.distance_to(p2);
                if min > d {
                    min = d;
                    p_nearest = Some(p2);
                    index = i;
                }
            }
        }
        let p = p_nearest.unwrap();
        unjoined.remove(&p.id());

        if index == 0 {
            out.insert(0, p);
        } else {
            out.push(p)
        }
        ends = 2;
    }

    out
}

pub fn permutations<'a, T>(input: &mut Vec<&'a T>) -> Vec<Vec<&'a T>>
where
    T: Distanced,
    T: Clone,
{
    let mut out = vec![];
    if input.len() == 2 {
        out.push(vec![input[0], input[1]]);
        out.push(vec![input[1], input[0]]);
        return out;
    }
    let mut i = 0;
    loop {
        let result = permutations(&mut input[1..].to_vec());
        for mut v in result {
            v.insert(0, &input[0]);
            out.push(v);
        }
        if i == input.len() - 1 {
            break;
        }
        i = i + 1;
        let tmp = input[0];
        input[0] = input[i];
        input[i] = tmp;
    }
    out
}
pub fn optimal_solution<T>(input: &Vec<T>) -> Vec<&T>
where
    T: Distanced,
    T: Clone,
{
    let mut min = (1 << 20) as f32;
    let mut input: Vec<&T> = input.iter().collect();
    let all_paths = permutations(&mut input);
    let mut min_path = None;
    for path in all_paths {
        let d = total_dist(&path);
        if min > d {
            min = d;
            min_path = Some(path);
        }
    }
    min_path.unwrap().to_vec()
}
// simulated annealing
pub fn sa_solution<'a, T>(input: &Vec<T>) -> Vec<&T>
where
    T: Distanced,
    T: std::fmt::Debug,
{
    type State<'a, T> = Rc<RefCell<Vec<&'a T>>>;

    let rng = Rc::new(RefCell::new(thread_rng()));
    let state: State<T> = Rc::new(RefCell::new(input.iter().collect()));
    let iter_num = input.len();
    const ALPHA: f32 = 0.99;
    const T0: f32 = 1.0;
    const MIN_T: f32 = 0.0001;

    let temp = |t| -> f32 { ALPHA * t };

    let transition_prob = |e1: f32, e2: f32, t: f32| -> f32 {
        if e2 < e1 {
            1.0
        } else {
            (-(e2 - e1) / t).exp()
        }
    };

    let energy = |s: State<T>| -> f32 { total_dist(&s.borrow()) };

    let neighbour = |v: State<T>| -> (usize, usize) {
        let mut v = v.borrow_mut();
        let mut rng = rng.borrow_mut();
        let pos1 = rng.gen_range(0, v.len());
        let mut pos2: usize;
        loop {
            pos2 = rng.gen_range(0, v.len());
            if pos1 != pos2 {
                break;
            }
        }

        let tmp = v[pos1];
        v[pos1] = v[pos2];
        v[pos2] = tmp;
        (pos1, pos2)
    };

    let revert = |pos1: usize, pos2: usize, v: State<T>| {
        let mut v = v.borrow_mut();
        let tmp = v[pos1];
        v[pos1] = v[pos2];
        v[pos2] = tmp;
    };

    let mut t = T0;
    // initial path state
    let mut e_last = energy(Rc::clone(&state));
    let mut e_new: f32;
    let mut energy_last_update_temp = T0;
    loop {
        t = temp(t);
        for _ in 0..iter_num {
            let (pos1, pos2) = neighbour(Rc::clone(&state));
            e_new = energy(Rc::clone(&state));
            if transition_prob(e_last, e_new, t) > rng.borrow_mut().gen_range(0.0, 1.0) {
                e_last = e_new;
                energy_last_update_temp = t;
            } else {
                revert(pos1, pos2, Rc::clone(&state));
            }
        }
        // stop if 10% of temp change didn't change energy level
        if (energy_last_update_temp - t) >= 0.1 * energy_last_update_temp {
            break;
        }
        if t < MIN_T {
            break;
        }
    }

    Rc::try_unwrap(state).unwrap().into_inner()
}

#[cfg(test)]
mod tests {
    fn data() -> [(&'static str, Vec<Point>); 3] {
        [
            ("CIRCULAR_POS", CIRCULAR_POS.to_vec()),
            ("INLINE_POS", INLINE_POS.to_vec()),
            ("RECTANGLE_POS", RECTANGLE_POS.to_vec()),
        ]
    }
    fn path_to_string<T: Distanced>(p: &Vec<&T>) -> String {
        p.iter()
            .fold(String::new(), |acc, p| acc + &p.id().to_string())
    }

    use crate::tsp::*;
    #[test]
    fn test_nearest_neighbour_solution() {
        for d in data().iter() {
            let result = nearest_neighbour_solution(&d.1);
            assert_eq!(result.len(), d.1.len());
            println!(
                "NN {} min dist {} path {}",
                d.0,
                total_dist(&result),
                path_to_string(&result)
            );
        }
    }

    #[test]
    fn test_closest_pair_brute_solution() {
        for d in data().iter() {
            let result = closest_pair_brute_solution(&d.1);
            assert_eq!(result.len(), d.1.len());
            println!(
                "CP {} min dist {} path {}",
                d.0,
                total_dist(&result),
                path_to_string(&result)
            );
        }
    }

    #[test]
    fn test_optimal_solution() {
        for d in data().iter() {
            let result = optimal_solution(&d.1);
            assert_eq!(result.len(), d.1.len());
            println!(
                "OS {} min dist {} path {}",
                d.0,
                total_dist(&result),
                path_to_string(&result)
            );
        }
    }
    #[test]
    fn test_sa_solution() {
        for d in data().iter() {
            let result = sa_solution(&d.1);
            assert_eq!(result.len(), d.1.len());
            println!(
                "SA {} min dist {} path {}",
                d.0,
                total_dist(&result),
                path_to_string(&result)
            );
        }
    }
    #[test]
    fn test_permutations() {
        struct Data<'a> {
            input: Vec<Point>,
            output: Vec<&'a str>,
        }
        let input_data = vec![
            Data {
                input: vec![
                    Point {
                        code: 1,
                        x: 5.0,
                        y: 5.0,
                    },
                    Point {
                        code: 2,
                        x: 7.0,
                        y: 4.0,
                    },
                    Point {
                        code: 3,
                        x: 6.0,
                        y: 2.0,
                    },
                    Point {
                        code: 4,
                        x: 6.0,
                        y: 2.0,
                    },
                ],
                output: vec![
                    "1234", "1243", "1324", "1342", "1423", "1432", "2134", "2143", "2314", "2341",
                    "2413", "2431", "3124", "3142", "3214", "3241", "3412", "3421", "4123", "4132",
                    "4213", "4231", "4312", "4321",
                ],
            },
            Data {
                input: vec![
                    Point {
                        code: 1,
                        x: 5.0,
                        y: 5.0,
                    },
                    Point {
                        code: 2,
                        x: 7.0,
                        y: 4.0,
                    },
                    Point {
                        code: 3,
                        x: 6.0,
                        y: 2.0,
                    },
                ],
                output: vec!["123", "132", "213", "231", "312", "321"],
            },
        ];
        for d in input_data {
            let output = permutations(&mut d.input.iter().collect());
            let output = output
                .iter()
                .map(|v| {
                    v.iter()
                        .fold(String::new(), |acc, p| acc + &p.id().to_string())
                })
                .collect::<Vec<String>>();

            assert_eq!(d.output, output);
        }
    }
    #[test]
    fn test_point_distanced() {
        let p1 = Point {
            code: 1,
            x: 5.0,
            y: 5.0,
        };
        let p2 = Point {
            code: 2,
            x: 4.0,
            y: 6.0,
        };
        let d1 = dist(&p1, &p2);
        let d2 = p1.distance_to(&p2);
        assert_eq!(d1, d2);
    }
    const CIRCULAR_POS: [Point; 5] = [
        Point {
            code: 1,
            x: 5.0,
            y: 5.0,
        },
        Point {
            code: 2,
            x: 7.0,
            y: 4.0,
        },
        Point {
            code: 3,
            x: 6.0,
            y: 2.0,
        },
        Point {
            code: 4,
            x: 4.0,
            y: 2.5,
        },
        Point {
            code: 5,
            x: 3.0,
            y: 4.0,
        },
    ];
    const INLINE_POS: [Point; 7] = [
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
    ];
    const RECTANGLE_POS: [Point; 6] = [
        Point {
            code: 5,
            x: 12.2,
            y: 2.0,
        },
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
            code: 4,
            x: 12.2,
            y: 6.9,
        },
        Point {
            code: 3,
            x: 7.1,
            y: 6.9,
        },
        Point {
            code: 6,
            x: 7.1,
            y: 2.0,
        },
    ];

}
