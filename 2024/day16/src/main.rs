use utils::Grid;

type Input = Grid<u8>;

fn parse(input: String) -> Input {
    Grid::from_ascii_lines(&input).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1::part1(&input));
    println!("{}", part2::part2(&input));
}

mod part1 {
    use crate::Input;
    use std::collections::{BinaryHeap, HashMap};
    use utils::IVec2;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        cost: usize,
        pos: IVec2,
        vel: IVec2,
    }

    /// std BinaryHeap is a max-heap so reverse the ord impl
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&self.cost))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    pub fn part1(input: &Input) -> usize {
        let mut visited = HashMap::new();
        let mut todo = BinaryHeap::new();

        todo.push(Node {
            cost: 0,
            pos: input.find(&b'S').unwrap(),
            vel: IVec2::X,
        });

        while let Some(n) = todo.pop() {
            if input[n.pos] == b'E' {
                return n.cost;
            }
            if let Some(c) = visited.get_mut(&(n.pos, n.vel)) {
                if *c < n.cost {
                    continue;
                }
            }
            visited.insert((n.pos, n.vel), n.cost);

            for node in [
                Node {
                    cost: n.cost + 1,
                    pos: n.pos + n.vel,
                    vel: n.vel,
                },
                Node {
                    cost: n.cost + 1000,
                    pos: n.pos,
                    vel: n.vel.rotate_cw(),
                },
                Node {
                    cost: n.cost + 1000,
                    pos: n.pos,
                    vel: n.vel.rotate_ccw(),
                },
            ] {
                if input[node.pos] == b'#'
                    || visited
                        .get(&(node.pos, node.vel))
                        .map(|c| *c > node.cost)
                        .unwrap_or(false)
                {
                    continue;
                }
                todo.push(node);
            }
        }

        unreachable!("Failed to find path")
    }
}

mod part2 {
    use crate::Input;
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use utils::IVec2;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Node {
        cost: usize,
        pos: IVec2,
        vel: IVec2,
        path: Vec<IVec2>,
    }

    /// std BinaryHeap is a max-heap so reverse the ord impl
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&self.cost))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    pub fn part2(input: &Input) -> usize {
        let mut visited = HashMap::new();
        let mut todo = BinaryHeap::new();

        let start = input.find(&b'S').unwrap();
        todo.push(Node {
            cost: 0,
            pos: start,
            vel: IVec2::X,
            path: vec![],
        });

        let mut bests = HashSet::new();
        let mut best_cost = None;

        while let Some(n) = todo.pop() {
            if input[n.pos] == b'E' {
                best_cost = best_cost.or(Some(n.cost));
                if Some(n.cost) != best_cost {
                    continue;
                }
                bests.insert(n.pos);
                for p in n.path {
                    bests.insert(p);
                }
                continue;
            }
            if let Some(c) = visited.get_mut(&(n.pos, n.vel)) {
                if *c < n.cost {
                    continue;
                }
            }
            visited.insert((n.pos, n.vel), n.cost);

            let mut path = n.path;
            path.push(n.pos);

            for node in [
                Node {
                    cost: n.cost + 1,
                    pos: n.pos + n.vel,
                    vel: n.vel,
                    path: path.clone(),
                },
                Node {
                    cost: n.cost + 1000,
                    pos: n.pos,
                    vel: n.vel.rotate_cw(),
                    path: path.clone(),
                },
                Node {
                    cost: n.cost + 1000,
                    pos: n.pos,
                    vel: n.vel.rotate_ccw(),
                    path: path.clone(),
                },
            ] {
                if input[node.pos] == b'#'
                    || visited
                        .get(&(node.pos, node.vel))
                        .map(|c| *c > node.cost)
                        .unwrap_or(false)
                {
                    continue;
                }
                todo.push(node);
            }
        }

        bests.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1::part1(&inp);

        assert_eq!(res, 7036);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2::part2(&inp);

        assert_eq!(res, 45);
    }
}
