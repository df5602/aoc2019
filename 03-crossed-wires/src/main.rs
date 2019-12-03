use std::collections::{HashMap, HashSet};
use std::env;

use aoc_util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: Vec<String> = match FileReader::new().split_lines().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let wire_paths: Vec<WirePath> = input
        .iter()
        .map(|definition| WirePath::parse_from_str(&definition))
        .collect();

    assert_eq!(2, wire_paths.len());

    let set_a = trace_path(&wire_paths[0]);
    let set_b = trace_path(&wire_paths[1]);
    let closest = find_closest_intersection(&set_a.0, &set_b.0);
    match closest {
        Some(distance) => println!("Distance to closest intersection: {}", distance),
        None => println!("No intersections found."),
    }

    let fewest_steps = find_fewest_steps_to_intersection(&set_a, &set_b);
    match fewest_steps {
        Some(steps) => println!("Fewest steps to intersection: {}", steps),
        None => println!("No intersections found."),
    }
}

fn trace_path(path: &WirePath) -> (HashSet<(isize, isize)>, HashMap<(isize, isize), u32>) {
    let mut set = HashSet::new();
    let mut map = HashMap::new();

    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut last_distance = 0;

    for segment in &path.segments {
        match segment.direction {
            Direction::Left => {
                for x in (curr_x - segment.length as isize..curr_x).rev() {
                    set.insert((x, curr_y));
                    let dist = map.entry((x, curr_y)).or_insert(0);
                    last_distance += 1;
                    if *dist == 0 {
                        *dist = last_distance;
                    }
                }
                curr_x -= segment.length as isize;
            }
            Direction::Right => {
                for x in curr_x + 1..=curr_x + segment.length as isize {
                    set.insert((x, curr_y));
                    let dist = map.entry((x, curr_y)).or_insert(0);
                    last_distance += 1;
                    if *dist == 0 {
                        *dist = last_distance;
                    }
                }
                curr_x += segment.length as isize;
            }
            Direction::Up => {
                for y in curr_y + 1..=curr_y + segment.length as isize {
                    set.insert((curr_x, y));
                    let dist = map.entry((curr_x, y)).or_insert(0);
                    last_distance += 1;
                    if *dist == 0 {
                        *dist = last_distance;
                    }
                }
                curr_y += segment.length as isize;
            }
            Direction::Down => {
                for y in (curr_y - segment.length as isize..curr_y).rev() {
                    set.insert((curr_x, y));
                    let dist = map.entry((curr_x, y)).or_insert(0);
                    last_distance += 1;
                    if *dist == 0 {
                        *dist = last_distance;
                    }
                }
                curr_y -= segment.length as isize;
            }
        }
    }

    (set, map)
}

fn find_closest_intersection(
    a: &HashSet<(isize, isize)>,
    b: &HashSet<(isize, isize)>,
) -> Option<isize> {
    a.intersection(b).map(|(x, y)| x.abs() + y.abs()).min()
}

fn find_fewest_steps_to_intersection(
    a: &(HashSet<(isize, isize)>, HashMap<(isize, isize), u32>),
    b: &(HashSet<(isize, isize)>, HashMap<(isize, isize), u32>),
) -> Option<u32> {
    a.0.intersection(&b.0)
        .map(|(x, y)| a.1.get(&(*x, *y)).unwrap() + b.1.get(&(*x, *y)).unwrap())
        .min()
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    direction: Direction,
    length: u32,
}

#[derive(Debug)]
struct WirePath {
    segments: Vec<Segment>,
}

impl WirePath {
    fn parse_from_str(definition: &str) -> Self {
        let segments = definition
            .split(',')
            .map(|chunk| {
                let direction = match chunk.chars().nth(0) {
                    Some('R') => Direction::Right,
                    Some('L') => Direction::Left,
                    Some('U') => Direction::Up,
                    Some('D') => Direction::Down,
                    _ => panic!("Invalid format!"),
                };
                let length: u32 = chunk[1..].parse().unwrap();
                Segment { direction, length }
            })
            .collect();

        Self { segments }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_intersection_1() {
        let definitions = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        let wire_paths: Vec<WirePath> = definitions
            .iter()
            .map(|definition| WirePath::parse_from_str(&definition))
            .collect();

        let set_a = trace_path(&wire_paths[0]);
        let set_b = trace_path(&wire_paths[1]);
        assert_eq!(Some(159), find_closest_intersection(&set_a.0, &set_b.0));
    }

    #[test]
    fn closest_intersection_2() {
        let definitions = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        let wire_paths: Vec<WirePath> = definitions
            .iter()
            .map(|definition| WirePath::parse_from_str(&definition))
            .collect();

        let set_a = trace_path(&wire_paths[0]);
        let set_b = trace_path(&wire_paths[1]);
        assert_eq!(Some(135), find_closest_intersection(&set_a.0, &set_b.0));
    }

    #[test]
    fn fewest_steps_1() {
        let definitions = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        let wire_paths: Vec<WirePath> = definitions
            .iter()
            .map(|definition| WirePath::parse_from_str(&definition))
            .collect();

        let set_a = trace_path(&wire_paths[0]);
        let set_b = trace_path(&wire_paths[1]);
        assert_eq!(Some(610), find_fewest_steps_to_intersection(&set_a, &set_b));
    }

    #[test]
    fn fewest_steps_2() {
        let definitions = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        let wire_paths: Vec<WirePath> = definitions
            .iter()
            .map(|definition| WirePath::parse_from_str(&definition))
            .collect();

        let set_a = trace_path(&wire_paths[0]);
        let set_b = trace_path(&wire_paths[1]);
        assert_eq!(Some(410), find_fewest_steps_to_intersection(&set_a, &set_b));
    }
}
