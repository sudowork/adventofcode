use std::collections::HashMap;
use util;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let joltages = get_joltages(&input);

    // part 1
    let diff_counts = count_joltage_diffs(&joltages);
    let product = diff_counts.get(&1).unwrap() * diff_counts.get(&3).unwrap();
    println!("{:?}, product={}", diff_counts, product);

    // part 2
    let num_paths = count_paths(&joltages);
    println!("Num paths: {}", num_paths);
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

fn count_paths(joltages: &Vec<usize>) -> usize {
    let num = joltages.len();
    let lookup = |i| joltages.get(i).unwrap();
    let mut memo = vec![0; num];
    for i in 0..3 {
        if lookup(i) <= &3 {
            memo[i] = 1;
        }
    }
    for i in 1..num {
        let mut prev_sum = 0;
        for j in (if i < 3 { 0 } else { i - 3 })..i {
            if lookup(i) - lookup(j) <= 3 {
                prev_sum += memo[j];
            }
        }
        memo[i] += prev_sum;
    }
    *memo.last().unwrap()
}
