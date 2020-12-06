use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let lines = read_lines_unfiltered(filepath);
    lines.iter().cloned().filter(|l| !l.is_empty()).collect()
}

pub fn read_line_groups<P>(filepath: P) -> Vec<Vec<String>>
where
    P: AsRef<Path>,
{
    let lines = read_lines_unfiltered(filepath);
    let mut vec = Vec::new();
    vec.push(Vec::new());
    lines.iter().fold(vec, |mut groups, line| {
        if line.is_empty() {
            groups.push(Vec::new());
        } else {
            groups.last_mut().unwrap().push(line.to_string());
        }
        groups
    })
}

pub fn read_lines_unfiltered<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|l| String::from(l.unwrap().trim())).collect()
}

pub fn get_input_file(default: &str) -> String {
    env::args().skip(1).next().unwrap_or(default.to_string())
}
