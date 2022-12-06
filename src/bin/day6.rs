use std::collections::{HashMap, VecDeque};
use std::fs;

struct Stats {
    total: usize,
    deq: VecDeque<char>,
    counts: HashMap<char, usize>,
}

impl Stats {
    fn new() -> Self {
        Stats {
            total: 0,
            deq: VecDeque::new(),
            counts: HashMap::new(),
        }
    }

    fn complete(&self) -> bool {
        self.total >= 4 && self.counts.iter().all(|(_, v)| *v <= 1)
    }

    fn add(&mut self, c: char) {
        self.total += 1;
        self.deq.push_back(c);
        *self.counts.entry(c).or_insert(0) += 1;
        if self.deq.len() > 4 {
            let d = self.deq.pop_front().unwrap();
            *self.counts.entry(d).or_insert(0) -= 1
        }
    }

    fn part1(s: String) -> usize {
        let mut stats = Stats::new();
        for c in s.chars() {
            stats.add(c);
            if stats.complete() {
                return stats.total;
            }
        }
        unreachable!()
    }
}

fn main() {
    let arg = std::env::args().nth(1).expect("need arg");
    let buffer = fs::read_to_string(&arg).unwrap_or(arg);
    println!("Part 1: {}", Stats::part1(buffer));
}
