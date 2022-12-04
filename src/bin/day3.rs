use core::fmt;
use std::collections::HashSet;
use std::error;
use std::fs;
use std::io::{self, BufRead};
use std::str::FromStr;

type BoxResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
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
    fn parse(path: String) -> BoxResult<Vec<Input>> {
        let file = fs::File::open(path)?;
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .collect::<io::Result<Vec<String>>>()?;
        let parsed: Vec<Input> = lines
            .iter()
            .map(|l| l.parse::<Input>())
            .collect::<Result<Vec<Input>, InputErr>>()?;
        Ok(parsed)
    }

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

    fn part1(inputs: &Vec<Input>) -> u32 {
        inputs
            .iter()
            .map(|i| Input::priority(i.common()))
            .sum::<u32>()
    }
}

fn main() -> BoxResult<()> {
    let input = Input::parse(std::env::args().nth(1).expect("need file"))?;
    println!("{:?}", Input::part1(&input));

    Ok(())
}
