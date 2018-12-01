use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run() {
    let file = File::open("data/day1.txt").expect("Failed to open input file!");
    let buf_reader = BufReader::new(file);
    let numbers = buf_reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let result = numbers.iter().sum::<i32>();
    println!("Day1 part1: {}", result);

    let result = part2(&numbers);
    println!("Day1 part2: {}", result);
}

fn part2(numbers: &Vec<i32>) -> i32 {
    let mut frequencies = HashSet::new();
    let mut current = 0;
    loop {
        for n in numbers.iter() {
            current += n;
            if frequencies.contains(&current) {
                return current;
            }
            frequencies.insert(current);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let numbers = vec![3, 3, 4, -2, -4];
        let result = part2(&numbers);
        assert_eq!(result, 10);
    }
}
