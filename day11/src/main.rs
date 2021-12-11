use std::fmt::Display;

type Point = (usize, usize);

struct Grid<T>(Vec<Vec<T>>);

const DX: [i32; 8] = [0, 0, 1, -1, -1, 1, -1, 1];
const DY: [i32; 8] = [1, -1, 0, 0, -1, 1, 1, -1];

impl<T> Grid<T>
where
    T: Copy + Display,
{
    fn get(&self, p: Point) -> T {
        self.0[p.0][p.1]
    }

    fn get_mut(&mut self, p: Point) -> &mut T {
        &mut self.0[p.0][p.1]
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

    fn _display(&self) {
        for row in &self.0 {
            println!(
                "{}",
                row.iter()
                    .map(|item| format!("{}", item))
                    .collect::<Vec<_>>()
                    .join("")
            )
        }
        println!();
    }
}

impl Grid<u8> {
    fn step(&mut self) -> usize {
        let mut nflashes = 0;
        let mut flash_queue = vec![];

        for p in self.iter_points() {
            *self.get_mut(p) += 1;

            if self.get(p) > 9 {
                *self.get_mut(p) = 0;
                flash_queue.push(p);
                nflashes += 1;
            }
        }

        while let Some(p) = flash_queue.pop() {
            for n in self.neighbors(p) {
                if self.get(n) == 0 {
                    continue;
                }

                *self.get_mut(n) += 1;
                if self.get(n) > 9 {
                    *self.get_mut(n) = 0;
                    flash_queue.push(n);
                    nflashes += 1;
                }
            }
        }

        nflashes
    }
}

fn parse_input(data: &str) -> Grid<u8> {
    Grid(
        data.trim()
            .lines()
            .map(|l| l.chars().map(|ch| ch as u8 - '0' as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

fn main() {
    let mut grid = parse_input(&std::fs::read_to_string("aoc11.txt").unwrap().trim());
    let grid_len = grid.iter_points().count();

    let mut p1 = 0;
    let mut p2: Option<u32> = None;

    for step_idx in 1.. {
        let step_fc = grid.step();

        if step_idx <= 100 {
            p1 += step_fc;
        }

        if step_fc == grid_len {
            p2 = Some(step_idx);
        }

        if step_idx > 100 && p2.is_some() {
            break;
        }
    }

    println!("Step 1: {}", p1);
    println!("Part 2: {}", p2.unwrap());
}
