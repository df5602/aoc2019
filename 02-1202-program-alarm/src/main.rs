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

    let input: Vec<u32> = match FileReader::new().split_char(',').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let result = run_program(&input, 12, 2);
    println!("Result of program execution (1202): {}", result);

    let output = 19_690_720;
    let inputs = find_output(&input, output);
    if let Some((a, b)) = inputs {
        println!("Input to create output {}: {}", output, a * 100 + b)
    } else {
        println!("No inputs found that create desired output.");
    }
}

fn run_program(input: &[u32], noun: u32, verb: u32) -> u32 {
    let mut computer = Computer::new(&input);
    computer.run_program(noun, verb)
}

fn find_output(input: &[u32], output: u32) -> Option<(u32, u32)> {
    for noun in 0..100 {
        for verb in 0..100 {
            let result = run_program(&input, noun, verb);
            if result == output {
                return Some((noun, verb));
            }
        }
    }
    None
}

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const HALT: u32 = 99;

enum Status {
    Continue,
    Terminate,
}

struct Computer {
    tape: Vec<u32>,
    ip: usize,
}

impl Computer {
    fn new(tape: &[u32]) -> Self {
        Self {
            tape: tape.to_vec(),
            ip: 0,
        }
    }

    fn run_program(&mut self, noun: u32, verb: u32) -> u32 {
        self.tape[1] = noun;
        self.tape[2] = verb;
        loop {
            if let Status::Terminate = self.execute_instruction() {
                break;
            }
            self.advance_program_counter();
        }
        self.tape[0]
    }

    fn advance_program_counter(&mut self) {
        self.ip += 4;
    }

    fn execute_instruction(&mut self) -> Status {
        match self.tape[self.ip] {
            opcode @ ADD | opcode @ MULTIPLY => {
                let a = self.tape[self.tape[self.ip + 1] as usize];
                let b = self.tape[self.tape[self.ip + 2] as usize];
                let output_pos = self.tape[self.ip + 3] as usize;
                self.tape[output_pos] = if opcode == ADD { a + b } else { a * b };
                Status::Continue
            }
            HALT => Status::Terminate,
            _ => panic!(
                "Invalid opcode ({}) at position {}!",
                self.tape[self.ip], self.ip
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_program_1() {
        let input = vec![1, 0, 0, 0, 99];
        let mut computer = Computer::new(&input);
        computer.run_program(0, 0);
        assert_eq!(vec![2, 0, 0, 0, 99], computer.tape);
    }

    #[test]
    fn example_program_2() {
        let input = vec![2, 3, 0, 3, 99];
        let mut computer = Computer::new(&input);
        computer.run_program(3, 0);
        assert_eq!(vec![2, 3, 0, 6, 99], computer.tape);
    }

    #[test]
    fn example_program_3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let mut computer = Computer::new(&input);
        computer.run_program(4, 4);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.tape);
    }

    #[test]
    fn example_program_4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = Computer::new(&input);
        computer.run_program(1, 1);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.tape);
    }

    #[test]
    fn part_1() {
        let input: Vec<u32> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();
        assert_eq!(4945026, run_program(&input, 12, 2));
    }

    #[test]
    fn part_2() {
        let input: Vec<u32> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();
        let inputs = find_output(&input, 19690720).unwrap();
        assert_eq!(5296, inputs.0 * 100 + inputs.1);
    }
}
