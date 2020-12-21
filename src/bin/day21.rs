use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day21.txt").expect("cannot read day21.txt");

    let mut allergen_words: HashMap<String, HashSet<String>> = HashMap::new();

    for l in contents.lines() {
        let parts = l.split(" (contains ").collect::<Vec<_>>();

        let foods = parts[0]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();

        for allergen in parts[1]
            .get(0..parts[1].len() - 1)
            .expect("allergens must be supplied")
            .split(", ")
            .map(|s| s.to_string())
        {
            let set = allergen_words.entry(allergen).or_insert(foods.clone());

            *set = set.intersection(&foods).cloned().collect();
        }
    }

    let all_allergen_words = allergen_words
        .iter()
        .flat_map(|(_, words)| words.clone())
        .collect::<HashSet<String>>();

    let mut cnt = 0;
    for l in contents.lines() {
        let parts = l.split(" (contains ").collect::<Vec<_>>();

        for food in parts[0]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<HashSet<String>>()
        {
            if !all_allergen_words.contains(&food) {
                cnt += 1
            }
        }
    }

    println!("{:?}", cnt);

    let mut resolved_allergens: HashMap<String, String> = HashMap::new();
    let mut used_words: HashSet<String> = HashSet::new();

    loop {
        let unresolved_allergens = allergen_words
            .iter()
            .filter(|(a, _)| !resolved_allergens.contains_key(*a))
            .collect::<Vec<_>>();

        if unresolved_allergens.len() == 0 {
            break;
        }

        unresolved_allergens
            .into_iter()
            .map(|(allergens, words)| (allergens, words - &used_words))
            .filter(|(_, words)| words.len() == 1)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(allergen, words)| {
                let word = words.iter().nth(0).unwrap();
                resolved_allergens.insert((*allergen).clone(), word.clone());
                used_words.insert(word.clone());
            })
    }

    let mut array = resolved_allergens.iter().collect::<Vec<_>>();
    array.sort_by_key(|(allergen, _)| allergen.clone());

    println!(
        "{}",
        array
            .into_iter()
            .map(|(_, w)| w)
            .fold(String::new(), |acc, w| format!("{},{}", acc, w))
    );
}
