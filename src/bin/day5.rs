use crate::MoveMode::{InOrder, Reverse};
use regex::Regex;
use std::fs;

#[derive(PartialEq)]
enum MoveMode {
    Reverse,
    InOrder,
}

#[derive(Debug)]
struct Stack(Vec<char>);

impl Stack {
    fn remove(&mut self, s: usize) -> Vec<char> {
        let at = self.0.len() - s;
        self.0.split_off(at)
    }

    fn add(&mut self, v: &mut Vec<char>) {
        self.0.append(v)
    }
}

#[derive(Debug)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn stack(&mut self, one_idx: usize) -> &mut Stack {
        self.0.get_mut(one_idx - 1).expect("no such stack")
    }

    fn do_move(&mut self, mv: &Move, mode: &MoveMode) {
        let mut crates = self.stack(mv.source).remove(mv.count);
        if *mode == Reverse {
            crates.reverse();
        }
        self.stack(mv.dest).add(&mut crates);
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    // one-based indexes
    source: usize,
    dest: usize,
}

#[derive(Debug)]
struct Input {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl Input {
    fn parse(file: &str) -> Input {
        let mut input = Input {
            stacks: Stacks(Vec::new()),
            moves: Vec::new(),
        };

        let contents = fs::read_to_string(file).expect("read file");
        let (stack_part, move_part) = contents.split_once("\n\n").expect("split file");
        let mut rev = stack_part.lines().rev();

        let header = rev.next().expect("header");
        let stack_count = header.split_whitespace().count();
        input
            .stacks
            .0
            .resize_with(stack_count, || Stack(Vec::new()));

        for line in rev {
            let chars = line.chars().collect::<Vec<_>>();
            for (i, st) in input.stacks.0.iter_mut().enumerate() {
                if let Some(chr) = chars.get(1 + i * 4) {
                    if chr.is_alphabetic() {
                        st.0.push(*chr);
                    }
                }
            }
        }

        let move_re = Regex::new(r"\Amove (\d+) from (\d+) to (\d+)\z").expect("regex compile");
        for line in move_part.lines().filter(|l| !l.is_empty()) {
            let caps = move_re.captures(line).expect("regex match");
            input.moves.push(Move {
                count: caps[1].parse::<usize>().expect("parse count"),
                source: caps[2].parse::<usize>().expect("parse source"),
                dest: caps[3].parse::<usize>().expect("parse dest"),
            })
        }

        input
    }

    fn execute(&mut self, mode: &MoveMode) {
        for m in &self.moves {
            self.stacks.do_move(&m, mode);
        }
    }

    fn tops(&self) -> String {
        let tops = self
            .stacks
            .0
            .iter()
            .map(|s| s.0.last().expect("non-empty").to_string())
            .collect::<Vec<_>>();
        tops.join("")
    }

    fn run(file: &str, mode: &MoveMode) -> String {
        let mut input = Input::parse(file);
        input.execute(mode);
        input.tops()
    }
}

fn main() {
    let file = std::env::args().nth(1).expect("need file");
    println!("Part 1: {}", Input::run(&file, &Reverse));
    println!("Part 2: {}", Input::run(&file, &InOrder));
}
