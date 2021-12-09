use counter::Counter;

fn solution(input: &str, part1: bool) {
    let counts = parse_input(input);
    let mut min_fuel_cost = u32::MAX;

    let start_pos = *counts.keys().min().unwrap();
    let end_pos = *counts.keys().max().unwrap();
    for target in start_pos..end_pos {
        let mut total_fuel_cost = 0;

        for (&pos, &ncrabs) in counts.into_iter() {
            let delta = (target as i32 - pos as i32).abs() as u32;
            let fuel_cost = if part1 {
                delta
            } else {
                ((delta as f32 / 2.0) * (delta + 1) as f32) as u32
            };

            total_fuel_cost += ncrabs as u32 * fuel_cost;
        }

        min_fuel_cost = min_fuel_cost.min(total_fuel_cost);
    }

    println!(
        "Part {}: {}",
        if part1 { '1' } else { '2' },
        min_fuel_cost
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
