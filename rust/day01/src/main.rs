use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() {
    let file = File::open("input.txt").expect("Failed to open input file!");
    let buf_reader = BufReader::new(file);
    let numbers = buf_reader
        .lines()
        .map(|line| {
            line.expect("Failed to read line")
                .parse::<i32>()
                .expect("Failed to parse line")
        })
        .collect::<Vec<i32>>();

    let result = numbers.iter().sum::<i32>();
    println!("Day1 part1: {}", result);

    let result = part2(&numbers);
    println!("Day1 part2: {}", result);
}

fn part2(numbers: &Vec<i32>) -> i32 {
    let mut frequencies = HashSet::with_capacity(numbers.len());
    let mut current = 0;
    numbers
        .iter()
        .cycle()
        .find_map(|n| {
            current += n;
            if frequencies.contains(&current) {
                return Some(current);
            }
            frequencies.insert(current);
            None
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2_1() {
        let numbers = vec![3, 3, 4, -2, -4];
        let result = part2(&numbers);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part2_2() {
        let numbers = vec![7, 7, -2, -7, -4];
        let result = part2(&numbers);
        assert_eq!(result, 14);
    }
}
