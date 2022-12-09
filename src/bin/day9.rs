use std::collections::HashSet;
use std::{env, fs};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_head(&mut self, dir: &str) {
        match dir {
            "U" => self.y -= 1,
            "D" => self.y += 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
            _ => panic!("no such dir {}", &dir),
        }
    }

    fn follow(&mut self, head: &Pos) {
        let (dx, dy) = (head.x - self.x, head.y - self.y);
        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }
}

fn move_rope(rope: &mut Vec<Pos>, dir: &str) {
    rope.get_mut(0).unwrap().move_head(dir);
    for i in 1..rope.len() {
        let last = rope.get(i - 1).unwrap().clone();
        rope.get_mut(i).unwrap().follow(&last);
    }
}

fn solve(lines: &[&str], len: usize) -> usize {
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut rope = vec![Pos { x: 0, y: 0 }; len];
    seen.insert(rope.last().unwrap().clone());

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let dir = parts[0];
        let count: usize = parts[1].parse().unwrap();
        for _ in 0..count {
            move_rope(&mut rope, dir);
            seen.insert(rope.last().unwrap().clone());
        }
    }

    seen.len()
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let contents = fs::read_to_string(arg).unwrap();
    let lines: Vec<_> = contents.lines().collect();
    println!("Part 1: {}", solve(&lines, 2));
    println!("Part 2: {}", solve(&lines, 10));
}
