use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::{thread, time};

use aoc_util::input::{FileReader, FromFile};
use intcode::{Computer, RunState};

const DELAY: std::time::Duration = time::Duration::from_millis(20);

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

    let mut game: Vec<i64> = match FileReader::new().split_char(',').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut arcade = ArcadeCabinet::new(&game);
    arcade.play(false);
    println!("Number of blocks: {}", arcade.block_count);

    game[0] = 2; // Insert two quarters
    let mut arcade = ArcadeCabinet::new(&game);
    arcade.play(false);
    println!("Final score: {}", arcade.score);
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
    ball_position: (usize, usize),
    paddle_position: (usize, usize),
    block_count: usize,
    score: usize,
}

impl ArcadeCabinet {
    fn new(game: &[i64]) -> Self {
        Self {
            game: Computer::new(0, game, VecDeque::new(), Vec::new()),
            screen_width: WIDTH,
            screen_height: HEIGHT,
            screen: vec![TileType::Empty; WIDTH * HEIGHT],
            ball_position: (0, 0),
            paddle_position: (0, 0),
            block_count: 0,
            score: 0,
        }
    }

    fn play(&mut self, visualize: bool) {
        let mut run_state = self.game.run_program();
        loop {
            match run_state {
                RunState::NotYetStarted => unreachable!(),
                RunState::NeedInput => {
                    // Update state
                    self.update_state();

                    // Decide on input
                    let input = match self.ball_position.0.cmp(&self.paddle_position.0) {
                        Ordering::Greater => 1,
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                    };

                    self.game.get_input().push_back(input);

                    // Draw screen
                    if visualize {
                        self.draw_screen();
                        println!();
                        thread::sleep(DELAY);
                    }
                }
                RunState::Stopped(_) => {
                    self.update_state();
                    if visualize {
                        self.draw_screen();
                        println!("STOPPED");
                    }
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
            // Update score
            if pixel[0] == -1 && pixel[1] == 0 {
                self.score = pixel[2] as usize;
                continue;
            }

            // Count blocks
            if pixel[2] == 2 {
                self.block_count += 1;
            }

            // Update paddle position
            if pixel[2] == 3 {
                self.paddle_position = (pixel[0] as usize, pixel[1] as usize);
            }

            // Update ball position
            if pixel[2] == 4 {
                self.ball_position = (pixel[0] as usize, pixel[1] as usize);
            }

            // Update tiles
            self.screen[pixel[1] as usize * self.screen_width + pixel[0] as usize] =
                TileType::from(pixel[2]);
        }
    }

    fn draw_screen(&self) {
        println!("+{:->42}", "+");
        println!("|SCORE:{:>35}|", self.score);
        println!("+{:->42}", "+");
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                match self.screen[y * self.screen_width + x] {
                    TileType::Empty => print!(" "),
                    TileType::Wall => print!("#"),
                    TileType::Block => print!("="),
                    TileType::Paddle => print!("-"),
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
        arcade.play(false);
        assert_eq!(284, arcade.block_count);
    }

    #[test]
    fn part_2() {
        let mut game: Vec<i64> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();
        game[0] = 2; // Insert two quarters
        let mut arcade = ArcadeCabinet::new(&game);
        arcade.play(false);
        assert_eq!(13581, arcade.score);
    }
}
