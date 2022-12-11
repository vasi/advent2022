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

impl Monkey {
    fn turn(&mut self) -> Vec<Throw> {
        let mut throws = Vec::new();
        for item in &self.items {
            let v = self.op.eval(*item) / 3;
            let target = if v % self.test_div == 0 {
                self.target_true
            } else {
                self.target_false
            };
            throws.push(Throw {
                item: v,
                target: target,
            });
            self.inspected += 1;
        }
        self.items.clear();
        throws
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for idx in 0..monkeys.len() {
        for throw in monkeys[idx].turn() {
            monkeys[throw.target].items.push(throw.item);
        }
    }
}

fn part1(monkeys: &mut Vec<Monkey>) -> usize {
    for _ in 0..20 {
        round(monkeys);
    }
    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspected.sort();
    inspected.reverse();
    inspected[0] * inspected[1]
}

fn parse_op(opstr: &str, operandstr: &str) -> Op {
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
            items: items,
            op: parse_op(
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

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let mut monkeys = parse(&arg);
    println!("Part 1: {}", part1(&mut monkeys));
}
