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

fn find_out_dfs<'a>(connections: &HashMap<&'a str, Vec<&'a str>>, current: &'a str) -> usize {
    let mut s = 0;
    for n in connections[current].iter().copied() {
        if n == "out" {
            // if any of the outputs is 'out', then short-circuit the recursion
            return 1;
        }
        s += find_out_dfs(connections, n);
    }
    s
}

fn part1(input: &Input) -> usize {
    find_out_dfs(&input.connections, "you")
}

fn find_out_dfs_v2<'a>(
    connections: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    goal: &'a str,
    visited: &mut HashSet<&'a str>,
) -> usize {
    if !visited.insert(current) {
        return 0;
    }

    macro_rules! visit_cleanup {
        () => {{
            visited.remove(current);
        }};
    }

    let Some(conn) = connections.get(current) else {
        visit_cleanup!();
        return 0;
    };
    // if any of the outputs is 'out', then short-circuit the recursion, without polluting 'visited'
    for n in conn.iter().copied() {
        if n == goal {
            visit_cleanup!();
            return 1;
        }
    }
    let mut s = 0;
    for n in conn.iter().copied() {
        s += find_out_dfs_v2(connections, n, goal, visited);
    }
    visit_cleanup!();
    s
}

fn part2(input: &Input) -> usize {
    let mut visited: HashSet<&str> = Default::default();
    visited.insert("dac");
    visited.insert("out");
    let a1 = find_out_dfs_v2(&input.connections, "svr", "fft", &mut visited);
    visited.remove("dac");
    let a2 = find_out_dfs_v2(&input.connections, "fft", "dac", &mut visited);
    visited.remove("out");
    let a3 = find_out_dfs_v2(&input.connections, "dac", "out", &mut visited);

    visited.clear();
    visited.insert("fft");
    visited.insert("out");
    let b1 = find_out_dfs_v2(&input.connections, "svr", "dac", &mut visited);
    visited.remove("fft");
    let b2 = find_out_dfs_v2(&input.connections, "dac", "fft", &mut visited);
    visited.remove("out");
    let b3 = find_out_dfs_v2(&input.connections, "fft", "out", &mut visited);

    dbg!(a1, a2, a3, b1, b2, b3);

    a1 * a2 * a3 + b1 * b2 * b3
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
    const INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT2);
        let res = part2(&inp);

        assert_eq!(res, 2);
    }
}
