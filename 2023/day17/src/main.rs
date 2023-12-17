use std::collections::{BinaryHeap, HashSet};

use utils::{Grid, IVec2};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

#[derive(PartialEq, Eq)]
struct Node {
    pos: IVec2,
    dir: IVec2,
    steps: i32,
    cost: usize,
}

// BinaryHeap is a max-heap
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn part1(input: &str) -> usize {
    let grid = {
        let width = input.lines().next().map(|l| l.len()).unwrap();
        let data = input
            .lines()
            .take_while(|l| l.len() == width)
            .flat_map(|l| l.bytes())
            .map(|c| c - b'0')
            .collect();
        Grid::from_data(data, width)
    };

    let end = IVec2::new(grid.width as i32 - 1, grid.height as i32 - 1);
    let mut todo: BinaryHeap<_> = [
        Node {
            pos: IVec2::ZERO,
            dir: IVec2::X,
            steps: 1,
            cost: 0,
        },
        Node {
            pos: IVec2::ZERO,
            dir: IVec2::Y,
            steps: 1,
            cost: 0,
        },
    ]
    .into();

    let mut visited = HashSet::new();

    while let Some(Node {
        pos,
        dir,
        steps,
        cost: cost_so_far,
    }) = todo.pop()
    {
        if pos == end {
            return cost_so_far;
        }
        let mut enqueue = |pos, dir, steps| {
            let pos = pos + dir;
            if grid.contains_point(pos) && !visited.contains(&(pos, dir, steps)) {
                visited.insert((pos, dir, steps));
                let new_cost = cost_so_far + grid[pos] as usize;
                todo.push(Node {
                    pos,
                    dir,
                    steps,
                    cost: new_cost,
                });
            }
        };
        if steps < 3 {
            enqueue(pos, dir, steps + 1);
        }
        enqueue(pos, dir.rotate_ccw(), 1);
        enqueue(pos, dir.rotate_cw(), 1);
    }
    unreachable!()
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 102);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 42);
    }
}
