use std::collections::HashMap;
use std::fs::read_to_string;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn cost(initial: i32, node: &str, g: &Graph) -> i32 {
    let mut c = initial;
    if let Some(ref node) = g.get(node) {
        for child in node.iter() {
            c += cost(initial + 1, child, g);
        }
    }
    c
}

fn part1(input: &str) -> i32 {
    let mut g = Graph::new();
    for conn in input.split('\n') {
        let mut it = conn.split(')');
        if let (Some(a), Some(b)) = (it.next(), it.next()) {
            g.entry(a).or_default().push(b);
            g.entry(b).or_default();
        }
    }
    println!("{:?}", g["COM"]);
    cost(0, "COM", &g)
}

fn main() {
    let input = &read_to_string("input.txt").unwrap();
    let input2 = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
    ";

    let res = part1(input);
    println!("{}", res);
}
