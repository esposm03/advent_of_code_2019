use std::fs::read_to_string;

fn main() {
    get_index_at(&vec![0], 0);

    let input = read_to_string("input").unwrap();

    let memory = input
        .split(',')
        .filter(|i| *i != "\n")
        .map(|i| i.replace("\n", ""))
        .map(|i| i.parse::<usize>().expect(&format!("Can't parse {}", i)))
        .collect::<Vec<_>>();

    for i1 in 0..100 {
        for i2 in 0..100 {
            let mut program_memory = memory.clone();
            program_memory[1] = i1;
            program_memory[2] = i2;
            if run_program(program_memory) == 19690720 {
                println!("Inputs: ({}, {})", i1, i2);
            }
        }
    }
}

fn run_program(mut memory: Vec<usize>) -> usize {
    let mut instruction_pointer = 0;

    //println!("[DEBUG] Memory: {:?}", memory);
    loop {
        match *memory.get(instruction_pointer).unwrap() {
            1 => add(&mut memory, instruction_pointer).unwrap(),
            2 => mul(&mut memory, instruction_pointer).unwrap(),
            99 => {
                return *memory.get(0).unwrap();
            }
            _ => unreachable!(),
        }

        //println!("[DEBUG] Memory: {:?}", memory);
        instruction_pointer += 4;
    }
}

fn add(mem: &mut Vec<usize>, index: usize) -> Option<()> {
    //println!("[DEBUG] Summing mem[{}] + mem[{}]: {:?} + {:?}", index+1, index+2, mem.get(index+1), mem.get(index+2));
    let sum = get_index_at(&mem, index+1) + get_index_at(&mem, index+2);
    let pos: usize = *mem.get(index+3).unwrap();
    //println!("[DEBUG] Putting the result at mem[mem[{}]]: {}", index+3, pos);
    mem[pos] = sum;

    Some(())
}

fn mul(mem: &mut Vec<usize>, index: usize) -> Option<()> {
    //println!("[DEBUG] Multiplying mem[{}] * mem[{}]: {:?} * {:?}", index+1, index+2, mem.get(index+1), mem.get(index+2));
    let mul = get_index_at(&mem, index+1) * get_index_at(&mem, index+2);
    let pos: usize = *mem.get(index+3).unwrap();
    //println!("[DEBUG] Putting the result at mem[mem[{}]]: {}", index+3, pos);
    mem[pos] = mul;

    Some(())
}

fn get_index_at(mem: &Vec<usize>, position: usize) -> usize {
    let index = *mem.get(position).unwrap();
    *mem.get(index).unwrap()
}

