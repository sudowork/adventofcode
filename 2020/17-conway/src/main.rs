use std::collections::HashSet;
use std::iter;

type Coord = Vec<isize>;
type Bound = (isize, isize);

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
    let bounds = find_bounds(actives);
    let dimension_iters: Vec<Vec<isize>> = bounds
        .iter()
        .map(|(min, max)| (*min..=*max).collect())
        .collect();
    for coord in &cartesian_product(&dimension_iters) {
        if should_activate(coord.clone(), actives) {
            new_actives.insert(coord.clone());
        }
    }
    new_actives
}

/// Returns the bounding box +/- 1 for the 3-D space.
fn find_bounds(actives: &HashSet<Coord>) -> Vec<Bound> {
    let mut actives = actives.iter();
    let mut bounds: Vec<Bound> = actives
        .next()
        .unwrap()
        .iter()
        .map(|dim| (dim - 1, dim + 1))
        .collect();
    for coord in actives {
        for i in 0..coord.len() {
            let (min, max) = bounds[i];
            if coord[i] - 1 < min {
                bounds[i] = (coord[i] - 1, max);
            }
            if coord[i] + 1 > max {
                bounds[i] = (min, coord[i] + 1);
            }
        }
    }
    bounds
}

fn should_activate(coord: Coord, actives: &HashSet<Coord>) -> bool {
    let mut neighbors = 0;
    let dimensions = coord.len();
    let coord_iters: Vec<Vec<isize>> = iter::repeat((-1..=1).collect()).take(dimensions).collect();

    for coord_ in cartesian_product(&coord_iters) {
        if coord_.iter().all(|&offset| offset == 0) {
            continue;
        }
        if actives.contains(
            &((0..dimensions)
                .map(|i| coord[i] + coord_[i])
                .collect::<Vec<isize>>()),
        ) {
            neighbors += 1;
        }
    }
    if actives.contains(&coord) {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
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
