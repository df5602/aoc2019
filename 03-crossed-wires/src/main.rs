use std::collections::HashMap;
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

    let points_a = trace_path(&wire_paths[0]);
    let points_b = trace_path(&wire_paths[1]);
    let intersections = map_intersection(&points_a, &points_b);
    let closest = find_closest_intersection(&intersections);
    match closest {
        Some(distance) => println!("Distance to closest intersection: {}", distance),
        None => println!("No intersections found."),
    }

    let fewest_steps = find_fewest_steps_to_intersection(&intersections, &points_a, &points_b);
    match fewest_steps {
        Some(steps) => println!("Fewest steps to intersection: {}", steps),
        None => println!("No intersections found."),
    }
}

fn trace_path(path: &WirePath) -> HashMap<Point, u32> {
    // Pre-allocate to gain performance
    let mut map = HashMap::with_capacity(path.segments.len() * 1000);

    let mut current = Point::new(0, 0);
    let mut distance = 0;

    for segment in &path.segments {
        for _ in 0..segment.length {
            current = match segment.direction {
                Direction::Left => Point::new(current.x - 1, current.y),
                Direction::Right => Point::new(current.x + 1, current.y),
                Direction::Up => Point::new(current.x, current.y + 1),
                Direction::Down => Point::new(current.x, current.y - 1),
            };
            distance += 1;
            map.entry(current).or_insert(distance);
        }
    }

    map
}

fn find_closest_intersection(intersections: &[Point]) -> Option<i32> {
    intersections.iter().map(|p| p.x.abs() + p.y.abs()).min()
}

fn find_fewest_steps_to_intersection(
    intersections: &[Point],
    a: &HashMap<Point, u32>,
    b: &HashMap<Point, u32>,
) -> Option<u32> {
    intersections
        .iter()
        .map(|p| a.get(&p).unwrap() + b.get(&p).unwrap())
        .min()
}

fn map_intersection<K, V>(a: &HashMap<K, V>, b: &HashMap<K, V>) -> Vec<K>
where
    K: std::cmp::Eq + std::hash::Hash + Copy,
{
    a.iter()
        .filter(|(k, _)| b.contains_key(k))
        .map(|(&k, _)| k)
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
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
    fn intersection() {
        let mut a: HashMap<u32, u32> = HashMap::new();
        a.insert(1, 1);
        a.insert(2, 4);
        a.insert(3, 9);

        let mut b: HashMap<u32, u32> = HashMap::new();
        b.insert(4, 32);
        b.insert(2, 8);
        b.insert(3, 18);

        let mut intersection = map_intersection(&a, &b);
        intersection.sort();

        assert_eq!(vec![2, 3], intersection);
    }

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

        let points_a = trace_path(&wire_paths[0]);
        let points_b = trace_path(&wire_paths[1]);
        let intersections = map_intersection(&points_a, &points_b);
        assert_eq!(Some(159), find_closest_intersection(&intersections));
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

        let points_a = trace_path(&wire_paths[0]);
        let points_b = trace_path(&wire_paths[1]);
        let intersections = map_intersection(&points_a, &points_b);
        assert_eq!(Some(135), find_closest_intersection(&intersections));
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

        let points_a = trace_path(&wire_paths[0]);
        let points_b = trace_path(&wire_paths[1]);
        let intersections = map_intersection(&points_a, &points_b);
        assert_eq!(
            Some(610),
            find_fewest_steps_to_intersection(&intersections, &points_a, &points_b)
        );
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

        let points_a = trace_path(&wire_paths[0]);
        let points_b = trace_path(&wire_paths[1]);
        let intersections = map_intersection(&points_a, &points_b);
        assert_eq!(
            Some(410),
            find_fewest_steps_to_intersection(&intersections, &points_a, &points_b)
        );
    }

    #[test]
    fn part_1() {
        let input: Vec<String> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();

        let wire_paths: Vec<WirePath> = input
            .iter()
            .map(|definition| WirePath::parse_from_str(&definition))
            .collect();

        assert_eq!(2, wire_paths.len());

        let points_a = trace_path(&wire_paths[0]);
        let points_b = trace_path(&wire_paths[1]);
        let intersections = map_intersection(&points_a, &points_b);
        let closest = find_closest_intersection(&intersections);

        assert_eq!(Some(8015), closest);
    }

    #[test]
    fn part_2() {
        let input: Vec<String> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();

        let wire_paths: Vec<WirePath> = input
            .iter()
            .map(|definition| WirePath::parse_from_str(&definition))
            .collect();

        assert_eq!(2, wire_paths.len());

        let points_a = trace_path(&wire_paths[0]);
        let points_b = trace_path(&wire_paths[1]);
        let intersections = map_intersection(&points_a, &points_b);
        let fewest_steps = find_fewest_steps_to_intersection(&intersections, &points_a, &points_b);

        assert_eq!(Some(163676), fewest_steps);
    }
}
