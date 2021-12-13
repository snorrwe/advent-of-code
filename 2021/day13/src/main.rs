use std::collections::HashSet;
use std::io::prelude::*;

fn mirror(axis: char, coord: i32, pos: [i32; 2]) -> [i32; 2] {
    let [mut x, mut y] = pos;
    match axis {
        'x' => {
            if x > coord {
                x = coord - (x - coord);
            }
        }
        'y' => {
            if y > coord {
                y = coord - (y - coord);
            }
        }
        _ => unreachable!(),
    }
    [x, y]
}

fn fold(
    instructions: &[(char, i32)],
    grid: &mut HashSet<[i32; 2]>,
    update: &mut Vec<([i32; 2], [i32; 2])>,
) {
    for (axis, coord) in instructions {
        update.clear();
        for pos in grid.iter().copied() {
            update.push((pos, mirror(*axis, *coord, pos)));
        }
        for (old, new) in update.iter() {
            grid.remove(old);
            grid.insert(*new);
        }
    }
}

fn main() {
    let mut grid = HashSet::new();

    let mut buffer = String::with_capacity(1024);
    // parse points
    while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if size == 0 || line.is_empty() {
            buffer.clear();
            break;
        }

        let mut s = line.split(',');
        let x: i32 = s.next().expect("x").parse().expect("parsex");
        let y: i32 = s.next().expect("y").parse().expect("parsey");

        grid.insert([x, y]);

        buffer.clear();
    }

    let mut instructions = Vec::new();
    // parse folds
    while let Ok(_size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end().trim_start_matches("fold along ");
        if line.is_empty() {
            break;
        }
        let mut l = line.split("=");
        let axis = l.next().unwrap().chars().next().unwrap();
        let value: i32 = l.next().expect("value").parse().expect("parse val");

        instructions.push((axis, value));

        buffer.clear();
    }

    // part 1
    let mut update = Vec::new();
    fold(&instructions[..1], &mut grid, &mut update);

    println!("p1: {}", grid.len());

    fold(&instructions[1..], &mut grid, &mut update);

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out.dat")
        .unwrap();
    for [x, y] in grid {
        writeln!(file, "{},{}", x, y).unwrap();
    }
}
