use std::collections::{HashMap, VecDeque};
use std::{env, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn adjacent(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x + 1, self.y),
        ]
    }
}

type Val = u32;

struct Grid {
    rows: Vec<Vec<Val>>,
    start: Pos,
    end: Pos,
    height: i32,
    width: i32,
}

impl Grid {
    fn new(rows: Vec<Vec<Val>>, start: Pos, end: Pos) -> Self {
        let height = rows.len() as i32;
        let width = rows.first().unwrap().len() as i32;
        Grid {
            rows,
            start,
            end,
            height,
            width,
        }
    }

    fn parse(file: &str) -> Self {
        let contents = fs::read_to_string(file).unwrap();
        let mut rows = Vec::new();
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);
        for line in contents.lines() {
            let mut row = Vec::new();
            for mut c in line.chars() {
                if c == 'S' {
                    start = Pos::new(row.len() as i32, rows.len() as i32);
                    c = 'a';
                } else if c == 'E' {
                    end = Pos::new(row.len() as i32, rows.len() as i32);
                    c = 'z';
                }
                row.push(c.into());
            }
            rows.push(row);
        }
        Grid::new(rows, start, end)
    }

    fn valid(&self, p: &Pos) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }

    fn get(&self, p: &Pos) -> Val {
        self.rows[p.y as usize][p.x as usize]
    }

    fn adjacent(&self, p: &Pos) -> Vec<Pos> {
        let h = self.get(p);
        p.adjacent()
            .iter()
            .filter(|o| self.valid(*o) && self.get(*o) - 1 <= h)
            .map(|o| o.clone())
            .collect()
    }

    fn part1(&self) -> u32 {
        let mut seen = HashMap::<Pos, u32>::new();
        let mut todo = VecDeque::new();
        todo.push_back(self.start);
        seen.insert(self.start, 0);

        loop {
            let p = todo.pop_front().unwrap();
            let dist = *seen.get(&p).unwrap();
            for a in self.adjacent(&p) {
                if a == self.end {
                    return dist + 1;
                } else if !seen.contains_key(&a) {
                    seen.insert(a, dist + 1);
                    todo.push_back(a);
                }
            }
        }
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let grid = Grid::parse(&arg);
    println!("Part 1: {}", grid.part1());
}
