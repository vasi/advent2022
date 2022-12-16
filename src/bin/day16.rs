use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{env, fs};

#[derive(Clone, Debug)]
struct Tunnel {
    target: String,
    len: i32,
}

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: i32,
    tunnels: Vec<Tunnel>,
}

const START: &str = "AA";

#[derive(Debug)]
struct Input {
    valves: HashMap<String, Valve>,
}

impl Input {
    fn parse(file: &str) -> Self {
        let contents = fs::read_to_string(file).unwrap();
        let re =
            Regex::new(r"Valve (..) has flow rate=(\d+); tunnel.? lead.? to valve.? (.*)").unwrap();
        let mut valves = HashMap::new();
        for line in contents.lines() {
            let caps = re.captures(line).unwrap();
            let name = &caps[1];
            valves.insert(
                name.to_owned(),
                Valve {
                    name: name.to_owned(),
                    rate: caps[2].parse().unwrap(),
                    tunnels: caps[3]
                        .split(", ")
                        .map(|s| Tunnel {
                            target: s.to_owned(),
                            len: 1,
                        })
                        .collect(),
                },
            );
        }
        Input { valves }
    }

    fn simplified_valve(&self, valve: &Valve) -> Option<Valve> {
        if valve.rate == 0 && valve.name != START {
            return None;
        }

        // Note: We don't need a pqueue, since len should always be one
        let mut seen = HashMap::new();
        seen.insert(&valve.name, 0);
        let mut todo = VecDeque::new();
        todo.push_back(&valve.name);

        while let Some(name) = todo.pop_front() {
            let v = &self.valves[name];
            let dist = seen[name];
            for t in &v.tunnels {
                assert!(t.len == 1);
                if !seen.contains_key(&t.target) {
                    seen.insert(&t.target, dist + t.len);
                    todo.push_back(&t.target);
                }
            }
        }

        let tunnels = seen
            .iter()
            .filter(|(k, _)| self.valves[**k].rate != 0)
            .map(|(k, d)| Tunnel {
                target: (*k).to_owned(),
                len: *d,
            })
            .collect();
        Some(Valve {
            name: valve.name.to_owned(),
            rate: valve.rate,
            tunnels,
        })
    }

    fn simplify(&self) -> Self {
        let valves = self
            .valves
            .values()
            .filter_map(|v| self.simplified_valve(v))
            .map(|v| (v.name.to_owned(), v))
            .collect();
        Input { valves }
    }

    fn part1_rec<'a>(
        &'a self,
        opened: &mut HashSet<&'a str>,
        pos: &str,
        start_score: i32,
        turns_left: i32,
    ) -> i32 {
        let mut best = start_score;
        let vstart = &self.valves[pos];
        for tun in &vstart.tunnels {
            let turns = turns_left - tun.len - 1;
            if turns > 0 && !opened.contains(&tun.target as &str) {
                opened.insert(&tun.target);
                let vtarget = &self.valves[&tun.target];
                let score = start_score + turns * vtarget.rate;
                best = best.max(self.part1_rec(opened, &tun.target, score, turns));
                opened.remove(&tun.target as &str);
            }
        }
        best
    }

    fn part1(&self) -> i32 {
        self.part1_rec(&mut HashSet::new(), START, 0, 30)
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let input = Input::parse(&arg).simplify();
    println!("Part 1: {}", input.part1());
}
