use std::env;

use aoc_util::input::{FileReader, FromFile};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let input: String = match FileReader::new().read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let numbers = convert_to_vec(&input);

    let numbers = run_n_phases(numbers, 100, false);

    print!("Result: ");
    for &number in &numbers[..8] {
        print!("{}", number);
    }
    println!("\n");

    let offset: usize = input[..7].parse().unwrap();
    let real_input = input.repeat(10000);
    let numbers = convert_to_vec(&real_input);

    let numbers = run_n_phases(numbers, 100, true);

    print!("Result: ");
    for &number in &numbers[offset..offset + 8] {
        print!("{}", number);
    }
    println!();
}

fn convert_to_vec(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("Input is not a number.") as u8)
        .collect()
}

fn run_n_phases(input_list: Vec<u8>, n: usize, show_progress: bool) -> Vec<u8> {
    let mut numbers = input_list;

    let progress = if show_progress {
        let progress = ProgressBar::new(n as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("Calculating: {bar:40.cyan/blue} {pos}/{len}")
                .progress_chars("##-"),
        );
        Some(progress)
    } else {
        None
    };

    for _ in 0..n {
        if show_progress {
            progress.as_ref().unwrap().inc(1);
        }
        numbers = calculate_next_phase(numbers);
    }
    if show_progress {
        progress.unwrap().finish();
    }

    numbers
}

fn calculate_next_phase(input_list: Vec<u8>) -> Vec<u8> {
    let mut output_list = Vec::with_capacity(input_list.len());
    let mut partial_sums: Vec<i32> = Vec::with_capacity(input_list.len() + 1);

    // Calculate partial sums of input list
    partial_sums.push(0);
    for (i, &number) in input_list.iter().enumerate() {
        let previous = partial_sums[i];
        partial_sums.push(previous + number as i32);
    }

    // Calculate next phase
    for i in 0..input_list.len() {
        let mut sign = 1i32;
        let mut sum = 0;
        for j in (i..input_list.len()).step_by((i + 1) * 2) {
            sum += sign
                * (partial_sums[usize::min(j + i + 1, partial_sums.len() - 1)] - partial_sums[j]);
            sign *= -1;
        }
        output_list.push((sum % 10).abs() as u8);
    }

    output_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let numbers = calculate_next_phase(numbers);
        assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8], numbers);

        let numbers = calculate_next_phase(numbers);
        assert_eq!(vec![3, 4, 0, 4, 0, 4, 3, 8], numbers);

        let numbers = calculate_next_phase(numbers);
        assert_eq!(vec![0, 3, 4, 1, 5, 5, 1, 8], numbers);

        let numbers = calculate_next_phase(numbers);
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8], numbers);
    }

    #[test]
    fn example_1() {
        let input = "80871224585914546619083218645595";
        let numbers: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).expect("Input is not a number.") as u8)
            .collect();

        let numbers = run_n_phases(numbers, 100, false);
        assert_eq!(&[2, 4, 1, 7, 6, 1, 7, 6], &numbers[..8]);
    }

    #[test]
    fn example_2() {
        let input = "19617804207202209144916044189917";
        let numbers: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).expect("Input is not a number.") as u8)
            .collect();

        let numbers = run_n_phases(numbers, 100, false);
        assert_eq!(&[7, 3, 7, 4, 5, 4, 1, 8], &numbers[..8]);
    }

    #[test]
    fn example_3() {
        let input = "69317163492948606335995924319873";
        let numbers: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).expect("Input is not a number.") as u8)
            .collect();

        let numbers = run_n_phases(numbers, 100, false);
        assert_eq!(&[5, 2, 4, 3, 2, 1, 3, 3], &numbers[..8]);
    }

    #[test]
    fn part_1() {
        let input: String = FileReader::new().read_from_file("input.txt").unwrap();
        let numbers = convert_to_vec(&input);
        let numbers = run_n_phases(numbers, 100, false);
        assert_eq!(&[1, 1, 8, 3, 3, 1, 8, 8], &numbers[..8]);
    }
}
