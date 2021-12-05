use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Point {
    fn vec_to(&self, other: &Point) -> Vector {
        Vector {
            x: other.x as i32 - self.x as i32,
            y: other.y as i32 - self.y as i32
        }
    }
}

impl Vector {
    fn normalize(&mut self) {
        if self.x != 0 {
            self.x /= self.x.abs();
        }

        if self.y != 0 {
            self.y /= self.y.abs();
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn as_points(&self) -> Vec<Point> {
        let mut v = self.p1.vec_to(&self.p2);
        v.normalize();

        let mut p = self.p1;
        let mut points = vec![p];
        loop {
            p.x = (p.x as i32 + v.x) as u32;
            p.y = (p.y as i32 + v.y) as u32;
            points.push(p);

            if p == self.p2 {
                break;
            }
        }
        
        points
    }

    fn is_horizontal(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }
}

fn part1(input: &str) {
    let lines = parse_input(input)
        .into_iter()
        .filter(|l| l.is_horizontal())
        .collect::<Vec<Line>>();
    let mut grid_hm: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        for point in line.as_points() {
            let hm_entry = grid_hm.entry(point).or_insert(0);
            (*hm_entry) += 1;
        }
    }

    let overlaps = grid_hm
        .iter()
        .filter(|&(_, &count)| count >= 2)
        .collect::<Vec<_>>();

    println!("Part 1: {}", overlaps.len());
}

fn part2(input: &str) {
    let lines = parse_input(input);
    let mut points_hm: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        for point in line.as_points() {
            let hm_entry = points_hm.entry(point).or_insert(0);
            (*hm_entry) += 1;
        }
    }

    let overlaps = points_hm
        .iter()
        .filter(|&(_, &count)| count >= 2)
        .collect::<Vec<_>>();
    
    println!("Part 2: {}", overlaps.len());
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut points = line.split("->").map(|ns_str| {
                let mut nums = ns_str.trim().split(",").map(|n| n.parse::<u32>().unwrap());
                Point {
                    x: nums.next().unwrap(),
                    y: nums.next().unwrap(),
                }
            });
            Line {
                p1: points.next().unwrap(),
                p2: points.next().unwrap(),
            }
        })
        .collect()
}
 
fn main() {
    let input = std::fs::read_to_string("aoc5.txt").expect("");
    part1(&input);
    part2(&input);
}
