use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut count = 0;
    let mut last_measure = None;
    let mut window = VecDeque::new();
    let data_iter = BufReader::new(File::open("aoc1.txt").expect("Can't open the file."))
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap());

    for item in data_iter {
        window.push_back(item);

        if window.len() < 3 {
            continue;
        }

        if last_measure.is_none() {
            last_measure = Some(window.iter().sum::<u32>());
            continue;
        }

        window.pop_front();
        let new_measure = window.iter().sum::<u32>();
        if new_measure > last_measure.unwrap() {
            count += 1;
        }

        last_measure = Some(new_measure);
    }

    println!("Total elevation increases: {}", count);
}
