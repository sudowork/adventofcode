use std::collections::HashMap;
use std::ops::RangeInclusive;

type Ticket = Vec<usize>;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines_unfiltered(input_file);

    let ranges = parse_ranges(&input);
    let tickets = parse_tickets(&input);

    let error_rate = check_error_rate(&tickets[1..], &ranges);
    println!("Error rate: {}", error_rate);
}

fn check_error_rate(
    tickets: &[Ticket],
    ranges: &HashMap<&str, Vec<RangeInclusive<usize>>>,
) -> usize {
    let mut error_rate = 0;
    for ticket in tickets {
        for field in ticket {
            if !ranges.values().flatten().any(|r| r.contains(field)) {
                error_rate += field;
            }
        }
    }
    error_rate
}

fn parse_ranges(input: &[String]) -> HashMap<&str, Vec<RangeInclusive<usize>>> {
    let mut ranges = HashMap::new();
    for line in input {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        let field = parts[0];
        let range_strs = parts[1].split(" or ");
        ranges.insert(
            field,
            range_strs
                .map(|range_str| {
                    let parts: Vec<&str> = range_str.split("-").collect();
                    parts[0].parse().unwrap()..=parts[1].parse().unwrap()
                })
                .collect(),
        );
    }
    ranges
}

fn parse_tickets(input: &[String]) -> Vec<Vec<usize>> {
    input
        .iter()
        .filter(|l| l.contains(","))
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect()
}
