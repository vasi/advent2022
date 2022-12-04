use std::error;
use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

type BoxErr = Box<dyn error::Error>;
type BoxResult<T> = Result<T, BoxErr>;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl FromStr for Range {
    type Err = BoxErr;

    fn from_str(s: &str) -> BoxResult<Self> {
        let parts = s
            .split("-")
            .map(|i| i.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Range {
            start: parts[0],
            end: parts[1],
        })
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

#[derive(Debug)]
struct Pair {
    a: Range,
    b: Range,
}

impl FromStr for Pair {
    type Err = BoxErr;

    fn from_str(s: &str) -> BoxResult<Self> {
        let parts = s
            .split(",")
            .map(|i| i.parse::<Range>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            a: parts[0],
            b: parts[1],
        })
    }
}

impl Pair {
    fn has_fully_contained(&self) -> bool {
        self.a.contains(&self.b) || self.b.contains(&self.a)
    }
}

#[derive(Debug)]
struct Inputs(Vec<Pair>);

impl Inputs {
    fn parse(path: String) -> BoxResult<Self> {
        let file = fs::File::open(path)?;
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .collect::<io::Result<Vec<String>>>()?;
        let parsed = lines
            .iter()
            .map(|l| l.parse::<Pair>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Inputs(parsed))
    }

    fn part1(&self) -> u32 {
        self.0.iter().filter(|p| p.has_fully_contained()).count() as u32
    }

    fn part2(&self) -> u32 {
        unimplemented!()
    }
}

fn main() -> BoxResult<()> {
    let inputs = Inputs::parse(std::env::args().nth(1).expect("need file"))?;
    // println!("{:?}", inputs);
    println!("Part 1: {}", inputs.part1());
    println!("Part 2: {}", inputs.part2());

    Ok(())
}
