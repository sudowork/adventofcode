use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let instructions = parse_instructions(&input);
    let (acc, _) = run(&instructions);
    println!("Acc at loop: {}", acc);

    let acc = run_with_swaps(&instructions);
    println!("Terminated acc: {}", acc);
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
            Instruction::Nop(offset)
        })
        .collect()
}

fn run_with_swaps(instructions: &Vec<Instruction>) -> isize {
    for (i, op) in instructions.iter().enumerate() {
        match op {
            Instruction::Acc(_) => continue,
            _ => {
                // Could implement by doing in-place swap instead of cloning
                // But wanted to leave run() method alone for clarity.
                let (acc, terminated) = run(&swap_instruction(instructions, i));
                if terminated {
                    return acc;
                }
            }
        }
    }
    panic!("No successful swap found");
}

fn swap_instruction(instructions: &Vec<Instruction>, pointer: usize) -> Vec<Instruction> {
    let mut new_instructions = vec![];
    new_instructions.extend_from_slice(&instructions[..pointer]);
    match instructions[pointer] {
        Instruction::Jmp(offset) => new_instructions.push(Instruction::Nop(offset)),
        Instruction::Nop(offset) => new_instructions.push(Instruction::Nop(offset)),
        _ => panic!("Did not expect acc"),
    }
    new_instructions.extend_from_slice(&instructions[pointer + 1..]);
    new_instructions
}

fn run(instructions: &Vec<Instruction>) -> (isize, bool) {
    let mut acc: isize = 0;
    let mut pointer: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    loop {
        if pointer >= instructions.len() {
            return (acc, true);
        }
        if seen.contains(&pointer) {
            return (acc, false);
        }
        seen.insert(pointer);
        match instructions[pointer] {
            Instruction::Acc(change) => {
                acc += change;
                pointer += 1;
            }
            Instruction::Jmp(offset) => pointer = (pointer as isize + offset) as usize,
            Instruction::Nop(_) => pointer += 1,
        }
    }
}
