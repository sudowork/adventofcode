#[derive(Clone, PartialEq, Hash, Copy)]
enum State {
    Floor,
    Empty,
    Occupied,
}

type SeatingLayout = Vec<Vec<State>>;
type SeatingLayoutArg = [Vec<State>];

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let seating = parse_seating(&input);

    // part 1
    let occupants = count_stable(&seating, &count_adjacent, 4);
    println!("Part 1 Occupants: {}", occupants);

    // part 2
    let occupants = count_stable(&seating, &count_eyeline, 5);
    println!("Part 2 Occupants: {}", occupants);
}

fn parse_seating(input: &[String]) -> SeatingLayout {
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

#[allow(clippy::ptr_arg)]
fn count_stable(
    seating: &SeatingLayout,
    count_seats: &dyn Fn(&SeatingLayoutArg, usize, usize) -> usize,
    occupant_limit: usize,
) -> usize {
    let mut curr_seating = seating.to_owned();
    let mut last_hash = util::hash(seating);
    loop {
        curr_seating = run(&curr_seating, count_seats, occupant_limit);
        let curr_hash = util::hash(&curr_seating);
        if last_hash == curr_hash {
            return count_occupants(&curr_seating);
        }
        last_hash = curr_hash;
    }
}

fn run(
    seating: &SeatingLayoutArg,
    count_seats: &dyn Fn(&SeatingLayoutArg, usize, usize) -> usize,
    occupant_limit: usize,
) -> SeatingLayout {
    let mut new_seating: SeatingLayout = seating.iter().map(|row| row.to_vec()).collect();
    for i in 0..seating.len() {
        for j in 0..seating[0].len() {
            new_seating[i][j] = match seating[i][j] {
                State::Empty => {
                    if count_seats(seating, i, j) == 0 {
                        State::Occupied
                    } else {
                        State::Empty
                    }
                }
                State::Occupied => {
                    if count_seats(seating, i, j) >= occupant_limit {
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

fn count_adjacent(seating: &SeatingLayoutArg, i: usize, j: usize) -> usize {
    let i = i as isize;
    let j = j as isize;
    let mut count = 0;
    for k in -1_isize..=1 {
        for l in -1_isize..=1 {
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

fn count_eyeline(seating: &SeatingLayoutArg, i: usize, j: usize) -> usize {
    let mut count = 0;
    for k in -1_isize..=1 {
        for l in -1_isize..=1 {
            if k == 0 && l == 0 {
                continue;
            }
            count += count_direction(seating, i, j, (k, l));
        }
    }
    count
}

fn count_direction(
    seating: &SeatingLayoutArg,
    i: usize,
    j: usize,
    offset: (isize, isize),
) -> usize {
    let (i_offset, j_offset) = offset;
    let mut i = i as isize;
    let mut j = j as isize;
    loop {
        i += i_offset;
        j += j_offset;
        if i < 0 || i as usize >= seating.len() {
            return 0;
        }
        if j < 0 || j as usize >= seating[0].len() {
            return 0;
        }
        let seat = seating[i as usize][j as usize];
        if seat == State::Empty {
            return 0;
        }
        if seat == State::Occupied {
            return 1;
        }
    }
}

fn count_occupants(seating: &SeatingLayoutArg) -> usize {
    seating
        .iter()
        .map(|row| row.iter().filter(|&s| s == &State::Occupied).count())
        .sum()
}

#[allow(dead_code)]
fn print_seating(seating: &SeatingLayoutArg) {
    for row in seating {
        for state in row {
            match state {
                State::Empty => print!("L"),
                State::Occupied => print!("#"),
                State::Floor => print!("."),
            }
        }
        println!();
    }
    println!();
}
