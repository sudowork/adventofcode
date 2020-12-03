use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    static ref PASSWORD_RE: Regex =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<char>.): (?P<pass>.*)$").unwrap();
}

#[derive(Debug)]
struct Rule {
    min: usize,
    max: usize,
    ch: char,
}

#[derive(Debug)]
struct Password {
    value: String,
    rule: Rule,
}

impl Password {
    fn is_valid(&self) -> bool {
        let char_count = self.value.chars().filter(|c| *c == self.rule.ch).count();
        char_count >= self.rule.min && char_count <= self.rule.max
    }

    fn is_new_valid(&self) -> bool {
        [self.rule.min, self.rule.max]
            .iter()
            .map(|pos| self.value.chars().nth(pos - 1).unwrap())
            .filter(|ch| *ch == self.rule.ch)
            .count()
            == 1
    }
}

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let passwords: Vec<_> = input.iter().map(|l| parse_line(l)).collect();
    let num_valid = passwords.iter().filter(|p| p.is_valid()).count();
    println!("Number of valid passwords: {}", num_valid);

    let num_valid = passwords.iter().filter(|p| p.is_new_valid()).count();
    println!("Number of valid passwords using new rule: {}", num_valid);
}

fn parse_line(line: &String) -> Password {
    let caps = PASSWORD_RE.captures(line.as_str()).unwrap();
    Password {
        value: String::from(caps.name("pass").unwrap().as_str()),
        rule: Rule {
            min: parse_size(caps.name("min")),
            max: parse_size(caps.name("max")),
            ch: caps.name("char").unwrap().as_str().chars().next().unwrap(),
        },
    }
}

fn parse_size(cap: Option<Match>) -> usize {
    cap.unwrap().as_str().parse().unwrap()
}
