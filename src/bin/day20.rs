use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day20.txt").unwrap();

    let tiles: HashMap<u64, Vec<Vec<char>>> =
        contents.split("\n\n").map(|t| parse_tile(t)).collect();

    let neighbours = find_neighbours(&tiles);

    println!(
        "answer: {}",
        neighbours
            .iter()
            .filter(|(_, n)| n.len() == 2)
            .fold(1, |acc, (id, _)| acc * id)
    );

    let tile_image = stitch_image(&neighbours, &tiles);

    for mut img in combinations(&tile_image) {
        if place_sea_monsters(&mut img) {
            print_tile(&img);

            println!(
                "answer: {}",
                img.iter()
                    .map(|row| row
                        .iter()
                        .fold(0, |acc, c| acc + if *c == '#' { 1 } else { 0 }))
                    .fold(0, |acc, r| acc + r)
            );
            break;
        }
    }
}

fn parse_tile(t: &str) -> (u64, Vec<Vec<char>>) {
    let id_str = t.lines().nth(0).unwrap();
    let id = id_str
        .get(5..id_str.len() - 1)
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let tile_contents = t
        .lines()
        .skip(1)
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    (id, tile_contents.to_vec())
}

fn find_neighbours(tiles: &HashMap<u64, Vec<Vec<char>>>) -> HashMap<u64, Vec<u64>> {
    let borders: HashMap<u64, HashSet<Vec<char>>> = tiles
        .iter()
        .map(|(id, tile)| {
            let top = extract_top(tile);
            let left = extract_left(tile);
            let right = extract_right(tile);
            let bottom = extract_bottom(tile);
            (
                *id,
                [
                    rev(&top),
                    rev(&left),
                    rev(&right),
                    rev(&bottom),
                    top,
                    left,
                    right,
                    bottom,
                ]
                .iter()
                .cloned()
                .collect::<HashSet<Vec<char>>>(),
            )
        })
        .collect();

    borders
        .iter()
        .map(|(id, tile_borders)| {
            (
                *id,
                borders
                    .iter()
                    .filter(|(id2, bs)| *id2 != id && !tile_borders.is_disjoint(bs))
                    .map(|(id, _)| *id)
                    .collect(),
            )
        })
        .collect()
}

fn extract_top(tile: &Vec<Vec<char>>) -> Vec<char> {
    tile[0].clone()
}

fn extract_bottom(tile: &Vec<Vec<char>>) -> Vec<char> {
    tile[tile.len() - 1].clone()
}

fn extract_left(tile: &Vec<Vec<char>>) -> Vec<char> {
    tile.iter().map(|t| t.get(0).unwrap()).cloned().collect()
}

fn extract_right(tile: &Vec<Vec<char>>) -> Vec<char> {
    tile.iter()
        .map(|t| t.get(t.len() - 1).unwrap())
        .cloned()
        .collect()
}

fn rev(v: &Vec<char>) -> Vec<char> {
    v.iter().rev().cloned().collect()
}

fn stitch_image(
    neighbours: &HashMap<u64, Vec<u64>>,
    tiles: &HashMap<u64, Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    let (corner, corner_neighbours) = neighbours.iter().find(|(_, n)| n.len() == 2).unwrap();

    let (mut image, right_tile, right_id, bottom_tile, bottom_id) = assemble_corner(
        &tiles[corner],
        corner_neighbours[0],
        &tiles[&corner_neighbours[0]],
        corner_neighbours[1],
        &tiles[&corner_neighbours[1]],
    );

    let mut placed = HashSet::new();
    let mut queue = Vec::new();

    queue.push((right_id, right_tile, 0, 1));
    queue.push((bottom_id, bottom_tile, 1, 0));
    placed.insert(*corner);
    placed.insert(right_id);
    placed.insert(bottom_id);

    while !queue.is_empty() {
        let (id, tile, y, x) = queue.pop().unwrap();

        let right = extract_right(&tile);
        let bottom = extract_bottom(&tile);

        for n_id in neighbours[&id].iter() {
            if placed.contains(&n_id) {
                continue;
            }

            for candidate in combinations(&tiles[&n_id]) {
                if extract_left(&candidate) == right {
                    place(&mut image, &candidate, y, x + 1);
                    placed.insert(*n_id);
                    queue.push((*n_id, candidate, y, x + 1));
                    break;
                }

                if extract_top(&candidate) == bottom {
                    place(&mut image, &candidate, y + 1, x);
                    placed.insert(*n_id);
                    queue.push((*n_id, candidate, y + 1, x));
                    break;
                }
            }
        }
    }

    image
}

fn print_tile(tile: &Vec<Vec<char>>) {
    for row in tile {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
    println!("");
}

fn assemble_corner(
    corner: &Vec<Vec<char>>,
    tile1_id: u64,
    tile1: &Vec<Vec<char>>,
    tile2_id: u64,
    tile2: &Vec<Vec<char>>,
) -> (Vec<Vec<char>>, Vec<Vec<char>>, u64, Vec<Vec<char>>, u64) {
    for tile in combinations(corner) {
        let right = extract_right(&tile);
        let bottom = extract_bottom(&tile);

        for tile1_candidate in combinations(tile1) {
            if extract_left(&tile1_candidate) == right {
                for tile2_candidate in combinations(tile2) {
                    if extract_top(&tile2_candidate) == bottom {
                        return (
                            connect_corner(&tile, &tile1_candidate, &tile2_candidate),
                            tile1_candidate,
                            tile1_id,
                            tile2_candidate,
                            tile2_id,
                        );
                    }
                }
            }

            if extract_top(&tile1_candidate) == bottom {
                for tile2_candidate in combinations(tile2) {
                    if extract_left(&tile2_candidate) == right {
                        return (
                            connect_corner(&tile, &tile2_candidate, &tile1_candidate),
                            tile2_candidate,
                            tile2_id,
                            tile1_candidate,
                            tile1_id,
                        );
                    }
                }
            }
        }
    }
    panic!("no matches")
}

fn connect_corner(
    top: &Vec<Vec<char>>,
    right: &Vec<Vec<char>>,
    bottom: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut r = Vec::new();

    place(&mut r, top, 0, 0);
    place(&mut r, right, 0, 1);
    place(&mut r, bottom, 1, 0);

    r
}

fn combinations(tile: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let flipped = flip(&tile);
    [
        tile.clone(),
        rotate(tile),
        rotate(&rotate(tile)),
        rotate(&rotate(&rotate(tile))),
        flip(tile),
        rotate(&flipped),
        rotate(&rotate(&flipped)),
        rotate(&rotate(&rotate(&flipped))),
    ]
    .to_vec()
}

fn rotate(tile: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..tile.len())
        .map(|i| tile.iter().map(|r| r[i]).rev().collect())
        .collect()
}

fn flip(tile: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    tile.iter().map(|t| rev(t)).collect()
}

fn place(image: &mut Vec<Vec<char>>, tile: &Vec<Vec<char>>, y: usize, x: usize) {
    let len_x = tile[0].len() - 2;
    let len_y = tile.len() - 2;

    let real_x = len_x * x;
    let real_y = len_y * y;

    if image.len() < real_y + len_y {
        image.resize_with(real_y + len_y, || Vec::new())
    }
    for y in 0..image.len() {
        if image[y].len() < real_x + len_x {
            image[y].resize_with(real_x + len_x, || ' ')
        }
    }

    for dy in 0..len_y {
        for dx in 0..len_x {
            image[real_y + dy][real_x + dx] = tile[dy + 1][dx + 1];
        }
    }
}

fn place_sea_monsters(vec: &mut Vec<Vec<char>>) -> bool {
    let sea_monster: Vec<(usize, usize)> = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .into_iter()
    .enumerate()
    .flat_map(|(y, row)| {
        [y].iter()
            .cycle()
            .zip(row.chars().enumerate())
            .map(|(y, (x, c))| (c, *y, x))
            .collect::<Vec<(char, usize, usize)>>()
    })
    .filter(|(c, _, _)| *c == '#')
    .map(|(_, y, x)| (y, x))
    .collect();

    let mut there_be_monsters = false;

    for y in 0..vec.len() {
        for x in 0..vec[0].len() {
            if sea_monster.iter().all(|(dy, dx)| {
                *(vec
                    .get(y + dy)
                    .and_then(|row| row.get(x + dx))
                    .unwrap_or(&' '))
                    == '#'
            }) {
                there_be_monsters = true;
                sea_monster
                    .iter()
                    .for_each(|(dy, dx)| vec[y + dy][x + dx] = 'o')
            }
        }
    }

    there_be_monsters
}
