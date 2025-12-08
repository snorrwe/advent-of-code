use std::{collections::HashMap, fmt::Display, num::ParseIntError, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn dist_sq(self, other: Vec3) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }

    pub fn dist(self, other: Vec3) -> f64 {
        (self.dist_sq(other) as f64).sqrt()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Vec3ParseError {
    #[error("Failed to parse component {0}: {1:?}")]
    ParseIntError(usize, ParseIntError),
    #[error("NotEnoughComponents")]
    NotEnoughComponents,
}

impl FromStr for Vec3 {
    type Err = Vec3ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = [0; 3];

        for (i, x) in s.split(',').enumerate() {
            components[i] = x
                .trim()
                .parse()
                .map_err(|e| Vec3ParseError::ParseIntError(i, e))?;
            if i == 2 {
                return Ok(Vec3 {
                    x: components[0],
                    y: components[1],
                    z: components[2],
                });
            }
        }

        Err(Vec3ParseError::NotEnoughComponents)
    }
}

type Input = Vec<Vec3>;

fn parse(input: &'_ str) -> Input {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Vec3::from_str(l).unwrap())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(1000, &input));
    println!("{}", part2(&input));
}

fn part1(n: usize, input: &Input) -> usize {
    let mut box_circuits = input
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();

    let mut circuits: HashMap<_, _> = box_circuits.iter().map(|(k, v)| (*v, vec![*k])).collect();

    let mut pairs: Vec<[Vec3; 2]> = input.iter().copied().array_combinations().collect();
    pairs.sort_unstable_by_key(|[a, b]| a.dist_sq(*b));

    for [a, b] in pairs.iter().take(n) {
        if box_circuits[a] == box_circuits[b] {
            continue;
        }
        let a_circuit = box_circuits[a];

        let b_circuit_items = std::mem::take(circuits.get_mut(&box_circuits[&b]).unwrap());
        let a_circuit_items = circuits.get_mut(&a_circuit).unwrap();
        for p in b_circuit_items {
            box_circuits.insert(p, a_circuit);
            a_circuit_items.push(p);
        }
    }

    let mut sizes: Vec<_> = circuits.into_values().map(|v| v.len()).collect();

    sizes.sort_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn part2(input: &Input) -> i64 {
    let mut box_circuits = input
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();

    let mut circuits: HashMap<_, _> = box_circuits.iter().map(|(k, v)| (*v, vec![*k])).collect();

    let mut pairs: Vec<[Vec3; 2]> = input.iter().copied().array_combinations().collect();
    pairs.sort_unstable_by_key(|[a, b]| a.dist_sq(*b));

    for [a, b] in pairs.iter() {
        if box_circuits[a] == box_circuits[b] {
            continue;
        }
        let a_circuit = box_circuits[a];

        let b_circuit_items = std::mem::take(circuits.get_mut(&box_circuits[&b]).unwrap());
        let a_circuit_items = circuits.get_mut(&a_circuit).unwrap();

        if b_circuit_items.len() + a_circuit_items.len() == input.len() {
            return a.x * b.x;
        }

        for p in b_circuit_items {
            box_circuits.insert(p, a_circuit);
            a_circuit_items.push(p);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(10, &inp);

        assert_eq!(res, 40);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 25272);
    }
}
