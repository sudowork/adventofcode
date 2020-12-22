#![feature(test)]
extern crate test;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);
    let (time, buses) = parse_input(&input);

    // part 1
    let (bus, bus_time) = find_min_bus(&buses, time);
    println!(
        "Bus {} @ {}. Answer: {}",
        bus,
        bus_time,
        (bus_time - time) * bus
    );

    // part 2
    let bus_time = find_bus_seq_time_iterative(&input);
    println!("Bus sequence time: {}", bus_time);
}

fn parse_input(input: &Vec<String>) -> (usize, Vec<usize>) {
    let time: usize = input[0].parse().unwrap();
    let buses: Vec<usize> = input[1]
        .split(",")
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();
    (time, buses)
}

fn find_min_bus(buses: &Vec<usize>, time: usize) -> (usize, usize) {
    let bus_times: Vec<(usize, usize)> = buses
        .iter()
        .map(|&b| (b, ((time as f64 / b as f64).ceil() as usize) * b))
        .collect();
    *bus_times.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap()
}

#[allow(dead_code)]
fn find_min_bus_cute(buses: &Vec<usize>, time: usize) -> (usize, usize) {
    let mut min_bus = 0;
    let mut min_time = usize::MAX;
    for bus in buses {
        let bus_times = (0..).step_by(*bus);
        let bus_time = bus_times.skip_while(|&t| t < time).next().unwrap();
        if bus_time < min_time {
            min_time = bus_time;
            min_bus = *bus;
        }
    }
    (min_bus, min_time)
}

fn find_bus_seq_time_iterative(input: &Vec<String>) -> u64 {
    let buses: Vec<(u64, u64)> = input[1]
        .split(",")
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(i, s)| (i as u64, s.parse().unwrap()))
        .collect();
    let mut time = buses[0].1;
    let mut multiple = buses[0].1;
    // Iteratively sieve:
    //   T = (T_n-1 + LCM(0..n-1)*x) + offset = 0
    //   Multiples of LCM will result in same offset pattern.
    // See also: Chinese Remainder Theorem
    for (offset, bus) in &buses[1..] {
        while (time + *offset) % *bus != 0 {
            time += multiple;
        }
        // bus #'s are primes, so LCM can be found just by multiplying
        multiple *= *bus;
    }
    time
}

#[allow(dead_code)]
fn find_bus_seq_time_brute_force(input: &Vec<String>) -> u64 {
    let buses: Vec<(usize, usize)> = input[1]
        .split(",")
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect();
    let (max_bus_offset, max_bus) = buses.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    (*max_bus as u64..)
        .step_by(*max_bus)
        .map(|time| time - *max_bus_offset as u64)
        .skip_while(|&time| {
            buses
                .iter()
                .any(|(offset, bus)| (time as u64 + *offset as u64) % *bus as u64 != 0)
        })
        .next()
        .unwrap()
}

#[bench]
fn bench_find_min_bus(b: &mut test::Bencher) {
    let input = util::read_lines("./input.txt");
    let (time, buses) = parse_input(&input);
    b.iter(|| find_min_bus(&buses, time));
}

#[bench]
fn bench_find_min_bus_cute(b: &mut test::Bencher) {
    let input = util::read_lines("./input.txt");
    let (time, buses) = parse_input(&input);
    b.iter(|| find_min_bus_cute(&buses, time));
}
