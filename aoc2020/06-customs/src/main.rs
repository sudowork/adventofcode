use std::collections::HashSet;
use util;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let groups = util::read_line_groups(input_file);

    let count = sum_declarations(get_declarations, &groups);
    println!("Total declaration count: {}", count);

    let count = sum_declarations(get_intersect_declarations, &groups);
    println!("Total intersecting declaration count: {}", count);
}

fn sum_declarations(
    group_fn: fn(&Vec<String>) -> HashSet<char>,
    groups: &Vec<Vec<String>>,
) -> usize {
    groups
        .iter()
        .map(group_fn)
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

fn get_intersect_declarations(group: &Vec<String>) -> HashSet<char> {
    group
        .iter()
        .map(|line| {
            line.chars().fold(HashSet::new(), |mut set, c| {
                set.insert(c);
                set
            })
        })
        .fold(('a'..='z').collect(), |a, b| {
            a.intersection(&b).cloned().collect()
        })
}
