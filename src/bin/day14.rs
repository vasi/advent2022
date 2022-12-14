use std::collections::HashMap;
use std::{env, fs};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    fn parse(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        Pos::new(x.parse().unwrap(), y.parse().unwrap())
    }

    fn drop_positions(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x + 1, self.y + 1),
        ]
    }
}

enum Point {
    Rock,
    Sand,
}

struct Cave {
    points: HashMap<Pos, Point>,
    bottom: i32,
    has_abyss: bool,
}

impl Cave {
    fn draw(&mut self, p1: &Pos, p2: &Pos) {
        let mut p = p1.clone();
        while p != *p2 {
            self.points.insert(p, Point::Rock);
            p.x += (p2.x - p1.x).signum();
            p.y += (p2.y - p1.y).signum();
        }
        self.points.insert(p, Point::Rock);
        self.bottom = self.bottom.max(p1.y.max(p2.y));
    }

    fn parse(file: &str) -> Self {
        let mut cave = Cave {
            points: HashMap::new(),
            bottom: 0,
            has_abyss: true,
        };
        let contents = fs::read_to_string(file).unwrap();
        for line in contents.lines() {
            let coords: Vec<Pos> = line.split(" -> ").map(|c| Pos::parse(&c)).collect();
            for i in 0..coords.len() - 1 {
                cave.draw(&coords[i], &coords[i + 1]);
            }
        }
        cave
    }

    fn do_drop(&mut self) -> Option<Pos> {
        let mut p = Pos::new(500, 0);
        while let Some(found) = p
            .drop_positions()
            .iter()
            .find(|o| !self.points.contains_key(o))
        {
            if p.y >= self.bottom + 1 {
                if self.has_abyss {
                    return None;
                } else {
                    break;
                }
            }
            p = *found;
        }
        self.points.insert(p, Point::Sand);
        Some(p)
    }

    fn part1(file: &str) -> usize {
        let mut cave = Cave::parse(file);
        let mut dropped = 0;
        while cave.do_drop().is_some() {
            dropped += 1;
        }
        dropped
    }

    fn part2(file: &str) -> usize {
        let mut cave = Cave::parse(file);
        cave.has_abyss = false;
        let mut dropped = 0;
        while cave.do_drop().unwrap() != Pos::new(500, 0) {
            dropped += 1;
        }
        dropped + 1
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    println!("Part 1: {}", Cave::part1(&arg));
    println!("Part 2: {}", Cave::part2(&arg));
}
