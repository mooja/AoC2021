use std::fs;

#[derive(Debug, Clone, Copy)]
struct MarkableNum {
    n: u32,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<MarkableNum>>,
}

impl Board {
    fn mark(&mut self, n: u32) {
        for row in self.grid.iter_mut() {
            for elem in row.iter_mut() {
                if (*elem).n == n {
                    (*elem).marked = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        let rows = (0..5).map(|row_idx| (0..5).map(move |col_idx| self.grid[row_idx][col_idx]));
        let cols = (0..5).map(|col_idx| (0..5).map(move |row_idx| self.grid[row_idx][col_idx]));
        for (mut row_line, mut col_line) in rows.zip(cols) {
            if row_line.all(|n| n.marked) || col_line.all(|n| n.marked) {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        self.grid
            .iter()
            .map(|row| row.iter())
            .flatten()
            .filter_map(|mn| (!mn.marked).then(|| mn.n))
            .sum()
    }
}

fn part1(input: &str) {
    let (nums, mut boards) = parse_input(input);
    for n in nums {
        for b in boards.iter_mut() {
            b.mark(n);
            if b.is_winner() {
                println!("Part 1: {}", b.unmarked_sum() * n);
                return;
            }
        }
    }
}

fn part2(input: &str) {
    let (nums, boards) = parse_input(input);
    let mut last_result = 0;
    let mut completed_ids = vec![];
    let mut boards = boards.into_iter().enumerate().collect::<Vec<_>>();

    for n in nums {
        for (id, b) in boards.iter_mut() {
            b.mark(n);
            if b.is_winner() {
                last_result = b.unmarked_sum() * n;
                completed_ids.push(*id);
            }
        }

        boards = boards
            .into_iter()
            .filter(|&(b_idx, _)| !completed_ids.contains(&b_idx))
            .collect();

        if boards.len() == 0 {
            break;
        }
    }

    println!("Part 2: {}", last_result);
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut input_iter = input.split("\n\n");
    let numbers = input_iter
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let boards = input_iter
        .map(|s| Board {
            grid: s
                .trim()
                .split("\n")
                .map(|row| {
                    row.trim()
                        .split_whitespace()
                        .map(|ss| MarkableNum {
                            n: ss.parse::<u32>().unwrap(),
                            marked: false,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    (numbers, boards)
}

fn main() {
    let input = fs::read_to_string("aoc4.txt").expect("");
    part1(&input);
    part2(&input);
}