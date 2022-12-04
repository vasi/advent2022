use core::fmt;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

type BoxResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct Input {
    a: Vec<char>,
    b: Vec<char>,
}

#[derive(Debug)]
struct InputErr {}

impl fmt::Display for InputErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "???")
    }
}

impl error::Error for InputErr {}

impl FromStr for Input {
    type Err = InputErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let (a, b) = chars.split_at(chars.len() / 2);
        Ok(Input {
            a: a.to_owned(),
            b: b.to_owned(),
        })
    }
}

impl Input {
    fn priority(c: char) -> u32 {
        if c.is_lowercase() {
            c.to_digit(36).unwrap() - 10 + 1
        } else {
            c.to_digit(36).unwrap() - 10 + 26 + 1
        }
    }

    fn common(&self) -> char {
        let aset = self.a.iter().collect::<HashSet<_>>();
        let bset = self.b.iter().collect::<HashSet<_>>();
        let c = aset.intersection(&bset).next().unwrap();
        **c
    }

    fn all(&self) -> HashSet<&char> {
        self.a.iter().chain(self.b.iter()).collect::<HashSet<_>>()
    }
}

struct Inputs(Vec<Input>);

impl Inputs {
    fn parse(path: String) -> BoxResult<Self> {
        let file = fs::File::open(path)?;
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .collect::<io::Result<Vec<String>>>()?;
        let parsed: Vec<Input> = lines
            .iter()
            .map(|l| l.parse::<Input>())
            .collect::<Result<Vec<Input>, InputErr>>()?;
        Ok(Inputs(parsed))
    }

    fn part1(&self) -> u32 {
        self.0
            .iter()
            .map(|i| Input::priority(i.common()))
            .sum::<u32>()
    }

    fn groups(&self) -> Vec<Group> {
        self.0.chunks(3).map(|g| Group(g.to_vec())).collect()
    }

    fn part2(&self) -> u32 {
        self.groups()
            .iter()
            .map(|g| Input::priority(g.badge()))
            .sum()
    }
}

struct Group(Vec<Input>);

impl Group {
    fn badge(&self) -> char {
        let mut counts = HashMap::new();
        for input in &self.0 {
            for c in input.all() {
                *counts.entry(*c).or_insert(0) += 1;
            }
        }

        *counts.iter().find(|e| *e.1 == 3).unwrap().0
    }
}

fn main() -> BoxResult<()> {
    let inputs = Inputs::parse(std::env::args().nth(1).expect("need file"))?;
    println!("Part 1: {}", inputs.part1());
    println!("Part 2: {}", inputs.part2());

    Ok(())
}
