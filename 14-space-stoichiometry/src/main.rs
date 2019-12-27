use std::collections::{HashMap, VecDeque};
use std::env;
use std::str::FromStr;

use adhoc_derive::FromStr;
use aoc_util::input::{FileReader, FromFile};

const ONE_TRILLION: u64 = 1_000_000_000_000;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let reactions: Vec<Reaction> = match FileReader::new().split_lines().read_from_file(input_file)
    {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let reactions = convert_to_map(reactions);

    let amount_of_ore = calculate_ore_requirements(&reactions, 1);
    println!("Amount of ore required: {}", amount_of_ore);

    let amount_of_fuel = calculate_max_fuel(&reactions, ONE_TRILLION);
    println!(
        "Maximum amount of fuel given 1 trillion ore: {}",
        amount_of_fuel
    );
}

fn calculate_max_fuel(reactions: &HashMap<String, Reaction>, ore_quantity: u64) -> u64 {
    let mut upper_bound = 1;

    // Find upper bound
    loop {
        let ore = calculate_ore_requirements(&reactions, upper_bound);
        if ore < ore_quantity {
            upper_bound *= 10;
        } else {
            break;
        }
    }

    let mut lower_bound = upper_bound / 10;

    // Binary search
    loop {
        let current_fuel = (lower_bound + upper_bound) / 2;
        let ore = calculate_ore_requirements(&reactions, current_fuel);
        if ore > ore_quantity {
            upper_bound = current_fuel;
        } else {
            lower_bound = current_fuel;
        }

        if lower_bound + 1 == upper_bound {
            break;
        }
    }

    lower_bound
}

fn calculate_ore_requirements(reactions: &HashMap<String, Reaction>, fuel_quantity: u64) -> u64 {
    let mut inventory: HashMap<String, u64> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(Material {
        quantity: fuel_quantity,
        name: String::from("FUEL"),
    });

    let mut amount_of_ore = 0;

    while let Some(material) = queue.pop_front() {
        if material.name == "ORE" {
            amount_of_ore += material.quantity;
            continue;
        }

        let reaction = reactions
            .get(&material.name)
            .expect("Required reaction not present!");

        let mut required_quantity = material.quantity;
        let surplus = inventory.get_mut(&material.name);

        if let Some(surplus) = surplus {
            if required_quantity >= *surplus {
                required_quantity -= *surplus;
                *surplus = 0;
            } else {
                *surplus -= required_quantity;
                required_quantity = 0;
            }
        }

        let multiplier = required_quantity / reaction.product.quantity
            + if required_quantity % reaction.product.quantity != 0 {
                1
            } else {
                0
            };

        let surplus = reaction.product.quantity * multiplier - required_quantity;
        if surplus > 0 {
            *inventory.entry(material.name.clone()).or_insert(0) += surplus;
        }

        for ingredient in &reaction.requirements {
            let mut required_quantity = ingredient.quantity * multiplier;
            let surplus = inventory.get_mut(&ingredient.name);

            if let Some(surplus) = surplus {
                if required_quantity >= *surplus {
                    required_quantity -= *surplus;
                    *surplus = 0;
                } else {
                    *surplus -= required_quantity;
                    required_quantity = 0;
                }
            }

            if required_quantity > 0 {
                queue.push_back(Material {
                    quantity: required_quantity,
                    name: ingredient.name.clone(),
                });
            }
        }
    }

    amount_of_ore
}

#[derive(Debug, FromStr)]
#[adhoc(regex = r"^(?P<quantity>\d+) (?P<name>.+)$")]
struct Material {
    quantity: u64,
    name: String,
}

fn parse_requirements(requirements: &str) -> Result<Vec<Material>, Box<dyn std::error::Error>> {
    requirements
        .split(',')
        .map(|ingredient| Material::from_str(ingredient.trim()))
        .collect()
}

#[derive(FromStr, Debug)]
#[adhoc(regex = r"^(?P<requirements>\d+ .+(, \d+ .+)*) => (?P<product>\d+ .+)$")]
struct Reaction {
    #[adhoc(construct_with = "parse_requirements(requirements: &str)?")]
    requirements: Vec<Material>,
    product: Material,
}

fn convert_to_map(mut reactions: Vec<Reaction>) -> HashMap<String, Reaction> {
    let mut map = HashMap::new();

    for reaction in reactions.drain(..) {
        let product_name = reaction.product.name.clone();
        let previous = map.insert(product_name, reaction);
        assert!(previous.is_none());
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ore_example_1() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_1.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_ore = calculate_ore_requirements(&reactions, 1);
        assert_eq!(31, amount_of_ore);
    }

    #[test]
    fn ore_example_2() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_2.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_ore = calculate_ore_requirements(&reactions, 1);
        assert_eq!(165, amount_of_ore);
    }

    #[test]
    fn ore_example_3() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_3.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_ore = calculate_ore_requirements(&reactions, 1);
        assert_eq!(13312, amount_of_ore);
    }

    #[test]
    fn ore_example_4() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_4.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_ore = calculate_ore_requirements(&reactions, 1);
        assert_eq!(180697, amount_of_ore);
    }

    #[test]
    fn ore_example_5() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_5.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_ore = calculate_ore_requirements(&reactions, 1);
        assert_eq!(2210736, amount_of_ore);
    }

    #[test]
    fn fuel_example_3() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_3.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_fuel = calculate_max_fuel(&reactions, ONE_TRILLION);
        assert_eq!(82892753, amount_of_fuel);
    }

    #[test]
    fn fuel_example_4() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_4.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_fuel = calculate_max_fuel(&reactions, ONE_TRILLION);
        assert_eq!(5586022, amount_of_fuel);
    }

    #[test]
    fn fuel_example_5() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("example_5.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_fuel = calculate_max_fuel(&reactions, ONE_TRILLION);
        assert_eq!(460664, amount_of_fuel);
    }

    #[test]
    fn part_1() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_ore = calculate_ore_requirements(&reactions, 1);
        assert_eq!(443537, amount_of_ore);
    }

    #[test]
    fn part_2() {
        let reactions: Vec<Reaction> = FileReader::new()
            .split_lines()
            .read_from_file("input.txt")
            .unwrap();
        let reactions = convert_to_map(reactions);
        let amount_of_fuel = calculate_max_fuel(&reactions, ONE_TRILLION);
        assert_eq!(2910558, amount_of_fuel);
    }
}
