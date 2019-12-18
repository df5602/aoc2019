use std::collections::VecDeque;
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

    let mut map = AsteroidMap::new(&input);
    let most_asteroids_detected = map.find_best_monitoring_location();
    println!(
        "Best location at position ({},{}). Asteroids detected: {}",
        most_asteroids_detected.0.x, most_asteroids_detected.0.y, most_asteroids_detected.1
    );

    let twohundredth = map.find_nth_vaporized_asteroid(most_asteroids_detected.0, 200);
    println!(
        "200th asteroid to be vaporized: ({},{})",
        twohundredth.x, twohundredth.y
    );
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
        self.dx /= gcd as isize;
        self.dy /= gcd as isize;
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

        a.abs() as usize
    }

    fn calculate_angle(&self) -> f64 {
        use std::f64::consts::PI;

        if self.dx >= 0 && self.dy < 0 {
            f64::atan(self.dx as f64 / -self.dy as f64)
        } else if self.dx > 0 && self.dy == 0 {
            PI / 2.0
        } else if self.dx >= 0 && self.dy > 0 {
            PI - f64::atan(self.dx as f64 / self.dy as f64)
        } else if self.dx < 0 && self.dy > 0 {
            PI + f64::atan(-self.dx as f64 / self.dy as f64)
        } else if self.dx < 0 && self.dy == 0 {
            3.0 * PI / 2.0
        } else if self.dx < 0 && self.dy < 0 {
            2.0 * PI - f64::atan(self.dx as f64 / self.dy as f64)
        } else {
            panic!(
                "Cannot calculate angle of vector of length zero [dx: {}, dy: {}]",
                self.dx, self.dy
            );
        }
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
        assert!(!input.is_empty());

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

    #[allow(clippy::many_single_char_names)]
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

        true
    }

    fn find_best_monitoring_location(&self) -> (Point, usize) {
        let mut numbers = Vec::with_capacity(self.asteroids.len());

        for location in &self.asteroids {
            let mut count = 0;
            for other in &self.asteroids {
                if self.line_of_sight(*location, *other) {
                    count += 1;
                }
            }
            numbers.push((*location, count));
        }

        *numbers.iter().max_by_key(|(_, count)| count).unwrap()
    }

    fn find_nth_vaporized_asteroid(&mut self, laser_location: Point, n: usize) -> Point {
        // Delete position of laser from map, given that we don't want to vaporize ourselves...
        let idx_laser_location = self
            .asteroids
            .iter()
            .position(|&asteroid| asteroid == laser_location)
            .unwrap();
        self.asteroids.remove(idx_laser_location);
        self.grid[laser_location.y * self.width + laser_location.x] = 0;

        // Sort asteroid list by angles
        let mut asteroids: VecDeque<(Point, f64)> = self
            .asteroids
            .iter()
            .map(|&asteroid| {
                (
                    asteroid,
                    Vector::from_points(laser_location, asteroid).calculate_angle(),
                )
            })
            .collect();

        asteroids
            .as_mut_slices()
            .0
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Start vaporizing
        let mut vaporization_count = 0;
        let mut count_since_last_vaporization = 0;
        let mut previous_angle = -10.0;

        let mut nth = Point { x: 0, y: 0 };

        while let Some((asteroid, angle)) = asteroids.pop_front() {
            if self.line_of_sight(laser_location, asteroid)
                && ((angle - previous_angle).abs() > std::f64::EPSILON
                    || count_since_last_vaporization == asteroids.len())
            {
                vaporization_count += 1;
                count_since_last_vaporization = 0;
                previous_angle = angle;
                self.grid[asteroid.y * self.width + asteroid.x] = 0;

                if vaporization_count == n {
                    nth = asteroid;
                    break;
                }
            } else {
                count_since_last_vaporization += 1;
                asteroids.push_back((asteroid, angle));
            }
        }

        nth
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
    fn trigonometry() {
        use std::f64::consts::PI;

        let up = Vector { dx: 0, dy: -3 };
        assert_eq!(0.0, up.calculate_angle());

        let q1 = Vector { dx: 3, dy: -3 };
        assert_eq!(PI / 4.0, q1.calculate_angle());

        let right = Vector { dx: 3, dy: 0 };
        assert_eq!(PI / 2.0, right.calculate_angle());

        let q2 = Vector { dx: 3, dy: 3 };
        assert_eq!(3.0 * PI / 4.0, q2.calculate_angle());

        let down = Vector { dx: 0, dy: 3 };
        assert_eq!(PI, down.calculate_angle());

        let q3 = Vector { dx: -3, dy: 3 };
        assert_eq!(5.0 * PI / 4.0, q3.calculate_angle());

        let left = Vector { dx: -3, dy: 0 };
        assert_eq!(3.0 * PI / 2.0, left.calculate_angle());

        let q4 = Vector { dx: -3, dy: -3 };
        assert_eq!(7.0 * PI / 4.0, q4.calculate_angle());
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
        assert_eq!((Point::new(3, 4), 8), most_asteroids_detected);
    }

    #[test]
    fn part_1() {
        let input: Vec<String> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let map = AsteroidMap::new(&input);
        let most_asteroids_detected = map.find_best_monitoring_location();
        assert_eq!(247, most_asteroids_detected.1);
    }

    #[test]
    fn part_2() {
        let input: Vec<String> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let mut map = AsteroidMap::new(&input);
        let most_asteroids_detected = map.find_best_monitoring_location();
        let twohundredth = map.find_nth_vaporized_asteroid(most_asteroids_detected.0, 200);
        assert_eq!(Point { x: 19, y: 19 }, twohundredth);
    }
}
