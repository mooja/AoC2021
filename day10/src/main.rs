use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("aoc10.txt").unwrap();

    let ope = "([{<";
    let clo = ")]}>";

    let opening_to_closing: HashMap<char, char> = ope.chars().zip(clo.chars()).collect();
    let closing_to_opening: HashMap<char, char> = clo.chars().zip(ope.chars()).collect();

    let p1scoring = vec![3, 57, 1197, 25137];
    let p1scoring: HashMap<char, u32> = p1scoring
        .into_iter()
        .zip(clo.chars())
        .map(|(i, ch)| (ch, i))
        .collect();

    let p2scoring = vec![1, 2, 3, 4];
    let p2scoring: HashMap<char, u32> = clo.chars().zip(p2scoring.clone().into_iter()).collect();

    let mut p1_answ = 0;
    for l in input.trim().lines() {
        let mut stack = vec![];
        for ch in l.chars() {
            if ope.contains(ch) {
                stack.push(ch);
            } else {
                let closing = ch;
                let opening = stack.pop().expect("Too many closing tokens.");
                if *closing_to_opening.get(&closing).unwrap() != opening {
                    p1_answ += p1scoring.get(&closing).unwrap();
                    break;
                }
            }
        }
    }
    println!("Part1: {}", p1_answ);

    let mut p2score_list = vec![];
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

        p2score_list.push(line_score);
    }

    p2score_list.sort();
    let mid_idx = (p2score_list.len() / 2) as usize;
    println!("Part2: {:?}", p2score_list[mid_idx]);
}
