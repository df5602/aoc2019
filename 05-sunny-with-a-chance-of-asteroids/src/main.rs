use std::collections::VecDeque;
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

    let program: Vec<i32> = match FileReader::new().split_char(',').read_from_file(input_file) {
        Ok(input) => input,
        Err(e) => {
            println!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let mut program_input = VecDeque::new();
    program_input.push_back(1);
    let mut computer = Computer::new(&program, program_input);
    computer.run_program();
    for value in computer.output {
        println!("{}", value);
    }
}

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const HALT: u32 = 99;

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from(value: u32) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            mode @ _ => panic!("Invalid parameter mode: {}", mode),
        }
    }

    fn as_u32(self) -> u32 {
        match self {
            ParameterMode::Position => 0,
            ParameterMode::Immediate => 1,
        }
    }
}

enum Status {
    Continue,
    Terminate,
}

struct Computer {
    tape: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
    ip: usize,
    last_opcode: u32,
}

impl Computer {
    fn new(program: &[i32], input: VecDeque<i32>) -> Self {
        Self {
            tape: program.to_vec(),
            input,
            output: Vec::new(),
            ip: 0,
            last_opcode: 0,
        }
    }

    fn run_program(&mut self) {
        loop {
            if let Status::Terminate = self.execute_instruction() {
                break;
            }
            self.advance_instruction_pointer();
        }
    }

    fn advance_instruction_pointer(&mut self) {
        assert!(self.last_opcode != 0);
        self.ip += match self.last_opcode {
            ADD | MULTIPLY => 4,
            INPUT | OUTPUT => 2,
            _ => panic!("Could not advance IP, invalid opcode: {}", self.last_opcode),
        };
    }

    fn load_operand(&self, parameter: i32, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.tape[parameter as usize],
            ParameterMode::Immediate => parameter,
        }
    }

    fn execute_instruction(&mut self) -> Status {
        let mut parameter_modes = [ParameterMode::Position; 2];
        let instruction = self.tape[self.ip] as u32;

        let opcode = instruction % 100;
        self.last_opcode = opcode;

        parameter_modes[0] = ParameterMode::from(((1000 + instruction - opcode) % 1000) / 100);
        parameter_modes[1] = ParameterMode::from(
            ((10000 + instruction - opcode - parameter_modes[0].as_u32() * 100) % 10000) / 1000,
        );

        match opcode {
            ADD | MULTIPLY => {
                let a = self.load_operand(self.tape[self.ip + 1], parameter_modes[0]);
                let b = self.load_operand(self.tape[self.ip + 2], parameter_modes[1]);
                let output_pos = self.tape[self.ip + 3] as usize;
                self.tape[output_pos] = if opcode == ADD { a + b } else { a * b };
                Status::Continue
            }
            INPUT => {
                let input_value = self.input.pop_front();
                let input_value = match input_value {
                    Some(input_value) => input_value,
                    None => panic!("Input queue is empty! [ip: {}]", self.ip),
                };
                let output_pos = self.tape[self.ip + 1] as usize;
                self.tape[output_pos] = input_value;
                Status::Continue
            }
            OUTPUT => {
                let output_value = self.load_operand(self.tape[self.ip + 1], parameter_modes[0]);
                self.output.push(output_value);
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
        let program = vec![1, 0, 0, 0, 99];
        let mut computer = Computer::new(&program, VecDeque::new());
        computer.run_program();
        assert_eq!(vec![2, 0, 0, 0, 99], computer.tape);
    }

    #[test]
    fn example_program_2() {
        let program = vec![2, 3, 0, 3, 99];
        let mut computer = Computer::new(&program, VecDeque::new());
        computer.run_program();
        assert_eq!(vec![2, 3, 0, 6, 99], computer.tape);
    }

    #[test]
    fn example_program_3() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let mut computer = Computer::new(&program, VecDeque::new());
        computer.run_program();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.tape);
    }

    #[test]
    fn example_program_4() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = Computer::new(&program, VecDeque::new());
        computer.run_program();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.tape);
    }

    #[test]
    fn input_output() {
        let program = vec![3, 0, 4, 0, 99];
        let mut input = VecDeque::new();
        input.push_back(42);
        let mut computer = Computer::new(&program, input);
        computer.run_program();
        assert_eq!(vec![42], computer.output);
    }

    #[test]
    fn parameter_modes() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = Computer::new(&program, VecDeque::new());
        computer.run_program();
        assert_eq!(vec![1002, 4, 3, 4, 99], computer.tape);
    }
}
