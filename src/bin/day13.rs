fn main() {
    let arrival = 1008713;

    let input = "13,x,x,41,x,x,x,x,x,x,x,x,x,467,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,353,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";

    let buses: Vec<u64> = input
        .split(",")
        .filter(|b| *b != "x")
        .map(|b| b.parse::<u64>().unwrap())
        .collect();

    let (bus_id, wait) = buses
        .iter()
        .map(|b| (b, b - arrival % b))
        .min_by_key(|(_, t)| *t)
        .unwrap();

    println!("bus: {}, wait: {}, answer: {}", bus_id, wait, bus_id * wait);

    println!("lottery: {}", lottery_finder(input));
}

fn lottery_finder(input: &str) -> u64 {
    let departures = parse(input);

    println!("{:?}", departures);

    let (mut candidate, offset) = find_candidate(&departures);

    let mut i = 0;

    loop {
        if departures.iter().all(|(b, d)| (candidate + d) % b == 0) {
            return candidate;
        }

        candidate += offset;

        i += 1;
        if i % 100_000_000 == 0 {
            println!("{}", candidate)
        }
    }
}

fn find_candidate(rules: &Vec<(u64, u64)>) -> (u64, u64) {
    let mut candidates = Vec::new();
    let mut candidate = 0;

    let shortened_rules: Vec<(u64, u64)> = rules.iter().cloned().take(4).collect();

    loop {
        if shortened_rules
            .iter()
            .all(|(b, d)| (candidate + d) % b == 0)
        {
            candidates.push(candidate);
            if candidates.len() == 2 {
                break;
            }
        }
        candidate += 1;
    }

    (candidates[0], candidates[1] - candidates[0])
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let mut buses_with_offset: Vec<(u64, u64)> = input
        .split(",")
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(o, b)| {
            let bus_id = b.parse::<u64>().unwrap();
            (bus_id, o as u64)
        })
        .collect();

    buses_with_offset.sort_by(|(b, _o), (b2, _o2)| b2.cmp(b));

    buses_with_offset
}
