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

    let map = AsteroidMap::new(&input);
    let most_asteroids_detected = map.find_best_monitoring_location();
    println!("Most asteroids detected: {}", most_asteroids_detected);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector {
    dx: isize,
    dy: isize,
}

impl Vector {
    fn from_points(a: Point, b: Point) -> Self {
        Self {
            dx: b.x as isize - a.x as isize,
            dy: b.y as isize - a.y as isize,
        }
    }

    fn minimize(&mut self) {
        let gcd = Self::greatest_common_divisor(self.dx, self.dy);
        self.dx = self.dx / (gcd as isize);
        self.dy = self.dy / (gcd as isize);
    }

    fn greatest_common_divisor(mut a: isize, mut b: isize) -> usize {
        if a == 0 {
            return b.abs() as usize;
        }
        if b == 0 {
            return a.abs() as usize;
        }

        loop {
            let h = a % b;
            a = b;
            b = h;

            if b == 0 {
                break;
            }
        }

        return a.abs() as usize;
    }
}

struct AsteroidMap {
    grid: Vec<usize>,
    asteroids: Vec<Point>,
    width: usize,
    height: usize,
}

impl AsteroidMap {
    fn new(input: &[String]) -> Self {
        assert!(input.len() > 0);

        let width = input[0].len();
        let height = input.len();

        let mut grid = Vec::with_capacity(width * height);
        let mut asteroids = Vec::new();

        for (y, line) in input.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        grid.push(1);
                        asteroids.push(Point::new(x, y));
                    }
                    '.' => grid.push(0),
                    c => panic!("Unexpected character: {}", c),
                }
            }
        }

        Self {
            grid,
            asteroids,
            width,
            height,
        }
    }

    fn line_of_sight(&self, a: Point, b: Point) -> bool {
        let mut v = Vector::from_points(a, b);
        if v.dx == 0 && v.dy == 0 {
            return false;
        }
        v.minimize();

        for i in 1..usize::max(self.width, self.height) {
            let dx = i as isize * v.dx;
            if dx < 0 && (a.x as isize) < dx {
                break;
            }
            let x = (a.x as isize + dx) as usize;

            let dy = i as isize * v.dy;
            if dy < 0 && (a.y as isize) < dy {
                break;
            }
            let y = (a.y as isize + dy) as usize;
            if x >= self.width || y >= self.height {
                break;
            }

            if x == b.x && y == b.y {
                break;
            }

            if self.grid[y * self.width + x] == 1 {
                return false;
            }
        }

        return true;
    }

    fn find_best_monitoring_location(&self) -> usize {
        let mut numbers = Vec::with_capacity(self.asteroids.len());

        for location in &self.asteroids {
            let mut count = 0;
            for other in &self.asteroids {
                if self.line_of_sight(*location, *other) {
                    count += 1;
                } else {
                }
            }
            numbers.push((*location, count));
        }

        numbers
            .iter()
            .max_by_key(|(_, count)| count)
            .map(|(_, count)| *count)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd() {
        assert_eq!(252, Vector::greatest_common_divisor(3528, 3780));
    }

    #[test]
    fn minimize_vector() {
        let a = Point::new(9, 3);
        let b = Point::new(6, 2);
        let mut v = Vector::from_points(a, b);
        v.minimize();

        assert_eq!(Vector { dx: -3, dy: -1 }, v);
    }

    #[test]
    fn example_1() {
        let input = vec![
            String::from(".#..#"),
            String::from("....."),
            String::from("#####"),
            String::from("....#"),
            String::from("...##"),
        ];
        let map = AsteroidMap::new(&input);
        let most_asteroids_detected = map.find_best_monitoring_location();
        assert_eq!(8, most_asteroids_detected);
    }
}
