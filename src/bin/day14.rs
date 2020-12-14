use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day14.txt").expect("Something went wrong reading the file");

    println!(
        "answer: {}",
        part1("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0")
    );

    println!("answer: {}", part1(&contents));

    println!("answer: {}", part2("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1"));
    println!("answer: {}", part2(&contents));
}

fn part1(contents: &str) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = String::new();

    for line in contents.lines() {
        let l = String::from(line);
        if l.starts_with("mask = ") {
            mask = l[7..].to_string();
        }
        if l.starts_with("mem[") {
            let c = l.find("]").unwrap();
            let eq = l.find("=").unwrap();

            let addr = l[4..c].parse::<u64>().unwrap();
            let value = l[eq + 2..].parse::<u64>().unwrap();

            mem.insert(addr, apply_mask(&mask, value));
        }
    }

    mem.iter().fold(0, |acc, (_, value)| acc + value)
}

fn apply_mask(mask: &String, mut value: u64) -> u64 {
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '0' => value &= !(1 << i),
            '1' => value |= 1 << i,
            _ => {}
        }
    }

    value
}

fn part2(contents: &str) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = String::new();

    for line in contents.lines() {
        let l = String::from(line);
        if l.starts_with("mask = ") {
            mask = l[7..].to_string();
        }
        if l.starts_with("mem[") {
            let c = l.find("]").unwrap();
            let eq = l.find("=").unwrap();

            let addr = l[4..c].parse::<u64>().unwrap();
            let value = l[eq + 2..].parse::<u64>().unwrap();

            for a in expand_addr(addr, &mask).iter() {
                mem.insert(*a, value);
            }
        }
    }

    mem.iter().fold(0, |acc, (_, value)| acc + value)
}

fn expand_addr(addr: u64, mask: &String) -> Vec<u64> {
    let mut v = Vec::new();

    let nums = mask.chars().filter(|c| *c == 'X').count();

    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut xs = Vec::new();

    for (i, c) in mask
        .chars()
        .rev()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .into_iter()
        .rev()
    {
        let (and_bit, or_bit) = match c {
            '0' => (1, 0),
            '1' => (0, 1),
            'X' => {
                xs.push(i);
                (0, 0)
            }
            _ => continue,
        };

        and_mask <<= 1;
        and_mask |= and_bit;
        or_mask <<= 1;
        or_mask |= or_bit;
    }

    for bitset in 0..(1 << nums) {
        let mut new_or_mask = or_mask;
        for (i, x) in xs.iter().enumerate() {
            new_or_mask |= (if bitset & (1 << i) > 0 { 1 } else { 0 }) << x
        }

        v.push((addr & and_mask) | new_or_mask);
    }

    v
}
