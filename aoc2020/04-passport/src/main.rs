use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use util;

macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

type Passport = HashMap<String, String>;
type ValidationRule = Box<dyn Fn(&str) -> bool>;

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
    let validation_rules = hashmap! {
        "byr" => vec![is_len(4), is_parsable::<usize>(), is_bounded(1920, 2002)],
        "iyr" => vec![is_len(4), is_parsable::<usize>(), is_bounded(2010, 2020)],
        "eyr" => vec![is_len(4), is_parsable::<usize>(), is_bounded(2020, 2030)],
        "hcl" => vec![matches_regex(Regex::new(r"^#[0-9a-f]{6}$").unwrap())],
        "ecl" => vec![is_one_of(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].to_vec())],
        "pid" => vec![matches_regex(Regex::new(r"^\d{9}$").unwrap())],
        "hgt" => vec![Box::new(|hgt: &str|
            (matches_regex(Regex::new(r"^\d+cm$").unwrap())(hgt)
                && all(vec![is_parsable::<usize>(), is_bounded(150, 193)])(&hgt[..3]))
            || (matches_regex(Regex::new(r"^\d+in$").unwrap())(hgt)
                && all(vec![is_parsable::<usize>(), is_bounded(59, 76)])(&hgt[..2]))
        )]
    };
    validate_passport(passport, &validation_rules)
}

fn has_required_fields(passport: &Passport) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|f| passport.contains_key(&f.to_string()))
}

fn validate_passport(passport: &Passport, rules: &HashMap<&str, Vec<ValidationRule>>) -> bool {
    for (&key, rules) in rules.iter() {
        let value = passport.get(key).unwrap();
        if !rules.iter().all(|rule| rule(value)) {
            return false;
        }
    }
    true
}

fn is_len(len: usize) -> ValidationRule {
    Box::new(move |val: &str| val.len() == len)
}

fn is_parsable<T>() -> ValidationRule
where
    T: FromStr,
{
    Box::new(|val: &str| val.parse::<T>().is_ok())
}

fn is_bounded(min: usize, max: usize) -> ValidationRule {
    Box::new(move |val: &str| {
        let val: usize = val.parse().unwrap();
        val >= min && val <= max
    })
}

fn matches_regex(re: regex::Regex) -> ValidationRule {
    Box::new(move |val: &str| re.is_match(val))
}

fn is_one_of(options: Vec<&'static str>) -> ValidationRule {
    Box::new(move |val: &str| options.iter().any(|&option| option == val))
}

fn all(rules: Vec<ValidationRule>) -> ValidationRule {
    Box::new(move |val: &str| rules.iter().all(|rule| rule(val)))
}
