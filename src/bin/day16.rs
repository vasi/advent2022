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

#[derive(Clone)]
struct Position {
    turns_left: i32,
    valve: String,
}

struct State {
    opened: HashSet<String>,
    positions: Vec<Position>,
    score: i32,
}

struct Move {
    idx: usize,
    pos: Position,
}

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

    fn valid_moves(&self, state: &State) -> Vec<Move> {
        let mut ret = Vec::new();
        let max_left = state.positions.iter().map(|p| p.turns_left).min().unwrap();

        for idx in 0..state.positions.len() {
            let pos = &state.positions[idx];
            let vstart = &self.valves[&pos.valve];
            for tun in &vstart.tunnels {
                let turns_left = pos.turns_left - tun.len - 1;
                if turns_left > 0 && turns_left <= max_left && !state.opened.contains(&tun.target) {
                    ret.push(Move {
                        idx: idx,
                        pos: Position {
                            turns_left,
                            valve: tun.target.clone(),
                        },
                    })
                }
            }
        }

        ret
    }

    fn next_state(&self, state: &State, mv: Move) -> State {
        let target = &mv.pos.valve;
        let score = state.score + &mv.pos.turns_left * self.valves[target].rate;

        let mut opened = state.opened.clone();
        opened.insert(target.clone());

        let mut positions = state.positions.clone();
        positions[mv.idx] = mv.pos;

        State {
            opened,
            positions,
            score,
        }
    }

    fn solve(&self, state: &State) -> i32 {
        let mut best = state.score;
        for mv in self.valid_moves(&state) {
            let next = self.next_state(&state, mv);
            best = best.max(self.solve(&next));
        }
        best
    }

    fn start_pos(&self, turns: i32) -> Position {
        Position {
            turns_left: turns,
            valve: START.to_owned(),
        }
    }

    fn part1(&self) -> i32 {
        let state = State {
            opened: HashSet::new(),
            positions: vec![self.start_pos(30)],
            score: 0,
        };
        self.solve(&state)
    }

    fn part2(&self) -> i32 {
        let state = State {
            opened: HashSet::new(),
            positions: vec![self.start_pos(26), self.start_pos(26)],
            score: 0,
        };
        self.solve(&state)
    }
}

fn main() {
    let arg = env::args().nth(1).expect("need arg");
    let input = Input::parse(&arg).simplify();
    println!("Part 1: {}", input.part1());
    println!("Part 2: {}", input.part2());
}
