extern crate core;

use std::collections::{hash_map, HashMap};
use std::fs;

struct Dir {
    children: HashMap<String, Node>,
    size: u64,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            children: HashMap::new(),
            size: 0,
        }
    }

    fn add(&mut self, dir: &[String], name: &str, node: Node) {
        self.size += node.size();
        if dir.len() == 0 {
            if self.children.contains_key(name) {
                panic!("node already exists at name {}", name);
            }
            self.children.insert(name.to_owned(), node);
        } else {
            match self.children.get_mut(&dir[0]) {
                Some(Node::Dir(d)) => d.add(&dir[1..], name, node),
                Some(_) => panic!("found a file, not a dir"),
                _ => panic!("node {} unknown", name),
            }
        }
    }

    fn print(&self, name: &str, prefix: usize) {
        println!("{}- {} (dir)", "  ".repeat(prefix), name);
        for (n, c) in &self.children {
            c.print(n, prefix + 1);
        }
    }

    fn iter(&self) -> DirIterator<'_> {
        let children = self.children.values();
        DirIterator {
            children: children,
            child_iter: None,
        }
    }

    fn part1(&self) -> u64 {
        let mut sum: u64 = 0;
        for node in self.iter() {
            match node {
                Node::Dir(d) if d.size <= 100_000 => sum += d.size,
                _ => (),
            }
        }
        sum
    }
}

struct DirIterator<'a> {
    children: hash_map::Values<'a, String, Node>,
    child_iter: Option<Box<DirIterator<'a>>>,
}

impl<'a> Iterator for DirIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.child_iter.as_mut().and_then(|i| i.next()) {
            Some(r)
        } else {
            let i = self.children.next();
            match i {
                Some(Node::Dir(d)) => self.child_iter = Some(Box::new(d.iter())),
                _ => self.child_iter = None,
            }
            i
        }
    }
}

enum Node {
    File(u64),
    Dir(Dir),
}

impl Node {
    fn size(&self) -> u64 {
        match self {
            Node::File(s) => *s,
            _ => 0,
        }
    }

    fn print(&self, name: &str, prefix: usize) {
        match self {
            Node::File(s) => println!("{}- {} (file, size={})", "  ".repeat(prefix), name, s),
            Node::Dir(d) => d.print(name, prefix),
        }
    }
}

#[derive(Debug)]
enum Input {
    Cd(String),
    Ls,
    File(String, u64),
    Dir(String),
}

fn parse_input(line: &str) -> Input {
    let parts: Vec<_> = line.split_whitespace().collect();
    let size: Option<u64> = parts[0].parse().ok();
    if line == "$ ls" {
        Input::Ls
    } else if parts.len() == 3 && parts[0] == "$" && parts[1] == "cd" {
        Input::Cd(parts[2].to_owned())
    } else if parts.len() == 2 && parts[0] == "dir" {
        Input::Dir(parts[1].to_owned())
    } else if parts.len() == 2 && size.is_some() {
        Input::File(parts[1].to_owned(), size.unwrap())
    } else {
        panic!("Unknown input {}", line)
    }
}

fn parse(file: &str) -> Dir {
    let contents = fs::read_to_string(file).expect("file read");
    let inputs: Vec<_> = contents.lines().map(parse_input).collect();

    let mut dir = Dir::new();
    let mut path: Vec<String> = Vec::new();
    let mut in_ls = false;
    for input in inputs {
        // dir.print("/", 0);
        // println!("input: {:?}", input);
        // println!("path: {}", path.join("/"));
        match (in_ls, &input) {
            (true, Input::File(p, size)) => dir.add(path.as_slice(), p, Node::File(*size)),
            (true, Input::Dir(p)) => dir.add(path.as_slice(), p, Node::Dir(Dir::new())),
            (_, Input::Ls) => in_ls = true,
            (_, Input::Cd(p)) => {
                if p == "/" {
                    path.clear();
                } else if p == ".." {
                    path.pop();
                } else {
                    path.push(p.to_owned());
                }
                in_ls = false
            }
            _ => panic!("bad input in_ls: {}, {:?}", in_ls, &input),
        }
    }

    dir
}

fn main() {
    let arg = std::env::args().nth(1).expect("need arg");
    let dir = parse(&arg);
    println!("Part 1: {}", dir.part1());
}
