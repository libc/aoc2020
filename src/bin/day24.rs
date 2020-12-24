use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day24.txt").expect("cannot read");

    let mut tiles: HashMap<(i64, i64), bool> = HashMap::new();

    contents.lines().for_each(|l| {
        apply_line(&mut tiles, l);
    });

    println!("answer: {}", tiles.iter().filter(|(_, c)| **c).count());

    for day in 1..=100 {
        tiles = evolve(&tiles);
        println!(
            "day {} answer: {}",
            day,
            tiles.iter().filter(|(_, c)| **c).count()
        );
    }
    println!("answer: {}", tiles.iter().filter(|(_, c)| **c).count());
}

fn apply_line(tiles: &mut HashMap<(i64, i64), bool>, input: &str) {
    let mut south = false;
    let mut north = false;

    let mut q = 0;
    let mut r = 0;

    for c in input.chars() {
        match c {
            'e' => {
                if south {
                    r += 1;
                } else if north {
                    q += 1;
                    r -= 1;
                } else {
                    q += 1;
                }
            }
            'w' => {
                if south {
                    q -= 1;
                    r += 1;
                } else if north {
                    r -= 1;
                } else {
                    q -= 1;
                }
            }
            's' => {
                south = true;
                continue;
            }
            'n' => {
                north = true;
                continue;
            }
            _ => panic!(format!("unknown char: {}", c)),
        }

        south = false;
        north = false;
    }
    tiles.entry((q, r)).and_modify(|v| *v = !*v).or_insert(true);
}

fn evolve(tiles: &HashMap<(i64, i64), bool>) -> HashMap<(i64, i64), bool> {
    let mut new_tiles = HashMap::new();

    let (min_x, max_x, min_y, max_y) =
        tiles
            .iter()
            .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), ((x, y), _)| {
                (
                    if *x < min_x { *x } else { min_x },
                    if *x > max_x { *x } else { max_x },
                    if *y < min_y { *y } else { min_y },
                    if *y > max_y { *y } else { max_y },
                )
            });

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let neighbors = vec![(0, -1), (1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0)]
                .into_iter()
                .map(|(dx, dy)| *tiles.get(&(x + dx, y + dy)).unwrap_or(&false))
                .fold(0, |acc, t| acc + if t { 1 } else { 0 });

            if *tiles.get(&(x, y)).unwrap_or(&false) {
                if neighbors == 1 || neighbors == 2 {
                    new_tiles.insert((x, y), true);
                }
            } else {
                if neighbors == 2 {
                    new_tiles.insert((x, y), true);
                }
            }
        }
    }
    new_tiles
}
