use std::env;

use adhoc_derive::FromStr;
use aoc_util::input::{FileReader, FromFile};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let moons: Vec<Moon> = match FileReader::new().split_lines().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut simulator = OrbitSimulator::new(&moons);
    simulator.simulate(1000);
    let total_energy = simulator.calculate_total_energy();
    println!("Total energy in system: {}", total_energy);
}

#[derive(Copy, Clone, Debug, FromStr)]
#[adhoc(regex = r"^<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>$")]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    #[adhoc(construct_with = "0")]
    dx: isize,
    #[adhoc(construct_with = "0")]
    dy: isize,
    #[adhoc(construct_with = "0")]
    dz: isize,
}

struct OrbitSimulator {
    moons: Vec<Moon>,
}

impl OrbitSimulator {
    fn new(moons: &[Moon]) -> Self {
        Self {
            moons: moons.to_vec(),
        }
    }

    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.simulate_step();
        }
    }

    fn simulate_step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn apply_gravity(&mut self) {
        for a in 0..self.moons.len() {
            for b in 0..self.moons.len() {
                let dx = (self.moons[b].x - self.moons[a].x).signum();
                let dy = (self.moons[b].y - self.moons[a].y).signum();
                let dz = (self.moons[b].z - self.moons[a].z).signum();
                self.moons[a].dx += dx;
                self.moons[a].dy += dy;
                self.moons[a].dz += dz;
            }
        }
    }

    fn apply_velocity(&mut self) {
        for moon in &mut self.moons {
            moon.x += moon.dx;
            moon.y += moon.dy;
            moon.z += moon.dz;
        }
    }

    fn calculate_total_energy(&self) -> u64 {
        self.moons
            .iter()
            .map(|moon| {
                let epot = (moon.x.abs() + moon.y.abs() + moon.z.abs()) as u64;
                let ekin = (moon.dx.abs() + moon.dy.abs() + moon.dz.abs()) as u64;
                epot * ekin
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let moons: Vec<Moon> = FileReader::new()
            .split_lines()
            .read_from_file("example1.txt")
            .unwrap();
        let mut simulator = OrbitSimulator::new(&moons);
        simulator.simulate(10);
        let total_energy = simulator.calculate_total_energy();
        assert_eq!(179, total_energy);
    }

    #[test]
    fn example_2() {
        let moons: Vec<Moon> = FileReader::new()
            .split_lines()
            .read_from_file("example2.txt")
            .unwrap();
        let mut simulator = OrbitSimulator::new(&moons);
        simulator.simulate(100);
        let total_energy = simulator.calculate_total_energy();
        assert_eq!(1940, total_energy);
    }

    #[test]
    fn part_1() {
        let moons: Vec<Moon> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let mut simulator = OrbitSimulator::new(&moons);
        simulator.simulate(1000);
        let total_energy = simulator.calculate_total_energy();
        assert_eq!(12466, total_energy);
    }
}
