use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day7.txt").expect("Something went wrong reading the file");

    let rules = contents
        .lines()
        .map(|l| {
            let rule = l.split(" bags contain ").collect::<Vec<&str>>();

            if rule.len() != 2 {
                panic!(format!("failed to parse {}", l))
            }

            let contents = if rule[1] == "no other bags." {
                Vec::new()
            } else {
                rule[1]
                    .split(", ")
                    .map(|r| {
                        let contents = r.split(' ').collect::<Vec<&str>>();

                        if contents.len() != 4 {
                            panic!(format!("failed to parse {}", l));
                        }

                        let number = contents[0].parse::<u32>().unwrap();
                        let color = format!("{} {}", contents[1], contents[2]);
                        (number, String::from(color))
                    })
                    .collect::<Vec<(u32, String)>>()
            };
            (String::from(rule[0]), contents)
        })
        .collect::<HashMap<String, Vec<(u32, String)>>>();

    println!(
        "answer: {}",
        rules
            .iter()
            .filter(|(c, _)| can_contain(&rules, "shiny gold", c))
            .count()
    );

    println!(
        "answer: {}",
        count_bags(&rules, &String::from("shiny gold")) - 1
    )
}

fn can_contain(
    rules: &HashMap<String, Vec<(u32, String)>>,
    contain_color: &str,
    color: &String,
) -> bool {
    rules
        .get(color)
        .map(|r| {
            r.iter().any(|(_, sub_color)| {
                if sub_color == contain_color {
                    true
                } else {
                    can_contain(rules, contain_color, sub_color)
                }
            })
        })
        .unwrap_or(false)
}

fn count_bags(rules: &HashMap<String, Vec<(u32, String)>>, color: &String) -> u32 {
    rules
        .get(color)
        .map(|r| {
            r.iter()
                .map(|(c, sub_color)| c * count_bags(rules, sub_color))
                .fold(1, |acc, count| acc + count)
        })
        .unwrap_or(0)
}
