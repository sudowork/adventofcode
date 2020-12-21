use util;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let instructions = parse_instructions(&input);

    // part 1
    let (x, y) = navigate(&instructions);
    println!("({}, {}) Manhattan Distance: {}", x, y, x.abs() + y.abs());
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input.iter().map(|l| parse_instruction(l)).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let amount = line[1..].parse().unwrap();
    match &line[0..1] {
        "N" => Instruction::North(amount),
        "S" => Instruction::South(amount),
        "E" => Instruction::East(amount),
        "W" => Instruction::West(amount),
        "L" => Instruction::Left(amount),
        "R" => Instruction::Right(amount),
        _ => Instruction::Forward(amount),
    }
}

fn navigate(instructions: &Vec<Instruction>) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut direction = Direction::East;
    for instruction in instructions {
        match instruction {
            Instruction::North(amount) => y += *amount as isize,
            Instruction::South(amount) => y -= *amount as isize,
            Instruction::East(amount) => x += *amount as isize,
            Instruction::West(amount) => x -= *amount as isize,
            Instruction::Left(_) | Instruction::Right(_) => {
                direction = get_new_direction(&direction, &instruction)
            }
            Instruction::Forward(amount) => {
                // TODO: clean up
                match direction {
                    Direction::North => y += *amount as isize,
                    Direction::South => y -= *amount as isize,
                    Direction::East => x += *amount as isize,
                    Direction::West => x -= *amount as isize,
                }
            }
        }
    }
    (x, y)
}

fn get_new_direction(direction: &Direction, instruction: &Instruction) -> Direction {
    let i = DIRECTIONS.iter().position(|d| d == direction).unwrap();
    let degrees = match instruction {
        Instruction::Left(degrees) => 360 - degrees,
        Instruction::Right(degrees) => *degrees,
        _ => panic!("Invalid instruction"),
    };
    let offset = degrees / 90;
    DIRECTIONS[(i + offset) % DIRECTIONS.len()]
}
