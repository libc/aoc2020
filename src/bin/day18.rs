use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day18.txt").expect("Something went wrong reading the file");

    println!(
        "answer {}",
        contents
            .lines()
            .map(|l| parse(l))
            .map(|l| run(&l))
            .fold(0, |acc, result| acc + result)
    );

    println!(
        "answer {}",
        contents
            .lines()
            .map(|l| parse2(l))
            .map(|l| run(&l))
            .fold(0, |acc, result| acc + result)
    );
}

enum Term {
    Number(i64),
    Plus(Vec<Term>),
    Multiply(Vec<Term>),
}

fn parse(input: &str) -> Term {
    let mut p = Parser {
        input: input.chars().collect(),
        pos: 0,
        second: false,
    };

    p.parse()
}

fn parse2(input: &str) -> Term {
    let mut p = Parser {
        input: input.chars().collect(),
        pos: 0,
        second: true,
    };

    p.parse2()
}

fn run(input: &Term) -> i64 {
    match input {
        Term::Number(n) => *n,
        Term::Plus(v) => v.iter().map(|t| run(t)).fold(0, |acc, e| acc + e),
        Term::Multiply(v) => v.iter().map(|t| run(t)).fold(1, |acc, e| acc * e),
    }
}

struct Parser {
    input: Vec<char>,
    pos: usize,
    second: bool,
}

impl Parser {
    fn parse(&mut self) -> Term {
        let mut last_token = self.token();
        self.skip_ws();
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '+' => {
                    self.pos += 1;
                    let t2 = self.token();
                    last_token = Term::Plus(vec![last_token, t2]);
                }
                '*' => {
                    self.pos += 1;
                    let t2 = self.token();
                    last_token = Term::Multiply(vec![last_token, t2]);
                }
                ')' => return last_token,
                _ => panic!("fail"),
            }
            self.skip_ws();
        }

        last_token
    }

    fn parse2(&mut self) -> Term {
        self.second = true;
        let mut last_token = self.plus_or_token();
        self.skip_ws();
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '*' => {
                    self.pos += 1;
                    let t2 = self.plus_or_token();
                    last_token = Term::Multiply(vec![last_token, t2]);
                }
                ')' => return last_token,
                _ => panic!(format!("fail {:?}", self.input[self.pos])),
            }
            self.skip_ws();
        }

        last_token
    }

    fn plus_or_token(&mut self) -> Term {
        let mut last_token = self.token();
        self.skip_ws();
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '+' => {
                    self.pos += 1;
                    let t2 = self.token();
                    last_token = Term::Plus(vec![last_token, t2]);
                }
                _ => return last_token,
            }
            self.skip_ws();
        }

        last_token
    }

    fn skip_ws(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos] == ' ' {
            self.pos += 1
        }
    }

    fn token(&mut self) -> Term {
        self.skip_ws();
        if self.pos >= self.input.len() {
            panic!(format!("unexpected end at {}", self.pos))
        }

        if self.input[self.pos] >= '0' && self.input[self.pos] <= '9' {
            let mut n = 0;
            while self.pos < self.input.len()
                && self.input[self.pos] >= '0'
                && self.input[self.pos] <= '9'
            {
                n = n * 10 + self.input[self.pos].to_digit(10).unwrap();
                self.pos += 1;
            }
            Term::Number(n as i64)
        } else if self.input[self.pos] == '(' {
            self.pos += 1;
            let t = if self.second {
                self.parse2()
            } else {
                self.parse()
            };
            self.skip_ws();
            if self.pos >= self.input.len() {
                panic!(format!("unexpected end at {}", self.pos))
            }
            if self.input[self.pos] != ')' {
                panic!("unbalanced parenthesis");
            }
            self.pos += 1;
            t
        } else {
            panic!(format!("unexpected token at {}", self.pos))
        }
    }
}
