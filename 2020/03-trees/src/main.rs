type Geology = Vec<Vec<bool>>;

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    let geo = parse_geo(input);

    // part 1
    let (x, y) = (3, 1);
    let num_trees = count_trees(&geo, x, y);
    println!(
        "Num trees at slope -{y}/{x}: {count}",
        x = x,
        y = y,
        count = num_trees
    );

    // part 2
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result = slopes
        .iter()
        .map(|(x, y)| count_trees(&geo, *x, *y))
        .fold(1, |total, x| total * x);
    println!("Product of trees on slopes: {}", result);
}

fn parse_geo(lines: Vec<String>) -> Geology {
    lines
        .iter()
        .map(|line| line.chars().map(|ch| ch == '#').collect())
        .collect()
}

fn count_trees(slope: &Geology, x: usize, y: usize) -> usize {
    let (rows, cols) = geo_shape(&slope);
    let mut count = 0;
    for i in 0.. {
        let row = i * y;
        if row >= rows {
            break;
        }
        let col = (i * x) % cols;
        count += slope[row][col] as usize
    }
    count
}

fn geo_shape(slope: &Geology) -> (usize, usize) {
    (slope.len(), slope[0].len())
}
