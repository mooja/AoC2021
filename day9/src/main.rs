use std::collections::HashSet;

type Point = (usize, usize);

struct DepthGrid(Vec<Vec<u8>>);

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];

impl DepthGrid {
    fn get(&self, p: Point) -> u8 {
        self.0[p.0][p.1]
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = Point> + '_ {
        let (mx, my) = (self.0.len() as i32, self.0[0].len() as i32);
        DX.into_iter()
            .zip(DY.into_iter())
            .map(move |(r_offset, c_offset)| (row as i32 + r_offset, col as i32 + c_offset))
            .filter_map(move |(r, c)| {
                let within_grid = r >= 0 && r < mx && c >= 0 && c < my;
                within_grid.then(|| (r as usize, c as usize))
            })
    }

    fn is_lowpoint(&self, row: usize, col: usize) -> bool {
        let p = self.get((row, col));
        self.neighbors(row, col).all(|(r, c)| p < self.get((r, c)))
    }

    fn lowpoints(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.0.len())
            .map(|r| (0..self.0[0].len()).map(move |c| (r, c)))
            .flatten()
            .filter(|&(r, c)| self.is_lowpoint(r, c))
    }
}

fn main() {
    let input = std::fs::read_to_string("aoc9.txt").unwrap();
    let depth_grid = parse_input(&input);
    println!(
        "Part 1: {}",
        depth_grid
            .lowpoints()
            .map(|p| depth_grid.get(p) as u32)
            .sum::<u32>()
    );

    let mut basins: Vec<HashSet<Point>> = depth_grid
        .lowpoints()
        .map(|p| vec![p].into_iter().collect())
        .collect();

    for b in basins.iter_mut() {
        let mut queue = vec![b.clone().into_iter().next().unwrap()];
        while let Some(p) = queue.pop() {
            let neighbors: Vec<Point> = depth_grid
                .neighbors(p.0, p.1)
                .filter(|&(r, c)| depth_grid.get((r, c)) != 9)
                .filter(|&(r, c)| !b.contains(&(r, c)))
                .collect();

            for n in neighbors {
                queue.push(n);
                b.insert(n);
            }
        }
    }

    let mut basin_sizes = basins.into_iter().map(|ps| ps.len()).collect::<Vec<_>>();
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
                    .map(|ch| ch as u8 - '0' as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}
