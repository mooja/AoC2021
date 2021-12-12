use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Cave<'a> {
    Sm(&'a str),
    Lg(&'a str),
}

impl<'a> Cave<'a> {
    fn name(&self) -> &'a str {
        match self {
            Cave::Sm(name) => name,
            Cave::Lg(name) => name,
        }
    }
}

#[derive(Debug)]
struct CaveGraph<'a> {
    caves: HashMap<Cave<'a>, HashSet<Cave<'a>>>,
}

impl<'a> From<&'a str> for CaveGraph<'a> {
    fn from(source: &'a str) -> Self {
        let mut caves = HashMap::new();

        for l in source.trim().lines() {
            let mut iter = l.split("-");

            let a = match iter.next().unwrap() {
                a if a.chars().all(|c| c.is_uppercase()) => Cave::Lg(a),
                a => Cave::Sm(a),
            };
            let b = match iter.next().unwrap() {
                b if b.chars().all(|c| c.is_uppercase()) => Cave::Lg(b),
                b => Cave::Sm(b),
            };

            let a_entry = caves.entry(a.clone()).or_insert(HashSet::new());
            (*a_entry).insert(b.clone());

            let b_entry = caves.entry(b).or_insert(HashSet::new());
            (*b_entry).insert(a);
        }

        CaveGraph { caves: caves }
    }
}

impl<'a> CaveGraph<'a> {
    fn npaths(&self, start: &str, end: &str, p2: bool) -> u32 {
        let mut count = 0u32;
        let mut queue: Vec<(Cave, Vec<Cave>)> = match self.caves.contains_key(&Cave::Lg(start)) {
            true => vec![(Cave::Lg(start), vec![Cave::Lg(start)])],
            false => vec![(Cave::Sm(start), vec![Cave::Sm(start)])],
        };

        while let Some((cave, history)) = queue.pop() {
            if cave.name() == end {
                count += 1;
                continue;
            }

            let visited_sm = history
                .iter()
                .copied()
                .filter(|cave| matches!(cave, Cave::Sm(_)))
                .collect::<Vec<_>>();

            let mut to_visit = self.caves.get(&cave).unwrap().clone();

            if p2 {
                let have_visited_2sm =
                    visited_sm.len() != visited_sm.iter().collect::<HashSet<_>>().len();

                if have_visited_2sm {
                    to_visit = &to_visit - &visited_sm.into_iter().collect();
                } else {
                    to_visit = &to_visit - &vec![Cave::Sm(start)].into_iter().collect();
                }
            } else {
                to_visit = &to_visit - &visited_sm.into_iter().collect();
            }

            let to_visit_with_histories = to_visit
                .into_iter()
                .zip(std::iter::repeat(history))
                .map(|(cave, mut history)| {
                    history.push(cave);
                    (cave, history)
                })
                .collect::<Vec<_>>();
            queue.extend(to_visit_with_histories);
        }

        count as u32
    }
}

fn main() {
    let input = std::fs::read_to_string("aoc12.txt").unwrap();
    let g = CaveGraph::from(&input[..]);
    println!("Part 1: {}", g.npaths("start", "end", false));
    println!("Part 2: {}", g.npaths("start", "end", true));
}
