use std::collections::HashSet;
use std::iter;

type Coord = Vec<isize>;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    // part 1
    let actives = parse_input(&input, 3);
    let actives = run(&actives, 6);
    println!("Num actives: {}", actives.len());

    // part 2
    let actives = parse_input(&input, 4);
    let actives = run(&actives, 6);
    println!("Num actives: {}", actives.len());
}

fn run(actives: &HashSet<Coord>, iterations: usize) -> HashSet<Coord> {
    let mut curr = actives.clone();
    for _ in 0..iterations {
        curr = _run(&curr);
    }
    curr
}

fn _run(actives: &HashSet<Coord>) -> HashSet<Coord> {
    let mut new_actives = HashSet::new();
    let all_neighbors: HashSet<Coord> = actives
        .iter()
        .flat_map(|coord| get_adjacent(&coord))
        .collect();
    for coord in &all_neighbors {
        if should_activate(coord, actives) {
            new_actives.insert(coord.clone());
        }
    }
    new_actives
}

fn should_activate(coord: &Coord, actives: &HashSet<Coord>) -> bool {
    let neighbors = count_neighbors(&coord, actives);
    if actives.contains(coord) {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}

fn count_neighbors(coord: &Coord, actives: &HashSet<Coord>) -> usize {
    get_adjacent(coord)
        .iter()
        .filter(|&c| actives.contains(c))
        .count()
}

fn get_adjacent(coord: &Coord) -> HashSet<Coord> {
    let dimensions = coord.len();
    let coord_iters: Vec<Vec<isize>> = iter::repeat((-1..=1).collect()).take(dimensions).collect();
    cartesian_product(&coord_iters)
        .iter()
        .filter(|coord_| !coord_.iter().all(|&offset| offset == 0))
        .map(|coord_| (0..dimensions).map(|i| coord[i] + coord_[i]).collect())
        .collect()
}

fn parse_input(input: &[String], dimensions: usize) -> HashSet<Coord> {
    let mut actives = HashSet::new();
    for (x, line) in input.iter().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                let mut coord = vec![x as isize, y as isize];
                coord.extend(iter::repeat(0).take(dimensions - 2));
                actives.insert(coord);
            }
        }
    }
    actives
}

// Borrowed from https://rosettacode.org/wiki/Cartesian_product_of_two_or_more_lists#Rust
fn cartesian_product<T: Copy>(lists: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = vec![];
    let mut list_iter = lists.iter();
    if let Some(first_list) = list_iter.next() {
        for &i in first_list {
            res.push(vec![i]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for &el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}
