use std::collections::{HashMap, HashSet};

type Pos = [i32; 4];
type Grid = HashSet<Pos>;

/// return active positions
fn parse(inp: &str) -> Grid {
    let mut res = HashSet::new();

    for (row, line) in inp.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == '#' {
                res.insert([column as i32, row as i32, 0, 0]);
            }
        }
    }

    res
}

enum Change {
    Del(Pos),
    Insert(Pos),
}

fn forward(active: &mut Grid) {
    let mut changes = Vec::with_capacity(active.len());
    let mut empty_neighbours = HashMap::new(); // (pos, num of active neighbours)
    for pos in active.iter() {
        let mut count = 0;
        let [x, y, z, w] = pos;
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        let p = [x + i, y + j, z + k, w + l];
                        if active.contains(&p) {
                            count += 1;
                        } else {
                            *empty_neighbours.entry(p).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        count -= 1; // the loop counted the position itself, subtract it
        if count != 2 && count != 3 {
            changes.push(Change::Del(*pos));
        }
    }
    for (pos, _) in empty_neighbours.into_iter().filter(|(_, v)| *v == 3) {
        changes.push(Change::Insert(pos));
    }

    for change in changes {
        match change {
            Change::Del(p) => {
                active.remove(&p);
            }
            Change::Insert(p) => {
                active.insert(p);
            }
        }
    }
}

pub fn part2(inp: &str) -> usize {
    let mut board = parse(inp);
    for _ in 0..6 {
        forward(&mut board);
    }
    board.len()
}

#[test]
fn test_part2() {
    let inp = r#".#.
..#
###"#;

    let res = part2(inp);
    assert_eq!(res, 848);
}
