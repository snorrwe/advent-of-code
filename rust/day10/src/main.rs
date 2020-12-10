use arrayvec::ArrayVec;
use std::collections::HashMap;

type Graph = HashMap<i32, ArrayVec<[i32; 3]>>;

/// build all possible connections
fn parse(inp: &str) -> Graph {
    let mut res: Vec<i32> = inp.lines().filter_map(|line| line.parse().ok()).collect();

    res.sort_unstable();
    res.push(res.last().unwrap() + 3); // our device

    let mut g = Graph::new();
    _build_graph(res.as_slice(), &mut g);
    g
}

fn _build_graph(inp: &[i32], g: &mut Graph) {
    let len = inp.len();
    'root: for (i, root) in inp.iter().take(len - 1).cloned().enumerate() {
        for x in &inp[i + 1..] {
            let d = x - root;
            if d <= 3 {
                g.entry(root).or_insert_with(ArrayVec::new).push(*x);
            } else {
                // assumed that inp is sorted
                continue 'root;
            }
        }
    }
}

fn part1(g: &Graph) -> i32 {
    // find a fully connected path
    let mut path = Vec::with_capacity(g.len() + 2);
    path.push(0);
    path.extend(g.keys());
    path.sort_unstable();
    path.push(path.last().unwrap() + 3); // our device
    let mut ones = 0;
    let mut threes = 0;
    // count the 1 and 3 diffs
    for i in 0..path.len() - 1 {
        let a = path[i];
        let b = path[i + 1];
        let ba = b - a;
        assert!(ba > 0);
        assert!(ba <= 3);
        if ba == 1 {
            ones += 1
        } else if ba == 3 {
            threes += 1
        }
    }
    ones * threes
}

fn part2(g: &Graph) -> usize {
    let target = g.keys().max().unwrap() + 3;
    _reachable(target, 0, g) * 2
}

/// count valid arrangements
fn _reachable(to: i32, from: i32, g: &Graph) -> usize {
    let mut res = 0;
    for i in 1..=3 {
        let n = from + i;
        if let Some(children) = g.get(&n) {
            for node in children {
                if node == &to {
                    return res + 1;
                }
                let r = _reachable(to, *node, g);
                res += r;
            }
        }
    }
    res
}

fn main() {
    let mut input = String::new();

    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let graph = parse(input.as_str());

    let res = part1(&graph);
    println!("part 1: {}", res);
    let res = part2(&graph);
    println!("part 2: {}", res);
}

#[test]
fn part1_1() {
    let inp = r#"
16
10
15
5
1
11
7
19
6
12
4
        "#;
    let nums = parse(inp);

    let res = part1(&nums);
    assert_eq!(res, 35);
}

#[test]
fn part1_2() {
    let inp = r#"

28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
        "#;
    let nums = parse(inp);

    let res = part1(&nums);
    assert_eq!(res, 220);
}

#[test]
fn part2_1() {
    let inp = r#"
16
10
15
5
1
11
7
19
6
12
4
        "#;
    let nums = parse(inp);

    let res = part2(&nums);
    assert_eq!(res, 8);
}

#[test]
fn part2_2() {
    let inp = r#"

28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
        "#;
    let nums = parse(inp);

    let res = part2(&nums);
    assert_eq!(res, 19208);
}
