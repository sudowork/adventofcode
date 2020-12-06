use std::collections::HashSet;
use util;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let groups = util::read_line_groups(input_file);

    let count = get_total_declarations(&groups);
    println!("Total declaration count: {}", count);
}

fn get_total_declarations(groups: &Vec<Vec<String>>) -> usize {
    groups
        .iter()
        .map(|g| get_declarations(g))
        .map(|ds| ds.len())
        .fold(0, |agg, ds| agg + ds)
}

fn get_declarations(group: &Vec<String>) -> HashSet<char> {
    group
        .iter()
        .flat_map(|line| line.chars())
        .fold(HashSet::new(), |mut set, c| {
            set.insert(c);
            set
        })
}
