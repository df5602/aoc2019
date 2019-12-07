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
    name: &str,
    program: &[i32],
    input: Receiver<i32>,
    output: Sender<i32>,
) -> i32 {
    let mut computer = Computer::new(name, program, input, output);
    computer.run_program();
    computer.last_output
}

fn run_amplifier_chain(program: &[i32], phase_settings: &[u8; 5], initial_input: i32) -> i32 {
    let (tx_a, rx_b) = channel();
    tx_a.send(phase_settings[1] as i32).unwrap();
    let (tx_b, rx_c) = channel();
    tx_b.send(phase_settings[2] as i32).unwrap();
    let (tx_c, rx_d) = channel();
    tx_c.send(phase_settings[3] as i32).unwrap();
    let (tx_d, rx_e) = channel();
    tx_d.send(phase_settings[4] as i32).unwrap();
    let (tx_e, rx_a) = channel();
    tx_e.send(phase_settings[0] as i32).unwrap();
    tx_e.send(initial_input).unwrap();

    let (result_tx, result_rx) = channel();

    thread::scope(|s| {
        let mut handles = Vec::new();
        let handle = s.spawn(move |_| {
            run_amplifier_program("A", program, rx_a, tx_a);
        });
        handles.push(handle);
        let handle = s.spawn(move |_| {
            run_amplifier_program("B", program, rx_b, tx_b);
        });
        handles.push(handle);
        let handle = s.spawn(move |_| {
            run_amplifier_program("C", program, rx_c, tx_c);
        });
        handles.push(handle);
        let handle = s.spawn(move |_| {
            run_amplifier_program("D", program, rx_d, tx_d);
        });
        handles.push(handle);
        let handle = s.spawn(move |_| {
            let thruster_input = run_amplifier_program("E", program, rx_e, tx_e);
            result_tx.send(thruster_input).unwrap();
        });
        handles.push(handle);

        for handle in handles {
            handle.join().unwrap();
        }
    })
    .unwrap();
    result_rx.recv().unwrap()
}

fn find_best_phase_settings(program: &[i32], initial_input: i32) -> i32 {
    let mut highest_thruster_value = 0;
    for a in 5..10 {
        for b in (5..10).filter(|&b| b != a) {
            for c in (5..10).filter(|&c| c != a && c != b) {
                for d in (5..10).filter(|&d| d != a && d != b && d != c) {
                    for e in (5..10).filter(|&e| e != a && e != b && e != c && e != d) {
                        let thruster_input =
                            run_amplifier_chain(program, &[a, b, c, d, e], initial_input);
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
    name: String,
    tape: Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
    last_output: i32,
    ip: usize,
}

impl Computer {
    fn new(name: &str, program: &[i32], input: Receiver<i32>, output: Sender<i32>) -> Self {
        Self {
            name: name.to_owned(),
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
                    Err(e) => {
                        self.last_output = output_value;
                        println!("[{}] OUTPUT: {}", self.name, output_value);
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
                println!("[{}] HALT", self.name);
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
