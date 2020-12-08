use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    bag: String,
    contains: Vec<(String, usize)>,
}

#[derive(Debug)]
struct Bag {
    color: String,
    contains: Vec<(String, usize)>,
    parents: Vec<(String, usize)>,
}

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let rules = parse_rules(&input);
    let bag_map = create_bag_map(&rules);
    println!("Total bag colors: {:?}", bag_map.keys().len());

    // part 1
    let containers = bags_containing(&bag_map, "shiny gold");
    println!(
        "Bag colors containing {}: {}",
        "shiny gold",
        containers.len()
    );

    // part 2
    let bag_count = bags_contained(&bag_map, "shiny gold");
    println!("Individuals bags containing: {}", bag_count - 1);
}

fn bags_containing(map: &HashMap<&str, Bag>, color: &str) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();
    let mut visit = |bag: &Bag| {
        set.insert(bag.color.to_string());
    };
    dfs(map, color, &mut visit, true);
    set.remove(color);
    set.clone()
}

fn bags_contained(map: &HashMap<&str, Bag>, color: &str) -> usize {
    let bag = map.get(color).unwrap();
    if bag.contains.is_empty() {
        return 1;
    }
    1 + bag
        .contains
        .iter()
        // TODO: Could optimize with memoization
        .map(|(color, ct)| ct * bags_contained(map, color))
        .fold(0, |a, b| a + b)
}

fn parse_rules(lines: &Vec<String>) -> Vec<Rule> {
    lines.iter().map(|l| parse_rule(l)).collect()
}

fn parse_rule(line: &str) -> Rule {
    let parts: Vec<&str> = line.split(" bags contain ").collect();
    let (container_part, contains_part) = (parts[0], parts[1]);

    let contains_bag_re = Regex::new(r"(?P<count>\d+) (?P<color>[^ ]+ [^ ]+) bag").unwrap();
    let mut vec: Vec<(String, usize)> = Vec::new();
    for cap in contains_bag_re.captures_iter(contains_part) {
        vec.push((cap["color"].to_string(), *&cap["count"].parse().unwrap()));
    }
    Rule {
        bag: container_part.to_string(),
        contains: vec,
    }
}

fn create_bag_map(rules: &Vec<Rule>) -> HashMap<&str, Bag> {
    let mut map = HashMap::new();
    for rule in rules {
        init_bag(&mut map, rule.bag.as_str());
        map.get_mut(rule.bag.as_str())
            .unwrap()
            .contains
            .extend(rule.contains.iter().cloned());
        // update parents
        for (color, count) in rule.contains.iter() {
            init_bag(&mut map, color);
            map.get_mut(color.as_str())
                .unwrap()
                .parents
                .push((rule.bag.clone(), *count));
        }
    }
    map
}

fn init_bag<'a>(map: &mut HashMap<&'a str, Bag>, color: &'a str) {
    let color = color;
    if !map.contains_key(color) {
        map.insert(
            color,
            Bag {
                color: color.to_string(),
                contains: vec![],
                parents: vec![],
            },
        );
    }
}

fn dfs<F>(map: &HashMap<&str, Bag>, color: &str, visit: &mut F, ascend: bool)
where
    F: FnMut(&Bag),
{
    let mut stack: Vec<Option<&Bag>> = Vec::new();
    stack.push(map.get(color));
    while !stack.is_empty() {
        let bag = stack.pop().unwrap();
        if bag.is_none() {
            continue;
        }
        let bag = bag.unwrap();
        visit(bag);
        let iter = if ascend {
            bag.parents.iter()
        } else {
            bag.contains.iter()
        };
        stack.extend(
            iter.map(|(c, _)| map.get(c.as_str()))
                .collect::<Vec<Option<&Bag>>>(),
        );
    }
}
