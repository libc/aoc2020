use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("answer: {}", run_file("input/day19.txt").len());
    println!("answer2: {}", run_file("input/day19_2.txt").len());
    println!("example: {:?}", run_file("input/day19_small.txt"));
}

fn run_file(fname: &str) -> Vec<String> {
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    let parts = contents.split("\n\n").collect::<Vec<&str>>();

    let rules = parse_rules(parts[0]);

    parts[1]
        .lines()
        .filter(|l| matches_rule(l, &rules))
        .map(|l| String::from(l))
        .collect()
}

fn parse_rules(lines: &str) -> HashMap<usize, Rule> {
    lines
        .lines()
        .map(|l| {
            let tmp = l.split(": ").collect::<Vec<&str>>();

            (tmp[0].parse::<usize>().unwrap(), parse_rule(tmp[1]))
        })
        .collect()
}

fn parse_rule(rule: &str) -> Rule {
    let or = rule.split(" | ").collect::<Vec<&str>>();
    if or.len() > 1 {
        return Rule::Or(or.iter().map(|r| parse_rule(r)).collect());
    }

    let seq = rule.split(" ").collect::<Vec<&str>>();
    if seq.len() > 1 {
        return Rule::Sequence(seq.iter().map(|r| parse_rule(r)).collect());
    }

    if rule.chars().nth(0).unwrap() == '"' {
        Rule::Char(rule.chars().nth(1).unwrap())
    } else {
        Rule::Rule(rule.parse::<usize>().unwrap())
    }
}

struct State {
    pos: HashSet<usize>,
}

fn matches_rule(l: &str, rules: &HashMap<usize, Rule>) -> bool {
    let mut s = State {
        pos: vec![0].into_iter().collect(),
    };
    let r = s.run(l, rules, &rules[&0]);
    r && s.pos.contains(&l.len())
}

impl State {
    fn run(&mut self, input: &str, rules: &HashMap<usize, Rule>, rule: &Rule) -> bool {
        let r = match rule {
            Rule::Char(c) => {
                self.pos = self
                    .pos
                    .iter()
                    .map(|&p| {
                        if p < input.len() && input.chars().nth(p).unwrap() == *c {
                            (p + 1, true)
                        } else {
                            (p, false)
                        }
                    })
                    .filter(|(_, matched)| *matched)
                    .map(|(p, _)| p)
                    .collect();
                self.pos.len() > 0
            }
            Rule::Rule(r) => self.run(input, rules, &rules[&r]),
            Rule::Sequence(seq) => seq.iter().all(|r| self.run(input, rules, r)),
            Rule::Or(or) => {
                let new_pos = or
                    .iter()
                    .flat_map(|r| {
                        let mut s = State {
                            pos: self.pos.clone(),
                        };
                        s.run(input, rules, r);
                        s.pos
                    })
                    .collect();
                self.pos = new_pos;
                self.pos.len() > 0
            }
        };
        r
    }
}

#[derive(Debug)]
enum Rule {
    Char(char),
    Rule(usize),
    Sequence(Vec<Rule>),
    Or(Vec<Rule>),
}
