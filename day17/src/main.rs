use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Projectile {
    p: Pos,
    v: Pos,
}

impl Projectile {
    fn advance(&mut self) {
        self.p.x += self.v.x;
        self.p.y += self.v.y;

        self.v.y -= 1;
        self.v.x = match self.v.x {
            0 => 0,
            x if x > 0 => x - 1,
            x => x + 1,
        }
    }
}

fn main() {
    let target_area = (
        60..=94,
        -171..=-136
    );

    let mut p1 = 0;
    let mut p2 = 0;

    for yv in -200..2000 {
        for xv in 0..2000 {
            let mut max_y = 0;
            let initial_v = Pos { x: xv, y: yv };
            let mut p = Projectile {
                p: Pos { x: 0, y: 0 },
                v: initial_v,
            };

            loop {
                p.advance();
                max_y = max_y.max(p.p.y);

                if target_area.0.contains(&p.p.x) && target_area.1.contains(&p.p.y) {
                    p1 = p1.max(max_y);
                    p2 += 1;
                    break;
                } else {
                    if p.p.x > *target_area.0.end() || p.p.y < *target_area.1.start() {
                        break;
                    }
                }
            }
        }
    }

    dbg!(p1);
    dbg!(p2);
}
