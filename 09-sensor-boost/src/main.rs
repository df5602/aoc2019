use std::collections::VecDeque;
use std::env;

use aoc_util::input::{FileReader, FromFile};

macro_rules! queue {
    ($($x:expr),*) => {
        {
            let mut q = VecDeque::new();
            $(q.push_back($x);)*
            q
        }
    };
}

fn main() {
    let input_file = match env::args().nth(1) {
        Some(input_file) => input_file,
        None => {
            println!("Please supply input file!");
            std::process::exit(1);
        }
    };

    let program: Vec<MemoryType> =
        match FileReader::new().split_char(',').read_from_file(input_file) {
            Ok(input) => input,
            Err(e) => {
                println!("Error reading input: {}", e);
                std::process::exit(1);
            }
        };

    let mut computer = Computer::new(0, &program, queue![1], Vec::new());
    computer.run_program();
    println!("{:?}", computer.output);

    let mut computer = Computer::new(0, &program, queue![2], Vec::new());
    computer.run_program();
    println!("{:?}", computer.output);
}

trait Input<T> {
    type ReadError;
    // Blocking read.
    fn read(&mut self) -> Result<T, Self::ReadError>;
}

impl<T> Input<T> for VecDeque<T> {
    type ReadError = String;

    fn read(&mut self) -> Result<T, Self::ReadError> {
        match self.pop_front() {
            Some(t) => Ok(t),
            None => Err(String::from("Queue is empty.")),
        }
    }
}

trait Output<T> {
    type WriteError;
    // Blocking write.
    fn write(&mut self, t: T) -> Result<(), Self::WriteError>;
}

impl<T> Output<T> for Vec<T> {
    type WriteError = ();

    fn write(&mut self, t: T) -> Result<(), Self::WriteError> {
        self.push(t);
        Ok(())
    }
}

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const JUMP_IF_TRUE: u32 = 5;
const JUMP_IF_FALSE: u32 = 6;
const LESS_THAN: u32 = 7;
const EQUALS: u32 = 8;
const RELATIVE_BASE_OFFSET: u32 = 9;
const HALT: u32 = 99;

type MemoryType = i64;

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<u32> for ParameterMode {
    fn from(value: u32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            mode => panic!("Invalid parameter mode: {}", mode),
        }
    }
}

enum NextState {
    ContinueAbsolute(usize),
    ContinueRelative(isize),
    Terminate,
}

const MAX_MEMORY: usize = 1024 * 1024;

struct Computer<I: Input<MemoryType>, O: Output<MemoryType>> {
    _id: usize,
    tape: Vec<MemoryType>,
    input: I,
    output: O,
    last_output: MemoryType,
    ip: usize,
    relative_base: MemoryType,
}

impl<I: Input<MemoryType>, O: Output<MemoryType>> Computer<I, O>
where
    I::ReadError: std::fmt::Debug,
{
    fn new(id: usize, program: &[MemoryType], input: I, output: O) -> Self {
        Self {
            _id: id,
            tape: program.to_vec(),
            input,
            output,
            last_output: 0,
            ip: 0,
            relative_base: 0,
        }
    }

    fn run_program(&mut self) -> MemoryType {
        loop {
            match self.execute_instruction() {
                NextState::ContinueAbsolute(offset) => self.ip = offset,
                NextState::ContinueRelative(offset) => {
                    self.ip = (self.ip as isize + offset) as usize
                }
                NextState::Terminate => break,
            }
        }
        self.last_output
    }

    fn load(&self, address: usize) -> MemoryType {
        if address < self.tape.len() {
            self.tape[address]
        } else {
            0
        }
    }

    fn store(&mut self, address: usize, value: MemoryType) {
        if address >= self.tape.len() {
            if address < MAX_MEMORY {
                self.tape.resize(address + 1, 0);
            } else {
                panic!(
                    "Attempt to resize beyond memory limit [request: {}, limit: {}]",
                    address, MAX_MEMORY
                );
            }
        }
        self.tape[address] = value;
    }

    fn load_operand(&self, offset: usize, mode: ParameterMode) -> MemoryType {
        match mode {
            ParameterMode::Position => self.load(self.load(offset) as usize),
            ParameterMode::Immediate => self.load(offset),
            ParameterMode::Relative => {
                self.load((self.load(offset) as MemoryType + self.relative_base) as usize)
            }
        }
    }

    fn store_operand(&mut self, offset: usize, mode: ParameterMode, value: MemoryType) {
        let output_pos = match mode {
            ParameterMode::Position => self.load(offset) as usize,
            ParameterMode::Relative => {
                (self.load(offset) as MemoryType + self.relative_base) as usize
            }
            ParameterMode::Immediate => {
                panic!("Write to immediate not allowed!");
            }
        };
        self.store(output_pos, value);
    }

    fn should_jump(condition: MemoryType, opcode: u32) -> bool {
        match opcode {
            JUMP_IF_TRUE => condition != 0,
            JUMP_IF_FALSE => condition == 0,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    fn operation(a: MemoryType, b: MemoryType, opcode: u32) -> MemoryType {
        match opcode {
            ADD => a + b,
            MULTIPLY => a * b,
            LESS_THAN => (a < b) as MemoryType,
            EQUALS => (a == b) as MemoryType,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    fn execute_instruction(&mut self) -> NextState {
        let instruction = self.load(self.ip) as u32;
        let opcode = instruction % 100;
        let mut modes = [ParameterMode::Position; 3];
        modes[0] = ParameterMode::from((instruction / 100) % 10);
        modes[1] = ParameterMode::from((instruction / 1000) % 10);
        modes[2] = ParameterMode::from((instruction / 10000) % 10);

        match opcode {
            ADD | MULTIPLY | LESS_THAN | EQUALS => {
                let a = self.load_operand(self.ip + 1, modes[0]);
                let b = self.load_operand(self.ip + 2, modes[1]);
                self.store_operand(self.ip + 3, modes[2], Self::operation(a, b, opcode));
                NextState::ContinueRelative(4)
            }
            INPUT => {
                let input_value = self.input.read();
                let input_value = match input_value {
                    Ok(input_value) => input_value,
                    Err(e) => panic!("Error receiving input: {:?}", e),
                };
                self.store_operand(self.ip + 1, modes[0], input_value);
                NextState::ContinueRelative(2)
            }
            OUTPUT => {
                let output_value = self.load_operand(self.ip + 1, modes[0]);
                let _ = self.output.write(output_value);
                self.last_output = output_value;
                NextState::ContinueRelative(2)
            }
            JUMP_IF_TRUE | JUMP_IF_FALSE => {
                let condition = self.load_operand(self.ip + 1, modes[0]);
                if Self::should_jump(condition, opcode) {
                    let next_ip = self.load_operand(self.ip + 2, modes[1]) as usize;
                    NextState::ContinueAbsolute(next_ip)
                } else {
                    NextState::ContinueRelative(3)
                }
            }
            RELATIVE_BASE_OFFSET => {
                let adjustion = self.load_operand(self.ip + 1, modes[0]);
                self.relative_base += adjustion;
                NextState::ContinueRelative(2)
            }
            HALT => NextState::Terminate,
            _ => panic!(
                "Invalid opcode ({}) at position {}!",
                self.load(self.ip),
                self.ip
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> Output<T> for () {
        type WriteError = ();
        fn write(&mut self, _t: T) -> Result<(), Self::WriteError> {
            Ok(())
        }
    }

    #[test]
    fn example_program_1() {
        let program = vec![1, 0, 0, 0, 99];
        let mut computer = Computer::new(0, &program, VecDeque::new(), ());
        computer.run_program();
        assert_eq!(vec![2, 0, 0, 0, 99], computer.tape);
    }

    #[test]
    fn example_program_2() {
        let program = vec![2, 3, 0, 3, 99];
        let mut computer = Computer::new(0, &program, VecDeque::new(), ());
        computer.run_program();
        assert_eq!(vec![2, 3, 0, 6, 99], computer.tape);
    }

    #[test]
    fn example_program_3() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let mut computer = Computer::new(0, &program, VecDeque::new(), ());
        computer.run_program();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.tape);
    }

    #[test]
    fn example_program_4() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = Computer::new(0, &program, VecDeque::new(), ());
        computer.run_program();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.tape);
    }

    #[test]
    fn input_output() {
        let program = vec![3, 0, 4, 0, 99];
        let mut computer = Computer::new(0, &program, queue![42], Vec::new());
        computer.run_program();
        assert_eq!(vec![42], computer.output);
    }

    #[test]
    fn parameter_modes() {
        let program = vec![1002, 4, 3, 4, 33];
        let mut computer = Computer::new(0, &program, VecDeque::new(), ());
        computer.run_program();
        assert_eq!(vec![1002, 4, 3, 4, 99], computer.tape);
    }

    #[test]
    fn example_program_5() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let mut computer = Computer::new(0, &program, queue![7], Vec::new());
        computer.run_program();
        assert_eq!(vec![999], computer.output);

        let mut computer = Computer::new(0, &program, queue![8], Vec::new());
        computer.run_program();
        assert_eq!(vec![1000], computer.output);

        let mut computer = Computer::new(0, &program, queue![9], Vec::new());
        computer.run_program();
        assert_eq!(vec![1001], computer.output);
    }

    #[test]
    fn quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = Computer::new(0, &program, VecDeque::new(), Vec::new());
        computer.run_program();
        assert_eq!(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            computer.output
        );
    }

    #[test]
    fn output_16_digit_number() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = Computer::new(0, &program, VecDeque::new(), Vec::new());
        computer.run_program();
        assert_eq!(vec![1219070632396864], computer.output);
    }

    #[test]
    fn output_large_number() {
        let program = vec![104, 1125899906842624, 99];
        let mut computer = Computer::new(0, &program, VecDeque::new(), Vec::new());
        computer.run_program();
        assert_eq!(vec![1125899906842624], computer.output);
    }

    #[test]
    fn part_1() {
        let program: Vec<MemoryType> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();

        let mut computer = Computer::new(0, &program, queue![1], Vec::new());
        computer.run_program();
        assert_eq!(vec![3780860499], computer.output);
    }

    #[test]
    fn part_2() {
        let program: Vec<MemoryType> = FileReader::new()
            .split_char(',')
            .read_from_file("input.txt")
            .unwrap();

        let mut computer = Computer::new(0, &program, queue![2], Vec::new());
        computer.run_program();
        assert_eq!(vec![33343], computer.output);
    }
}
