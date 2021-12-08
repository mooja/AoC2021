#[macro_use]
extern crate maplit;

use itertools::Itertools;
use std::collections::HashMap;

struct Codec(HashMap<char, char>);

impl Codec {
    fn decode(&self, source: &str) -> String {
        source
            .chars()
            .map(|ch| self.0.get(&ch).unwrap())
            .sorted()
            .collect::<String>()
    }
}

fn main() {
    let input = std::fs::read_to_string("aoc7.txt").unwrap();
    let data: Vec<(Vec<&str>, Vec<&str>)> = input
        .trim()
        .split("\n")
        .map(|l| {
            let mut iter = l.split("|");
            let patterns = iter.next().unwrap().trim().split(" ").collect::<Vec<_>>();
            let outputs = iter.next().unwrap().trim().split(" ").collect::<Vec<_>>();
            (patterns, outputs)
        })
        .collect();

    let mut p1_count = 0;
    for d in data.iter() {
        for output in &d.1 {
            if vec![2, 3, 4, 7].contains(&output.len()) {
                p1_count += 1;
            }
        }
    }

    println!("Part1: {}", p1_count);

    let s_to_n: HashMap<&str, u32> = hashmap! {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
    };

    let mut part2sum = 0;
    'next_display: for (sample, display) in data.iter() {
        'next_codec: for perm in "abcdefg".chars().permutations(7) {
            let codec_candidate = Codec(
                perm.into_iter()
                    .zip("abcdefg".chars())
                    .collect::<HashMap<_, _>>(),
            );

            let mut decoded_segment_sets = sample
                .iter()
                .chain(display.iter())
                .map(|&seg| codec_candidate.decode(seg));

            if !decoded_segment_sets.all(|ss| s_to_n.contains_key(&ss[..])) {
                continue 'next_codec;
            }

            let display_n = display
                .into_iter()
                .map(|&seg| codec_candidate.decode(seg))
                .map(|s| {
                    s_to_n
                        .get(&s[..])
                        .map(|&i| ((i as u8) + '0' as u8) as char)
                        .unwrap()
                })
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            part2sum += display_n;
            continue 'next_display;
        }
    }

    println!("Part 2: {}", part2sum);
}
