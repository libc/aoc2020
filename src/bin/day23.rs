use std::collections::VecDeque;
use std::io;
use std::io::Write;

fn main() {
    println!("test answer: {}", play("389125467"));
    println!("answer: {}", play("974618352"));
    println!("test answer: {}", play2(&vec![3, 8, 9, 1, 2, 5, 4, 6, 7]));
    println!("answer: {}", play2(&vec![9, 7, 4, 6, 1, 8, 3, 5, 2]));
}

fn play(input: &str) -> String {
    let mut cups = input.chars().collect::<Vec<_>>();

    for _ in 0..100 {
        let current = cups[0];
        cups.remove(0);

        let three = cups[0..3].iter().cloned().collect::<Vec<char>>();
        (0..3).for_each(|_| {
            cups.remove(0);
        });

        let mut destination = sub_one(current);
        loop {
            let idx =
                cups.iter().enumerate().find_map(
                    |(idx, e)| {
                        if *e == destination {
                            Some(idx)
                        } else {
                            None
                        }
                    },
                );

            match idx {
                Some(i) => {
                    three.iter().rev().for_each(|e| cups.insert(i + 1, *e));
                    break;
                }
                None => {
                    destination = sub_one(destination);
                }
            }
        }

        cups.push(current);
    }

    while cups[0] != '1' {
        let c = cups[0];
        cups.remove(0);
        cups.push(c);
    }

    cups[1..].iter().collect::<String>()
}

fn sub_one(c: char) -> char {
    // char::from_digit is unstable, wtf rust, really?
    match c {
        '1' => '9',
        '2' => '1',
        '3' => '2',
        '4' => '3',
        '5' => '4',
        '6' => '5',
        '7' => '6',
        '8' => '7',
        '9' => '8',
        _ => panic!("not a digit"),
    }
}

fn play2(input: &Vec<u32>) -> u64 {
    let mut cups = VecDeque::new();
    input.iter().cloned().for_each(|c| {
        cups.push_back(c);
    });
    (10..=1000000).into_iter().for_each(|c| {
        cups.push_back(c);
    });

    println!("");

    for i in 0..10000000 {
        if i % 100 == 0 {
            print!("\r{}", i);
            io::stdout().flush();
        }
        let current = cups.pop_front().unwrap();

        let three = (0..3)
            .map(|_| cups.pop_front().unwrap())
            .collect::<Vec<u32>>();

        let mut destination = current - 1;
        if destination == 0 {
            destination = 1000000;
        }
        while three[0] == destination || three[1] == destination || three[2] == destination {
            destination -= 1;
            if destination == 0 {
                destination = 1000000;
            }
        }
        let idx = cups
            .iter()
            .enumerate()
            .find_map(|(idx, e)| if *e == destination { Some(idx) } else { None })
            .unwrap();

        three.iter().rev().for_each(|e| cups.insert(idx + 1, *e));

        cups.push_back(current);
    }

    println!("");

    while cups[0] != 1 {
        let v = cups.pop_front().unwrap();
        cups.push_back(v);
    }
    return (cups[1] as u64) * (cups[2] as u64);
}
