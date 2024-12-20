use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use utils::{Grid, IVec2};

type Input = Grid<u8>;

fn parse(input: String) -> Input {
    Grid::from_ascii_lines(&input).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input, 100));
    println!("{}", part2(&input, 100));
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

fn find_path(grid: &Grid<u8>, start: IVec2, goal: IVec2) -> Option<Vec<IVec2>> {
    let mut todo = BinaryHeap::new();
    todo.push(Node {
        c_cost: 0,
        h_cost: start.manhatten(goal),
        pos: start,
    });

    let mut visited = HashMap::new();
    visited.insert(start, start);

    while let Some(n) = todo.pop() {
        if n.pos == goal {
            let mut path = Vec::with_capacity(n.c_cost as usize);

            let mut pos = n.pos;
            while pos != start {
                path.push(pos);
                pos = visited[&pos];
            }

            return Some(path);
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

fn part1(input: &Input, savings: usize) -> i32 {
    solve(input, savings, 2)
}

fn solve(input: &Input, savings: usize, radius: i32) -> i32 {
    let start = input.find(&b'S').unwrap();
    let end = input.find(&b'E').unwrap();

    let mut initial = find_path(&input, start, end).unwrap();
    initial.push(start);
    initial.reverse();
    let initial_score = initial.len();

    let mut total = 0;

    let mut cache = HashMap::new();

    let mut cost_so_far = 0;
    for pos in initial.into_iter() {
        cost_so_far += 1;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let posi = pos + IVec2::new(dx, dy);
                if (dy == 0 && dx == 0)
                    || pos.manhatten(posi) > radius
                    || !input.contains_point(posi)
                    || input[posi] == b'#'
                {
                    continue;
                }
                if let Some(cost) = cache
                    .entry(posi)
                    .or_insert_with(|| find_path(input, posi, end).map(|p| p.len()))
                {
                    let total_cost = cost_so_far + *cost + pos.manhatten(posi) as usize;
                    if initial_score.saturating_sub(total_cost) >= savings {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn part2(input: &Input, savings: usize) -> i32 {
    solve(input, savings, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;

    #[test]
    fn test_p1() {
        let mut inp = parse(INPUT.to_string());
        let res = part1(&mut inp, 1);

        assert_eq!(res, 14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1);
    }

    #[test]
    fn test_p2() {
        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp, 50);

        assert_eq!(
            res,
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
