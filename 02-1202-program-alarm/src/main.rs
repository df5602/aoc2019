use std::env;

//use adhoc_derive::FromStr;
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

    let mut computer = Computer::new(&input);
    let result = computer.run_program(12, 2);
    println!("Result of program execution: {}", result);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = Computer::new(&input);
            let result = computer.run_program(noun, verb);
            if result == 19690720 {
                println!("Noun: {}, verb: {}", noun, verb);
            }
        }
    }
}

struct Computer {
    tape: Vec<u32>,
    pos: usize,
}

impl Computer {
    fn new(tape: &[u32]) -> Self {
        Self {
            tape: tape.to_vec(),
            pos: 0,
        }
    }

    fn run_program(&mut self, noun: u32, verb: u32) -> u32 {
        self.tape[1] = noun;
        self.tape[2] = verb;
        loop {
            let terminate = self.execute_instruction();
            if terminate {
                break;
            }
            self.advance_program_counter();
        }
        self.tape[0]
    }

    fn advance_program_counter(&mut self) {
        self.pos += 4;
    }

    fn execute_instruction(&mut self) -> bool {
        match self.tape[self.pos] {
            1 => {
                let a = self.tape[self.tape[self.pos + 1] as usize];
                let b = self.tape[self.tape[self.pos + 2] as usize];
                let output_pos = self.tape[self.pos + 3] as usize;
                self.tape[output_pos] = a + b;
                false
            }
            2 => {
                let a = self.tape[self.tape[self.pos + 1] as usize];
                let b = self.tape[self.tape[self.pos + 2] as usize];
                let output_pos = self.tape[self.pos + 3] as usize;
                self.tape[output_pos] = a * b;
                false
            }
            99 => true,
            _ => panic!(
                "Invalid opcode ({}) at position {}!",
                self.tape[self.pos], self.pos
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
}
