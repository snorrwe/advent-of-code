use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn fuel(module: i32) -> i32 {
    module / 3 - 2
}

fn part1(input: File) -> i32 {
    let f = BufReader::new(input);
    f.lines()
        .map(|line| {
            let module = line.unwrap().parse::<i32>().unwrap();
            fuel(module)
        })
        .sum()
}

fn part2(input: File) -> i32 {
    let f = BufReader::new(input);
    f.lines()
        .map(|line| {
            let module = line.unwrap().parse::<i32>().unwrap();
            let mut f = fuel(module);
            let mut x = fuel(f);
            while  x > 0 {
                f += x;
                x = fuel(x);
            }
            f
        })
        .sum()
}

fn main() {
    let f = File::open("input.txt").unwrap();
    println!("{}", part1(f));
    let f = File::open("input.txt").unwrap();
    println!("{}", part2(f));
}
