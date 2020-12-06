fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let highest_id = find_highest_seat_id(&input);
    println!("Highest Seat ID: {}", highest_id);

    let my_seat_id = find_missing_seat_id(&input);
    println!("My Seat ID: {}", my_seat_id.unwrap());
}

fn find_highest_seat_id(lines: &Vec<String>) -> u16 {
    *get_seat_ids(lines).iter().max().unwrap()
}

fn find_missing_seat_id(lines: &Vec<String>) -> Option<u16> {
    let mut seat_ids = get_seat_ids(lines);
    seat_ids.sort();
    for (prev, next) in seat_ids.iter().zip(seat_ids.iter().skip(1)) {
        if next - prev > 1 {
            return Some(next - 1);
        }
    }
    None
}

fn get_seat_ids(lines: &Vec<String>) -> Vec<u16> {
    lines
        .iter()
        .map(|l| get_seat(l))
        .map(|(r, c)| get_seat_id(r, c))
        .collect()
}

fn get_seat(line: &str) -> (u8, u8) {
    let row_encoding = &line[..7];
    let col_encoding = &line[7..];
    let row = calculate_binary(row_encoding, 'B');
    let col = calculate_binary(col_encoding, 'R');
    (row, col)
}

fn calculate_binary(encoded: &str, one: char) -> u8 {
    let mut number = 0b0;
    for (i, ch) in encoded.chars().rev().enumerate() {
        if ch == one {
            number = number | (0b1 << i);
        }
    }
    number
}

fn get_seat_id(row: u8, col: u8) -> u16 {
    ((row as u16) * 8) + col as u16
}
