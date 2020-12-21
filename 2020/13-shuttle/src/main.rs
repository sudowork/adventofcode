fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);
    let time: usize = input[0].parse().unwrap();
    let buses: Vec<usize> = input[1]
        .split(",")
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    // part 1
    let (bus, bus_time) = find_min_bus(&buses, time);
    println!(
        "Bus {} @ {}. Answer: {}",
        bus,
        bus_time,
        (bus_time - time) * bus
    );
}

fn find_min_bus(buses: &Vec<usize>, time: usize) -> (usize, usize) {
    let bus_times: Vec<(usize, usize)> = buses
        .iter()
        .map(|&b| (b, ((time as f64 / b as f64).ceil() as usize) * b))
        .collect();
    *bus_times.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap()
}
