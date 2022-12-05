use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;

type BoxErr = Box<dyn Error>;
type BoxResult<T> = Result<T, BoxErr>;

#[derive(Debug)]
struct ParseError(String);

impl ParseError {
    fn new(s: &str) -> BoxErr {
        Box::new(ParseError(s.to_owned()))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
struct Stack(Vec<char>);

impl Stack {
    fn new() -> Self {
        Stack(Vec::new())
    }
}

#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl FromStr for Stacks {
    type Err = BoxErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<_>>();
        let mut rev = lines.iter().rev();
        let first = rev.next().ok_or(ParseError::new("no stack lines"))?;

        let len = first.split_whitespace().count();
        let mut stacks = Vec::new();
        stacks.resize(len, Stack::new());
        for line in rev {
            let chars = line.chars().collect::<Vec<_>>();
            for (idx, st) in stacks.iter_mut().enumerate() {
                let chr_idx = 1 + 4 * idx;
                if let Some(chr) = chars.get(chr_idx) {
                    if !chr.is_whitespace() {
                        st.0.push(*chr);
                    }
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

#[derive(Debug)]
struct Move {
    count: i32,
    // one-indexed
    source: i32,
    dest: i32,
}

impl Move {
    fn parse_int(match_opt: Option<regex::Match>) -> BoxResult<i32> {
        let m = match_opt.ok_or(ParseError::new("no capture"))?;
        let i = m.as_str().parse::<i32>()?;
        Ok(i)
    }
}

impl FromStr for Move {
    type Err = BoxErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        let re = Regex::new(r"\Amove (\d+) from (\d+) to (\d+)\z")?;
        let caps = re.captures(s).ok_or(ParseError::new("no match"))?;
        let count = Move::parse_int(caps.get(1))?;
        let source = Move::parse_int(caps.get(2))?;
        let dest = Move::parse_int(caps.get(3))?;
        Ok(Move {
            count: count,
            source: source,
            dest: dest,
        })
    }
}

#[derive(Debug)]
struct Inputs {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl FromStr for Inputs {
    type Err = BoxErr;

    fn from_str(data: &str) -> BoxResult<Self> {
        let (stack_part, moves_part) = data
            .split_once("\n\n")
            .ok_or(ParseError::new("no delimiter"))?;
        let stacks = stack_part.parse::<Stacks>()?;
        let moves: Vec<Move> = moves_part
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<Move>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Inputs {
            stacks: stacks,
            moves: moves,
        })
    }
}

impl Inputs {
    fn parse(path: String) -> BoxResult<Self> {
        let data = fs::read_to_string(path)?;
        data.parse::<Inputs>()
    }
}

fn main() -> BoxResult<()> {
    let file = std::env::args().nth(1).expect("need file");
    let inputs = Inputs::parse(file)?;
    println!("{:?}", inputs);

    Ok(())
}
