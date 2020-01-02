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

    let program: Vec<i64> = match FileReader::new().split_char(',').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut robot = VacuumRobot::new(&program);
    robot.run();

    robot.draw_scaffolding();

    let sum_of_alignment_parameters = robot.sum_of_alignment_parameters();
    println!(
        "Sum of alignment parameters: {}",
        sum_of_alignment_parameters
    );
}

struct VacuumRobot {
    computer: Computer<Option<i64>, VecDeque<i64>>,
    scaffolding: Vec<Tile>,
    intersections: Vec<Position>,
    camera_width: usize,
    camera_height: usize,
}

impl VacuumRobot {
    fn new(program: &[i64]) -> Self {
        Self {
            computer: Computer::new(0, program, None, VecDeque::new()),
            scaffolding: Vec::new(),
            intersections: Vec::new(),
            camera_width: 0,
            camera_height: 0,
        }
    }

    fn run(&mut self) {
        let run_state = self.computer.run_program();

        loop {
            match run_state {
                RunState::NotYetStarted => unreachable!(),
                RunState::NeedInput => println!("NEED INPUT"),
                RunState::Stopped(_) => break,
            }
        }

        for (i, &output) in self.computer.get_output().iter().enumerate() {
            assert!(output >= 0 && output < 256);
            match output as u8 {
                b'.' => self.scaffolding.push(Tile::OpenSpace),
                b'#' => self.scaffolding.push(Tile::Scaffold),
                b'^' => self.scaffolding.push(Tile::Robot(Direction::Up)),
                b'<' => self.scaffolding.push(Tile::Robot(Direction::Left)),
                b'>' => self.scaffolding.push(Tile::Robot(Direction::Right)),
                b'v' => self.scaffolding.push(Tile::Robot(Direction::Down)),
                b'\n' => {
                    if self.camera_width == 0 {
                        self.camera_width = i
                    }
                }
                c => panic!("Unexpected output: {}", c),
            }
        }
        self.camera_height = self.scaffolding.len() / self.camera_width;

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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: isize,
    y: isize,
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

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     assert!(1 < 2);
    // }
}
