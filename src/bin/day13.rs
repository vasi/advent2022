use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter::Peekable;
use std::{env, fs};

#[derive(Debug, Eq)]
enum Val {
    Int(i32),
    List(Box<Vec<Val>>),
}

impl Val {
    fn cmp_list(vec1: &Vec<Val>, vec2: &Vec<Val>) -> Ordering {
        let (mut it1, mut it2) = (vec1.iter(), vec2.iter());
        loop {
            match (it1.next(), it2.next()) {
                (Some(v1), Some(v2)) => {
                    let ord = v1.cmp(v2);
                    if ord.is_ne() {
                        return ord;
                    }
                }
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (None, None) => return Ordering::Equal,
            }
        }
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::Int(i1), Val::Int(i2)) => i1.cmp(i2),
            (Val::List(l1), Val::List(l2)) => Val::cmp_list(l1, l2),
            (Val::Int(i1), Val::List(l2)) => Val::cmp_list(&vec![Val::Int(*i1)], l2),
            (Val::List(l1), Val::Int(i2)) => Val::cmp_list(l1, &vec![Val::Int(*i2)]),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug)]
struct Input(Vec<(Val, Val)>);

impl Input {
    fn parse_int<I: Iterator<Item = char>>(it: &mut Peekable<I>) -> i32 {
        let mut s = String::new();
        while it.peek().unwrap().is_digit(10) {
            s += &it.next().unwrap().to_string();
        }
        s.parse().unwrap()
    }

    fn parse_list<I: Iterator<Item = char>>(it: &mut Peekable<I>) -> Vec<Val> {
        assert!(it.next().unwrap() == '[');
        let mut ret = Vec::new();
        while *it.peek().unwrap() != ']' {
            ret.push(Self::parse_val(it));
            if *it.peek().unwrap() == ',' {
                it.next();
            }
        }
        assert!(it.next().unwrap() == ']');
        ret
    }

    fn parse_val<I: Iterator<Item = char>>(it: &mut Peekable<I>) -> Val {
        if *it.peek().unwrap() == '[' {
            Val::List(Box::new(Self::parse_list(it)))
        } else {
            Val::Int(Self::parse_int(it))
        }
    }

    fn parse_line<I: Iterator<Item = char>>(it: &mut Peekable<I>) -> Val {
        let v = Self::parse_val(it);
        assert!(it.next().unwrap() == '\n');
        v
    }

    fn parse_pair<I: Iterator<Item = char>>(it: &mut Peekable<I>) -> (Val, Val) {
        let a = Self::parse_line(it);
        let b = Self::parse_line(it);
        while it.peek().map_or(false, |c| *c == '\n') {
            it.next();
        }
        (a, b)
    }

    fn parse(file: &str) -> Self {
        let contents = fs::read_to_string(file).unwrap();
        let mut it = contents.chars().peekable();
        let mut pairs = Vec::new();
        while it.peek().is_some() {
            let pair = Self::parse_pair(&mut it);
            pairs.push(pair);
        }
        Input(pairs)
    }

    fn part1(&self) -> usize {
        let mut sum = 0;
        for (i, (a, b)) in self.0.iter().enumerate() {
            if a <= b {
                sum += i + 1;
            }
        }
        sum
    }

    fn part2(&self) -> usize {
        let divs = vec!["[[2]]", "[[6]]"]
            .iter()
            .map(|s| Self::parse_val(&mut s.chars().peekable()))
            .collect::<BTreeSet<_>>();
        let mut all = self
            .0
            .iter()
            .flat_map(|(a, b)| vec![a, b])
            .chain(divs.iter())
            .collect::<Vec<_>>();
        all.sort();
        all.iter()
            .enumerate()
            .filter(|(_, v)| divs.contains(v))
            .map(|(i, _)| i + 1)
            .product()
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let input = Input::parse(&arg);
    println!("Part 1: {}", input.part1());
    println!("Part 2: {}", input.part2());
}
