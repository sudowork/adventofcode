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
