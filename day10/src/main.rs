use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("aoc10.txt").unwrap();

    let ope = "([{<";
    let clo = ")]}>";

    let opening_to_closing: HashMap<char, char> = ope.chars().zip(clo.chars()).collect();
    let closing_to_opening: HashMap<char, char> = clo.chars().zip(ope.chars()).collect();

    let p1scoring = vec![3, 57, 1197, 25137];
    let p1scoring: HashMap<char, u32> = clo.chars().zip(p1scoring.into_iter()).collect();

    let p2scoring = vec![1, 2, 3, 4];
    let p2scoring: HashMap<char, u32> = clo.chars().zip(p2scoring.clone().into_iter()).collect();

    let mut p1 = 0;
    let mut p2list = vec![];
    for l in input.trim().lines() {
        let mut stack = vec![];
        let mut corrupted = false;
        for ch in l.chars() {
            if ope.contains(ch) {
                stack.push(ch);
            } else {
                let closing = ch;
                let opening = stack.pop().expect("Too many closing tokens.");
                if *closing_to_opening.get(&closing).unwrap() != opening {
                    p1 += p1scoring.get(&closing).unwrap();
                    corrupted = true;
                    break;
                }
            }
        }

        if corrupted {
            continue;
        }

        let mut line_score = 0u64;
        while let Some(opening) = stack.pop() {
            let closing = *opening_to_closing.get(&opening).unwrap();
            line_score *= 5;
            line_score += *p2scoring.get(&closing).unwrap() as u64;
        }

        p2list.push(line_score);
    }

    println!("Part1: {}", p1);

    p2list.sort();
    println!("Part2: {:?}", p2list[p2list.len() / 2]);
}