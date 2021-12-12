use std::collections::HashMap;

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

    let mut explored = HashMap::new();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut small_twice = false;
    visit_paths_p2(
        &graph,
        "start",
        &mut explored,
        &mut p1,
        &mut p2,
        &mut small_twice,
    );

    println!("P1: {} P2: {}", p1, p2);
}

// dfs
fn visit_paths_p2<'a>(
    graph: &Edges<'a>,
    parent: &'a str,
    explored: &mut HashMap<&'a str, usize>,
    p1: &mut usize,
    p2: &mut usize,
    small_twice: &mut bool,
) {
    if parent == "end" {
        *p2 += 1;
        if !*small_twice {
            *p1 += 1;
        }
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
            visit_paths_p2(graph, node, explored, p1, p2, small_twice);
        }
    }
    let exp = explored.entry(parent).or_insert(0);
    *exp -= 1;
    if *exp == 1 && parent.chars().next().unwrap().is_lowercase() {
        *small_twice = false;
    }
}
