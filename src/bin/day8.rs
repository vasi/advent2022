use std::collections::HashSet;
use std::{env, fs};

type Grid = Vec<Vec<i32>>;
type Visibilities = HashSet<(i32, i32)>;

fn parse(file: &str) -> Grid {
    let content = fs::read_to_string(file).expect("read file");
    content
        .lines()
        .map(|l| l.bytes().map(|b| (b as i32) - ('0' as i32)).collect())
        .collect()
}

struct VizFinder<'a> {
    grid: &'a Grid,
    viz: Visibilities,
}

impl<'a> VizFinder<'a> {
    fn get(&self, x: i32, y: i32) -> i32 {
        *self.grid.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn width(&self) -> i32 {
        self.grid.first().unwrap().len() as i32
    }

    fn valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height() as i32
    }

    fn find_row(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32) {
        let mut highest = -1;
        while self.valid(x, y) {
            let h = self.get(x, y);
            if h > highest {
                self.viz.insert((x, y));
                highest = h;
            }
            x += dx;
            y += dy;
        }
    }

    fn find(grid: &'a Grid) -> Self {
        let mut v = VizFinder {
            grid: grid,
            viz: Visibilities::new(),
        };
        for y in 0..v.height() {
            v.find_row(0, y, 1, 0);
            v.find_row(v.width() - 1, y, -1, 0);
        }
        for x in 0..v.width() {
            v.find_row(x, 0, 0, 1);
            v.find_row(x, v.height() - 1, 0, -1);
        }
        v
    }
}

fn part1(grid: &Grid) -> usize {
    let finder = VizFinder::find(&grid);
    finder.viz.len()
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let grid = parse(&arg);
    println!("Part 1: {}", part1(&grid));
}
