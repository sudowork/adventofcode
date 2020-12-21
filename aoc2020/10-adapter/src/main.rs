use std::collections::HashMap;
use util;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let joltages = get_joltages(&input);

    let diff_counts = count_joltage_diffs(&joltages);
    let product = diff_counts.get(&1).unwrap() * diff_counts.get(&3).unwrap();
    println!("{:?}, product={}", diff_counts, product);
}

fn get_joltages(input: &Vec<String>) -> Vec<usize> {
    let mut joltages: Vec<usize> = input.iter().map(|j| j.parse().unwrap()).collect();
    joltages.sort();
    let device_joltage = joltages.iter().max().unwrap() + 3;
    joltages.push(device_joltage);
    joltages
}

fn count_joltage_diffs(joltages: &Vec<usize>) -> HashMap<usize, usize> {
    let mut diff_counts = HashMap::new();
    let diffs: Vec<usize> = joltages
        .iter()
        .scan(0, |last, &j| {
            let diff = j - *last;
            *last = j;
            Some(diff)
        })
        .collect();
    for diff in diffs {
        *diff_counts.entry(diff).or_insert(0) += 1
    }
    diff_counts
}
