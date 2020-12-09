use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day9.txt").expect("Something went wrong reading the file");

    let numbers = contents
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let answer1 = not_a_sum(&numbers);
    println!("answer: {}", answer1);

    let (i, j) = range_answer2(&numbers, answer1);
    println!(
        "answer: {}",
        numbers[i..=j].iter().min().unwrap() + numbers[i..=j].iter().max().unwrap()
    );
}

fn not_a_sum(numbers: &Vec<u64>) -> u64 {
    for (i, n) in numbers.iter().enumerate() {
        if i < 25 {
            continue;
        }

        let mut found = false;
        for j in i - 25..i {
            for k in i - 25..i {
                if numbers[j] == numbers[k] {
                    continue;
                }

                if numbers[j] + numbers[k] == *n {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return *n;
        }
    }
    panic!("not found");
}

fn range_answer2(numbers: &Vec<u64>, answer1: u64) -> (usize, usize) {
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == answer1 {
                return (i, j);
            }
            if sum > answer1 {
                break;
            }
        }
    }
    panic!("failed to find the number");
}
