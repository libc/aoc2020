use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day12.txt").expect("Something went wrong reading the file");

    let instructions: Vec<Instruction> = contents
        .lines()
        .map(|t| {
            let value = t[1..].parse::<i32>().unwrap();
            match t.chars().nth(0).unwrap() {
                'N' => Instruction::North(value),
                'S' => Instruction::South(value),
                'E' => Instruction::East(value),
                'W' => Instruction::West(value),
                'L' => Instruction::Left(value),
                'R' => Instruction::Right(value),
                'F' => Instruction::Forward(value),
                _ => panic!(format!("unknown instruction {}", t)),
            }
        })
        .collect();

    let (x, y) = run(&instructions);

    println!("x:{} y:{} answer:{}", x, y, x.abs() + y.abs());

    let (x, y) = run2(&vec![
        Instruction::Forward(10),
        Instruction::North(3),
        Instruction::Forward(7),
        Instruction::Right(90),
        Instruction::Forward(11),
    ]);

    println!("x:{} y:{} answer:{}", x, y, x.abs() + y.abs());

    let (x, y) = run2(&instructions);

    println!("x:{} y:{} answer:{}", x, y, x.abs() + y.abs());
}

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn coords(&self) -> (i32, i32) {
        match *self {
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
        }
    }
}

fn turn_left(d: Direction) -> Direction {
    match d {
        Direction::East => Direction::North,
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
    }
}

fn run(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut d = Direction::East;

    for i in instructions {
        match i {
            Instruction::North(v) => y -= v,
            Instruction::South(v) => y += v,
            Instruction::East(v) => x += v,
            Instruction::West(v) => x -= v,
            Instruction::Left(v) => match v {
                90 => d = turn_left(d),
                180 => d = turn_left(turn_left(d)),
                270 => d = turn_left(turn_left(turn_left(d))),
                _ => panic!(format!("don't know how to left for {} degrees", v)),
            },
            Instruction::Right(v) => match v {
                90 => d = turn_left(turn_left(turn_left(d))),
                180 => d = turn_left(turn_left(d)),
                270 => d = turn_left(d),

                _ => panic!(format!("don't know how to right for {} degrees", v)),
            },
            Instruction::Forward(v) => {
                let (dx, dy) = d.coords();
                x += v * dx;
                y += v * dy;
            }
        }
    }

    (x, y)
}

fn run2(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut ship_x = 0;
    let mut ship_y = 0;

    let mut waypoint_x = 10;
    let mut waypoint_y = -1;

    for i in instructions {
        match i {
            Instruction::North(v) => waypoint_y -= v,
            Instruction::South(v) => waypoint_y += v,
            Instruction::East(v) => waypoint_x += v,
            Instruction::West(v) => waypoint_x -= v,
            Instruction::Left(v) => {
                let (sin, cos) = (-v as f64).to_radians().sin_cos();
                let (ox, oy) = (waypoint_x, waypoint_y);

                waypoint_x = ox * (cos as i32) - oy * (sin as i32);
                waypoint_y = ox * (sin as i32) + oy * (cos as i32);
            }
            Instruction::Right(v) => {
                let (sin, cos) = (*v as f64).to_radians().sin_cos();
                let (ox, oy) = (waypoint_x, waypoint_y);

                waypoint_x = ox * (cos as i32) - oy * (sin as i32);
                waypoint_y = ox * (sin as i32) + oy * (cos as i32);
            }
            Instruction::Forward(v) => {
                ship_x += v * waypoint_x;
                ship_y += v * waypoint_y;
            }
        }
    }

    (ship_x, ship_y)
}
