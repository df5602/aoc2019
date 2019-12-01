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

    let fuel_requirements: u32 = input
        .iter()
        .map(|&module_weight| calculate_fuel_weight(module_weight))
        .sum();
    println!("Total fuel requirement: {}", fuel_requirements);
}

fn calculate_fuel_weight(module_weight: u32) -> u32 {
    module_weight / 3 - 2
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
}
