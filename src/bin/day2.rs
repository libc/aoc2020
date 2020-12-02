use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day2.txt").expect("Something went wrong reading the file");

    let input: Vec<Line> = contents.lines().map(|l| Line::parse(&l)).collect();

    println!("answer 1: {}", input.iter().filter(|l| l.valid()).count());
    println!("answer 2: {}", input.iter().filter(|l| l.valid2()).count());
}

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Line {
    fn parse(s: &str) -> Line {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();

        Line {
            min: caps[1].parse::<usize>().unwrap(),
            max: caps[2].parse::<usize>().unwrap(),
            letter: caps[3].chars().nth(0).unwrap(),
            password: caps[4].to_string(),
        }
    }

    fn valid(&self) -> bool {
        let n = self.password.chars().filter(|&c| c == self.letter).count();
        self.min <= n && n <= self.max
    }

    fn valid2(&self) -> bool {
        let l1 = self.password.chars().nth(self.min - 1).unwrap_or('\0');
        let l2 = self.password.chars().nth(self.max - 1).unwrap_or('\0');

        (l1 == self.letter && l2 != self.letter) || (l2 == self.letter && l1 != self.letter)
    }
}
