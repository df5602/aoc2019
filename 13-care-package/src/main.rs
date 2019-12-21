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

    let mut arcade = ArcadeCabinet::new(&game);
    arcade.play();

    println!("Number of blocks: {}", arcade.block_count);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for TileType {
    fn from(value: i64) -> Self {
        match value {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            val => panic!("Invalid tile type: {}", val),
        }
    }
}

struct ArcadeCabinet {
    game: Computer<VecDeque<i64>, Vec<i64>>,
    screen_width: usize,
    screen_height: usize,
    screen: Vec<TileType>,
    block_count: usize,
}

impl ArcadeCabinet {
    fn new(game: &[i64]) -> Self {
        Self {
            game: Computer::new(0, game, VecDeque::new(), Vec::new()),
            screen_width: WIDTH,
            screen_height: HEIGHT,
            screen: vec![TileType::Empty; WIDTH * HEIGHT],
            block_count: 0,
        }
    }

    fn play(&mut self) {
        let mut run_state = self.game.run_program();
        loop {
            match run_state {
                RunState::NotYetStarted => unreachable!(),
                RunState::NeedInput => {
                    println!("NEED INPUT");
                    self.update_state();
                    self.draw_screen();
                    break;
                }
                RunState::Stopped(_) => {
                    println!("STOPPED");
                    self.update_state();
                    self.draw_screen();
                    break;
                }
            }

            self.game.get_output().clear();
            run_state = self.game.resume();
        }
    }

    fn update_state(&mut self) {
        let output = self.game.get_output();
        self.block_count = 0;
        for pixel in output.chunks_exact(3) {
            if pixel[2] == 2 {
                self.block_count += 1;
            }
            self.screen[pixel[1] as usize * self.screen_width + pixel[0] as usize] =
                TileType::from(pixel[2]);
        }
    }

    fn draw_screen(&self) {
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                match self.screen[y * self.screen_width + x] {
                    TileType::Empty => print!(" "),
                    TileType::Wall => print!("#"),
                    TileType::Block => print!("="),
                    TileType::Paddle => print!("_"),
                    TileType::Ball => print!("o"),
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let game: Vec<i64> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();
        let mut arcade = ArcadeCabinet::new(&game);
        arcade.play();
        assert_eq!(284, arcade.block_count);
    }
}
