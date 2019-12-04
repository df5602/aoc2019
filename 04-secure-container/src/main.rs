use std::env;
use std::io::Write;

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

    let count = count_valid_passwords(min, max, true);
    println!("Different valid passwords: {}", count);

    let count = count_valid_passwords(min, max, false);
    println!("Different valid passwords (no large groups): {}", count);
}

fn count_valid_passwords(min: u32, max: u32, allow_larger_group: bool) -> usize {
    let range = Range::new(min, max, allow_larger_group);
    (min..=max).filter(|&pwd| range.check_valid(pwd)).count()
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
        // Check here to prevent buffer overflow when converting to digits
        if !Range::six_digits(password) {
            return false;
        }

        let mut digits = [0 as u8; 6];
        write!(&mut digits[..], "{}", password).unwrap();

        self.in_range(password)
            && self.check_adjacent_digits(&digits)
            && Range::never_decrease(&digits)
    }

    fn six_digits(password: u32) -> bool {
        password > 99999 && password < 1_000_000
    }

    fn in_range(&self, password: u32) -> bool {
        password >= self.min && password <= self.max
    }

    fn check_adjacent_digits(&self, password: &[u8]) -> bool {
        let mut previous = 0;
        let mut has_adjacent = false;
        let mut current_adjacent = 0;
        for &c in password.iter() {
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

    fn never_decrease(password: &[u8]) -> bool {
        let mut previous = 0;
        for &c in password.iter() {
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
        assert!(!range.check_valid(99999));
        assert!(range.check_valid(599999));
        assert!(range.check_valid(999999));
        assert!(!range.check_valid(1222222));
    }

    #[test]
    fn within_range() {
        let range = Range::new(234456, 456677, true);
        assert!(!range.check_valid(234455));
        assert!(range.check_valid(234456));
        assert!(range.check_valid(444444));
        assert!(range.check_valid(456677));
        assert!(!range.check_valid(456678));
    }

    #[test]
    fn adjacent_digits() {
        let range = Range::new(300000, 500000, true);
        assert!(!range.check_valid(345678));
        assert!(range.check_valid(344478));
        assert!(range.check_valid(344567));
    }

    #[test]
    fn never_decrease() {
        let range = Range::new(300000, 500000, true);
        assert!(!range.check_valid(432100));
        assert!(range.check_valid(444444));
        assert!(range.check_valid(455677));
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

    #[test]
    fn part_1() {
        let input: Vec<u32> = FileReader::new()
            .split_char('-')
            .read_from_file("input.txt")
            .unwrap();
        assert_eq!(2, input.len());

        let min = input[0];
        let max = input[1];
        let count = count_valid_passwords(min, max, true);
        assert_eq!(1694, count);
    }

    #[test]
    fn part_2() {
        let input: Vec<u32> = FileReader::new()
            .split_char('-')
            .read_from_file("input.txt")
            .unwrap();
        assert_eq!(2, input.len());

        let min = input[0];
        let max = input[1];
        let count = count_valid_passwords(min, max, false);
        assert_eq!(1148, count);
    }
}
