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

    let input: Vec<u32> = match FileReader::new().split_char('-').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    assert_eq!(2, input.len());
    let min = input[0];
    let max = input[1];

    let range = Range::new(min, max, true);
    let count = (min..=max).filter(|&pwd| range.check_valid(pwd)).count();
    println!("Different valid passwords: {}", count);

    let range = Range::new(min, max, false);
    let count = (min..=max).filter(|&pwd| range.check_valid(pwd)).count();
    println!("Different valid passwords (no large groups): {}", count);
}

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
    allow_larger_group: bool,
}

impl Range {
    fn new(min: u32, max: u32, allow_larger_group: bool) -> Self {
        Self {
            min,
            max,
            allow_larger_group,
        }
    }

    fn check_valid(&self, password: u32) -> bool {
        let pwd_as_string = password.to_string();

        Range::six_digits(password)
            && self.in_range(password)
            && self.check_adjacent_digits(&pwd_as_string)
            && Range::never_decrease(&pwd_as_string)
    }

    fn six_digits(password: u32) -> bool {
        password > 9999 && password < 1_000_000
    }

    fn in_range(&self, password: u32) -> bool {
        password >= self.min && password <= self.max
    }

    fn check_adjacent_digits(&self, password: &str) -> bool {
        let mut previous = ' ';
        let mut has_adjacent = false;
        let mut current_adjacent = 0;
        for c in password.chars() {
            if c == previous {
                current_adjacent += 1;
                has_adjacent = true;
            } else {
                if current_adjacent == 1 {
                    return true;
                }
                current_adjacent = 0;
            }
            previous = c;
        }
        self.allow_larger_group && has_adjacent || current_adjacent == 1
    }

    fn never_decrease(password: &str) -> bool {
        let mut previous = '0';
        for c in password.chars() {
            if c < previous {
                return false;
            }
            previous = c;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_digit_number() {
        let range = Range::new(0, u32::max_value(), true);
        assert!(!range.check_valid(9999));
        assert!(range.check_valid(59999));
        assert!(range.check_valid(999999));
        assert!(!range.check_valid(1222222));
    }

    #[test]
    fn within_range() {
        let range = Range::new(23445, 45667, true);
        assert!(!range.check_valid(23444));
        assert!(range.check_valid(23445));
        assert!(range.check_valid(44444));
        assert!(range.check_valid(45667));
        assert!(!range.check_valid(45668));
    }

    #[test]
    fn adjacent_digits() {
        let range = Range::new(30000, 50000, true);
        assert!(!range.check_valid(34567));
        assert!(range.check_valid(34447));
        assert!(range.check_valid(34456));
    }

    #[test]
    fn never_decrease() {
        let range = Range::new(30000, 50000, true);
        assert!(!range.check_valid(43210));
        assert!(range.check_valid(44444));
        assert!(range.check_valid(45567));
    }

    #[test]
    fn examples() {
        let range = Range::new(0, u32::max_value(), true);
        assert!(range.check_valid(111111));
        assert!(!range.check_valid(223450));
        assert!(!range.check_valid(123789));
    }

    #[test]
    fn larger_groups() {
        let range = Range::new(0, u32::max_value(), false);
        assert!(range.check_valid(112233));
        assert!(!range.check_valid(123444));
        assert!(range.check_valid(111122));
    }
}
