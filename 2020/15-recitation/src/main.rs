use std::collections::HashMap;
use std::iter::Iterator;
use util;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let numbers: Vec<usize> = input[0].split(",").map(|l| l.parse().unwrap()).collect();

    let answer = recite(&numbers, 2020);
    println!("2020th number: {}", answer);
}

fn recite(numbers: &Vec<usize>, nth: usize) -> usize {
    recitation_seq(numbers.clone()).nth(nth - 1).unwrap()
}

fn recitation_seq(seed: Vec<usize>) -> impl std::iter::Iterator<Item = usize> {
    // map num -> last utterance
    let mut last_pos: HashMap<usize, usize> = HashMap::new();
    for (i, num) in seed[..seed.len() - 1].iter().enumerate() {
        last_pos.insert(*num, i);
    }
    let mut called = 0;
    let mut last = seed[seed.len() - 1];
    std::iter::from_fn(move || {
        if called < seed.len() {
            let res = Some(seed[called]);
            called += 1;
            return res;
        }
        let idx = called - 1;
        let next = if last_pos.contains_key(&last) {
            idx - last_pos[&last]
        } else {
            0
        };
        last_pos.insert(last, idx);
        called += 1;
        last = next;
        Some(next)
    })
}
