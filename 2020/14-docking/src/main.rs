use std::collections::HashMap;
use std::iter::Iterator;

const BITS: u8 = 36;

enum Instruction {
    Mask(u64, u64),
    Mem(u64, u64),
}

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let instructions = parse_instructions(&input);

    // part 1
    let memory = run(&instructions);
    println!("Sum of memory: {}", sum_memory(&memory));

    // part 2
    let memory = run2(&instructions);
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

fn run2(instructions: &Vec<Instruction>) -> HashMap<u64, u64> {
    let mut mem = HashMap::new();
    let mut mask: u64 = 0b0;
    let mut mask_value: u64 = 0b0;
    for instruction in instructions {
        match instruction {
            Instruction::Mem(addr, value) => {
                for addr in fluct_addr(*addr | mask_value, mask) {
                    mem.insert(addr, *value);
                }
            }
            Instruction::Mask(m, mv) => {
                mask = *m;
                mask_value = *mv;
            }
        }
    }
    mem
}

fn fluct_addr(addr: u64, mask: u64) -> impl std::iter::Iterator<Item = u64> {
    let mut ones: Vec<u8> = Vec::new();
    for i in 0..=BITS {
        if mask >> i & 0b1 == 1 {
            ones.push(i);
        }
    }
    let mut i = 0;
    std::iter::from_fn(move || {
        // Enumerate 0..2^(num ones) and encode address mask
        if i == (2 as u32).pow(ones.len() as u32) {
            return None;
        }
        let addr_mask_value = ones.iter().enumerate().fold(0b0, |value, (j, bit)| {
            // `i` is a one-hot encoding of which bits in `ones` to activate in the mask
            if i >> j & 0b1 == 0b1 {
                value | 0b1 << bit
            } else {
                value
            }
        });
        i += 1;
        Some((addr & !mask) | (addr_mask_value & mask))
    })
}

fn sum_memory(memory: &HashMap<u64, u64>) -> u64 {
    memory.values().sum()
}
