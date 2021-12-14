use counter::Counter;
use std::collections::HashMap;

type Pair = [char; 2];
type State = Counter<Pair, u128>;
type Rules = HashMap<Pair, (Pair, Pair)>;

fn main() {
    let input = std::fs::read_to_string("aoc14.txt").unwrap();
    let (mut state, rules) = parse_input(&input);

    for _ in 0..40 {
        let mut new_state = Counter::new();

        for (old_pair, &count) in &state {
            let new_pairs = rules.get(old_pair).unwrap();

            for new_pair in &[new_pairs.0, new_pairs.1] {
                new_state[new_pair] += count as u128;
            }
        }

        state = new_state;
    }

    let mut el_counts = state
        .iter()
        .map(|(pair, _)| pair.iter())
        .flatten()
        .map(|&el| (el, 0))
        .collect::<Counter<char, u128>>();

    for (&elem, el_count) in el_counts.iter_mut() {
        let ls_count = state
            .iter()
            .filter_map(|(&pair, &count)| (pair[0] == elem).then(|| count as u128))
            .sum::<u128>();

        let rs_count = state
            .iter()
            .filter_map(|(&pair, &count)| (pair[1] == elem).then(|| count as u128))
            .sum::<u128>();

        *el_count = ls_count.max(rs_count);
    }

    let ordered = el_counts.most_common_ordered();
    println!(
        "Part 2: {}",
        ordered[0].1 - ordered[ordered.len() - 1].1
    );
}

fn parse_input(input: &str) -> (State, Rules) {
    let (state, rules) = input.trim().split_once("\n\n").unwrap();

    let init_state: State = (0..state.len() - 2)
        .map(|idx| [state.as_bytes()[idx] as char, state.as_bytes()[idx+1] as char])
        .collect::<Counter<[char; 2], u128>>();

    let rules: Rules = rules
        .trim()
        .split('\n')
        .map(|line| {
            let (k, v) = line.split_once(" -> ").unwrap();

            let k = k.as_bytes();
            let k: [char; 2] = [k[0] as char, k[1] as char];

            let v = v.as_bytes()[0];
            let v = ([k[0] as char, v as char], [v as char, k[1] as char]);

            (k, v)
        })
        .collect();

    (init_state, rules)
}
