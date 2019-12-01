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

    let input: Vec<u32> = match FileReader::new().split_lines().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let fuel_requirements: i32 = input
        .iter()
        .map(|&module_weight| calculate_fuel_weight(module_weight))
        .sum();
    println!("Total fuel requirement: {}", fuel_requirements);

    let including_fuel: u32 = input
        .iter()
        .map(|&module_weight| calculate_fuel_weight_including_fuel(module_weight))
        .sum();
    println!("Fuel requirements (including fuel): {}", including_fuel);
}

fn calculate_fuel_weight(module_weight: u32) -> i32 {
    module_weight as i32 / 3 - 2
}

fn calculate_fuel_weight_including_fuel(module_weight: u32) -> u32 {
    let mut total_fuel_weight = 0;
    let mut fuel_weight = module_weight as i32;
    loop {
        fuel_weight = calculate_fuel_weight(fuel_weight as u32);
        if fuel_weight <= 0 {
            return total_fuel_weight;
        } else {
            total_fuel_weight += fuel_weight as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_weight() {
        assert_eq!(2, calculate_fuel_weight(12));
        assert_eq!(2, calculate_fuel_weight(14));
        assert_eq!(654, calculate_fuel_weight(1969));
        assert_eq!(33583, calculate_fuel_weight(100756));
    }

    #[test]
    fn fuel_weight_including_fuel() {
        assert_eq!(2, calculate_fuel_weight_including_fuel(14));
        assert_eq!(966, calculate_fuel_weight_including_fuel(1969));
        assert_eq!(50346, calculate_fuel_weight_including_fuel(100756));
    }

    #[test]
    fn part_1() {
        let input: Vec<u32> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let fuel_requirements: i32 = input
            .iter()
            .map(|&module_weight| calculate_fuel_weight(module_weight))
            .sum();
        assert_eq!(3399394, fuel_requirements);
    }

    #[test]
    fn part_2() {
        let input: Vec<u32> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let including_fuel: u32 = input
            .iter()
            .map(|&module_weight| calculate_fuel_weight_including_fuel(module_weight))
            .sum();
        assert_eq!(5096223, including_fuel);
    }
}
