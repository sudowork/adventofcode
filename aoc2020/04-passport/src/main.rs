use regex::Regex;
use std::collections::HashMap;
use util;

type Passport = HashMap<String, String>;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
// const OPTIONAL_FIELDS: [&str; 1] = ["cid"];

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let groups = util::read_line_groups(input_file);

    let passports = parse_passports(groups);
    let valid = passports.iter().filter(|p| has_required_fields(p)).count();
    println!("Num passports with fields: {}", valid);

    let valid = passports.iter().filter(|p| is_valid(p)).count();
    println!("Num valid: {}", valid);
}

fn parse_passports(groups: Vec<Vec<String>>) -> Vec<Passport> {
    let mut passports = Vec::new();
    for group in groups {
        let mut passport = Passport::new();
        for line in group.iter() {
            for (key, value) in parse_passport_line(line) {
                passport.insert(key, value);
            }
        }
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
    if !has_required_fields(passport) {
        return false;
    }
    if !validate_digits(passport.get("byr").unwrap(), 4, 1920, 2002) {
        return false;
    }
    if !validate_digits(passport.get("iyr").unwrap(), 4, 2010, 2020) {
        return false;
    }
    if !validate_digits(passport.get("eyr").unwrap(), 4, 2020, 2030) {
        return false;
    }
    let hgt = passport.get("hgt").unwrap();
    if !((validate_regex(hgt, &Regex::new(r"^\d+cm$").unwrap())
        && validate_digits(&hgt[..3], 3, 150, 193))
        || (validate_regex(hgt, &Regex::new(r"^\d+in$").unwrap())
            && validate_digits(&hgt[..2], 2, 59, 76)))
    {
        return false;
    }
    if !validate_regex(
        passport.get("hcl").unwrap(),
        &Regex::new("^#[0-9a-f]{6}$").unwrap(),
    ) {
        return false;
    }
    if !validate_oneof(
        passport.get("ecl").unwrap(),
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].to_vec(),
    ) {
        return false;
    }
    if !validate_regex(
        passport.get("pid").unwrap(),
        &Regex::new(r"^\d{9}$").unwrap(),
    ) {
        return false;
    }
    true
}

fn validate_digits(digits: &str, len: usize, min: usize, max: usize) -> bool {
    if digits.len() != len {
        return false;
    }
    match digits.parse::<usize>() {
        Ok(digits) => digits >= min && digits <= max,
        Err(_) => false,
    }
}

fn validate_regex(s: &str, re: &Regex) -> bool {
    re.is_match(s)
}

fn validate_oneof(s: &str, options: Vec<&str>) -> bool {
    options.iter().any(|&option| option == s)
}

fn has_required_fields(passport: &Passport) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|f| passport.contains_key(&f.to_string()))
}
