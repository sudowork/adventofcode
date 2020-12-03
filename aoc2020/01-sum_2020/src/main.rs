use std::collections::HashSet;
use std::env;

fn main() {
    let input_file = env::args()
        .skip(1)
        .next()
        .unwrap_or("./input.txt".to_string());
    let input = util::read_lines(input_file);
    let input: Vec<i32> = input.iter().map(|line| line.parse().unwrap()).collect();
    let (a, b) = complements(&input, 2020).unwrap();
    println!("Found Pair: ({}, {})", a, b);
    println!("{}", a * b);

    let (a, b, c) = triplets(&input, 2020).unwrap();
    println!("Found Triplet: ({}, {}, {})", a, b, c);
    println!("{}", a * b * c);
}

fn complements(xs: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let xs: HashSet<_> = xs.iter().cloned().collect();
    let complement = xs.iter().map(|x| sum - x).collect();
    let pairs: Vec<_> = xs.intersection(&complement).cloned().collect();
    match pairs.len() {
        2 => Some((pairs[0], pairs[1])),
        _ => None,
    }
}

fn triplets(xs: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    for x in xs.iter() {
        match complements(xs, sum - x) {
            Some((y, z)) => return Some((*x, y, z)),
            _ => continue,
        };
    }
    None
}
