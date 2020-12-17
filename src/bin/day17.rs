use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day17.txt").expect("Something went wrong reading the file");

    let original_world: HashMap<i64, HashMap<i64, HashMap<i64, Cell>>> = vec![(
        0,
        contents
            .lines()
            .enumerate()
            .map(|(y, l)| {
                let row = l
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let cell = match c {
                            '#' => Cell::Active,
                            _ => Cell::Inactive,
                        };

                        (i as i64, cell)
                    })
                    .collect::<HashMap<i64, Cell>>();
                (y as i64, row)
            })
            .collect::<HashMap<i64, HashMap<i64, Cell>>>(),
    )]
    .into_iter()
    .collect();

    let mut world = original_world.clone();

    for _ in 0..6 {
        world = evolve(&world);
    }

    let answer = world
        .iter()
        .map(|(_, grid)| {
            grid.iter()
                .map(|(_, row)| {
                    row.iter()
                        .map(|(_, cell)| if *cell == Cell::Active { 1 } else { 0 })
                        .fold(0, |acc, c| acc + c)
                })
                .fold(0, |acc, c| acc + c)
        })
        .fold(0, |acc, c| acc + c);

    println!("answer: {}", answer);

    let mut world4d = vec![(0, original_world)]
        .into_iter()
        .collect::<HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>>>();

    for _ in 0..6 {
        world4d = evolve4d(&world4d);
    }

    let answer = world4d
        .iter()
        .map(|(_, cube)| {
            cube.iter()
                .map(|(_, grid)| {
                    grid.iter()
                        .map(|(_, row)| {
                            row.iter()
                                .map(|(_, cell)| if *cell == Cell::Active { 1 } else { 0 })
                                .fold(0, |acc, c| acc + c)
                        })
                        .fold(0, |acc, c| acc + c)
                })
                .fold(0, |acc, c| acc + c)
        })
        .fold(0, |acc, c| acc + c);
    println!("answer2 {}", answer);
}

#[derive(PartialEq, Clone)]
enum Cell {
    Active,
    Inactive,
}

fn evolve(
    world: &HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>,
) -> HashMap<i64, HashMap<i64, HashMap<i64, Cell>>> {
    let mut new = HashMap::new();

    let (mut min_z, mut max_z, mut min_y, mut max_y, mut min_x, mut max_x) = (0, 0, 0, 0, 0, 0);

    for (z, grid) in world {
        if *z < min_z {
            min_z = *z
        }
        if *z > max_z {
            max_z = *z
        }
        for (y, row) in grid {
            if *y < min_y {
                min_y = *y
            }
            if *y > max_y {
                max_y = *y
            }

            for (x, _) in row {
                if *x < min_x {
                    min_x = *x
                }
                if *x > max_x {
                    max_x = *x
                }
            }
        }
    }

    for z in min_z - 1..=max_z + 1 {
        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let nc = neighbour_count(world, x, y, z);
                let cell = world
                    .get(&z)
                    .and_then(|grid| grid.get(&y))
                    .and_then(|row| row.get(&x))
                    .unwrap_or(&Cell::Inactive);
                if *cell == Cell::Active {
                    if nc == 2 || nc == 3 {
                        set_active(&mut new, x, y, z);
                    }
                } else if nc == 3 {
                    set_active(&mut new, x, y, z);
                }
            }
        }
    }

    new
}

fn set_active(world: &mut HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>, x: i64, y: i64, z: i64) {
    let grid = world.entry(z).or_insert(HashMap::new());
    let row = grid.entry(y).or_insert(HashMap::new());
    row.insert(x, Cell::Active);
}

fn neighbour_count(
    world: &HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>,
    x: i64,
    y: i64,
    z: i64,
) -> usize {
    let mut n = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx == dy && dy == dz && dx == 0 {
                    continue;
                }

                n += world
                    .get(&(z + dz))
                    .and_then(|grid| grid.get(&(y + dy)))
                    .and_then(|row| row.get(&(x + dx)))
                    .and_then(|c| if *c == Cell::Active { Some(1) } else { Some(0) })
                    .unwrap_or(0);
            }
        }
    }

    n
}

fn evolve4d(
    world: &HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>>,
) -> HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>> {
    let mut new = HashMap::new();

    let (mut min_w, mut max_w, mut min_z, mut max_z, mut min_y, mut max_y, mut min_x, mut max_x) =
        (0, 0, 0, 0, 0, 0, 0, 0);

    for (w, cubes) in world {
        if *w < min_w {
            min_w = *w
        }
        if *w > max_w {
            max_w = *w
        }
        for (z, grid) in cubes {
            if *z < min_z {
                min_z = *z
            }
            if *z > max_z {
                max_z = *z
            }
            for (y, row) in grid {
                if *y < min_y {
                    min_y = *y
                }
                if *y > max_y {
                    max_y = *y
                }

                for (x, _) in row {
                    if *x < min_x {
                        min_x = *x
                    }
                    if *x > max_x {
                        max_x = *x
                    }
                }
            }
        }
    }

    for w in min_w - 1..=max_w + 1 {
        for z in min_z - 1..=max_z + 1 {
            for y in min_y - 1..=max_y + 1 {
                for x in min_x - 1..=max_x + 1 {
                    let nc = neighbour_count_4d(world, x, y, z, w);
                    let cell = world
                        .get(&w)
                        .and_then(|cube| cube.get(&z))
                        .and_then(|grid| grid.get(&y))
                        .and_then(|row| row.get(&x))
                        .unwrap_or(&Cell::Inactive);
                    if *cell == Cell::Active {
                        if nc == 2 || nc == 3 {
                            set_active_4d(&mut new, x, y, z, w);
                        }
                    } else if nc == 3 {
                        set_active_4d(&mut new, x, y, z, w);
                    }
                }
            }
        }
    }

    new
}

fn set_active_4d(
    world: &mut HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>>,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
) {
    let cube = world.entry(w).or_insert(HashMap::new());
    let grid = cube.entry(z).or_insert(HashMap::new());
    let row = grid.entry(y).or_insert(HashMap::new());
    row.insert(x, Cell::Active);
}

fn neighbour_count_4d(
    world: &HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, Cell>>>>,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
) -> usize {
    let mut n = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dw == dz && dx == dy && dy == dz && dx == 0 {
                        continue;
                    }

                    n += world
                        .get(&(w + dw))
                        .and_then(|cube| cube.get(&(z + dz)))
                        .and_then(|grid| grid.get(&(y + dy)))
                        .and_then(|row| row.get(&(x + dx)))
                        .and_then(|c| if *c == Cell::Active { Some(1) } else { Some(0) })
                        .unwrap_or(0);
                }
            }
        }
    }

    n
}
