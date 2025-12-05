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
    println!("{}", part2(input.fresh));
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

fn part2(input: Vec<[u64; 2]>) -> u64 {
    let mut a = input;
    let mut b = Vec::with_capacity(a.len());
    a.sort_unstable();

    // merge overlapping ranges
    loop {
        let mut merged = false;
        b.clear();
        for (i, current) in a.iter().copied().enumerate() {
            if let Some(next) = a.get(i + 1)
                && next[0] <= current[1]
            {
                // merge 1 at a time to get around some edge cases
                b.push([current[0], current[1].max(next[1])]);
                merged = true;
                b.extend_from_slice(&a[i + 2..]);
                break;
            } else {
                b.push(current);
            }
        }
        if !merged {
            break;
        }
        std::mem::swap(&mut a, &mut b);
    }

    a.into_iter().map(|[a, b]| b - a + 1).sum()
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
        let res = part2(inp.fresh);

        assert_eq!(res, 14);
    }
}
