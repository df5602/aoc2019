use std::collections::VecDeque;
use std::convert::From;
use std::env;
use std::sync::mpsc::{channel, Receiver, Sender};

use aoc_util::input::{FileReader, FromFile};
use crossbeam::thread;

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

    let thruster_input = find_best_phase_settings(&program, 0);
    println!("Best thruster input: {}", thruster_input);
}

macro_rules! queue {
    ($($x:expr),*) => {
        {
            let mut q = VecDeque::new();
            $(q.push_back($x);)*
            q
        }
    };
}

fn run_amplifier_program(
    id: usize,
    program: &[i32],
    input: Receiver<i32>,
    output: Sender<i32>,
) -> i32 {
    let mut computer = Computer::new(id, program, input, output);
    computer.run_program();
    computer.last_output
}

fn run_amplifier_chain_with_feedback(
    program: &[i32],
    phase_settings: [u8; 5],
    initial_input: i32,
) -> i32 {
    thread::scope(|s| {
        let mut txs = Vec::with_capacity(5);
        let mut rxs = Vec::with_capacity(5);

        for i in 0..5 {
            let (tx, rx) = channel();
            tx.send(phase_settings[(i + 1) % 5] as i32).unwrap();
            txs.push(tx);
            rxs.push(rx);
        }

        txs[4].send(initial_input).unwrap();

        let mut handles = Vec::with_capacity(5);
        for i in 0..5 {
            // Needed to pull a few tricks to be able to move the senders/receivers into the closure.
            // The indices of Vec::remove() below take into account the fact, that an entry was just
            // removed from the vector during the previous iteration.
            // TODO: might be more readable using a VecDeque for this...
            let (tx, rx) = (txs.remove(0), rxs.remove(if i < 4 { 1 } else { 0 }));
            let handle = s.spawn(move |_| run_amplifier_program(i, program, rx, tx));
            handles.push(handle);
        }

        let mut result = 0;
        for handle in handles {
            result = handle.join().unwrap();
        }

        result
    })
    .unwrap()
}

fn find_best_phase_settings(program: &[i32], initial_input: i32) -> i32 {
    let mut highest_thruster_value = 0;
    for a in 5..10 {
        for b in (5..10).filter(|&b| b != a) {
            for c in (5..10).filter(|&c| c != a && c != b) {
                for d in (5..10).filter(|&d| d != a && d != b && d != c) {
                    for e in (5..10).filter(|&e| e != a && e != b && e != c && e != d) {
                        let thruster_input = run_amplifier_chain_with_feedback(
                            program,
                            [a, b, c, d, e],
                            initial_input,
                        );
                        if thruster_input > highest_thruster_value {
                            highest_thruster_value = thruster_input
                        }
                    }
                }
            }
        }
    }

    highest_thruster_value
}

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const JUMP_IF_TRUE: u32 = 5;
const JUMP_IF_FALSE: u32 = 6;
const LESS_THAN: u32 = 7;
const EQUALS: u32 = 8;
const HALT: u32 = 99;

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<u32> for ParameterMode {
    fn from(value: u32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            mode => panic!("Invalid parameter mode: {}", mode),
        }
    }
}

enum NextState {
    ContinueAbsolute(usize),
    ContinueRelative(isize),
    Terminate,
}

struct Computer {
    id: usize,
    tape: Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
    last_output: i32,
    ip: usize,
}

impl Computer {
    fn new(id: usize, program: &[i32], input: Receiver<i32>, output: Sender<i32>) -> Self {
        Self {
            id,
            tape: program.to_vec(),
            input,
            output,
            last_output: 0,
            ip: 0,
        }
    }

    fn run_program(&mut self) {
        loop {
            match self.execute_instruction() {
                NextState::ContinueAbsolute(offset) => self.ip = offset,
                NextState::ContinueRelative(offset) => {
                    self.ip = (self.ip as isize + offset) as usize
                }
                NextState::Terminate => break,
            }
        }
    }

    fn load_operand(&self, offset: usize, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.tape[self.tape[offset] as usize],
            ParameterMode::Immediate => self.tape[offset],
        }
    }

    fn should_jump(condition: i32, opcode: u32) -> bool {
        match opcode {
            JUMP_IF_TRUE => condition != 0,
            JUMP_IF_FALSE => condition == 0,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    fn operation(a: i32, b: i32, opcode: u32) -> i32 {
        match opcode {
            ADD => a + b,
            MULTIPLY => a * b,
            LESS_THAN => (a < b) as i32,
            EQUALS => (a == b) as i32,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    fn execute_instruction(&mut self) -> NextState {
        let instruction = self.tape[self.ip] as u32;
        let opcode = instruction % 100;
        let mut modes = [ParameterMode::Position; 2];
        modes[0] = ParameterMode::from((instruction / 100) % 10);
        modes[1] = ParameterMode::from((instruction / 1000) % 10);

        match opcode {
            ADD | MULTIPLY | LESS_THAN | EQUALS => {
                let a = self.load_operand(self.ip + 1, modes[0]);
                let b = self.load_operand(self.ip + 2, modes[1]);
                let output_pos = self.tape[self.ip + 3] as usize;
                self.tape[output_pos] = Self::operation(a, b, opcode);
                NextState::ContinueRelative(4)
            }
            INPUT => {
                let input_value = self.input.recv();
                let input_value = match input_value {
                    Ok(input_value) => input_value,
                    Err(e) => panic!("Error receiving input: {:?}", e),
                };
                let output_pos = self.tape[self.ip + 1] as usize;
                self.tape[output_pos] = input_value;
                NextState::ContinueRelative(2)
            }
            OUTPUT => {
                let output_value = self.load_operand(self.ip + 1, modes[0]);
                match self.output.send(output_value) {
                    Ok(_) => {}
                    Err(_) => {
                        self.last_output = output_value;
                        println!("[{}] OUTPUT: {}", self.id, output_value);
                    }
                }
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
            HALT => {
                println!("[{}] HALT", self.id);
                NextState::Terminate
            }
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

    /*#[test]
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
        let mut computer = Computer::new(&program, queue![42]);
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

    #[test]
    fn example_program_5() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let mut computer = Computer::new(&program, queue![7]);
        computer.run_program();
        assert_eq!(vec![999], computer.output);

        let mut computer = Computer::new(&program, queue![8]);
        computer.run_program();
        assert_eq!(vec![1000], computer.output);

        let mut computer = Computer::new(&program, queue![9]);
        computer.run_program();
        assert_eq!(vec![1001], computer.output);
    }

    #[test]
    fn amplifier_example_1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let thruster_input = find_best_phase_settings(&program, 0);
        assert_eq!(43210, thruster_input);
    }

    #[test]
    fn amplifier_example_2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let thruster_input = find_best_phase_settings(&program, 0);
        assert_eq!(54321, thruster_input);
    }

    #[test]
    fn amplifier_example_3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let thruster_input = find_best_phase_settings(&program, 0);
        assert_eq!(65210, thruster_input);
    }*/
}
