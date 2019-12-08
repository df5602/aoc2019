use std::env;

use aoc_util::input::{FileReader, FromFile};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

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

    let image = Image::new(&input);
    let counts = image.count_digits();

    let mut result = 0;
    let mut min_zeros = usize::max_value();
    for count in counts {
        if count.0 < min_zeros {
            result = count.1 * count.2;
            min_zeros = count.0;
        }
    }

    println!("1*2 on layer with smallest number of zeros: {}", result);
}

struct Image {
    layers: Vec<Vec<u32>>,
}

impl Image {
    fn new(data: &str) -> Self {
        let mut layers = Vec::new();
        let mut current_layer = Vec::new();

        for (i, ch) in data.char_indices() {
            current_layer.push(ch.to_digit(10).unwrap());
            if i % (WIDTH * HEIGHT) == WIDTH * HEIGHT - 1 {
                layers.push(current_layer);
                current_layer = Vec::new();
            }
        }

        Self { layers: layers }
    }

    fn count_digits(&self) -> Vec<(usize, usize, usize)> {
        let mut counts = Vec::new();

        for layer in &self.layers {
            let mut count = (0, 0, 0);
            for &digit in layer {
                match digit {
                    0 => count.0 += 1,
                    1 => count.1 += 1,
                    2 => count.2 += 1,
                    d => panic!("Unexpected digit: {}", d),
                }
            }
            counts.push(count);
        }

        counts
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
