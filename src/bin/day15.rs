use std::collections::HashMap;

fn main() {
    let input = vec![6, 3, 15, 13, 1, 0];

    println!("0,3,6 - {}", part1(&vec![0, 3, 6], 2020));
    println!("{:?} - {}", input, part1(&input, 2020));
    println!("{:?} - {}", input, part1(&input, 30000000));
}

fn part1(input: &Vec<u64>, remaining: u64) -> u64 {
    let mut last_n = 0;
    let mut g = Game::new();

    for n in input.iter() {
        last_n = g.play(*n);
    }

    while g.turn < remaining {
        last_n = g.play(last_n);
    }

    println!("{}", last_n);

    last_n
}

struct Game {
    seen: HashMap<u64, Vec<u64>>,
    turn: u64,
}

impl Game {
    fn new() -> Game {
        Game {
            seen: HashMap::new(),
            turn: 1,
        }
    }

    fn play(&mut self, n: u64) -> u64 {
        let turn = self.turn;
        let v = self
            .seen
            .entry(n)
            .and_modify(|v| v.push(turn))
            .or_insert(vec![turn]);

        let last_n = if v.len() == 1 {
            0
        } else {
            v[v.len() - 1] - v[v.len() - 2]
        };
        self.turn += 1;

        last_n
    }
}
