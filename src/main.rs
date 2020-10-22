mod intcode;

use intcode::Program;
use itertools::Itertools;
use std::io::{stdin, stdout, BufReader};

fn main() {
    println!(
        "{}",
        (0..=4)
            .permutations(5)
            .map(|phases: Vec<isize>| {
                println!("[DEBUG] Running with phases: {:?}", phases);
                let mut last_output = 0;

                for phase in &phases {
                    let inputs = [*phase, last_output];
                    let mut program =
                        Program::load(include_str!("../input"), inputs.iter().copied());
                    program.execute();

                    last_output = program.outputs()[0];
                }

                last_output
            })
            .max()
            .unwrap()
    );
}
