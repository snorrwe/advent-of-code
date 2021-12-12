use std::collections::{HashMap, HashSet};

type Edges<'a> = HashMap<&'a str, Vec<&'a str>>;

fn graph_insert<'a>(graph: &mut Edges<'a>, from: &'a str, to: &'a str) {
    graph.entry(from).or_default().push(to);
}

fn main() {
    let mut graph = Edges::with_capacity(512);

    let mut buffer = String::with_capacity(1024 * 1024);
    while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        if size == 0 {
            break;
        }
    }

    for line in buffer.lines() {
        let mut nodes = line.split('-');
        let lhs = nodes.next().unwrap();
        let rhs = nodes.next().unwrap().trim_end();

        graph_insert(&mut graph, lhs, rhs);
        graph_insert(&mut graph, rhs, lhs);
    }

    let mut explored = HashSet::new();
    let mut p1 = 0;
    visit_paths(&graph, "start", &mut explored, &mut p1);

    println!("P1: {}", p1);
}

// dfs
fn visit_paths<'a>(
    graph: &Edges<'a>,
    parent: &'a str,
    explored: &mut HashSet<&'a str>,
    count: &mut usize,
) {
    if parent == "end" {
        // dbg!(&path[1..]);
        *count += 1;
        return;
    }
    explored.insert(parent);
    for node in graph[parent].iter().copied() {
        if !explored.contains(node)
            // big caves can be visited multiple times
            || (node.chars().next().unwrap().is_uppercase())
        {
            visit_paths(graph, node, explored, count);
        }
    }
    explored.remove(parent);
}
