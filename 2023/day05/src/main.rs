use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mapping = parse(&input);
    println!("{}", part1(&mapping));
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<(i64, i64, i64)>,
}

impl Mapping {
    fn map(&self, i: i64) -> i64 {
        self.maps
            .iter()
            .copied()
            .find_map(|(src, dst, len)| (src <= i && i < src + len).then_some(dst + i - src))
            .unwrap_or(i)
    }
}

#[derive(Debug)]
struct Input<'a> {
    mappings: HashMap<(&'a str, &'a str), Mapping>,
    graph: HashMap<&'a str, &'a str>,
    seeds: Vec<i64>,
}

fn part1(input: &Input) -> i64 {
    let mut min = std::i64::MAX;
    for mut src in input.seeds.iter().copied() {
        let mut current = "seed";
        while current != "location" {
            let to = input.graph[current];
            src = input.mappings[&(current, to)].map(src);
            current = to;
        }
        if min > src {
            min = src
        }
    }
    min
}

fn parse(inp: &str) -> Input {
    let seeds = regex::Regex::new(r"(\d+)").unwrap();
    let mut lines = inp.lines();

    let seeds = lines
        .next()
        .map(|l| {
            assert!(l.starts_with("seeds:"));
            seeds
                .captures_iter(l)
                .map(|num| {
                    let (_, [num]) = num.extract();
                    num.parse::<i64>().unwrap()
                })
                .collect()
        })
        .unwrap_or_default();

    let mapre = regex::Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let numre = regex::Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();

    let mut mappings = HashMap::new();
    let mut graph = HashMap::new();

    while let Some(l) = lines.next() {
        let Some(mapping) = mapre.captures(l) else {
            continue;
        };
        let (_, [from, to]) = mapping.extract();
        let mut map = Vec::new();

        while let Some(l) = lines.next() {
            let Some(mappings) = numre.captures(l) else {
                break;
            };
            let (_, [dst, src, len]) = mappings.extract();
            let dst: i64 = dst.parse().unwrap();
            let src: i64 = src.parse().unwrap();
            let len: i64 = len.parse().unwrap();

            map.push((src, dst, len))
        }

        assert!(!graph.contains_key(from));
        graph.insert(from, to);
        mappings.insert((from, to), Mapping { maps: map });
    }

    Input {
        seeds,
        mappings,
        graph,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_p1() {
        let map = parse(INPUT);
        let res = part1(&map);
        assert_eq!(res, 35);
    }
}
