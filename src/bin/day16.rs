use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let contents =
        fs::read_to_string("input/day16.txt").expect("Something went wrong reading the file");

    let parts = contents.split("\n\n").collect::<Vec<&str>>();

    let rules = parse_rules(parts[0]);

    let my_ticket = parse_tickets(&parts[1]["your tickets:\n".len()..])[0].clone();
    let nearby_tickets = parse_tickets(&parts[2]["nearby tickets:\n".len()..]);

    let answer = nearby_tickets
        .iter()
        .flat_map(|t| {
            t.iter().filter(|number| {
                !rules
                    .iter()
                    .any(|(_, r)| r.iter().any(|rule| rule.contains(number)))
            })
        })
        .fold(0, |acc, number| acc + number);

    println!("{}", answer);

    let good_tickets = nearby_tickets
        .iter()
        .filter(|t| {
            !t.iter().any(|number| {
                !rules
                    .iter()
                    .any(|(_, r)| r.iter().any(|rule| rule.contains(number)))
            })
        })
        .collect::<Vec<&Vec<u16>>>();

    let mut indices_candidates = rules
        .iter()
        .map(|(name, rules)| {
            let candidates: Vec<usize> = (0..my_ticket.len())
                .filter(|idx| {
                    good_tickets
                        .iter()
                        .all(|t| rules.iter().any(|r| r.contains(&t[*idx])))
                })
                .collect();
            (name, candidates)
        })
        .collect::<Vec<(&String, Vec<usize>)>>();
    indices_candidates.sort_by_key(|(_name, candidates)| candidates.len());

    let mut taken = HashSet::new();

    let indices = indices_candidates.into_iter().map(|(name, candidates)| {
        let idx = candidates.into_iter().find(|i| !taken.contains(i)).unwrap();
        taken.insert(idx);
        (name, idx)
    });

    let answer2 = indices
        .filter(|(name, _idx)| name.starts_with("departure "))
        .map(|(_name, idx)| my_ticket[idx] as u64)
        .fold(1, |acc, n| acc * n);
    println!("answer: {}", answer2);
}

fn parse_rules(input: &str) -> HashMap<String, Vec<RangeInclusive<u16>>> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<&str>>();
            let nums = parts[1]
                .split(" or ")
                .map(|r| {
                    let range: Vec<u16> = r.split("-").map(|n| n.parse::<u16>().unwrap()).collect();
                    range[0]..=range[1]
                })
                .collect();

            (String::from(parts[0]), nums)
        })
        .collect()
}

fn parse_tickets(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| l.split(",").map(|t| t.parse::<u16>().unwrap()).collect())
        .collect()
}
