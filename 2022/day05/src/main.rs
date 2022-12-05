use std::collections::VecDeque;

use regex::Regex;

fn solve(input: &str) -> (String, ()) {
    let mut stacks = Vec::new();
    stacks.reserve(10);

    let mut lines = input.lines();

    let re = Regex::new(r"\[([A-Z])\]|( {3})").unwrap();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let captures = re.captures_iter(line);

        for x in captures {
            if let Some(x) = x.get(1) {
                let i = x.start() / 4;
                while stacks.len() <= i {
                    stacks.push(VecDeque::<&str>::new());
                }
                stacks[i].push_front(x.as_str());
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        let Some(captures) = re.captures(line) else {
            continue;
        };
        let n: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let to: usize = captures.get(3).unwrap().as_str().parse().unwrap();

        for _ in 0..n {
            let x = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(x);
        }
    }
    let part1 = stacks
        .into_iter()
        .map(|mut stack| stack.pop_back().unwrap_or(" "))
        .collect();
    (part1, ())
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (p1, _p2) = solve(&input);
    println!("p1: {}", p1);
    // println!("p2: {}", p2);
}

#[test]
fn basic_test() {
    let (part1, _part2) = solve(
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
    );

    assert_eq!(part1, "CMZ")
}
