use std::collections::{HashMap, VecDeque};
use std::env;
use std::ops::Add;
use std::{thread, time};

use aoc_util::input::{FileReader, FromFile};
use intcode::{Computer, RunState};

const DELAY: std::time::Duration = time::Duration::from_millis(100);

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

    let mut repair_droid = RepairDroid::new(&program, false);
    repair_droid.map_terrain();
    repair_droid
        .terrain
        .calculate_distances_from_oxygen_system();

    let distance_to_oxygen_system = repair_droid.terrain.distance(Position { x: 0, y: 0 });
    println!(
        "Distance between starting position and oxygen system: {}",
        distance_to_oxygen_system
    );

    let max_distance = repair_droid.terrain.max_distance();
    println!("Time to fill area with oxygen: {} minutes", max_distance);
}

struct RepairDroid {
    terrain: Terrain,
    computer: Computer<Option<i64>, Option<i64>>,
    visualize: bool,
}

impl RepairDroid {
    fn new(program: &[i64], visualize: bool) -> Self {
        Self {
            terrain: Terrain::new(),
            computer: Computer::new(0, program, None, None),
            visualize,
        }
    }

    fn map_terrain(&mut self) {
        let run_state = self.computer.run_program();
        if run_state != RunState::NeedInput {
            panic!("Run state was {:?}", run_state);
        }

        let starting_position = Position { x: 0, y: 0 };
        self.terrain.set_at(starting_position, Tile::Floor);
        self.explore(starting_position);
    }

    fn explore(&mut self, droid_position: Position) {
        self.explore_direction(Direction::North, droid_position);
        self.explore_direction(Direction::South, droid_position);
        self.explore_direction(Direction::West, droid_position);
        self.explore_direction(Direction::East, droid_position);
    }

    fn explore_direction(&mut self, direction: Direction, droid_position: Position) {
        // Check that we haven't mapped that direction already
        if self.terrain.at(droid_position + direction).is_some() {
            return;
        }

        if self.visualize {
            println!(
                "Exploring direction {:?} from position ({}, {})",
                direction, droid_position.x, droid_position.y
            );
        }

        // Command direction to explore
        *self.computer.get_input() = Some(direction.into());

        // Explore direction
        let run_state = self.computer.resume();
        if run_state != RunState::NeedInput {
            panic!("Run state was {:?}", run_state);
        }

        // Check status
        let mut obstacle = false;
        let status = self
            .computer
            .get_output()
            .take()
            .expect("Expected status report!");
        match status {
            0 => {
                self.terrain.set_at(droid_position + direction, Tile::Wall);
                obstacle = true;
            }
            1 => self.terrain.set_at(droid_position + direction, Tile::Floor),
            2 => {
                self.terrain
                    .set_at(droid_position + direction, Tile::OxygenSystem);
                self.terrain.oxygen_system = Some(droid_position + direction);
            }
            _ => panic!("Unexpected status: {}", status),
        }

        // Continue exploring and then backtrack
        if !obstacle {
            if self.visualize {
                println!(
                    "Position is now: ({}, {})",
                    (droid_position + direction).x,
                    (droid_position + direction).y
                );
                self.terrain.draw(droid_position + direction);
                thread::sleep(DELAY);
            }

            self.explore(droid_position + direction);

            if self.visualize {
                println!("Backtracking...");
            }
            *self.computer.get_input() = Some(direction.reverse().into());

            let run_state = self.computer.resume();
            if run_state != RunState::NeedInput {
                panic!("Run state was {:?}", run_state);
            }

            // Consume status
            self.computer.get_output().take();
        }

        if self.visualize {
            println!(
                "Position is now: ({}, {})",
                droid_position.x, droid_position.y
            );
            self.terrain.draw(droid_position);
            thread::sleep(DELAY);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Add<Direction> for Position {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: Direction) -> Self {
        match other {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Wall,
    Floor,
    OxygenSystem,
}

struct Terrain {
    max_x: isize,
    min_x: isize,
    max_y: isize,
    min_y: isize,
    tiles: HashMap<Position, Tile>,
    oxygen_system: Option<Position>,
    distances: Vec<usize>,
}

impl Terrain {
    fn new() -> Self {
        Self {
            max_x: 0,
            min_x: 0,
            max_y: 0,
            min_y: 0,
            tiles: HashMap::new(),
            oxygen_system: None,
            distances: Vec::new(),
        }
    }

    fn at(&self, pos: Position) -> Option<Tile> {
        match self.tiles.get(&pos) {
            Some(&tile) => Some(tile),
            None => None,
        }
    }

    fn set_at(&mut self, pos: Position, t: Tile) {
        self.max_x = isize::max(self.max_x, pos.x);
        self.min_x = isize::min(self.min_x, pos.x);
        self.max_y = isize::max(self.max_y, pos.y);
        self.min_y = isize::min(self.min_y, pos.y);
        self.tiles.insert(pos, t);
    }

    fn calculate_distances_from_oxygen_system(&mut self) {
        let mut queue: VecDeque<(Position, usize)> = VecDeque::new();

        // Initialize distances
        self.distances.resize(
            ((self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)) as usize,
            usize::max_value(),
        );

        // BFS
        queue.push_back((
            self.oxygen_system
                .expect("Location of oxygen system unknown."),
            0,
        ));

        while let Some((position, distance)) = queue.pop_front() {
            self.add_neighbor_to_queue(&mut queue, position, Direction::North, distance + 1);
            self.add_neighbor_to_queue(&mut queue, position, Direction::South, distance + 1);
            self.add_neighbor_to_queue(&mut queue, position, Direction::West, distance + 1);
            self.add_neighbor_to_queue(&mut queue, position, Direction::East, distance + 1);
        }
    }

    fn distance(&self, position: Position) -> usize {
        self.distances[self.index(position)]
    }

    fn max_distance(&self) -> usize {
        if self.distances.is_empty() {
            0
        } else {
            *self
                .distances
                .iter()
                .filter(|&distance| *distance < usize::max_value())
                .max()
                .unwrap()
        }
    }

    fn add_neighbor_to_queue(
        &mut self,
        queue: &mut VecDeque<(Position, usize)>,
        position: Position,
        direction: Direction,
        distance: usize,
    ) {
        let neighbor = self.at(position + direction);
        if let Some(Tile::Floor) = neighbor {
            let index = self.index(position + direction);
            if self.distances[index] == usize::max_value() {
                queue.push_back((position + direction, distance));
                self.distances[index] = distance;
            }
        }
    }

    fn index(&self, position: Position) -> usize {
        ((position.y - self.min_y) * (self.max_x - self.min_x + 1) + (position.x - self.min_x))
            as usize
    }

    fn draw(&self, droid_position: Position) {
        println!("***************************************************\n");
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if x == droid_position.x && y == droid_position.y {
                    print!("D");
                    continue;
                }
                match self.tiles.get(&Position { x, y }) {
                    Some(&tile) => match tile {
                        Tile::Wall => print!("#"),
                        Tile::Floor => print!("."),
                        Tile::OxygenSystem => print!("O"),
                    },
                    None => print!(" "),
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let program: Vec<i64> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();

        let mut repair_droid = RepairDroid::new(&program, false);
        repair_droid.map_terrain();
        repair_droid
            .terrain
            .calculate_distances_from_oxygen_system();

        let distance_to_oxygen_system = repair_droid.terrain.distance(Position { x: 0, y: 0 });
        assert_eq!(212, distance_to_oxygen_system);
    }

    #[test]
    fn part_2() {
        let program: Vec<i64> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();

        let mut repair_droid = RepairDroid::new(&program, false);
        repair_droid.map_terrain();
        repair_droid
            .terrain
            .calculate_distances_from_oxygen_system();

        let max_distance = repair_droid.terrain.max_distance();
        assert_eq!(358, max_distance);
    }
}
