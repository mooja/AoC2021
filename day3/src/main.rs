use std::fs;
use std::io::{BufRead, BufReader};

fn most_common_bits<T>(data: &Vec<T>) -> Vec<usize>
where
    T: AsRef<str>,
{
    let initial = vec![0; data[0].as_ref().len()];
    let counts = data.iter().fold(initial, |mut acc, s| {
        let s = s.as_ref();
        for (ch, count) in s.chars().zip(acc.iter_mut()) {
            if ch == '1' {
                *count += 1;
            }
        }

        acc
    });

    let cutoff = data.len() / 2;
    counts
        .iter()
        .map(|&count| {
            if count > cutoff {
                1
            } else if data.len() - count == count {
                1
            } else {
                0
            }
        })
        .collect()
}

fn least_common_bits<T>(data: &Vec<T>) -> Vec<usize>
where
    T: AsRef<str>,
{
    most_common_bits(data)
        .iter()
        .map(|&n| if n == 1 { 0 } else { 1 })
        .collect()
}

fn part1(data: &Vec<String>) -> u32 {
    let gamma = most_common_bits(data)
        .iter()
        .map(|&count| if count == 1 { '1' } else { '0' })
        .collect::<String>();

    let epsilon = gamma
        .chars()
        .map(|ch| if ch == '1' { '0' } else { '1' })
        .collect::<String>();

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn part2(data: &Vec<String>) -> u32 {
    let data: Vec<&String> = data.iter().map(|item| item).collect();
    let max_len = data[0].len();

    let oxygen: u32 = {
        let mut search_space = data.clone();
        for idx in 0..max_len {
            let criteria_bit = most_common_bits(&search_space)[idx];
            search_space = search_space
                .into_iter()
                .filter(|&s| {
                    let idx_bit = s.as_bytes()[idx] as u8 - 48;
                    idx_bit == criteria_bit as u8
                })
                .collect::<Vec<_>>();

            if search_space.len() == 1 {
                break;
            }
        }

        if search_space.len() != 1 {
            panic!("More than one number meeting the criteria.")
        }

        u32::from_str_radix(search_space[0], 2).unwrap()
    };

    let co2: u32 = {
        let mut search_space = data.clone();
        for idx in 0..max_len {
            let criteria_bit = least_common_bits(&search_space)[idx];
            search_space = search_space
                .into_iter()
                .filter(|&s| {
                    let idx_bit = s.as_bytes()[idx] as u8 - 48;
                    idx_bit == criteria_bit as u8
                })
                .collect::<Vec<_>>();

            if search_space.len() == 1 {
                break;
            }
        }

        if search_space.len() != 1 {
            panic!("More than one number meeting the criteria.")
        }

        u32::from_str_radix(search_space[0], 2).unwrap()
    };

    oxygen * co2
}

fn main() {
    let f = fs::File::open("aoc3.txt").expect("Unable to open input.");
    let data = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
