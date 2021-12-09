use std::collections::HashMap;

type Point = (usize, usize);

struct DepthGrid(Vec<Vec<u32>>);

impl DepthGrid {
    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (row, col) = (row as i32, col as i32);
        vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ]
        .into_iter()
        .filter_map(|(r, c)| {
            let within_grid =
                r >= 0 && r < self.0.len() as i32 && c >= 0 && c < self.0[0].len() as i32;
            within_grid.then(|| (r as usize, c as usize))
        })
    }

    fn is_lowpoint(&self, row: usize, col: usize) -> bool {
        let p = self.0[row as usize][col as usize];
        self.neighbors(row, col).all(|(r, c)| p < self.0[r][c])
    }

    fn lowpoints(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.0.len())
            .map(|r| (0..self.0[0].len()).map(move |c| (r, c)))
            .flatten()
            .filter(|&(r, c)| self.is_lowpoint(r, c))
    }

    fn lowpoints_sum(&self) -> u32 {
        let mut acc = 0;
        for row_idx in 0..self.0.len() {
            for col_idx in 0..self.0[0].len() {
                if self.is_lowpoint(row_idx, col_idx) {
                    acc += 1 + self.0[row_idx][col_idx];
                }
            }
        }

        acc
    }
}

fn main() {
    let input = std::fs::read_to_string("aoc9.txt").unwrap();
    let depth_grid = parse_input(&input);
    println!("Part 1: {}", depth_grid.lowpoints_sum());

    let mut basins: HashMap<Point, Vec<Point>> =
        depth_grid.lowpoints().map(|lp| (lp, vec![lp])).collect();

    for (&initial_p, seen_basin_ps) in basins.iter_mut() {
        let mut queue = vec![initial_p];
        while let Some(p) = queue.pop() {
            let neighbors = depth_grid
                .neighbors(p.0, p.1)
                .filter(|&(r, c)| depth_grid.0[r][c] != 9)
                .filter(|&(r, c)| !seen_basin_ps.contains(&(r, c)))
                .collect::<Vec<_>>();

            for n in neighbors {
                queue.push(n);
                seen_basin_ps.push(n);
            }
        }
    }

    let mut basin_sizes = basins.iter().map(|(_, ps)| ps.len()).collect::<Vec<_>>();

    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!(
        "Part 2: {}",
        basin_sizes.into_iter().take(3).product::<usize>()
    );
}

fn parse_input(input: &str) -> DepthGrid {
    DepthGrid(
        input
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|ch| (ch as u8 - '0' as u8) as u32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}
