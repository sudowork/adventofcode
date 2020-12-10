use std::collections::HashSet;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let sequence: Vec<u64> = input.iter().map(|l| l.parse().unwrap()).collect();

    let invalid = find_first_invalid(&sequence, 25);
    println!("First invalid: {}", invalid);

    let encryption_weakness = find_encryption_weakness(invalid, &sequence);
    println!("Encryption weakness: {:?}", encryption_weakness);
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

fn find_encryption_weakness(sum: u64, xs: &Vec<u64>) -> Option<u64> {
    // Using O(n^2) algo, but practically O(n) since num contiguous to form sum is likely a small number
    for i in 0..xs.len() - 1 {
        for j in (i + 1)..xs.len() {
            let seq = &xs[i..j];
            let cont_sum = seq.iter().fold(0, |a, b| a + b);
            if cont_sum > sum {
                break;
            }
            if cont_sum == sum {
                println!("Seq: {:?}", seq);
                return Some(seq.iter().min().unwrap() + seq.iter().max().unwrap());
            }
        }
    }
    None
}
