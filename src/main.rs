use std::fs::read_to_string;

fn main() {
    get_index_at(&vec![0], 0);

    let input = read_to_string("input").unwrap();

    let mut memory = input
        .split(',')
        .filter(|i| *i != "\n")
        .map(|i| i.replace("\n", ""))
        .map(|i| i.parse::<usize>().expect(&format!("Can't parse {}", i)))
        .collect::<Vec<_>>();
    let mut position = 0usize;

    memory[1] = 12;
    memory[2] = 2;

    //println!("[DEBUG] Memory: {:?}", memory);
    loop {
        match *memory.get(position).unwrap() {
            1 => add(&mut memory, position).unwrap(),
            2 => mul(&mut memory, position).unwrap(),
            99 => {
                println!("{:?}", memory);
                break;
            }
            _ => unreachable!(),
        }

        //println!("[DEBUG] Memory: {:?}", memory);
        position += 4;
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
}
