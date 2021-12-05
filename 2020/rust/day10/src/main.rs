#[cfg(test)]
mod tests;

use arrayvec::ArrayVec;
use std::collections::{HashMap, HashSet};

type Graph = HashMap<i32, ArrayVec<[i32; 3]>>;

/// build all possible connections
fn parse(inp: &str) -> Graph {
    let mut res: Vec<i32> = inp.lines().filter_map(|line| line.parse().ok()).collect();

    res.push(0); // wall socket
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
    let edges = topological_sort(g);

    let mut counts: HashMap<i32, usize> = HashMap::new();
    counts.insert(edges[0], 1);
    for i in 1..edges.len() {
        let children = &g[&edges[i]];
        let mut c = 0;
        for child in children {
            c += counts.get(child).cloned().unwrap_or(0);
        }
        counts.insert(edges[i], c);
    }

    counts[&0]
}

/// returns vertices topologically sorted IN REVERSE ORDER!
fn topological_sort(g: &Graph) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut stack = Vec::with_capacity(g.len());

    for node in g.keys() {
        if !visited.contains(node) {
            topological_sort_unil(*node, g, &mut visited, &mut stack);
        }
    }

    stack
}

fn topological_sort_unil(v: i32, g: &Graph, visited: &mut HashSet<i32>, stack: &mut Vec<i32>) {
    visited.insert(v);
    if let Some(children) = g.get(&v) {
        for child in children {
            if !visited.contains(child) {
                topological_sort_unil(*child, g, visited, stack);
            }
        }
    }
    stack.push(v);
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
