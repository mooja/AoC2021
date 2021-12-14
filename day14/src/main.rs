use counter::Counter;

use std::collections::HashMap;

type State = Counter<[char; 2], u128>;
type Rules = HashMap<[char; 2], ([char; 2], [char; 2])>;

fn main() {
    let input = std::fs::read_to_string("aoc14.txt").unwrap();
    let (mut state, rules) = parse_input(&input);

    for _ in 0..40 {
        let mut new_state = state.clone();

        for (old_pair, count) in &state {
            let (a, b) = rules.get(old_pair).unwrap();

            for &new_pair in &[a, b] {
                new_state[new_pair] += *count as u128;
            }

            new_state[old_pair] -= count;
        }

        state = new_state;
    }

    let mut el_counts = state
        .iter()
        .map(|(keys, _)| keys.iter())
        .flatten()
        .map(|el| (*el, 0))
        .collect::<Counter<char, u128>>();

    let elem_names = el_counts.keys().copied().collect::<Vec<_>>();
    for &elem in &elem_names {
        let ls_count = state
            .iter()
            .filter_map(|(&pair, count)| (pair[0] == elem).then(|| *count as u128))
            .sum::<u128>();

        let rs_count = state
            .iter()
            .filter_map(|(&pair, count)| (pair[1] == elem).then(|| *count as u128))
            .sum::<u128>();

        el_counts[&elem] = ls_count.max(rs_count);
    }

    let ordered = el_counts.most_common_ordered();
    println!(
        "Part 2: {}",
        ordered[0].1 - ordered[ordered.len() - 1].1
    );
}

fn parse_input(input: &str) -> (State, Rules) {
    let mut iter = input.trim().split("\n\n");
    let state: State = iter
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&b| b as char)
        .collect::<Vec<_>>()
        .windows(2)
        .into_iter()
        .map(|arr| [arr[0], arr[1]])
        .collect::<Counter<[char; 2], u128>>();
    let rules: HashMap<[char; 2], ([char; 2], [char; 2])> = iter
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(l, r)| {
            let l = l.as_bytes();
            let l: [char; 2] = [l[0] as char, l[1] as char];
            let r = r.as_bytes()[0];
            let r = ([l[0] as char, r as char], [r as char, l[1] as char]);
            (l, r)
        })
        .collect();

    (state, rules)
}
