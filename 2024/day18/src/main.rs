use std::collections::{BinaryHeap, HashMap};

use utils::{Grid, IVec2};

type Input = Vec<IVec2>;

fn parse(input: String) -> Input {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (x, y) = l.trim().split_once(',').unwrap();
            IVec2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input, 1024, 71));
    let p2 = part2(&input, 1024, 71);
    println!("{},{}", p2.x, p2.y);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    c_cost: i32,
    h_cost: i32,
    pos: IVec2,
}

impl Node {
    fn cost(self) -> i32 {
        self.c_cost + self.h_cost
    }
}

/// std BinaryHeap is a max-heap so reverse the ord impl
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost().cmp(&self.cost()))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost().cmp(&self.cost())
    }
}

fn reachable(grid: &Grid<u8>, goal: IVec2) -> Option<usize> {
    let mut todo = BinaryHeap::new();
    todo.push(Node {
        c_cost: 0,
        h_cost: IVec2::ZERO.manhatten(goal),
        pos: IVec2::ZERO,
    });

    let mut visited = HashMap::new();
    visited.insert(IVec2::ZERO, IVec2::ZERO);

    while let Some(n) = todo.pop() {
        if n.pos == goal {
            return Some(n.c_cost as usize);
        }
        for neighbour in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y]
            .into_iter()
            .map(|d| n.pos + d)
        {
            if grid.contains_point(neighbour)
                && grid[neighbour] != b'#'
                && !visited.contains_key(&neighbour)
            {
                visited.insert(neighbour, n.pos);
                todo.push(Node {
                    pos: neighbour,
                    h_cost: neighbour.manhatten(goal),
                    c_cost: n.c_cost + 1,
                });
            }
        }
    }
    None
}

fn part1(input: &Input, take: usize, size: usize) -> usize {
    let mut grid = Grid::new(size, size);
    let goal = IVec2::new(size as i32 - 1, size as i32 - 1);
    grid.fill(b'.');
    for pos in input.iter().take(take) {
        grid[pos] = b'#';
    }

    reachable(&grid, goal).expect("No solution found")
}

fn part2(input: &Input, take: usize, size: usize) -> IVec2 {
    let mut grid = Grid::new(size, size);
    let goal = IVec2::new(size as i32 - 1, size as i32 - 1);
    grid.fill(b'.');
    for pos in input.iter().take(take) {
        grid[pos] = b'#';
    }

    for pos in input.iter().skip(take) {
        grid[pos] = b'#';
        if reachable(&grid, goal).is_none() {
            return *pos;
        }
    }

    unreachable!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp, 12, 7);

        assert_eq!(res, 22);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp, 12, 7);

        assert_eq!(res, IVec2::new(6, 1));
    }
}
