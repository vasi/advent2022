use regex::Regex;
use std::ops::Range;
use std::{env, fs};

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    fn dist(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn parse(line: &str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let caps = re.captures(line).unwrap();
        Sensor {
            pos: Pos::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            beacon: Pos::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        }
    }

    fn range_at(&self, y: i32) -> Option<Range<i32>> {
        let beacon_dist = self.pos.dist(&self.beacon);
        let y_dist = (y - self.pos.y).abs();
        let x_dist = beacon_dist - y_dist;
        if x_dist < 0 {
            None
        } else {
            Some((self.pos.x - x_dist)..(self.pos.x + x_dist + 1))
        }
    }
}

struct Input {
    sensors: Vec<Sensor>,
}

impl Input {
    fn parse(file: &str) -> Self {
        let contents = fs::read_to_string(file).unwrap();
        let sensors = contents.lines().map(|l| Sensor::parse(l)).collect();
        Input { sensors }
    }

    fn xranges(&self, y: i32) -> Vec<Range<i32>> {
        let mut xmax = i32::MIN;
        let mut ranges = self
            .sensors
            .iter()
            .filter_map(|s| s.range_at(y))
            .collect::<Vec<_>>();
        let mut ret = Vec::new();
        ranges.sort_by_key(|r| r.start);
        for rng in ranges {
            let clamped = xmax.max(rng.start)..rng.end;
            if clamped.len() > 0 {
                ret.push(clamped);
            }
            xmax = xmax.max(rng.end);
        }
        ret
    }

    fn part1(&self, y: i32) -> usize {
        let mut beacons_at_y = self
            .sensors
            .iter()
            .map(|s| s.beacon)
            .filter(|b| b.y == y)
            .map(|b| b.x)
            .collect::<Vec<_>>();
        beacons_at_y.dedup();

        let mut found = 0;
        for r in self.xranges(y) {
            found += r.len() - beacons_at_y.iter().filter(|b| r.contains(b)).count();
        }
        found
    }

    fn part2(&self, max: i32) -> i64 {
        for y in 0..(max + 1) {
            let rngs = self.xranges(y);
            for i in 0..rngs.len() - 1 {
                let (r1, r2) = (&rngs[i], &rngs[i + 1]);
                let cand = r1.end;
                if r2.start > cand && cand >= 0 && cand <= max {
                    return (cand as i64) * 4_000_000 + (y as i64);
                }
            }
        }
        panic!("unreachable");
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let input = Input::parse(&arg);
    if arg.contains("sample") {
        println!("Part 1: {}", input.part1(10));
        println!("Part 2: {}", input.part2(20));
    } else {
        println!("Part 1: {}", input.part1(2000000));
        println!("Part 2: {}", input.part2(4000000));
    }
}
