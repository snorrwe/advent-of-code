use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() {
    let file = File::open("input.txt").expect("Failed to open input file!");
    let buf_reader = BufReader::new(file);
    let claims: Vec<Claim> = buf_reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|line| Claim::parse(line.as_str()))
        .collect();

    let map = build_map(&claims);
    let part1 = part1(&map);
    println!("Day3 part1: {}", part1);
    let part2 = part2(&claims, &map);
    println!("Day3 part2: {}", part2);
}

fn part1(map: &HashMap<[i32; 2], usize>) -> usize {
    map.values().filter(|v| **v > 1).count()
}

fn part2<'a>(claims: &'a Vec<Claim>, map: &HashMap<[i32; 2], usize>) -> &'a str {
    claims
        .iter()
        .find_map(|claim| {
            for i in 0..claim.dim[0] {
                for j in 0..claim.dim[1] {
                    if map[&[claim.pos[0] + i, claim.pos[1] + j]] > 1 {
                        return None;
                    }
                }
            }
            Some(claim.id.as_str())
        })
        .expect("Failed to find the answer")
}

fn build_map(claims: &Vec<Claim>) -> HashMap<[i32; 2], usize> {
    let mut map = HashMap::with_capacity(350000);
    claims.iter().for_each(|claim| {
        for i in 0..claim.dim[0] {
            for j in 0..claim.dim[1] {
                let [x, y] = claim.pos;
                let pos = [x + i, y + j];
                if let Some(x) = map.get_mut(&pos) {
                    *x += 1;
                } else {
                    map.insert(pos, 1);
                }
            }
        }
    });
    map
}

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    id: String,
    pos: [i32; 2],
    dim: [i32; 2],
}

impl Claim {
    pub fn new(id: String, left: i32, top: i32, width: i32, height: i32) -> Claim {
        Claim {
            id: id,
            pos: [left, top],
            dim: [width, height],
        }
    }

    pub fn parse(line: &str) -> Claim {
        let mut parts = line.split(' ');
        let id = parts.next().expect("Failed to read id");
        parts.next().expect("Missing '@'");
        let pos = parts
            .next()
            .expect("Failed to read position")
            .split(',')
            .filter_map(|s| s.split(':').next())
            .map(|x| x.parse::<i32>().expect("Expected position to be a number"))
            .collect::<Vec<i32>>();
        assert_eq!(pos.len(), 2);
        let dim = parts
            .next()
            .expect("Missing dimensions")
            .split('x')
            .map(|s| s.parse::<i32>().expect("Expected dimension to be a number"))
            .collect::<Vec<i32>>();

        Claim::new(id.to_string(), pos[0], pos[1], dim[0], dim[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let result = Claim::parse("#123 @ 3,2: 5x4");

        let expected = Claim::new("#123".to_string(), 3, 2, 5, 4);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let claims = ["#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2", "#1 @ 1,3: 4x4"]
            .iter()
            .map(|s| Claim::parse(s))
            .collect();

        let map = build_map(&claims);
        let result = part1(&map);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2() {
        let claims = ["#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2", "#1 @ 1,3: 4x4"]
            .iter()
            .map(|s| Claim::parse(s))
            .collect();

        let map = build_map(&claims);
        let result = part2(&claims, &map);

        assert_eq!(result, "#3");
    }
}
