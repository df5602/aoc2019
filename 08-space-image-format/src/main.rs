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

    let image = RawImage::new(&input, WIDTH, HEIGHT);
    let counts = image.count_colors_in_layers();

    let result = product_of_layer_with_minimum_number_of_zeros(&counts);
    println!("1*2 on layer with smallest number of zeros: {}", result);

    let stacked_image = image.stack_layers();
    stacked_image.draw();
}

fn product_of_layer_with_minimum_number_of_zeros(counts: &[(usize, usize, usize)]) -> usize {
    counts
        .iter()
        .map(|(zeros, ones, twos)| (zeros, ones * twos))
        .min_by_key(|(&zeros, _)| zeros)
        .map(|(_, product)| product)
        .unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Black,
    White,
    Transparent,
}

impl From<char> for Color {
    fn from(ch: char) -> Self {
        match ch.to_digit(10).unwrap() {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            ch => panic!("Invalid character: {}", ch),
        }
    }
}

struct RawImage {
    layers: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl RawImage {
    fn new(data: &str, width: usize, height: usize) -> Self {
        let mut layers = Vec::new();
        let mut current_layer = Vec::new();

        for (i, ch) in data.char_indices() {
            current_layer.push(Color::from(ch));
            if i % (width * height) == width * height - 1 {
                layers.push(current_layer);
                current_layer = Vec::new();
            }
        }

        Self {
            layers,
            width,
            height,
        }
    }

    fn count_colors_in_layers(&self) -> Vec<(usize, usize, usize)> {
        let mut counts = Vec::new();

        for layer in &self.layers {
            let mut count = (0, 0, 0);
            for &digit in layer {
                match digit {
                    Color::Black => count.0 += 1,
                    Color::White => count.1 += 1,
                    Color::Transparent => count.2 += 1,
                }
            }
            counts.push(count);
        }

        counts
    }

    fn stack_layers(&self) -> StackedImage {
        let mut stacked = vec![Color::Transparent; self.width * self.height];

        for layer in &self.layers {
            for (i, &digit) in layer.iter().enumerate() {
                stacked[i] = match (digit, stacked[i]) {
                    (_, Color::Black) => Color::Black,
                    (_, Color::White) => Color::White,
                    (d, Color::Transparent) => d,
                };
            }
        }

        StackedImage {
            image: stacked,
            width: self.width,
        }
    }
}

struct StackedImage {
    image: Vec<Color>,
    width: usize,
}

impl StackedImage {
    fn draw(&self) {
        for (i, &pixel) in self.image.iter().enumerate() {
            match pixel {
                Color::Black => print!("\u{2588}"),
                Color::White => print!("\u{2591}"),
                Color::Transparent => print!("."),
            }
            if i % self.width == self.width - 1 {
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input: String = FileReader::new().read_from_file("input.txt").unwrap();
        let image = RawImage::new(&input, WIDTH, HEIGHT);
        let counts = image.count_colors_in_layers();
        let result = product_of_layer_with_minimum_number_of_zeros(&counts);
        assert_eq!(1340, result);
    }

    #[test]
    fn part_2() {
        let correct_stacked_image = vec![
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::Black,
            Color::White,
            Color::White,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::Black,
            Color::Black,
            Color::White,
            Color::White,
            Color::Black,
            Color::Black,
        ];

        let input: String = FileReader::new().read_from_file("input.txt").unwrap();
        let image = RawImage::new(&input, WIDTH, HEIGHT);
        let stacked_image = image.stack_layers();
        assert_eq!(correct_stacked_image, stacked_image.image);
    }
}
