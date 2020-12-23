#![feature(test)]
extern crate test;

use std::collections::HashSet;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let sequence: Vec<u64> = input.iter().map(|l| l.parse().unwrap()).collect();

    let invalid = find_first_invalid(&sequence, 25);
    println!("First invalid: {}", invalid);

    let encryption_weakness = find_encryption_weakness(invalid, &sequence);
    println!("Encryption weakness: {:?}", encryption_weakness);

    let encryption_weakness = find_encryption_weakness_efficient(invalid, &sequence);
    println!("Encryption weakness: {:?}", encryption_weakness);
}

fn find_first_invalid(xs: &[u64], preamble: usize) -> u64 {
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

fn find_encryption_weakness(sum: u64, xs: &[u64]) -> Option<u64> {
    // Using O(n^2) algo, but practically O(n) since num contiguous to form sum is likely a small number
    for i in 0..xs.len() - 1 {
        for j in (i + 1)..xs.len() {
            let seq = &xs[i..j];
            let cont_sum: u64 = seq.iter().sum();
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

fn find_encryption_weakness_efficient(sum: u64, xs: &[u64]) -> Option<u64> {
    let mut cont_sum = 0;
    let mut i = 0;
    for j in 0..xs.len() {
        cont_sum += xs[j];
        while cont_sum > sum && j > i {
            cont_sum -= xs[i];
            i += 1;
            if cont_sum == sum {
                break;
            }
        }
        if cont_sum == sum {
            let seq = &xs[i..j + 1];
            println!("{:?}", seq);
            return Some(seq.iter().min().unwrap() + seq.iter().max().unwrap());
        }
    }
    None
}

#[bench]
fn bench_find_ew_slow(b: &mut test::Bencher) {
    let input = util::read_lines("./input.txt");
    let sequence: Vec<u64> = input.iter().map(|l| l.parse().unwrap()).collect();
    let invalid = find_first_invalid(&sequence, 25);
    b.iter(|| find_encryption_weakness(invalid, &sequence));
}

#[bench]
fn bench_find_ew_fast(b: &mut test::Bencher) {
    let input = util::read_lines("./input.txt");
    let sequence: Vec<u64> = input.iter().map(|l| l.parse().unwrap()).collect();
    let invalid = find_first_invalid(&sequence, 25);
    b.iter(|| find_encryption_weakness(invalid, &sequence));
}
