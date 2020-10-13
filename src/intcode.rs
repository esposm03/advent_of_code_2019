#![allow(dead_code)]

use std::io::stdin;
use digits_iterator::DigitsExtension;

pub struct Program {
    memory: Vec<isize>,
    instruction_pointer: usize,
}

impl Program {

    /// Load a program from its string representation
    pub fn load(input: &str) -> Self {
        let memory = input
            .split(',')
            .filter(|i| *i != "\n")
            .map(|i| i.replace("\n", ""))
            .map(|i| i.parse::<isize>().unwrap())
            .collect();
        let instruction_pointer = 0usize;

        Program { memory, instruction_pointer }
    }

    /// Execute the programs without pauses, until it halts
    pub fn execute(&mut self) {
        while let Some(()) = self.step() {}
        println!("Program halted, memory dump: {:?}", self.memory);
    }

    /// Execute a single instruction and update the instruction pointer
    fn step(&mut self) -> Option<()> {
        let mem = &self.memory;
        let ins_ptr = self.instruction_pointer;
        let operation = *mem.get(ins_ptr).unwrap();
        assert!(operation >= 0);

        match Mode::parse_opcode(operation as usize) {
            (1, m) => self.op_add(m),
            (2, m) => self.op_mul(m),
            (3, _) => self.op_input(),
            (4, m) => self.op_output(m[0]),
            (99, _) => return None,
            _ => unreachable!(),
        }

        Some(())
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
        let mut raw_input = String::new();
        stdin().read_line(&mut raw_input).unwrap();
        let input = raw_input.trim_end();

        self.write(input.parse::<isize>().unwrap(), 1);
        self.instruction_pointer += 2;
    }

    /// Provide an output to the user (opcode `4`)
    fn op_output(&mut self, mode: usize) {
        println!("{}", self.read_mode(1, mode));

        self.instruction_pointer += 2;
    }

    /// Return the value that the parameter
    /// at the specified offset references.
    /// The parameter is assumed to have mode
    /// 0 (position mode)
    fn read(&self, offset: usize) -> isize {
        let pointer = self.memory[self.instruction_pointer+offset];
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
        let pointer = self.memory[self.instruction_pointer+offset];
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
            .collect();

        (opcode % 100, Mode {mode: digits.iter().rev().copied().collect()})
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
    use super::Mode;

    #[test]
    fn test() {
        assert_eq!(
            Mode::parse_opcode(1002),
            (02, Mode {mode: vec![0, 1]})
        );

        assert_eq!(
            Mode::parse_opcode(225),
            (25, Mode {mode: vec![2]})
        )
    }
}

