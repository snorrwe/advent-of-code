use std::collections::HashMap;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(help = "Path to input")]
    input: std::path::PathBuf,
}

fn part1(s: &str, rules: &HashMap<&str, i32>) -> i32 {
    let gamere = regex::Regex::new(r"^Game (\d+):").unwrap();
    let cubesre = regex::Regex::new(r"(\d+) (\w+)").unwrap();
    let mut possible = 0;
    'lines: for line in s.lines() {
        let Some(id) = gamere.captures(line) else {
            continue;
        };
        let (_, [id]) = id.extract();
        let id: i32 = id.parse().unwrap();
        for cubes in cubesre.captures_iter(line) {
            let (_, [n, color]) = cubes.extract();
            let n: i32 = n.parse().unwrap();
            if rules[color] < n {
                continue 'lines;
            }
        }
        possible += id;
    }
    possible
}

fn part2(s: &str) -> i64 {
    let cubesre = regex::Regex::new(r"(\d+) (\w+)").unwrap();
    let mut least_cubes = HashMap::new();
    let mut res = 0;
    for line in s.lines() {
        least_cubes.clear();
        for cubes in cubesre.captures_iter(line) {
            let (_, [n, color]) = cubes.extract();
            let n: i64 = n.parse().unwrap();
            let x = least_cubes.entry(color).or_insert(n);
            // need at least as many cubes as the max
            if *x < n {
                *x = n;
            }
        }
        // empty lines would add 1 because of product(), skip them
        if least_cubes.is_empty() {
            continue;
        }
        let power: i64 = least_cubes.values().copied().product();
        res += power;
    }
    res
}

fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input).expect("failed to read input");
    let rules = [("red", 12), ("green", 13), ("blue", 14)].into();
    let res = part1(&input, &rules);
    println!("part1: {res}");
    let res = part2(&input);
    println!("part2: {res}");
}

#[cfg(test)]
const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

"#;

#[test]
fn part1_test() {
    let res = part1(INPUT, &[("red", 12), ("green", 13), ("blue", 14)].into());
    assert_eq!(res, 8)
}

#[test]
fn part2_test() {
    let res = part2(INPUT);
    assert_eq!(res, 2286)
}
