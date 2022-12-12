use std::collections::{HashMap, HashSet};
use utils::IVec2;

#[derive(Default)]
struct Scene {
    grid: Vec<Vec<u8>>,
    start: IVec2,
    end: IVec2,
    cols: i32,
    rows: i32,
}

fn parse_grid(input: &str) -> Scene {
    let mut result = Scene::default();
    for (y, line) in input.lines().enumerate() {
        result.grid.push(Vec::new());
        result.rows += 1;
        result.cols = 0;
        for (x, mut b) in line.as_bytes().iter().copied().enumerate() {
            result.cols += 1;
            if b == b'S' {
                result.start = IVec2::new(x as i32, y as i32);
                b = b'a';
            } else if b == b'E' {
                result.end = IVec2::new(x as i32, y as i32);
                b = b'z';
            }
            result.grid.last_mut().unwrap().push(b - b'a');
        }
    }
    result
}

fn neighbours(scene: &Scene, pos: IVec2) -> impl Iterator<Item = IVec2> + '_ {
    let elevation = scene.grid[pos.y as usize][pos.x as usize];
    (-1..=1)
        .flat_map(move |y| {
            (-1..=1).filter_map(move |x| {
                let d = IVec2::new(x, y);
                let pos = pos + d;
                ((x != 0 || y != 0)
                    && (y == 0 || x == 0)
                    && pos.x >= 0
                    && pos.x < scene.cols
                    && pos.y >= 0
                    && pos.y < scene.rows)
                    .then_some(pos)
            })
        })
        .filter(move |pos| scene.grid[pos.y as usize][pos.x as usize] <= elevation + 1)
}

fn part1(scene: &Scene) -> i32 {
    let mut dist = scene
        .grid
        .iter()
        .map(|row| row.iter().map(|_| std::i32::MAX).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut prev = HashMap::<IVec2, IVec2>::new();
    let mut open_set = HashSet::<IVec2>::new();

    open_set.insert(scene.start);
    dist[scene.start.y as usize][scene.start.x as usize] = 0;

    while !open_set.is_empty() {
        let next = *open_set
            .iter()
            .min_by_key(|p| dist[p.y as usize][p.x as usize])
            .unwrap();
        if next == scene.end {
            break;
        }
        open_set.remove(&next);
        for n in neighbours(scene, next) {
            let d = dist[next.y as usize][next.x as usize] + 1;
            if d < dist[n.y as usize][n.x as usize] {
                dist[n.y as usize][n.x as usize] = d;
                prev.insert(n, next);
                open_set.insert(n);
            }
        }
    }
    let mut count = 0;
    let mut pos = scene.end;
    while pos != scene.start {
        debug_assert_eq!(prev[&pos].manhatten(pos), 1);
        pos = prev[&pos];
        count += 1;
    }
    count
}

fn part2(scene: &Scene) -> i32 {
    let mut open_set = HashSet::<IVec2>::new();
    let mut dist = scene
        .grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .inspect(|(x, d)| {
                    if d == &&0 {
                        open_set.insert(IVec2::new(*x as i32, y as i32));
                    }
                })
                .map(|(_, d)| if d == &0 { 0 } else { std::i32::MAX })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut prev = HashMap::<IVec2, IVec2>::new();

    while !open_set.is_empty() {
        let next = *open_set
            .iter()
            .min_by_key(|p| dist[p.y as usize][p.x as usize])
            .unwrap();
        if next == scene.end {
            break;
        }
        open_set.remove(&next);
        for n in neighbours(scene, next) {
            let d = dist[next.y as usize][next.x as usize] + 1;
            if d < dist[n.y as usize][n.x as usize] {
                dist[n.y as usize][n.x as usize] = d;
                prev.insert(n, next);
                open_set.insert(n);
            }
        }
    }
    let mut count = 0;
    let mut pos = scene.end;
    while dist[pos.y as usize][pos.x as usize] != 0 {
        debug_assert_eq!(prev[&pos].manhatten(pos), 1);
        pos = prev[&pos];
        count += 1;
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let scene = parse_grid(&input);
    let res = part1(&scene);
    println!("part1: {res}");
    let res = part2(&scene);
    println!("part2: {res}");
}

#[test]
fn part1_test() {
    let scene = parse_grid(
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
    );

    let res = part1(&scene);

    assert_eq!(31, res);
}

#[test]
fn part2_test() {
    let scene = parse_grid(
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
    );

    let res = part2(&scene);

    assert_eq!(29, res);
}
