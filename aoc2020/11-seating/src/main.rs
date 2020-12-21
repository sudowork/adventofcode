use util;

#[derive(Clone, PartialEq, Hash, Copy)]
enum State {
    Floor,
    Empty,
    Occupied,
}

type SeatingLayout = Vec<Vec<State>>;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let seating = parse_seating(&input);

    // part 1
    let occupants = count_stable(&seating);
    println!("Occupants: {}", occupants);
}

fn parse_seating(input: &Vec<String>) -> SeatingLayout {
    input
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    _ => State::Occupied, // should not happen
                })
                .collect()
        })
        .collect()
}

fn count_stable(seating: &SeatingLayout) -> usize {
    let mut curr_seating = seating.clone();
    let mut last_hash = util::hash(seating);
    loop {
        curr_seating = run(&curr_seating);
        let curr_hash = util::hash(&curr_seating);
        if last_hash == curr_hash {
            return count_occupants(&curr_seating);
        }
        last_hash = curr_hash;
    }
}

fn run(seating: &SeatingLayout) -> SeatingLayout {
    let mut new_seating: SeatingLayout = seating
        .iter()
        .map(|row| row.iter().cloned().collect())
        .collect();
    for i in 0..seating.len() {
        for j in 0..seating[0].len() {
            new_seating[i][j] = match seating[i][j] {
                State::Empty => {
                    if count_neighbors(seating, i, j) == 0 {
                        State::Occupied
                    } else {
                        State::Empty
                    }
                }
                State::Occupied => {
                    if count_neighbors(seating, i, j) >= 4 {
                        State::Empty
                    } else {
                        State::Occupied
                    }
                }
                _ => continue,
            }
        }
    }
    new_seating
}

fn count_neighbors(seating: &SeatingLayout, i: usize, j: usize) -> usize {
    let i = i as isize;
    let j = j as isize;
    let mut count = 0;
    for k in (-1 as isize)..=1 {
        for l in (-1 as isize)..=1 {
            if k == 0 && l == 0 {
                continue;
            }
            if i + k < 0 || (i + k) as usize >= seating.len() {
                continue;
            }
            if j + l < 0 || (j + l) as usize >= seating[0].len() {
                continue;
            }
            if seating[(i + k) as usize][(j + l) as usize] == State::Occupied {
                count += 1;
            }
        }
    }
    count
}

fn count_occupants(seating: &SeatingLayout) -> usize {
    seating
        .iter()
        .map(|row| row.iter().filter(|&s| s == &State::Occupied).count())
        .sum()
}

#[allow(dead_code)]
fn print_seating(seating: &SeatingLayout) {
    for row in seating {
        for state in row {
            match state {
                State::Empty => print!("L"),
                State::Occupied => print!("#"),
                State::Floor => print!("."),
            }
        }
        println!("");
    }
    println!("");
}
