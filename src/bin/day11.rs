use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day11.txt").expect("Something went wrong reading the file");

    let original_map = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Cell::Floor,
                    'L' => Cell::Seat,
                    '#' => Cell::Person,
                    _ => panic!(format!("unknown cell {}", c)),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let mut map = original_map
        .iter()
        .map(|row| row.iter().cloned().collect())
        .collect();

    loop {
        let (new_map, changed) = evolution_step(&map, empty_around, too_crowded);

        if !changed {
            break;
        }

        map = new_map;
    }

    println!(
        "answer: {}",
        map.iter()
            .map(|row| row.iter().filter(|c| **c == Cell::Person).count())
            .fold(0, |acc, seats| acc + seats)
    );

    map = original_map
        .iter()
        .map(|row| row.iter().cloned().collect())
        .collect();
    loop {
        let (new_map, changed) = evolution_step(&map, empty_around2, too_crowded2);

        if !changed {
            break;
        }

        map = new_map;
    }

    println!(
        "answer: {}",
        map.iter()
            .map(|row| row.iter().filter(|c| **c == Cell::Person).count())
            .fold(0, |acc, seats| acc + seats)
    );
}

fn evolution_step(
    m: &Vec<Vec<Cell>>,
    empty_check: fn(&Vec<Vec<Cell>>, usize, usize) -> bool,
    crowded_check: fn(&Vec<Vec<Cell>>, usize, usize) -> bool,
) -> (Vec<Vec<Cell>>, bool) {
    let mut changed = false;
    let new_map = m
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| match cell {
                    Cell::Floor => Cell::Floor,
                    Cell::Seat => {
                        if empty_check(m, x, y) {
                            changed = true;
                            Cell::Person
                        } else {
                            Cell::Seat
                        }
                    }
                    Cell::Person => {
                        if crowded_check(m, x, y) {
                            changed = true;
                            Cell::Seat
                        } else {
                            Cell::Person
                        }
                    }
                })
                .collect()
        })
        .collect();
    (new_map, changed)
}

fn empty_around(m: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    {
        let xx = x as i32 + dx;
        let yy = y as i32 + dy;

        if xx < 0 || xx >= m[0].len() as i32 {
            continue;
        }
        if yy < 0 || yy >= m.len() as i32 {
            continue;
        }

        if m[yy as usize][xx as usize] == Cell::Person {
            return false;
        }
    }

    return true;
}

fn too_crowded(m: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let mut occupied = 0;
    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    {
        let xx = x as i32 + dx;
        let yy = y as i32 + dy;

        if xx < 0 || xx >= m[0].len() as i32 {
            continue;
        }
        if yy < 0 || yy >= m.len() as i32 {
            continue;
        }

        if m[yy as usize][xx as usize] == Cell::Person {
            occupied += 1
        }
    }
    occupied >= 4
}

fn neighbours(m: &Vec<Vec<Cell>>, x: usize, y: usize) -> Vec<Cell> {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .map(|(dx, dy)| {
        let mut c = Cell::Floor;
        let mut xx = x as i32;
        let mut yy = y as i32;

        loop {
            xx += dx;
            yy += dy;

            if xx < 0 || xx >= m[0].len() as i32 {
                break;
            }
            if yy < 0 || yy >= m.len() as i32 {
                break;
            }

            if m[yy as usize][xx as usize] != Cell::Floor {
                c = m[yy as usize][xx as usize].clone();
                break;
            }
        }
        c
    })
    .collect()
}

fn empty_around2(m: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    neighbours(m, x, y).iter().all(|c| *c != Cell::Person)
}

fn too_crowded2(m: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    neighbours(m, x, y)
        .iter()
        .filter(|c| **c == Cell::Person)
        .count()
        >= 5
}

#[derive(PartialEq, Clone)]
enum Cell {
    Floor,
    Seat,
    Person,
}
