fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let highest_id = find_highest_seat_id(&input);
    println!("Highest Seat ID: {}", highest_id);
}

fn find_highest_seat_id(lines: &Vec<String>) -> u16 {
    lines
        .iter()
        .map(|l| get_seat(l))
        .map(|(r, c)| get_seat_id(r, c))
        .max()
        .unwrap()
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
