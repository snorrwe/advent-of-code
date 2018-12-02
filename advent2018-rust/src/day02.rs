use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run() {
    let file = File::open("data/day2.txt").expect("Failed to open input file!");
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let part1 = part1(&lines);
    println!("Day2 part1: {}", part1);

    let part2 = part2(&lines).expect("Failed to find the answer");
    println!("Day2 part2: {}", part2);
}

fn part1(lines: &Vec<String>) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    lines.iter().for_each(|line| {
        let count = count(line.as_str());
        if count.0 != 0 {
            twos += 1;
        }
        if count.1 != 0 {
            threes += 1;
        }
    });
    twos * threes
}

fn count(line: &str) -> (usize, usize) {
    let mut count: BTreeMap<char, usize> = BTreeMap::new();
    for chr in line.chars() {
        if !count.contains_key(&chr) {
            count.insert(chr, 0);
        }
        let c = count.get_mut(&chr).unwrap();
        *c += 1;
    }
    let mut result = [0, 0];
    count.values().for_each(|value| {
        if *value == 2 || *value == 3 {
            result[value - 2] += 1;
        }
    });
    (result[0], result[1])
}

fn part2(lines: &Vec<String>) -> Option<String> {
    lines.iter().enumerate().find_map(|(i, line0)| {
        lines[i..].iter().find_map(|line1| {
            let mut diffid = None;
            for (x, (chr0, chr1)) in line0.chars().zip(line1.chars()).enumerate() {
                if chr0 != chr1 {
                    if diffid.is_some() {
                        return None;
                    }
                    diffid = Some(x);
                }
            }
            diffid.map_or(None, |diffid| {
                Some(
                    line0
                        .chars()
                        .enumerate()
                        .filter(|(i, _c)| *i != diffid)
                        .map(|(_i, c)| c)
                        .collect::<String>(),
                )
            })
        })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_0() {
        let line = "abcdef";

        let (twos, threes) = count(line);

        assert_eq!(twos, 0);
        assert_eq!(threes, 0);
    }

    #[test]
    fn test_count_1() {
        let line = "bababc";

        let (twos, threes) = count(line);

        assert_eq!(twos, 1);
        assert_eq!(threes, 1);
    }

    #[test]
    fn test_count_2() {
        let line = "abcccd";

        let (twos, threes) = count(line);

        assert_eq!(twos, 0);
        assert_eq!(threes, 1);
    }

    #[test]
    fn test_part2() {
        let lines = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = part2(&lines);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), "fgij".to_string());
    }
}
