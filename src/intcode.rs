#![allow(dead_code)]

use digits_iterator::DigitsExtension;
//use std::io::stdin;

pub struct Program<I: Iterator<Item = isize>> {
    memory: Vec<isize>,
    instruction_pointer: usize,

    input: I,
    output: Vec<isize>,
}

impl<I: Iterator<Item = isize>> Program<I> {
    /// Load a program from its string representation
    pub fn load(program: &str, input: I) -> Self {
        let memory = program
            .split(',')
            .filter(|i| *i != "\n")
            .map(|i| i.replace("\n", ""))
            .map(|i| i.parse::<isize>().unwrap())
            .collect();
        let instruction_pointer = 0usize;

        Program {
            memory,
            instruction_pointer,
            input,
            output: vec![],
        }
    }

    /// Retrieve the outputs this program did during its execution
    pub fn outputs(&self) -> &[isize] {
        &self.output
    }

    /// Execute the programs without pauses, until it halts
    pub fn execute(&mut self) {
        while self.step() {}
    }

    /// Execute a single instruction and update the instruction pointer
    fn step(&mut self) -> bool {
        let mem = &self.memory;
        let ins_ptr = self.instruction_pointer;
        let operation = *mem.get(ins_ptr).unwrap();
        assert!(operation >= 0);

        match Mode::parse_opcode(operation as usize) {
            (1, m) => self.op_add(m),
            (2, m) => self.op_mul(m),
            (3, _) => self.op_input(),
            (4, m) => self.op_output(m[0]),
            (5, m) => self.op_jump_if_true(m),
            (6, m) => self.op_jump_if_false(m),
            (7, m) => self.op_less_than(m),
            (8, m) => self.op_equals(m),
            (99, _) => return false,
            (i, m) => unreachable!("Unsupported instruction: {} (mode {:?})", i, m.mode),
        }

        true
    }

    /// Perform an addition (opcode `1`)
    fn op_add(&mut self, mode: Mode) {
        let sum = self.read_mode(1, mode[0]) + self.read_mode(2, mode[1]);

        self.write(sum, 3);
        self.instruction_pointer += 4;
    }

    /// Perform a multiplication (opcode `2`)
    fn op_mul(&mut self, mode: Mode) {
        let mul = self.read_mode(1, mode[0]) * self.read_mode(2, mode[1]);

        self.write(mul, 3);
        self.instruction_pointer += 4;
    }

    /// Request an input from the user (opcode `3`)
    fn op_input(&mut self) {
        let input = self.input.next().expect("Input stream finished");

        self.write(input, 1);
        self.instruction_pointer += 2;
    }

    /// Provide an output to the user (opcode `4`)
    fn op_output(&mut self, mode: usize) {
        self.output.push(self.read_mode(1, mode));
        self.instruction_pointer += 2;
    }

    /// Modify the instruction pointer if the condition is true (opcode `5`)
    fn op_jump_if_true(&mut self, mode: Mode) {
        if self.read_mode(1, mode[0]) != 0 {
            assert!(self.read_mode(2, mode[1]) >= 0, "Negative instruction");
            self.instruction_pointer = self.read_mode(2, mode[1]) as usize;
        } else {
            self.instruction_pointer += 3
        }
    }

    /// Modify the instruction pointer if the condition is false (opcode `6`)
    fn op_jump_if_false(&mut self, mode: Mode) {
        if self.read_mode(1, mode[0]) == 0 {
            assert!(self.read_mode(2, mode[1]) >= 0, "Negative instruction");
            self.instruction_pointer = self.read_mode(2, mode[1]) as usize;
        } else {
            self.instruction_pointer += 3
        }
    }

    /// Compare the first two parameters, and set the third to 1 or 0 depending
    /// on their comparison (opcode `7`)
    fn op_less_than(&mut self, mode: Mode) {
        if self.read_mode(1, mode[0]) < self.read_mode(2, mode[1]) {
            self.write(1, 3);
        } else {
            self.write(0, 3);
        }

        self.instruction_pointer += 4;
    }

    /// Compare the first two parameters, and set the third to 1 or 0 if they
    /// are equals (opcode `8`)
    fn op_equals(&mut self, mode: Mode) {
        if self.read_mode(1, mode[0]) == self.read_mode(2, mode[1]) {
            self.write(1, 3);
        } else {
            self.write(0, 3);
        }

        self.instruction_pointer += 4;
    }

    /// Return the value that the parameter
    /// at the specified offset references.
    /// The parameter is assumed to have mode
    /// 0 (position mode)
    fn read(&self, offset: usize) -> isize {
        let pointer = self.memory[self.instruction_pointer + offset];
        assert!(pointer >= 0);

        self.memory[pointer as usize]
    }

    /// Return the value specified by the parameter,
    /// with the mode specified.
    fn read_mode(&self, offset: usize, mode: usize) -> isize {
        if mode == 0 {
            self.read(offset)
        } else {
            self.memory[self.instruction_pointer + offset]
        }
    }

    /// Write `value` at the position specified
    /// by the parameter with the given `offset`.
    /// This parameter is assumed to be at mode 0
    /// (position mode)
    fn write(&mut self, value: isize, offset: usize) {
        let pointer = self.memory[self.instruction_pointer + offset];
        assert!(pointer >= 0);

        self.memory[pointer as usize] = value;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Mode {
    mode: Vec<usize>,
}

impl Mode {
    /// Parse an opcode, returning a tuple containing
    /// the opcode itself as the first member, and a
    /// list of the parameters' modes as the second member
    fn parse_opcode(opcode: usize) -> (usize, Self) {
        let raw_digits_count = opcode.digits().count();
        let digits_count = if raw_digits_count >= 2 {
            raw_digits_count - 2
        } else {
            0
        };

        let digits: Vec<_> = opcode
            .digits()
            .enumerate()
            .filter(|(i, _)| *i < digits_count)
            .map(|(_, k)| k as usize)
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .copied()
            .collect();

        (opcode % 100, Mode { mode: digits })
    }

    fn get(&self, index: usize) -> usize {
        self[index]
    }
}

impl std::ops::Index<usize> for Mode {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.mode.get(index).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    use super::{Mode, Program};

    fn basic_template(input: &str) -> Vec<isize> {
        let mut program = Program::load(input, std::iter::empty());
        program.execute();

        program.memory
    }

    #[test]
    fn test() {
        assert_eq!(Mode::parse_opcode(1002), (02, Mode { mode: vec![0, 1] }));

        assert_eq!(Mode::parse_opcode(225), (25, Mode { mode: vec![2] }))
    }

    /// This test only uses basic instructions, it
    /// is specified in Day 1 - Part 1
    #[test]
    fn test_basic() {
        assert_eq!(
            basic_template("1,9,10,3,2,3,11,0,99,30,40,50"),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        assert_eq!(basic_template("1,0,0,0,99"), vec![2, 0, 0, 0, 99],);

        assert_eq!(basic_template("2,3,0,3,99"), vec![2, 3, 0, 6, 99],);
    }
}
