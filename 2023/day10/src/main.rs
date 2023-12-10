use std::collections::{HashMap, HashSet, VecDeque};

use arrayvec::ArrayVec;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    let (p1, visited) = part1(&input);
    println!("{}", p1);
    println!("{}", part2(&input, &visited));
}

type Point = [i32; 2];

#[derive(Debug, Default)]
struct Input<'a> {
    connections: HashMap<Point, ArrayVec<Point, 4>>,
    start: Point,
    grid: Vec<&'a str>,
}

impl Input<'_> {
    pub fn insert(&mut self, a: Point, b: Point) {
        let e = self.connections.entry(a).or_default();
        if !e.contains(&b) {
            e.push(b);
        }
        let e = self.connections.entry(b).or_default();
        if !e.contains(&a) {
            e.push(a);
        }
    }
}

fn is_valid(x: i32, y: i32, c: char, grid: &[&str]) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    match c {
        '|' => {
            if y <= 0 || y as usize + 1 >= height {
                return false;
            }
            match grid[y as usize - 1].as_bytes()[x as usize] as char {
                'S' | 'F' | '7' | '|' => {}
                _ => return false,
            }
            match grid[y as usize + 1].as_bytes()[x as usize] as char {
                'S' | 'L' | 'J' | '|' => {}
                _ => return false,
            }
        }
        '-' => {
            if x <= 0 || x as usize + 1 >= width {
                return false;
            }
            match grid[y as usize].as_bytes()[x as usize - 1] as char {
                'S' | '-' | 'L' | 'F' => {}
                _ => return false,
            }
            match grid[y as usize].as_bytes()[x as usize + 1] as char {
                'S' | '-' | 'J' | '7' => {}
                _ => return false,
            }
        }
        'F' => {
            if x as usize + 1 >= width || y as usize + 1 >= height {
                return false;
            }
            match grid[y as usize].as_bytes()[x as usize + 1] as char {
                'S' | '-' | 'J' | '7' => {}
                _ => return false,
            }
            match grid[y as usize + 1].as_bytes()[x as usize] as char {
                'S' | 'L' | 'J' | '|' => {}
                _ => return false,
            }
        }
        'J' => {
            if x <= 0 || y <= 0 {
                return false;
            }
            match grid[y as usize].as_bytes()[x as usize - 1] as char {
                'S' | '-' | 'L' | 'F' => {}
                _ => return false,
            }
            match grid[y as usize - 1].as_bytes()[x as usize] as char {
                'S' | 'F' | '7' | '|' => {}
                _ => return false,
            }
        }
        'L' => {
            if x as usize + 1 >= width || y <= 0 {
                return false;
            }
            match grid[y as usize].as_bytes()[x as usize + 1] as char {
                'S' | '-' | 'J' | '7' => {}
                _ => return false,
            }
            match grid[y as usize - 1].as_bytes()[x as usize] as char {
                'S' | 'F' | '7' | '|' => {}
                _ => return false,
            }
        }
        '7' => {
            if x <= 0 || y as usize + 1 >= height {
                return false;
            }
            match grid[y as usize].as_bytes()[x as usize - 1] as char {
                'S' | '-' | 'L' | 'F' => {}
                _ => return false,
            }
            match grid[y as usize + 1].as_bytes()[x as usize] as char {
                'S' | 'L' | 'J' | '|' => {}
                _ => return false,
            }
        }
        _ => return false,
    }
    true
}

fn parse(input: &str) -> Input {
    let mut result = Input::default();
    let lines = input.lines().collect::<Vec<_>>();
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let pos = [x, y];
            if c == 'S' {
                result.start = pos;
                continue;
            }
            if is_valid(x, y, c, &lines) {
                match c {
                    '|' => {
                        result.insert(pos, [x, y + 1]);
                        result.insert(pos, [x, y - 1]);
                    }
                    '-' => {
                        result.insert(pos, [x - 1, y]);
                        result.insert(pos, [x + 1, y]);
                    }
                    'F' => {
                        result.insert(pos, [x, y + 1]);
                        result.insert(pos, [x + 1, y]);
                    }
                    'J' => {
                        result.insert(pos, [x, y - 1]);
                        result.insert(pos, [x - 1, y]);
                    }
                    'L' => {
                        result.insert(pos, [x, y - 1]);
                        result.insert(pos, [x + 1, y]);
                    }
                    '7' => {
                        result.insert(pos, [x, y + 1]);
                        result.insert(pos, [x - 1, y]);
                    }
                    _ => {}
                }
            }
        }
    }
    result.grid = lines;
    result
}

fn part1(input: &Input) -> (i32, HashMap<Point, i32>) {
    let mut visited = HashMap::new();
    let mut todo = VecDeque::new();
    todo.push_back((input.start, 0));
    while let Some((current, steps)) = todo.pop_front() {
        let s = visited.entry(current).or_insert(i32::MAX);
        if *s > steps {
            *s = steps;
            for next in input.connections[&current].iter().copied() {
                todo.push_back((next, steps + 1));
            }
        }
    }

    (visited.values().copied().max().unwrap(), visited)
}

#[allow(unused)]
fn printcontour(loop_pts: &HashMap<Point, i32>, input: &Input) {
    let height = input.grid.len();
    let width = input.grid[0].len();

    for y in 0..height {
        for x in 0..width {
            if loop_pts.contains_key(&[x as i32, y as i32]) {
                print!("{}", input.grid[y].as_bytes()[x] as char);
            } else {
                print!(" ");
            }
        }
        println!(" ");
    }
}

fn rotate_ccw([x, y]: [i32; 2]) -> [i32; 2] {
    [-y, x]
}

fn rotate_cw([x, y]: [i32; 2]) -> [i32; 2] {
    [y, -x]
}

fn part2(input: &Input, loop_pts: &HashMap<Point, i32>) -> i32 {
    let height = input.grid.len();
    let width = input.grid[0].len();
    // 0=unmarked, 1=inside, 2=loop
    let mut grid = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            if loop_pts.contains_key(&[x as i32, y as i32]) {
                grid[y][x] = 2;
            }
        }
    }

    let mut loop_pts_vec: Vec<_> = loop_pts.keys().collect();
    loop_pts_vec.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
    loop_pts_vec.sort_by(|a, b| a[0].cmp(&b[0]));
    // top edge, leftmost pos
    let start = loop_pts_vec[0];

    // TODO: support 'S'?
    assert_eq!(
        input.grid[start[1] as usize].as_bytes()[start[0] as usize],
        b'F',
        "{}",
        input.grid[start[1] as usize].as_bytes()[start[0] as usize] as char,
    );
    // walk the contour, mark areas inside the loop
    let mut visited = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((*start, [start[0], start[1] + 1], [1, 0]));
    visited.insert([start[0], start[1] + 1]); // ensures that only 1 path is started
    let mut mark_neighbour = |current: [i32; 2], tangent: [i32; 2]| {
        let n = [current[0] + tangent[0], current[1] + tangent[1]];
        if (n[1] as usize) < height
            && (n[0] as usize) < width
            && grid[n[1] as usize][n[0] as usize] == 0
        {
            grid[n[1] as usize][n[0] as usize] = 1;
        }
    };
    while let Some((current, from, mut tangent)) = todo.pop_front() {
        mark_neighbour(current, tangent);

        let c = input.grid[current[1] as usize].as_bytes()[current[0] as usize];
        let delta = [current[0] - from[0], current[1] - from[1]];
        match c {
            b'F' => {
                if delta[0] == -1 {
                    tangent = rotate_cw(tangent);
                } else {
                    assert_eq!(delta[1], -1);
                    tangent = rotate_ccw(tangent);
                }
                mark_neighbour(current, tangent);
            }
            b'7' => {
                if delta[0] == 1 {
                    tangent = rotate_ccw(tangent);
                } else {
                    assert_eq!(delta[1], -1);
                    tangent = rotate_cw(tangent);
                }
                mark_neighbour(current, tangent);
            }
            b'J' => {
                if delta[0] == 1 {
                    tangent = rotate_cw(tangent);
                } else {
                    assert_eq!(delta[1], 1);
                    tangent = rotate_ccw(tangent);
                }
                mark_neighbour(current, tangent);
            }
            b'L' => {
                if delta[0] == -1 {
                    tangent = rotate_ccw(tangent);
                } else {
                    assert_eq!(delta[1], 1);
                    tangent = rotate_cw(tangent);
                }
                mark_neighbour(current, tangent);
            }
            b'S' => {
                let next = input.connections[&current]
                    .iter()
                    .find(|n| *n != &from)
                    .unwrap();

                let dd = [next[0] - current[0], next[1] - current[1]];
                if dd != delta {
                    // S is a corner
                    let det = delta[0] * dd[1] - delta[1] * dd[0];
                    match det {
                        1 => tangent = rotate_ccw(tangent),
                        -1 => tangent = rotate_cw(tangent),
                        _ => unreachable!(),
                    }
                }
            }
            _ => {}
        }
        visited.insert(current);
        for next in input.connections[&current].iter().copied() {
            if !visited.contains(&next) {
                todo.push_back((next, current, tangent));
            }
        }
    }

    // flood fill marked areas
    let mut todo = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| **c == 1)
        .map(|(x, y, _)| [x as i32, y as i32])
        .collect::<Vec<_>>();
    dbg!(todo.len());

    while let Some(pos) = todo.pop() {
        for delta in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
            let next = [pos[0] + delta[0], pos[1] + delta[1]];
            if next[0] < 0
                || next[0] as usize >= width
                || next[1] < 0
                || next[1] as usize >= height
                || grid[next[1] as usize][next[0] as usize] != 0
            {
                continue;
            }
            grid[next[1] as usize][next[0] as usize] = 1;
            todo.push(next);
        }
    }

    // count
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == 1)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_2() {
        let inp = parse(
            r#".....
.S-7.
.|.|.
.L-J.
....."#,
        );
        let res = part1(&inp).0;

        assert_eq!(res, 4);
    }

    #[test]
    fn test_p2_1() {
        let inp = parse(
            r#"..........
.F---S--7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."#,
        );
        let (_, visited) = part1(&inp);
        let res = part2(&inp, &visited);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_p2_2() {
        let inp = parse(
            r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#,
        );
        let (_, visited) = part1(&inp);
        let res = part2(&inp, &visited);

        assert_eq!(res, 8);
    }

    #[test]
    fn test_p2_3() {
        let inp = parse(
            r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#,
        );
        let (_, visited) = part1(&inp);
        let res = part2(&inp, &visited);

        assert_eq!(res, 10);
    }

    #[test]
    fn test_p2_4() {
        let inp = parse(
            r#"...........
.F---S---7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
........... "#,
        );
        let (_, visited) = part1(&inp);
        let res = part2(&inp, &visited);

        assert_eq!(res, 4);
    }
}
