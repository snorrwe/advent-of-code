use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

type Point = [i32; 2];

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    let points = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(',')
                .map(|n: &str| n.trim().parse().unwrap())
                .collect()
        })
        .map(|x: Vec<_>| [x[0], x[1]])
        .collect();

    let result = part1(&points);
    println!("Part1: {:?}", result);

    Ok(())
}

fn part1(points: &Vec<Point>) -> Option<usize> {
    let [x, y] = points.iter().next().unwrap();
    let edges = points.iter().fold([*x, *y, *x, *y], |mut dim, point| {
        for i in 0..2 {
            let p = point[i];
            if p < dim[i] {
                dim[i] = p;
            } else if p > dim[i + 2] {
                dim[i + 2] = p;
            }
        }
        dim
    });

    let mut map: HashMap<Point, usize> = HashMap::new();
    let mut infinites: HashSet<Point> = HashSet::new();
    let [x1, y1, x2, y2] = edges;
    let offset = 1;
    let edge_x = [x1 - offset, x2 + offset];
    let edge_y = [y1 - offset, y2 + offset];
    for x in edge_x[0]..=edge_x[1] {
        for y in edge_y[0]..=edge_y[1] {
            let pos = [x, y];
            let mut distances = points
                .iter()
                .map(|point| (point, manhattan(point, &pos)))
                .collect::<Vec<(&Point, i32)>>();
            distances.sort_unstable_by(|a, b| a.1.cmp(&b.1));

            if distances[0].1 != distances[1].1 {
                let closest = distances[0].0;
                if edge_x.contains(&x) || edge_y.contains(&y) {
                    infinites.insert(*closest);
                }
                *map.entry(*closest).or_insert(0) += 1;
            }
        }
    }

    map.iter()
        .filter(|(pos, _)| !infinites.contains(*pos))
        .map(|(_, v)| *v)
        .max()
}

fn manhattan(x: &Point, y: &Point) -> i32 {
    x.iter().zip(y.iter()).map(|(a, b)| (a - b).abs()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![[1, 1], [1, 6], [8, 3], [3, 4], [5, 5], [8, 9]];

        let result = part1(&input).expect("Failed to find the answer");

        assert_eq!(result, 17);
    }
}

