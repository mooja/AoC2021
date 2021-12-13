use std::ops::Range;

type Grid = Vec<Vec<char>>;
type Fold = (char, usize);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct GridBox {
    xr: Range<usize>,
    yr: Range<usize>,
}

impl GridBox {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.yr.clone().flat_map(|row_idx| {
            self.xr.clone().map(move |col_idx| Point {
                x: col_idx,
                y: row_idx,
            })
        })
    }

    fn points_mirror_left(&self) -> impl Iterator<Item = Point> + '_ {
        self.yr.clone().flat_map(|row_idx| {
            self.xr.clone().rev().map(move |col_idx| Point {
                x: col_idx,
                y: row_idx,
            })
        })
    }

    fn points_mirror_up(&self) -> impl Iterator<Item = Point> + '_ {
        self.yr.clone().rev().flat_map(|row_idx| {
            self.clone().xr.map(move |col_idx| Point {
                x: col_idx,
                y: row_idx,
            })
        })
    }

    fn display(&self, grid: &Grid) {
        for (idx, p) in self.points().enumerate() {
            if idx != 0 && idx % self.xr.len() == 0 {
                println!();
            }

            print!("{}", grid[p.y][p.x]);
        }
        println!();
    }

    fn width(&self) -> usize {
        self.xr.len()
    }

    fn height(&self) -> usize {
        self.yr.len()
    }
}

fn main() {
    let input = std::fs::read_to_string("aoc13.txt").unwrap();

    let (points, folds) = parse_input(&input);

    let (mx, my) = (
        points.iter().map(|p| p.x).max().unwrap() + 1 as usize,
        points.iter().map(|p| p.y).max().unwrap() + 1 as usize,
    );

    let mut g: Grid = vec![vec!['.'; mx]; my];
    for p in points {
        g[p.y][p.x] = '#';
    }

    let mut outer_box = GridBox {
        xr: 0..mx,
        yr: 0..my
    };

    for (direction, axis_idx) in folds.into_iter() {
        let mut fold_from_box = match direction {
            'x' => GridBox {
                xr: (axis_idx + 1)..outer_box.xr.end,
                yr: outer_box.yr.clone()
            },

            'y' => GridBox {
                xr: outer_box.xr.clone(),
                yr: (axis_idx + 1)..outer_box.yr.end
            },

            _ => panic!()
        };

        let fold_into_box = match direction {
            'x' => GridBox {
                xr: match axis_idx.checked_sub(fold_from_box.xr.len()) {
                    Some(idx) => idx..axis_idx,
                    None => 0..axis_idx
                },
                yr: outer_box.yr.clone()
            },

            'y' => GridBox {
                xr: outer_box.xr.clone(),
                yr: match axis_idx.checked_sub(fold_from_box.yr.len()) {
                    Some(idx) => idx..axis_idx,
                    None => 0..axis_idx
                }
            },

            _ => panic!()
        };

        if fold_from_box.width() > fold_into_box.width() {
            fold_from_box.xr.end = fold_from_box.xr.start + fold_into_box.width();
        }

        if fold_from_box.height() > fold_into_box.height() {
            fold_from_box.yr.end = fold_from_box.yr.start + fold_into_box.height();
        }

        outer_box.xr.end = fold_into_box.xr.end;
        outer_box.yr.end = fold_into_box.yr.end;

        let from_ps: Vec<Point> = if direction == 'y' {
            fold_from_box.points_mirror_up().collect()
        } else {
            fold_from_box.points_mirror_left().collect()
        };

        for (into_p, from_p) in fold_into_box.points().zip(from_ps.into_iter()) {
            if g[from_p.y][from_p.x] == '#' {
                g[into_p.y][into_p.x] = '#';
            }
        }
    }

    outer_box.display(&g);
}

fn parse_input(src: &str) -> (Vec<Point>, Vec<Fold>) {
    let points: Vec<Point> = src
        .trim()
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|l| {
            let (c, r) = l.split_once(",").unwrap();
            Point {
                x: c.parse::<usize>().unwrap(),
                y: r.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let folds = src
        .trim()
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("=").unwrap();
            (l.chars().last().unwrap(), r.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    (points, folds)
}
