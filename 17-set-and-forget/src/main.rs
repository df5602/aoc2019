use std::collections::VecDeque;
use std::env;

use aoc_util::input::{FileReader, FromFile};
use intcode::{Computer, RunState};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let mut program: Vec<i64> = match FileReader::new().split_char(',').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut robot = VacuumRobot::new(&program);
    robot.dry_run();

    robot.draw_scaffolding();
    println!(
        "Robot position: ({},{})",
        robot.position.x, robot.position.y
    );

    let sum_of_alignment_parameters = robot.sum_of_alignment_parameters();
    println!(
        "Sum of alignment parameters: {}",
        sum_of_alignment_parameters
    );

    let path = robot.find_path();
    for segment in path {
        print!("{},", segment)
    }
    println!("\n");

    /* Solution (hand-crafted):
        A,B,A,B,C,C,B,A,B,C

        A: L,4,R,8,L,6,L,10
        B: L,6,R,8,R,10,L,6,L,6
        C: L,4,L,4,L,10
    */

    program[0] = 2;
    robot.reset_program(&program);
    robot.run();

    for &mut output in robot.computer.get_output() {
        println!("{}", output);
    }
}

struct VacuumRobot {
    computer: Computer<VecDeque<i64>, VecDeque<i64>>,
    scaffolding: Vec<Tile>,
    intersections: Vec<Position>,
    position: Position,
    direction: Direction,
    camera_width: usize,
    camera_height: usize,
}

impl VacuumRobot {
    fn new(program: &[i64]) -> Self {
        Self {
            computer: Computer::new(0, program, VecDeque::new(), VecDeque::new()),
            scaffolding: Vec::new(),
            intersections: Vec::new(),
            position: Position { x: -1, y: -1 },
            direction: Direction::Up,
            camera_width: 0,
            camera_height: 0,
        }
    }

    fn reset_program(&mut self, program: &[i64]) {
        self.computer = Computer::new(0, program, VecDeque::new(), VecDeque::new());
    }

    fn run(&mut self) {
        let input = self.computer.get_input();
        let routine =
            "A,B,A,B,C,C,B,A,B,C\nL,4,R,8,L,6,L,10\nL,6,R,8,R,10,L,6,L,6\nL,4,L,4,L,10\nn\n";
        for c in routine.chars() {
            input.push_back(c as i64);
        }

        let run_state = self.computer.run_program();

        loop {
            match run_state {
                RunState::NotYetStarted => unreachable!(),
                RunState::NeedInput => {
                    println!("NEED INPUT");
                    break;
                }
                RunState::Stopped(_) => break,
            }
        }
    }

    fn dry_run(&mut self) {
        let run_state = self.computer.run_program();

        loop {
            match run_state {
                RunState::NotYetStarted => unreachable!(),
                RunState::NeedInput => println!("NEED INPUT"),
                RunState::Stopped(_) => break,
            }
        }

        let mut line = 0;
        let mut robot_position = 0;
        for (i, &output) in self.computer.get_output().iter().enumerate() {
            assert!(output >= 0 && output < 256);
            match output as u8 {
                b'.' => self.scaffolding.push(Tile::OpenSpace),
                b'#' => self.scaffolding.push(Tile::Scaffold),
                b'^' => {
                    self.direction = Direction::Up;
                    self.scaffolding.push(Tile::Robot(Direction::Up));
                    robot_position = i - line;
                }
                b'<' => {
                    self.direction = Direction::Left;
                    self.scaffolding.push(Tile::Robot(Direction::Left));
                    robot_position = i - line;
                }
                b'>' => {
                    self.direction = Direction::Right;
                    self.scaffolding.push(Tile::Robot(Direction::Right));
                    robot_position = i - line;
                }
                b'v' => {
                    self.direction = Direction::Down;
                    self.scaffolding.push(Tile::Robot(Direction::Down));
                    robot_position = i - line;
                }
                b'\n' => {
                    if self.camera_width == 0 {
                        self.camera_width = i
                    }
                    line += 1;
                }
                c => panic!("Unexpected output: {}", c),
            }
        }
        self.camera_height = self.scaffolding.len() / self.camera_width;
        self.position = Position {
            x: (robot_position - (robot_position / self.camera_width) * self.camera_width) as isize,
            y: (robot_position / self.camera_width) as isize,
        };

        self.find_intersections();
    }

    fn find_intersections(&mut self) {
        assert!(!self.scaffolding.is_empty());

        for y in 0..self.camera_height as isize {
            for x in 0..self.camera_width as isize {
                if self.is_scaffold(Position { x: x, y: y })
                    && self.is_scaffold(Position { x: x, y: y - 1 })
                    && self.is_scaffold(Position { x: x, y: y + 1 })
                    && self.is_scaffold(Position { x: x - 1, y: y })
                    && self.is_scaffold(Position { x: x + 1, y: y })
                {
                    self.intersections.push(Position { x: x, y: y });
                }
            }
        }
    }

    fn is_scaffold(&self, position: Position) -> bool {
        if position.x < 0
            || position.x as usize >= self.camera_width
            || position.y < 0
            || position.y as usize >= self.camera_height
        {
            return false;
        }

        match self.scaffolding[position.y as usize * self.camera_width + position.x as usize] {
            Tile::OpenSpace => false,
            Tile::Scaffold => true,
            Tile::Robot(_) => true,
        }
    }

    fn sum_of_alignment_parameters(&self) -> usize {
        self.intersections
            .iter()
            .map(|pos| (pos.x * pos.y) as usize)
            .sum()
    }

    fn find_path(&self) -> Vec<PathSegment> {
        let mut path = Vec::new();

        let mut current_position = self.position;
        let mut current_direction = self.direction;
        let mut forward_length = 0;

        loop {
            if self.is_scaffold(current_position + current_direction) {
                // Try to move forward
                forward_length += 1;
                current_position = current_position + current_direction;
            } else if self.is_scaffold(current_position + current_direction.left()) {
                // Try left
                if forward_length > 0 {
                    path.push(PathSegment::Forward(forward_length));
                    forward_length = 0;
                }
                path.push(PathSegment::Left);
                current_direction = current_direction.left();
            } else if self.is_scaffold(current_position + current_direction.right()) {
                // Try right
                if forward_length > 0 {
                    path.push(PathSegment::Forward(forward_length));
                    forward_length = 0;
                }
                path.push(PathSegment::Right);
                current_direction = current_direction.right();
            } else {
                if forward_length > 0 {
                    path.push(PathSegment::Forward(forward_length));
                }
                break;
            }
        }

        path
    }

    fn draw_scaffolding(&self) {
        for (i, tile) in self.scaffolding.iter().enumerate() {
            print!("{}", tile);
            if (i + 1) % self.camera_width == 0 {
                println!();
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: Direction) -> Self {
        match other {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    OpenSpace,
    Scaffold,
    Robot(Direction),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Tile::OpenSpace => write!(f, "."),
            &Tile::Scaffold => write!(f, "#"),
            &Tile::Robot(Direction::Up) => write!(f, "^"),
            &Tile::Robot(Direction::Left) => write!(f, "<"),
            &Tile::Robot(Direction::Right) => write!(f, ">"),
            &Tile::Robot(Direction::Down) => write!(f, "v"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PathSegment {
    Left,
    Right,
    Forward(usize),
}

impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &PathSegment::Left => write!(f, "L"),
            &PathSegment::Right => write!(f, "R"),
            &PathSegment::Forward(steps) => write!(f, "{}", steps),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     assert!(1 < 2);
    // }
}
