use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Input {
    other: char,
    me: char,
}

impl Input {
    fn iother(&self) -> i32 {
        (self.other as i32) - ('A' as i32)
    }

    fn score(&self, ime: i32) -> i32 {
        let result = (ime - self.iother() + 1).rem_euclid(3);
        1 + ime + result * 3
    }

    fn score1(&self) -> i32 {
        let ime = (self.me as i32) - ('X' as i32);
        self.score(ime)
    }

    fn score2(&self) -> i32 {
        let result = (self.me as i32) - ('X' as i32);
        let ime = (self.iother() - 1 + result).rem_euclid(3);
        self.score(ime)
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

    let score1 = inputs.iter().map(|i| i.score1()).sum::<i32>();
    println!("Part 1: {}", score1);

    let score2 = inputs.iter().map(|i| i.score2()).sum::<i32>();
    println!("Part 2: {}", score2);

    Ok(())
}
