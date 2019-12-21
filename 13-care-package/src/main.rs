use std::collections::VecDeque;
use std::env;

use aoc_util::input::{FileReader, FromFile};
use intcode::{Computer, RunState};

const WIDTH: usize = 43;
const HEIGHT: usize = 21;

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let game: Vec<i64> = match FileReader::new().split_char(',').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut game = Computer::new(0, &game, VecDeque::new(), Vec::new());
    match game.run_program() {
        RunState::NotYetStarted => unreachable!(),
        RunState::NeedInput => println!("NEED INPUT"),
        RunState::Stopped(_) => println!("STOPPED"),
    }

    let output = game.get_output();
    let mut screen = vec![0; output.len()];
    let mut block_count = 0;
    for pixel in output.chunks_exact(3) {
        if pixel[2] == 2 {
            block_count += 1;
        }
        screen[pixel[1] as usize * WIDTH + pixel[0] as usize] = pixel[2];
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            match screen[y * WIDTH + x] {
                0 => print!(" "),
                1 => print!("#"),
                2 => print!("="),
                3 => print!("_"),
                4 => print!("o"),
                t => panic!("Invalid tile type: {}", t),
            }
        }
        println!();
    }

    println!("Number of blocks: {}", block_count);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     assert!(1 < 2);
    // }
}
