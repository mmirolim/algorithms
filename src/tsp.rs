use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub code: usize,
    pub x: f32,
    pub y: f32,
}

pub fn dist(p1: &Point, p2: &Point) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}
pub fn total_dist(path: &Vec<Point>) -> f32 {
    let mut d = 0.0;
    for i in 0..path.len() - 1 {
        d += dist(&path[i], &path[i + 1]);
    }
    d + dist(&path[0], &path[path.len() - 1])
}
// TODO use linked list as output
pub fn nearest_neighbour_solution<'a>(input: &'a Vec<Point>) -> Vec<Point> {
    let mut out = vec![];
    let choose_start_point = |ps: &'a [Point]| -> &'a Point { &ps[0] };
    let nearest_unvisited_neighbour =
        |p1: &Point, unvisited: &HashMap<usize, &'a Point>| -> Option<&'a Point> {
            let mut min = (1 << 20) as f32;
            let mut p_nearest = None;
            for (_, p) in unvisited {
                let d = dist(p1, &p);
                if min > d {
                    min = d;
                    p_nearest = Some(*p);
                }
            }
            p_nearest
        };

    let mut p = choose_start_point(input);
    let mut unvisited_points = |start_point: &'a Point| -> HashMap<usize, &Point> {
        let mut dic = HashMap::new();
        for p in input {
            if start_point.code == p.code {
                continue;
            }
            dic.insert(p.code, p);
        }
        dic
    }(p);
    // store start point
    out.push(*p);
    while !&unvisited_points.is_empty() {
        p = nearest_unvisited_neighbour(p, &unvisited_points).unwrap();
        out.push(*p);
        unvisited_points.remove(&p.code);
    }

    out
}

pub fn closest_pair_solution<'a>(input: &'a Vec<Point>) -> Vec<Point> {
    let mut out = vec![];
    out.push(input[0]);

    let mut unjoined = HashMap::new();
    for i in 1..input.len() {
        unjoined.insert(input[i].code, &input[i]);
    }

    while !&unjoined.is_empty() {
        let mut min = (1 << 20) as f32;
        let mut p_nearest = None;
        let mut index = 0;
        for i in 0..2 {
            let p1 = if i == 0 { out[0] } else { out[out.len() - 1] };
            for (_, p2) in &unjoined {
                let d = dist(&p1, p2);
                if min > d {
                    min = d;
                    p_nearest = Some(p2);
                    index = i;
                }
            }
        }
        let p = *(p_nearest.unwrap());
        unjoined.remove(&p.code);

        if index == 0 {
            out.insert(0, *p);
        } else {
            out.push(*p)
        }
    }

    out
}
pub fn permutations(input: &mut Vec<Point>) -> Vec<Vec<Point>> {
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
            v.insert(0, input[0]);
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
pub fn optimal_solution(input: &mut Vec<Point>) -> Vec<Point> {
    let mut min = (1 << 20) as f32;
    let all_paths = permutations(input);
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

pub fn sa_solution(input: &Vec<Point>) -> Vec<Point> {
    let mut out = vec![];

    out
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
    fn path_to_string(p: &Vec<Point>) -> String {
        p.iter()
            .fold(String::new(), |acc, p| acc + &p.code.to_string())
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
    fn test_closest_pair_solution() {
        for d in data().iter() {
            let result = closest_pair_solution(&d.1);
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
            let result = optimal_solution(&mut d.1.to_owned());
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
        for mut d in input_data {
            let output = permutations(&mut d.input);
            let output = output
                .iter()
                .map(|v| {
                    v.iter()
                        .fold(String::new(), |acc, p| acc + &p.code.to_string())
                })
                .collect::<Vec<String>>();

            assert_eq!(d.output, output);
        }
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
    ];

}
