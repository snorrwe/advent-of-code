use std::collections::{BinaryHeap, HashMap};

fn index(x: i32, y: i32, width: i32, height: i32) -> Option<usize> {
    if x >= width || y >= height || x < 0 || y < 0 {
        return None;
    }
    Some((y * width + x) as usize)
}

fn neighbours(x: i32, y: i32) -> [[i32; 2]; 4] {
    [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]]
}

#[derive(Eq, PartialEq)]
struct Node {
    pos: [i32; 2],
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn main() {
    let mut buffer = String::with_capacity(1024);
    let mut cave = Vec::with_capacity(10000);
    let mut w = 0;
    let mut h = 0;

    while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if line.len() == 0 {
            break;
        }
        h += 1;
        w = 0;
        for c in line.chars() {
            cave.push(c as u8 - '0' as u8);
            w += 1;
        }
        buffer.clear();
    }

    let mut parent = HashMap::new();
    let mut nodes = BinaryHeap::new();
    nodes.push(Node {
        pos: [0, 0],
        cost: 0,
    });

    let finish = [w - 1, h - 1];
    while let Some(node) = nodes.pop() {
        for neighbour in neighbours(node.pos[0], node.pos[1]).into_iter() {
            if neighbour == finish {
                let score = cave[index(w - 1, h - 1, w, h).unwrap()] as usize + node.cost;
                println!("p1: {}", score);
                return;
            }
            if parent.contains_key(&neighbour) {
                continue;
            }
            if let Some(i) = index(neighbour[0], neighbour[1], w, h) {
                let new_cost = cave[i] as usize + node.cost;
                parent.insert(neighbour, node.pos);
                nodes.push(Node {
                    pos: neighbour,
                    cost: new_cost,
                });
            }
        }
    }

    unreachable!()
}
