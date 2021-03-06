use std::collections::HashMap;
use std::iter::Iterator;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let numbers: Vec<usize> = input[0].split(',').map(|l| l.parse().unwrap()).collect();

    // part 1
    let answer = recite(&numbers, 2020);
    println!("2020th number: {}", answer);

    // part 2
    let answer = recite(&numbers, 30_000_000);
    println!("30,000,000th number: {}", answer);
}

fn recite(numbers: &[usize], nth: usize) -> usize {
    recitation_seq(numbers).nth(nth - 1).unwrap()
}

fn recitation_seq(seed: &[usize]) -> impl std::iter::Iterator<Item = usize> {
    let seed = seed.to_owned();
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
