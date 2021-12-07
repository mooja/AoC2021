use counter::Counter;

fn solution(input: &str, part1: bool) {
    let counts = parse_input(input);
    let min_key = *counts.keys().min().unwrap();
    let max_key = *counts.keys().max().unwrap();
    let mut distances = vec![];

    for target in min_key..max_key {
        let mut total_distance = 0;

        for (&pos, &ncrabs) in counts.into_iter() {
            let delta = (target as i32 - pos as i32).abs() as u32;
            let fuel_cost = if part1 {
                delta
            } else {
                ((delta as f32 / 2.0) * (delta + 1) as f32) as u32
            };

            total_distance += ncrabs as u32 * fuel_cost;
        }

        distances.push(total_distance);
    }

    println!(
        "Part {}: {}",
        if part1 { '1' } else { '2' },
        distances.iter().min().unwrap()
    );
}

fn parse_input(input: &str) -> Counter<u32> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn main() {
    let input = &std::fs::read_to_string("aoc7.txt").unwrap();
    solution(input, true);
    solution(input, false);
}
