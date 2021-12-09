use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
struct Position {
    horizontal: u32,
    depth: u32,
}

#[derive(Default, Debug)]
struct AimedPosition {
    horizontal: u32,
    depth: u32,
    aim: i32
}

enum Direction {
    Up,
    Down,
    Forward,
}

struct Movement {
    direction: Direction,
    distance: u32,
}

fn part1(fname: &str) {
    let f = fs::File::open(fname).expect("Unable to open input.");
    let data_iter = BufReader::new(f).lines().map(|line| {
        let line = line.unwrap();
        let mut parts_iter = line.split_whitespace();
        let direction = match parts_iter.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Uknown direction."),
        };

        let distance = parts_iter.next().unwrap().parse::<u32>().unwrap();

        Movement {
            direction,
            distance,
        }
    });

    let mut pos = Position::default();
    for movement in data_iter {
        match movement.direction {
            Direction::Up => pos.depth -= movement.distance,
            Direction::Down => pos.depth += movement.distance,
            Direction::Forward => pos.horizontal += movement.distance,
        }
    }

    println!("Final position: {:?}. Product: {}", pos, pos.horizontal * pos.depth);
}

fn part2(fname: &str) {
    let f = fs::File::open(fname).expect("Unable to open input.");
    let data_iter = BufReader::new(f).lines().map(|line| {
        let line = line.unwrap();
        let mut parts_iter = line.split_whitespace();
        let direction = match parts_iter.next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Uknown direction."),
        };

        let distance = parts_iter.next().unwrap().parse::<u32>().unwrap();

        Movement {
            direction,
            distance,
        }
    });

    let mut pos = AimedPosition::default();
    for movement in data_iter {
        match movement.direction {
            Direction::Up => pos.aim -= movement.distance as i32,
            Direction::Down => pos.aim += movement.distance as i32,
            Direction::Forward => {
                pos.horizontal += movement.distance;
                pos.depth += (movement.distance as i32 * pos.aim) as u32;
            }
        }
    }

    println!("Final position: {:?}. Product: {}", pos, pos.horizontal * pos.depth);
}

fn main() {
    part1("aoc2part1.txt");
    part2("aoc2part2.txt");
}
