use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day1.txt").expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let mut answer = 1;

    for (a_idx, a) in numbers.iter().enumerate() {
        for (b_idx, b) in numbers.iter().enumerate() {
            if a_idx < b_idx && a + b == 2020 {
                println!("found pair  {} {}", a, b);
                answer *= a * b;
            }
        }
    }

    println!("answer {}", answer);

    answer = 1;

    for (a_idx, a) in numbers.iter().enumerate() {
        for (b_idx, b) in numbers.iter().enumerate() {
            for (c_idx, c) in numbers.iter().enumerate() {
                if a_idx < b_idx && b_idx < c_idx && a + b + c == 2020 {
                    println!("found pair  {} {} {}", a, b, c);
                    answer *= a * b * c;
                }
            }
        }
    }

    println!("answer {}", answer);
}
