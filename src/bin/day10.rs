use std::{env, fs};

enum Inst {
    Addx(i32),
    Noop,
}

impl Inst {
    fn run<F: FnMut(i32, i32)>(&self, cycle: &mut i32, x: &mut i32, f: &mut F) {
        match self {
            Inst::Noop => {
                f(*cycle, *x);
                *cycle += 1;
            }
            Inst::Addx(i) => {
                f(*cycle, *x);
                f(*cycle + 1, *x);
                *x += i;
                *cycle += 2;
            }
        }
    }
}

fn parse(file: &str) -> Vec<Inst> {
    let contents = fs::read_to_string(file).unwrap();
    contents
        .lines()
        .map(|line| {
            if line.starts_with("addx ") {
                Inst::Addx(line[5..].parse::<i32>().unwrap())
            } else if line == "noop" {
                Inst::Noop
            } else {
                panic!("can't parse {}", line)
            }
        })
        .collect()
}

fn run<F: FnMut(i32, i32)>(insts: &Vec<Inst>, mut f: F) {
    let mut cycle = 1;
    let mut x = 1;
    for inst in insts {
        inst.run(&mut cycle, &mut x, &mut f);
    }
}

fn part1(insts: &Vec<Inst>) -> i32 {
    let mut ret = 0;
    run(insts, |cycle, x| {
        if cycle % 40 == 20 {
            ret += cycle * x
        }
    });
    ret
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let insts = parse(&arg);
    println!("Part 1: {}", part1(&insts));
}
