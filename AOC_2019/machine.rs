use std::fs::read_to_string;

pub enum Status {
    Running,
    Halted,
}

#[derive(Copy, Clone)]
pub enum OperationMode {
    Positional,
    Immediate,
    Relative,
}

impl OperationMode {
    pub fn resolve(val: i128) -> OperationMode {
        match val {
            0 => OperationMode::Positional,
            1 => OperationMode::Immediate,
            2 => OperationMode::Relative,
            _ => panic!("Unknown operation mode '{}'", val)
        }
    }
}

enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Relative,
    Halt,
}

impl Operation {
    pub fn resolve(val: i128) -> Operation {
        match val {
            1  => Operation::Add,
            2  => Operation::Multiply,
            3  => Operation::Input,
            4  => Operation::Output,
            5  => Operation::JumpIfTrue,
            6  => Operation::JumpIfFalse,
            7  => Operation::LessThan,
            8  => Operation::Equals,
            9  => Operation::Relative,
            99 => Operation::Halt,
            _ => panic!("Unknown operation '{}'", val)
        }
    }
}


pub struct Machine {
    pub pointer:  usize,
    pub rel_base: isize,
    pub ticks:    usize,
    pub memory:   Vec<i128>,
    pub input:    Vec<i128>,
    pub output:   Vec<i128>,
    pub status:   Status,
}

impl Machine {


    pub fn new(memory: Vec<i128>, input: Vec<i128>) -> Self {
        Self {
            pointer: 0,
            rel_base: 0,
            ticks: 0,
            memory,
            input,
            output: Vec::new(),
            status: Status::Running,
        }
    }

    pub fn write(&mut self, mode: OperationMode, to: usize, val: i128) {
        let address: usize = match mode {
            OperationMode::Positional => self.memory[to] as usize,
            OperationMode::Relative   => (self.memory[to] + self.rel_base as i128).abs() as usize,
            OperationMode::Immediate  => panic!("Immediate mode not available for write"),
        };

        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }

        self.memory[address] = val;
    }


    pub fn read(&mut self, mode: OperationMode, from: usize) -> i128 {
        let address = match mode {
            OperationMode::Immediate  => from,
            OperationMode::Relative   => (self.memory[from] + self.rel_base as i128).abs() as usize,
            OperationMode::Positional => self.memory[from] as usize
        };

        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }

        self.memory[address]
    }

    pub fn read_param(&mut self, mode: OperationMode, nth: usize) -> i128 {
        self.read(mode, self.pointer + nth)
    }

    pub fn run(&mut self) -> Option<i128> {
        while let Status::Running = self.status {
            self.tick();
        }

        if self.output.last().is_some() {
            Some(*self.output.clone().last().unwrap())
        }
        else {
            None
        }
    }

    pub fn tick(&mut self) {
        let val         = self.memory[self.pointer];
        let opcode      = Operation::resolve(val % 100);
        let first_mode  = OperationMode::resolve((val / 100) % 10);
        let second_mode = OperationMode::resolve((val / 1000) % 10);
        let third_mode  = OperationMode::resolve((val / 10000) % 10);

        match opcode {
            Operation::Add => {
                let (v1, v2) = (self.read_param(first_mode, 1), 
                                self.read_param(second_mode, 2));
                self.write(third_mode, self.pointer + 3, v1 + v2);             
                self.pointer += 4;
            }

            Operation::Multiply => {
                let (v1, v2) = (self.read_param(first_mode, 1), 
                                self.read_param(second_mode, 2));
                self.write(third_mode, self.pointer + 3, v1 * v2);
                self.pointer += 4;
            }

            Operation::Input => {
                let value = self.input.remove(0);
                self.write(first_mode, self.pointer + 1, value);
                self.pointer += 2;
            }

            Operation::Output => {
                let value = self.read_param(first_mode, 1);
                self.output.push(value);
                self.pointer += 2;
            }

            Operation::JumpIfTrue => {
                if self.read_param(first_mode, 1) != 0 {
                    self.pointer = self.read_param(second_mode, 2) as usize;
                }
                else {
                    self.pointer += 3;
                }
            }

            Operation::JumpIfFalse => {
                if self.read_param(first_mode, 1) == 0 {
                    self.pointer = self.read_param(second_mode, 2) as usize;
                }
                else {
                    self.pointer += 3;
                }
            }

            Operation::LessThan => {
                let (v1, v2) = (self.read_param(first_mode, 1), 
                                self.read_param(second_mode, 2));
                self.write(third_mode, self.pointer + 3, (v1 < v2) as i128);
                self.pointer += 4;
            }

            Operation::Equals => {
                let (v1, v2) = (self.read_param(first_mode, 1), 
                                self.read_param(second_mode, 2));
                self.write(third_mode, self.pointer + 3, (v1 == v2) as i128);
                self.pointer += 4;
            }

            Operation::Relative => {
                self.rel_base += self.read_param(first_mode, 1) as isize;
                self.pointer += 2
            }

            Operation::Halt => {
                self.status = Status::Halted;
            }
        }

        self.ticks += 1;
    }

}

pub fn read_codes(file: &str) -> Vec<i128> {
    read_to_string(file)
        .expect("Unable to read file")
        .trim()
        .split(",")
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}