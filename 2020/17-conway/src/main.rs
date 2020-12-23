use std::cmp;
use std::collections::HashSet;

type Coord = (isize, isize, isize);
type Bound = (isize, isize);

fn main() {
    let input_file = util::get_input_file("./input.txt");
    let input = util::read_lines(input_file);

    // part 1
    let actives = parse_input(&input);
    let actives = run(&actives, 6);
    println!("Num actives: {}", actives.len());
}

fn run(actives: &HashSet<Coord>, iterations: usize) -> HashSet<Coord> {
    let mut curr = actives.clone();
    for _ in 0..iterations {
        curr = _run(&curr);
    }
    curr
}

fn _run(actives: &HashSet<Coord>) -> HashSet<Coord> {
    let mut new_actives = HashSet::new();
    let ((minx, maxx), (miny, maxy), (minz, maxz)) = find_bounds(actives);
    for x in minx..=maxx {
        for y in miny..=maxy {
            for z in minz..=maxz {
                if should_activate((x, y, z), actives) {
                    new_actives.insert((x, y, z));
                }
            }
        }
    }
    new_actives
}

/// Returns the bounding box +/- 1 for the 3-D space.
fn find_bounds(actives: &HashSet<Coord>) -> (Bound, Bound, Bound) {
    let init = (
        (isize::MAX, isize::MIN),
        (isize::MAX, isize::MIN),
        (isize::MAX, isize::MIN),
    );
    actives.iter().fold(
        init,
        |((minx, maxx), (miny, maxy), (minz, maxz)), (x, y, z)| {
            (
                (cmp::min(minx, *x - 1), cmp::max(maxx, *x + 1)),
                (cmp::min(miny, *y - 1), cmp::max(maxy, *y + 1)),
                (cmp::min(minz, *z - 1), cmp::max(maxz, *z + 1)),
            )
        },
    )
}

fn should_activate(coords: Coord, actives: &HashSet<Coord>) -> bool {
    let (x, y, z) = coords;
    let mut neighbors = 0;
    for x_ in -1..=1 {
        for y_ in -1..=1 {
            for z_ in -1..=1 {
                if x_ == 0 && y_ == 0 && z_ == 0 {
                    continue;
                }
                if actives.contains(&(x + x_, y + y_, z + z_)) {
                    neighbors += 1;
                }
            }
        }
    }
    if actives.contains(&(x, y, z)) {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}

fn parse_input(input: &[String]) -> HashSet<Coord> {
    let mut actives = HashSet::new();
    for (x, line) in input.iter().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                actives.insert((x as isize, y as isize, 0));
            }
        }
    }
    actives
}
