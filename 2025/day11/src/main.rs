use std::collections::{HashMap, HashSet};

struct Input<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}

fn parse(input: &'_ str) -> Input<'_> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in input.lines() {
        let Some((from, to)) = l.split_once(':') else {
            continue;
        };
        let from = from.trim();
        for to in to.split(' ').filter(|l| !l.is_empty()) {
            connections.entry(from).or_default().push(to);
        }
    }

    Input { connections }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn find_out_dfs<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    visited: &mut HashSet<&'a str>,
) -> usize {
    let mut s = 0;
    for n in connections[current].iter().copied() {
        if n == "out" {
            // if any of the outputs is 'out', then short-circuit the recursion
            return 1;
        }
        s += find_out_dfs(connections, n, visited);
    }
    s
}

fn part1(input: &Input) -> usize {
    find_out_dfs(&input.connections, "you", &mut Default::default())
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
