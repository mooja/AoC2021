use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use std::fmt::Display;

type Point = (usize, usize);

#[derive(PartialEq, Eq)]
struct WeightedEdge {
    p: Point,
    cost: u32,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    p: Point,
    cost: u32,
}

struct Grid<T>(Vec<Vec<T>>);

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];

impl<T> Grid<T>
where
    T: Copy + Display,
{
    fn get(&self, p: Point) -> T {
        self.0[p.0][p.1]
    }

    fn neighbors(&self, (row, col): Point) -> impl Iterator<Item = Point> {
        let (mx, my) = (self.0.len() as i32, self.0[0].len() as i32);
        let offsets = DX.into_iter().zip(DY.into_iter());
        offsets
            .map(move |(r_offset, c_offset)| (row as i32 + r_offset, col as i32 + c_offset))
            .filter_map(move |(r, c)| {
                let within_grid = r >= 0 && r < mx && c >= 0 && c < my;
                within_grid.then(|| (r as usize, c as usize))
            })
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> {
        let (mx, my) = (self.0.len(), self.0[0].len());
        (0..mx).map(move |x| (0..my).map(move |y| (x, y))).flatten()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}

struct WeightedAdjList(HashMap<Point, Vec<WeightedEdge>>);

impl WeightedAdjList {
    fn shortest_path(&self, start: Point, end: Point) -> Option<u32> {
        let mut dist_from_start: HashMap<Point, u32> =
            self.0.keys().copied().map(|p| (p, u32::MAX)).collect();
        dist_from_start.insert(start, 0);

        let mut heap: BinaryHeap<_> = [State { p: start, cost: 0 }].into_iter().collect();

        while let Some(State { p, cost }) = heap.pop() {
            if p == end {
                return Some(cost);
            }

            if cost > *dist_from_start.get(&p).unwrap() {
                continue;
            }

            for edge in self.0.get(&p).unwrap() {
                let next = State {
                    p: edge.p,
                    cost: cost + edge.cost,
                };

                if next.cost < *dist_from_start.get(&next.p).unwrap() {
                    heap.push(next);

                    dist_from_start.insert(next.p, next.cost);
                }
            }
        }

        None
    }
}

impl Grid<u32> {
    fn as_adj_list(&self) -> WeightedAdjList {
        let mut hm = HashMap::new();
        for p in self.iter_points() {
            hm.insert(
                p,
                self.neighbors(p)
                    .map(|other_p| WeightedEdge {
                        p: other_p,
                        cost: self.get(other_p),
                    })
                    .collect(),
            );
        }

        WeightedAdjList(hm)
    }
}

fn main() {
    let input = &std::fs::read_to_string("aoc15.txt").unwrap();
    let g = parse_input(input);

    let start = (0, 0);
    let end = (g.width() - 1, g.height() - 1);
    let p1 = g.as_adj_list().shortest_path(start, end).unwrap();
    println!("Part 1: {:?}", p1);

    let g2 = parse_input2(input);
    let start = (0, 0);
    let end = (g2.width() - 1, g2.height() - 1);
    let p2 = g2.as_adj_list().shortest_path(start, end).unwrap();
    println!("Part 2: {:?}", p2);
}

fn parse_input(input: &str) -> Grid<u32> {
    Grid(
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|ch| (ch as u8 - '0' as u8) as u32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

fn parse_input2(input: &str) -> Grid<u32> {
    let initial_g = parse_input(input).0;
    let mut new_g = vec![];
    for outer_row_idx in 0..5 {
        for row in &initial_g {
            let mut new_row = vec![];
            for outer_col_idx in 0..5 {
                new_row.extend(row.iter().copied().map(|n| {
                    let rv = (n + outer_row_idx + outer_col_idx - 1) % 9;
                    rv + 1
                }));
            }
            new_g.push(new_row);
        }
    }

    Grid(new_g)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.p.cmp(&other.p))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}