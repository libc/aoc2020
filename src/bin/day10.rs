use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day10.txt").expect("Something went wrong reading the file");

    let mut numbers = contents
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    numbers.sort();

    let mut prev = 0;
    let mut one_diff = 0;
    let mut three_diff = 1;

    for v in numbers.iter() {
        if v - prev == 1 {
            one_diff += 1;
        }
        if v - prev == 3 {
            three_diff += 1;
        }
        prev = *v;
    }

    println!("{:?}", one_diff * three_diff);

    let mut data1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    data1.sort();
    println!("{}", number_of_ways(&data1));
    let mut data2 = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    data2.sort();
    println!("{}", number_of_ways(&data2));
    println!("{}", number_of_ways(&numbers));
}

fn number_of_ways(num: &Vec<u64>) -> u64 {
    let mut sums = Vec::new();
    sums.resize(num.len(), 0);

    for i in 0..3 {
        if num[i] <= 3 {
            sums[i] = 1
        }
    }

    for i in 0..num.len() {
        let n = num[i];
        for j in i + 1..i + 4 {
            if j < num.len() && num[j] - n <= 3 {
                sums[j] += sums[i]
            }
        }
    }

    sums[num.len() - 1]
}
