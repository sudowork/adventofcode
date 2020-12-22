use std::collections::HashMap;

enum Instruction {
    Mask(u64, u64),
    Mem(u64, u64),
}

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let instructions = parse_instructions(&input);

    let memory = run(&instructions);
    println!("Sum of memory: {}", sum_memory(&memory));
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input.iter().map(|l| parse_instruction(l)).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" = ").collect();
    if parts[0].starts_with("mem") {
        let addr = parts[0][4..parts[0].len() - 1].parse().unwrap();
        Instruction::Mem(addr, parts[1].parse().unwrap())
    } else {
        let mask = parts[1]
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, b)| *b == 'X')
            .fold(0b0, |mask, (offset, _)| mask | 0b1 << offset);
        let mask_value = parts[1]
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, b)| *b == '1')
            .fold(0b0, |mask, (offset, _)| mask | 0b1 << offset);
        Instruction::Mask(mask, mask_value)
    }
}

fn run(instructions: &Vec<Instruction>) -> HashMap<u64, u64> {
    let mut mem = HashMap::new();
    let mut mask: u64 = 0b0;
    let mut mask_value: u64 = 0b0;
    for instruction in instructions {
        match instruction {
            Instruction::Mem(addr, value) => {
                mem.insert(*addr, (value & mask) | mask_value);
            }
            Instruction::Mask(m, mv) => {
                mask = *m;
                mask_value = *mv;
            }
        }
    }
    mem
}

fn sum_memory(memory: &HashMap<u64, u64>) -> u64 {
    memory.values().sum()
}
