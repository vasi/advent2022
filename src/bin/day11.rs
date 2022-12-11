use regex::Regex;
use std::{env, fs};

#[derive(Debug)]
enum Operand {
    Value,
    Imm(u64),
}

impl Operand {
    fn eval(&self, v: u64) -> u64 {
        match self {
            Operand::Value => v,
            Operand::Imm(i) => *i,
        }
    }
}

#[derive(Debug)]
enum Op {
    Plus(Operand),
    Times(Operand),
}

impl Op {
    fn eval(&self, v: u64) -> u64 {
        match self {
            Op::Plus(o) => v + o.eval(v),
            Op::Times(o) => v * o.eval(v),
        }
    }

    fn parse(opstr: &str, operandstr: &str) -> Op {
        let operand = if operandstr == "old" {
            Operand::Value
        } else {
            Operand::Imm(operandstr.parse().unwrap())
        };
        if opstr == "+" {
            Op::Plus(operand)
        } else if opstr == "*" {
            Op::Times(operand)
        } else {
            panic!("unknown op")
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    test_div: u64,
    target_true: usize,
    target_false: usize,
    inspected: usize,
}

struct Throw {
    item: u64,
    target: usize,
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    transform: Box<dyn Fn(u64) -> u64>,
}

impl Monkey {
    fn turn(&mut self, transform: &Box<dyn Fn(u64) -> u64>) -> Vec<Throw> {
        let mut throws = Vec::new();
        for item in &self.items {
            let v = transform(self.op.eval(*item));
            let target = if v % self.test_div == 0 {
                self.target_true
            } else {
                self.target_false
            };
            throws.push(Throw { item: v, target });
            self.inspected += 1;
        }
        self.items.clear();
        throws
    }
}

impl Monkeys {
    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            for throw in self.monkeys[idx].turn(&self.transform) {
                self.monkeys[throw.target].items.push(throw.item);
            }
        }
    }

    fn run(
        file: &str,
        rounds: usize,
        gen_transform: fn(&Vec<Monkey>) -> Box<dyn Fn(u64) -> u64>,
    ) -> u64 {
        let ms = Monkeys::parse(file);
        let transform = gen_transform(&ms);
        let mut monkeys = Monkeys {
            monkeys: ms,
            transform,
        };
        for _ in 0..rounds {
            monkeys.round();
        }
        let mut inspected = monkeys
            .monkeys
            .iter()
            .map(|m| m.inspected as u64)
            .collect::<Vec<_>>();
        inspected.sort();
        inspected.reverse();
        inspected[0] * inspected[1]
    }

    fn part1(file: &str) -> u64 {
        Monkeys::run(file, 20, |_| Box::new(|v| v / 3))
    }

    fn part2(file: &str) -> u64 {
        Monkeys::run(file, 10_000, |monkeys| {
            let modulus: u64 = monkeys.iter().map(|m| m.test_div).product();
            Box::new(move |v| v % modulus)
        })
    }

    fn parse(file: &str) -> Vec<Monkey> {
        let contents = fs::read_to_string(file).unwrap();
        let mut ret = Vec::new();
        let re = Regex::new(
            r"\s*Monkey \d+:
  Starting items: (?P<items>(?:\d+,\s)*\d+)
  Operation: new = old (?P<op>[+*]) (?P<operand>\d+|old)
  Test: divisible by (?P<div>\d+)
    If true: throw to monkey (?P<target_true>\d+)
    If false: throw to monkey (?P<target_false>\d+)",
        )
        .unwrap();
        for cap in re.captures_iter(&contents) {
            let items = cap
                .name("items")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|i| i.parse())
                .collect::<Result<_, _>>()
                .unwrap();
            ret.push(Monkey {
                items,
                op: Op::parse(
                    cap.name("op").unwrap().as_str(),
                    cap.name("operand").unwrap().as_str(),
                ),
                test_div: cap.name("div").unwrap().as_str().parse().unwrap(),
                target_true: cap.name("target_true").unwrap().as_str().parse().unwrap(),
                target_false: cap.name("target_false").unwrap().as_str().parse().unwrap(),
                inspected: 0,
            })
        }
        ret
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    println!("Part 1: {}", Monkeys::part1(&arg));
    println!("Part 1: {}", Monkeys::part2(&arg));
}
