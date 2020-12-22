use std::collections::HashSet;

fn main() {
    let mut deck1 = vec![
        44, 24, 36, 6, 27, 46, 33, 45, 47, 41, 15, 23, 40, 38, 43, 42, 25, 5, 30, 35, 34, 13, 29,
        1, 50,
    ];

    let mut deck2 = vec![
        32, 28, 4, 12, 9, 21, 48, 18, 31, 39, 20, 16, 3, 37, 49, 7, 17, 22, 8, 26, 2, 14, 11, 19,
        10,
    ];

    while !(deck1.is_empty() || deck2.is_empty()) {
        let card1 = deck1[0];
        let card2 = deck2[0];

        deck1.remove(0);
        deck2.remove(0);

        if card1 > card2 {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }

    let non_empty = if deck1.is_empty() { deck2 } else { deck1 };

    let answer = non_empty
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, e)| acc + (idx + 1) * e);

    println!("answer: {}", answer);

    let (_, winning_deck) = recursive_play(&vec![9, 2, 6, 3, 1], &vec![5, 8, 4, 7, 10]);
    let score = winning_deck
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, e)| acc + ((idx + 1) as u64) * (e as u64));
    println!("answer: {}", score);

    let (_, winning_deck) = recursive_play(
        &vec![
            44, 24, 36, 6, 27, 46, 33, 45, 47, 41, 15, 23, 40, 38, 43, 42, 25, 5, 30, 35, 34, 13,
            29, 1, 50,
        ],
        &vec![
            32, 28, 4, 12, 9, 21, 48, 18, 31, 39, 20, 16, 3, 37, 49, 7, 17, 22, 8, 26, 2, 14, 11,
            19, 10,
        ],
    );
    let score = winning_deck
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, e)| acc + ((idx + 1) as u64) * (e as u64));
    println!("answer: {}", score);
}

fn recursive_play(start_deck1: &Vec<u8>, start_deck2: &Vec<u8>) -> (u8, Vec<u8>) {
    let mut deck1 = start_deck1.iter().cloned().collect::<Vec<u8>>();
    let mut deck2 = start_deck2.iter().cloned().collect::<Vec<u8>>();

    let mut states: HashSet<(u8, Vec<u8>)> = HashSet::new();

    while !(deck1.is_empty() || deck2.is_empty()) {
        if states.contains(&(0, deck1.clone())) || states.contains(&(1, deck2.clone())) {
            return (1, deck1);
        }
        states.insert((0, deck1.clone()));
        states.insert((1, deck2.clone()));

        let card1 = deck1[0];
        let card2 = deck2[0];

        deck1.remove(0);
        deck2.remove(0);

        let winner = if deck1.len() as u8 >= card1 && deck2.len() as u8 >= card2 {
            let (w, _) = recursive_play(
                &deck1.get(0..card1 as usize).unwrap().to_vec(),
                &deck2.get(0..card2 as usize).unwrap().to_vec(),
            );
            w
        } else if card1 > card2 {
            1
        } else {
            2
        };

        if winner == 1 {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }

    if deck1.is_empty() {
        (2, deck2)
    } else {
        (1, deck1)
    }
}
