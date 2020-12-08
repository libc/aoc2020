use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day8.txt").expect("Something went wrong reading the file");

    let program = parse_program(&contents);
    let mut program2: Vec<Op> = program.iter().cloned().collect();

    println!("answer {}", run_program(&program));

    for (i, _) in program.iter().enumerate() {
        let old_op = program2[i];

        match old_op {
            Op::Acc(_) => continue,
            Op::Nop(v) => program2[i] = Op::Jmp(v),
            Op::Jmp(v) => program2[i] = Op::Nop(v),
        }
        let (acc, finished) = run_program2(&program2);
        if finished {
            println!("answer {}", acc);
            break;
        }

        program2[i] = old_op;
    }
}

#[derive(Clone, Copy)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_program(contents: &String) -> Vec<Op> {
    contents
        .lines()
        .map(|l| {
            let v = l[4..].parse::<i32>().unwrap();
            if l.starts_with("nop ") {
                Op::Nop(v)
            } else if l.starts_with("acc ") {
                Op::Acc(v)
            } else if l.starts_with("jmp ") {
                Op::Jmp(v)
            } else {
                panic!(format!("unknown op {}", l))
            }
        })
        .collect()
}

fn run_program(program: &Vec<Op>) -> i32 {
    let mut eip = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&eip) {
            return acc;
        }

        visited.insert(eip);

        match program[eip as usize] {
            Op::Nop(_) => eip += 1,
            Op::Acc(inc) => {
                eip += 1;
                acc += inc;
            }
            Op::Jmp(j) => eip += j,
        }
    }
}

fn run_program2(program: &Vec<Op>) -> (i32, bool) {
    let mut eip = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&eip) {
            return (acc, false);
        }

        visited.insert(eip);

        if eip as usize == program.len() {
            return (acc, true);
        }

        match program[eip as usize] {
            Op::Nop(_) => eip += 1,
            Op::Acc(inc) => {
                eip += 1;
                acc += inc;
            }
            Op::Jmp(j) => eip += j,
        }
    }
}
