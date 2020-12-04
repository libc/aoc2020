use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day4.txt").expect("Something went wrong reading the file");

    let passports = contents.split("\n\n").map(|txt| Passport::from_string(txt));

    let valid = passports
        .clone()
        .filter(|passport| passport.valid())
        .count();

    println!("valid rule 1: {}", valid);
    let valid = passports
        .clone()
        .filter(|passport| passport.valid2())
        .count();
    println!("valid rule 2: {}", valid);
}

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn from_string(txt: &str) -> Passport {
        let fields = txt
            .split(|c: char| c == ' ' || c == '\n')
            .fold(HashMap::new(), |mut m, f| {
                let kv: Vec<&str> = f.split(':').collect();
                if kv.len() == 2 {
                    m.insert(String::from(kv[0]), String::from(kv[1]));
                }
                m
            });

        Passport { fields: fields }
    }

    fn valid(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .all(|f| self.fields.contains_key(&String::from(f)))
    }

    fn valid2(&self) -> bool {
        lazy_static! {
            static ref RULES: HashMap<String, Regex> = vec![
                ("byr", r"^(19[2-9][0-9]|200[012])$"),
                ("iyr", r"^(201[0-9]|2020)$"),
                ("eyr", r"^(202[0-9]|2030)$"),
                ("hgt", r"^((1[5678][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$"),
                ("hcl", r"^#([0-9a-f]{6})$"),
                ("ecl", r"^(amb|blu|brn|gry|grn|hzl|oth)$"),
                ("pid", r"^[0-9]{9}$"),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), Regex::new(v).unwrap()))
            .collect();
        }

        RULES.iter().all(|(field_name, rule)| {
            self.fields
                .get(field_name)
                .map_or(false, |v| rule.is_match(v))
        })
    }
}
