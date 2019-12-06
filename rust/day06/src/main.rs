#![feature(test)]
extern crate test;

use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Default, Debug)]
struct Node<'a> {
    children: Vec<&'a str>,
    parent: Option<&'a str>,
}

type Graph<'a> = HashMap<&'a str, Node<'a>>;

fn cost(initial: i32, node: &str, g: &Graph) -> i32 {
    let mut c = initial;
    if let Some(ref node) = g.get(node) {
        for child in node.children.iter() {
            c += cost(initial + 1, child, g);
        }
    }
    c
}

fn get_g<'a>(input: &'a str) -> Graph<'a> {
    let mut g = Graph::with_capacity(1 << 12);
    for conn in input.split('\n') {
        let mut it = conn.split(')');
        if let (Some(a), Some(b)) = (it.next(), it.next()) {
            g.entry(a).or_default().children.push(b);
            g.entry(b).or_default().parent = Some(a);
        }
    }
    g
}

fn part1(input: &str) -> i32 {
    let g = get_g(input);
    cost(0, "COM", &g)
}

fn part2<'a>(input: &'a str) -> i32 {
    let g = get_g(input);
    let mut you = HashMap::<&'a str, i32>::with_capacity(1 << 8);
    let mut san = HashMap::<&'a str, i32>::with_capacity(1 << 8);
    let mut y = "YOU";
    let mut s = "SAN";
    let mut i = 0i32;
    loop {
        you.insert(y, i);
        san.insert(s, i);
        y = g[y].parent.expect("y parent");
        if san.contains_key(y) {
            return san[y] + i - 1;
        }
        s = g[s].parent.expect("s parent");
        if you.contains_key(s) {
            return you[s] + i - 1;
        }
        i += 1;
    }
}

fn main() {
    let input = &read_to_string("input.txt").unwrap();
    let res = part1(input);
    println!("{}", res);
    let res = part2(input);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| main());
    }
}
