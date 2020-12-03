use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filepath: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|l| !l.is_empty())
        .collect()
}

pub fn get_input_file(default: &str) -> String {
    env::args().skip(1).next().unwrap_or(default.to_string())
}
