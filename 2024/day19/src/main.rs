use std::collections::HashMap;

type Input = String;

fn parse(input: String) -> Input {
    input
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn check_pattern_v1<'a>(
    haystack: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if haystack.is_empty() {
        return true;
    }
    if let Some(v) = cache.get(haystack) {
        return *v;
    }
    for p in patterns {
        if let Some(suffix) = haystack.strip_prefix(p) {
            if check_pattern_v1(suffix, patterns, cache) {
                cache.insert(haystack, true);
                return true;
            }
        }
    }
    cache.insert(haystack, false);
    false
}

fn part1(input: &Input) -> i32 {
    let mut patterns: Vec<_> = input.lines().next().unwrap().split(", ").collect();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut cache = Default::default();
    let mut count = 0;
    for towels in input
        .lines()
        .skip(2)
        .map(|x| x.trim_end())
        .filter(|l| !l.is_empty())
    {
        if check_pattern_v1(towels, &patterns, &mut cache) {
            count += 1;
        }
    }
    count
}

fn check_pattern_v2<'a>(
    haystack: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if haystack.is_empty() {
        return 1;
    }
    if let Some(v) = cache.get(haystack) {
        return *v;
    }
    let mut count = 0;
    for p in patterns {
        if let Some(suffix) = haystack.strip_prefix(p) {
            count += check_pattern_v2(suffix, patterns, cache);
        }
    }
    cache.insert(haystack, count);
    count
}

fn part2(input: &Input) -> usize {
    let mut patterns: Vec<_> = input.lines().next().unwrap().split(", ").collect();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut cache = Default::default();
    let mut count = 0;
    for towels in input
        .lines()
        .skip(2)
        .map(|x| x.trim_end())
        .filter(|l| !l.is_empty())
    {
        count += check_pattern_v2(towels, &patterns, &mut cache);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 6);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 16);
    }
}
