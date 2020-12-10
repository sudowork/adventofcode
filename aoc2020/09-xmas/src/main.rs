use std::collections::HashSet;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let sequence: Vec<u64> = input.iter().map(|l| l.parse().unwrap()).collect();

    println!("First invalid: {}", find_first_invalid(&sequence, 25));
}

fn find_first_invalid(xs: &Vec<u64>, preamble: usize) -> u64 {
    let mut previous: HashSet<u64> = xs[..preamble].iter().cloned().collect();
    for (i, x) in xs[preamble..].iter().enumerate() {
        if !sum2(*x, &previous) {
            return *x;
        }
        previous.remove(&xs[i]);
        previous.insert(*x);
    }
    0
}

fn sum2(sum: u64, xs: &HashSet<u64>) -> bool {
    let complements: HashSet<_> = xs.iter().filter(|&&x| x < sum).map(|x| sum - x).collect();
    xs.intersection(&complements).count() >= 2
}
