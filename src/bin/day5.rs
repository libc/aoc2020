use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day5.txt").expect("Something went wrong reading the file");

    let seats_in_the_file: HashSet<u32> = contents
        .lines()
        .map(|l| boarding_pass_to_rc(l))
        .map(|rc| seat_id(rc))
        .collect();

    println!("answer: {}", seats_in_the_file.iter().max().unwrap());

    let all_seats: HashSet<u32> = (0..=127)
        .flat_map(|row| {
            [row]
                .iter()
                .cycle()
                .zip(1..=7)
                .map(|(row, col)| seat_id((*row, col)))
                .collect::<Vec<u32>>()
        })
        .collect();

    let empty_seats: Vec<u32> = all_seats
        .difference(&seats_in_the_file)
        .filter(|&sid| {
            seats_in_the_file.contains(&(sid - 1)) && seats_in_the_file.contains(&(sid + 1))
        })
        .cloned()
        .collect();

    println!("{:?}", empty_seats);
}

fn boarding_pass_to_rc(input: &str) -> (u32, u32) {
    let mut row_low = 0;
    let mut row_high = 127;
    let mut col_low = 0;
    let mut col_high = 7;

    for c in input.chars() {
        match c {
            'F' => row_high -= upper_half(row_low, row_high),
            'B' => row_low += lower_half(row_low, row_high),
            'L' => col_high -= upper_half(col_low, col_high),
            'R' => col_low += lower_half(col_low, col_high),
            _ => {}
        }
    }

    (row_low, col_low)
}

fn upper_half(start: u32, end: u32) -> u32 {
    (((end as f64) - (start as f64)) / 2_f64).floor() as u32
}

fn lower_half(start: u32, end: u32) -> u32 {
    (((end as f64) - (start as f64)) / 2_f64).ceil() as u32
}

fn seat_id(rc: (u32, u32)) -> u32 {
    let (row, column) = rc;

    row * 8 + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boarding_pass() {
        assert_eq!(boarding_pass_to_rc("FBFBBFFRLR"), (44, 5));
        assert_eq!(boarding_pass_to_rc("BFFFBBFRRR"), (70, 7));
        assert_eq!(boarding_pass_to_rc("FFFBBBFRRR"), (14, 7));
        assert_eq!(boarding_pass_to_rc("BBFFBBFRLL"), (102, 4));
    }
}
