use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
struct Input {
    fresh: Vec<[u64; 2]>,
    available: HashSet<u64>,
}

fn parse(input: &'_ str) -> Input {
    let mut result = Input::default();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let Some((from, to)) = line.trim().split_once('-') else {
            break;
        };
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        assert!(from <= to);

        result.fresh.push([from, to]);
    }
    result.fresh.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
    for line in lines {
        if let Ok(i) = line.parse() {
            result.available.insert(i);
        }
    }

    result
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> usize {
    input
        .available
        .iter()
        .copied()
        .filter(|ingredient| {
            let mut i = 0;
            while i < input.fresh.len() && input.fresh[i][1] < *ingredient {
                i += 1;
            }
            for r in &input.fresh[i..] {
                if r[0] <= *ingredient && *ingredient <= r[1] {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 3);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
