use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Ticket = Vec<usize>;
type FieldRanges = Vec<RangeInclusive<usize>>;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines_unfiltered(input_file);

    let ranges = parse_ranges(&input);
    let tickets = parse_tickets(&input);

    // part 1
    let error_rate = check_error_rate(&tickets[1..], &ranges);
    println!("Error rate: {}", error_rate);

    // part 2
    let field_map = get_field_map(&tickets[1..], &ranges);
    println!("Field mapping: {:?}", field_map);
    let departure_product = get_departure_product(&tickets[0], &field_map);
    println!("My ticket departure product: {}", departure_product);
}

fn check_error_rate(tickets: &[Ticket], ranges: &HashMap<&str, FieldRanges>) -> usize {
    tickets.iter().flat_map(|t| invalid_fields(t, ranges)).sum()
}

fn get_field_map<'a>(
    tickets: &[Ticket],
    ranges: &'a HashMap<&str, FieldRanges>,
) -> HashMap<&'a str, usize> {
    let mut field_map = HashMap::new();
    let tickets: Vec<&Ticket> = tickets.iter().filter(|t| is_valid(t, ranges)).collect();
    let cols = tickets[0].len();
    let mut unused_fields: HashSet<&str> = ranges.keys().cloned().collect();
    let mut found_cols: HashSet<usize> = HashSet::new();
    // Continue looping until all columns found.
    //   In each loop, iterate over the unfound cols
    //     For a given col, start with unfound fields
    //     Eliminate fields by checking ranges against tickets
    //     If a single field is left, add it to the map, mark as used/found.
    while !unused_fields.is_empty() {
        for col in 0..cols {
            if found_cols.contains(&col) {
                continue;
            }
            let mut possible_fields: HashSet<&str> = unused_fields.iter().cloned().collect();
            for ticket in &tickets {
                let field = ticket[col];
                for (key, field_ranges) in ranges {
                    if !possible_fields.contains(key) {
                        continue;
                    }
                    if !field_ranges.iter().any(|r| r.contains(&field)) {
                        possible_fields.remove(key);
                    }
                }
                if possible_fields.len() == 1 {
                    let field = *possible_fields.iter().next().unwrap();
                    field_map.insert(field, col);
                    unused_fields.remove(field);
                    found_cols.insert(col);
                    break;
                }
            }
        }
    }
    field_map
}

fn get_departure_product(ticket: &Ticket, field_map: &HashMap<&str, usize>) -> usize {
    field_map
        .iter()
        .filter(|(key, _)| key.contains("departure"))
        .map(|(_, idx)| ticket[*idx])
        .product()
}

fn is_valid(ticket: &Ticket, ranges: &HashMap<&str, FieldRanges>) -> bool {
    invalid_fields(ticket, ranges).is_empty()
}

fn invalid_fields(ticket: &Ticket, ranges: &HashMap<&str, FieldRanges>) -> Ticket {
    ticket
        .iter()
        .cloned()
        .filter(|&f| !is_field_valid(f, ranges))
        .collect()
}

fn is_field_valid(field: usize, ranges: &HashMap<&str, FieldRanges>) -> bool {
    ranges.values().flatten().any(|r| r.contains(&field))
}

fn parse_ranges(input: &[String]) -> HashMap<&str, FieldRanges> {
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
