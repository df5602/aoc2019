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

    let module_masses: Vec<u32> = match FileReader::new().split_lines().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let fuel_requirements = fuel_requirements(&module_masses);
    println!("Fuel requirement: {}", fuel_requirements);

    let including_fuel = fuel_requirements_refined(&module_masses);
    println!("Fuel requirements (including fuel): {}", including_fuel);
}

fn calculate_fuel(mass: u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

fn calculate_fuel_refined(mass: u32) -> u32 {
    std::iter::successors(Some(mass), |&mass| calculate_fuel(mass))
        .skip(1)
        .sum()
}

fn fuel_requirements(module_masses: &[u32]) -> u32 {
    module_masses
        .iter()
        .map(|&mass| calculate_fuel(mass).unwrap())
        .sum()
}

fn fuel_requirements_refined(module_masses: &[u32]) -> u32 {
    module_masses
        .iter()
        .map(|&mass| calculate_fuel_refined(mass))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_weight() {
        assert_eq!(Some(2), calculate_fuel(12));
        assert_eq!(Some(2), calculate_fuel(14));
        assert_eq!(Some(654), calculate_fuel(1969));
        assert_eq!(Some(33583), calculate_fuel(100756));
    }

    #[test]
    fn fuel_weight_including_fuel() {
        assert_eq!(2, calculate_fuel_refined(14));
        assert_eq!(966, calculate_fuel_refined(1969));
        assert_eq!(50346, calculate_fuel_refined(100756));
    }

    #[test]
    fn part_1() {
        let input: Vec<u32> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let fuel_requirements = fuel_requirements(&input);
        assert_eq!(3399394, fuel_requirements);
    }

    #[test]
    fn part_2() {
        let input: Vec<u32> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let including_fuel = fuel_requirements_refined(&input);
        assert_eq!(5096223, including_fuel);
    }
}
