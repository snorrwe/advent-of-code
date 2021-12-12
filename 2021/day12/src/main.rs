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
    visit_paths_p1(&graph, "start", &mut explored, &mut p1);

    let mut explored = HashMap::new();
    let mut p2 = 0;
    let mut small_twice = false;
    visit_paths_p2(&graph, "start", &mut explored, &mut p2, &mut small_twice);

    println!("P1: {} P2: {}", p1, p2);
}

// dfs
fn visit_paths_p1<'a>(
    graph: &Edges<'a>,
    parent: &'a str,
    explored: &mut HashSet<&'a str>,
    count: &mut usize,
) {
    if parent == "end" {
        *count += 1;
        return;
    }
    explored.insert(parent);
    for node in graph[parent].iter().copied() {
        if !explored.contains(node)
            // big caves can be visited multiple times
            || (node.chars().next().unwrap().is_uppercase())
        {
            visit_paths_p1(graph, node, explored, count);
        }
    }
    explored.remove(parent);
}

fn visit_paths_p2<'a>(
    graph: &Edges<'a>,
    parent: &'a str,
    explored: &mut HashMap<&'a str, usize>,
    count: &mut usize,
    small_twice: &mut bool,
) {
    if parent == "end" {
        *count += 1;
        return;
    }

    let exp = explored.entry(parent).or_insert(0);
    *exp += 1;
    if *exp == 2 && parent.chars().next().unwrap().is_lowercase() {
        if *small_twice {
            *exp -= 1;
            return;
        }
        *small_twice = true;
    }
    for node in graph[parent].iter().copied() {
        //big caves can be visited multiple times
        if (node.chars().next().unwrap().is_uppercase())
            || (node != "start" && explored.get(node).map(|n| *n <= 1).unwrap_or(true))
        {
            visit_paths_p2(graph, node, explored, count, small_twice);
        }
    }
    let exp = explored.entry(parent).or_insert(0);
    *exp -= 1;
    if *exp == 1 && parent.chars().next().unwrap().is_lowercase() {
        *small_twice = false;
    }
}
