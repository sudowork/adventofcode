use std::collections::HashMap;
use util;

type Passport = HashMap<String, String>;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
// const OPTIONAL_FIELDS: [&str; 1] = ["cid"];

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines_unfiltered(input_file);

    let passports = parse_passports(input);
    let valid = passports.iter().filter(|p| is_valid(p)).count();
    println!("Num valid passports: {}", valid);
}

fn parse_passports(input: Vec<String>) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut passport = Passport::new();
    for line in input.iter() {
        if line.is_empty() && !passport.is_empty() {
            passports.push(passport);
            passport = Passport::new();
            continue;
        }
        for (key, value) in parse_passport_line(line) {
            passport.insert(key, value);
        }
    }
    if !passport.is_empty() {
        passports.push(passport);
    }
    passports
}

fn parse_passport_line(line: &str) -> Passport {
    let mut partial_passport = Passport::new();
    if line.is_empty() {
        return partial_passport;
    }
    let parts = line.split_whitespace();
    for split in parts.map(|part| part.split(":").collect::<Vec<&str>>()) {
        partial_passport.insert(split[0].to_string(), split[1].to_string());
    }
    partial_passport
}

fn is_valid(passport: &Passport) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|f| passport.contains_key(&f.to_string()))
}
