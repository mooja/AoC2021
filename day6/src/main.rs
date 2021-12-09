#[derive(Debug)]
struct Fish {
    age: u8,
}

fn part1(input: &str) {
    let mut fish_list = parse_input(input);
    let mut new_fish_list = vec![];
    for _ in 0..80 {
        for f in fish_list.iter_mut() {
            if f.age == 0 {
                f.age = 6;
                new_fish_list.push(Fish { age: 8 });
            } else {
                f.age -= 1;
            }
        }

        fish_list.append(&mut new_fish_list);
    }

    println!("Part 1: {}", fish_list.len());
}

fn part2(input: &str) {
    let data = parse_input(input);
    let mut state = [0u64; 9];

    for f in data {
        state[f.age as usize] += 1;
    }

    for _ in 0..=256 {
        state.rotate_left(1);
        state[6] += state[8];
    }

    println!("Part 2: {:?}", state.iter().take(8).sum::<u64>());
}

fn parse_input(input: &str) -> Vec<Fish> {
    input
        .trim()
        .split(",")
        .map(|n| Fish {
            age: n.parse::<u32>().unwrap() as u8
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("aoc6.txt").unwrap();
    part1(&input);
    part2(&input);
}
