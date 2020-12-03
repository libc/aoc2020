use std::fs;
use std::ops::Index;

fn main() {
    let contents =
        fs::read_to_string("input/day3.txt").expect("Something went wrong reading the file");

    let m = Map::from_string(&contents);

    println!("answer: {}", m.count_trees(3, 1));

    println!(
        "answer: {}",
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|(dx, dy)| m.count_trees(dx, dy))
            .fold(1, |acc, trees| acc * trees)
    );
}

struct Map {
    cells: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Map {
    fn from_string(contents: &String) -> Map {
        let cells: Vec<Vec<Cell>> = contents
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Cell::Empty,
                        '#' => Cell::Tree,
                        _ => panic!(format!("unknown character {:?}", c)),
                    })
                    .collect()
            })
            .collect();

        Map {
            height: cells.len(),
            width: cells[0].len(),
            cells: cells,
        }
    }

    fn count_trees(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;

        while y < self.height {
            if self[(x, y)] == Cell::Tree {
                trees += 1;
            }
            x += dx;
            y += dy;
        }

        trees
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Cell;

    fn index(&self, xy: (usize, usize)) -> &Self::Output {
        let (x, y) = xy;

        &self.cells[y][x % self.width]
    }
}

#[derive(PartialEq)]
enum Cell {
    Empty,
    Tree,
}
