use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day6.txt").expect("Something went wrong reading the file");

    let answer1 = contents
        .split("\n\n")
        .map(|txt| {
            txt.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<char>>()
                .len()
        })
        .fold(0, |acc, c| acc + c);
    println!("{}", answer1);

    let answer2 = contents
        .trim_end()
        .split("\n\n")
        .map(|txt| {
            let declarations = txt
                .lines()
                .map(|l| {
                    l.chars()
                        .filter(|c| c.is_alphabetic())
                        .collect::<HashSet<char>>()
                })
                .collect::<Vec<HashSet<char>>>();
            declarations
                .iter()
                .fold(declarations[0].clone(), |acc, d| {
                    acc.intersection(d).cloned().collect()
                })
                .len()
        })
        .fold(0, |acc, c| acc + c);
    println!("{}", answer2);
}
