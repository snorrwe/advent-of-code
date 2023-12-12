fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("part1 {}", part1(input.as_str()));
    println!("part2 {}", part2(input.as_str()));
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let Some((segments, rules)) = line.split_once(' ') else {
            continue;
        };
        let rules: Vec<usize> = rules.split(',').map(|x| x.parse().unwrap()).collect();
        let segments = segments.to_owned();
        let res = fit(segments.as_bytes().to_vec(), rules);
        sum += res;
    }
    sum
}

fn segment_begins_with_rule(segment: &[u8], rule: usize, is_last: bool) -> bool {
    if segment.len() < rule {
        return false;
    }
    for c in &segment[..rule] {
        if c == &b'.' {
            return false;
        }
    }
    if is_last {
        if segment.len() > rule {
            for c in &segment[rule..] {
                if c == &b'#' {
                    return false;
                }
            }
        }
    } else {
        // springs can't be adjacent
        if segment.len() == rule || segment[rule] == b'#' {
            return false;
        }
    }
    true
}

#[memoize::memoize]
fn fit(segment: Vec<u8>, rules: Vec<usize>) -> usize {
    if rules.is_empty() {
        return 0;
    }
    let mut segment = &segment[..];
    while let Some(s) = segment.strip_prefix(&[b'.']) {
        segment = s;
    }
    let mut res = 0;
    if segment_begins_with_rule(segment, rules[0], rules.len() == 1) {
        if rules.len() > 1 {
            if segment.len() > rules[0] + 1 {
                res = fit(segment[rules[0] + 1..].to_vec(), rules[1..].to_vec());
            }
            // else res = 0
        } else {
            res = 1;
        }
    }
    // total size + padding
    let space_required = rules.iter().copied().sum::<usize>() + rules.len() - 1;
    if space_required <= segment.len() && segment[0] == b'?' {
        res += fit(segment[1..].to_vec(), rules);
    }
    res
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let Some((segments, rules)) = line.split_once(' ') else {
                return 0;
            };
            let rules = [rules; 5].join(",");
            let rules: Vec<usize> = rules.split(',').map(|x| x.parse().unwrap()).collect();
            let segments = [segments; 5].join("?");
            fit(segments.as_bytes().to_vec(), rules)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_p1() {
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part1("?###???????? 3,2,1"), 10);

        let res = part1(INPUT);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part2("???.### 1,1,3"), 1);
        assert_eq!(part2("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part2(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(part2("?###???????? 3,2,1"), 506250);

        let res = part2(INPUT);
        assert_eq!(res, 525152);
    }
}
