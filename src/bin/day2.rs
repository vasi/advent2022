use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Input {
    other: char,
    me: char,
}

impl Input {
    fn score(&self) -> i32 {
        let iother = (self.other as i32) - ('A' as i32);
        let ime = (self.me as i32) - ('X' as i32);
        let result = (ime - iother + 1).rem_euclid(3);
        1 + ime + result * 3
    }
}

// TODO: Use FromStr? Better parsing with regex?
fn parse(s: &String) -> Input {
    let parts: Vec<&str> = s.split_whitespace().collect();
    Input {
        other: parts[0].chars().nth(0).unwrap(),
        me: parts[1].chars().nth(0).unwrap(),
    }
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("need file");
    let file = fs::File::open(path)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;
    let inputs: Vec<Input> = lines.iter().map(|l| parse(l)).collect();
    let score = inputs.iter().map(|i| i.score()).sum::<i32>();
    println!("Score: {}", score);
    Ok(())
}
