use std::collections::HashSet;
use std::{env, fs};

struct Grid(Vec<Vec<i32>>);

type CoordSet = HashSet<(i32, i32)>;

impl Grid {
    fn get(&self, x: i32, y: i32) -> i32 {
        *self.0.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    fn height(&self) -> i32 {
        self.0.len() as i32
    }

    fn width(&self) -> i32 {
        self.0.first().unwrap().len() as i32
    }

    fn valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height() as i32
    }

    fn out_visible(&self, coords: &mut CoordSet, mut x: i32, mut y: i32, dx: i32, dy: i32) {
        let mut highest = -1;
        while self.valid(x, y) {
            let h = self.get(x, y);
            if h > highest {
                coords.insert((x, y));
                highest = h;
            }
            x += dx;
            y += dy;
        }
    }

    fn viewing_distance(&self, mut x: i32, mut y: i32, dx: i32, dy: i32) -> usize {
        let mut seen = 0;
        let h = self.get(x, y);
        loop {
            x += dx;
            y += dy;
            if !self.valid(x, y) {
                break;
            }
            seen += 1;
            if self.get(x, y) >= h {
                break;
            }
        }
        seen
    }

    fn score(&self, x: i32, y: i32) -> usize {
        self.viewing_distance(x, y, 0, -1)
            * self.viewing_distance(x, y, 0, 1)
            * self.viewing_distance(x, y, -1, 0)
            * self.viewing_distance(x, y, 1, 0)
    }

    fn part1(&self) -> usize {
        let mut coords = CoordSet::new();
        for y in 0..self.height() {
            self.out_visible(&mut coords, 0, y, 1, 0);
            self.out_visible(&mut coords, self.width() - 1, y, -1, 0);
        }
        for x in 0..self.width() {
            self.out_visible(&mut coords, x, 0, 0, 1);
            self.out_visible(&mut coords, x, self.height() - 1, 0, -1);
        }
        coords.len()
    }

    fn part2(&self) -> usize {
        let mut score = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                score = score.max(self.score(x, y));
            }
        }
        score
    }
}

fn parse(file: &str) -> Grid {
    let content = fs::read_to_string(file).expect("read file");
    let rows = content
        .lines()
        .map(|l| l.bytes().map(|b| (b as i32) - ('0' as i32)).collect())
        .collect();
    Grid(rows)
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let grid = parse(&arg);
    println!("Part 1: {}", grid.part1());
    println!("Part 2: {}", grid.part2());
}
