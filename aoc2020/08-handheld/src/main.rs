use std::collections::HashSet;

enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let instructions = parse_instructions(&input);
    let acc = run(&instructions);
    println!("Acc at loop: {}", acc);
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input
        .iter()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let op = iter.next().unwrap();
            let offset: isize = iter.next().unwrap().parse().unwrap();
            if op == "acc" {
                return Instruction::Acc(offset);
            } else if op == "jmp" {
                return Instruction::Jmp(offset);
            }
            Instruction::Nop
        })
        .collect()
}

fn run(instructions: &Vec<Instruction>) -> isize {
    let mut acc: isize = 0;
    let mut pointer: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    loop {
        if pointer >= instructions.len() || seen.contains(&pointer) {
            return acc;
        }
        seen.insert(pointer);
        match instructions[pointer] {
            Instruction::Acc(change) => {
                acc += change;
                pointer += 1;
            }
            Instruction::Jmp(offset) => pointer = (pointer as isize + offset) as usize,
            Instruction::Nop => pointer += 1,
        }
    }
}
