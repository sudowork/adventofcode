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
