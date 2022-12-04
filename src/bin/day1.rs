use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("no file");
    let contents = fs::read_to_string(path).unwrap();
    let elves: Vec<Vec<i32>> = contents
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect()
        })
        .filter(|e: &Vec<i32>| !e.is_empty())
        .collect();
    let sums: Vec<i32> = elves.iter().map(|e| e.iter().sum::<i32>()).collect();
    let max = sums.iter().max().unwrap();
    println!("{:?}", max)
}
