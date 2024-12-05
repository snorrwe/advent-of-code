use std::{cmp::Ordering, collections::HashSet};

type Precedence = HashSet<(i32, i32)>;

struct Input {
    /// values come before the key
    precedence: Precedence,
    pages: Vec<Vec<i32>>,
}

fn parse(input: String) -> Input {
    let mut res = Input {
        precedence: Default::default(),
        pages: Default::default(),
    };

    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }

        let (a, b) = l.trim().split_once('|').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();

        res.precedence.insert((a, b));
    }
    while let Some(l) = lines.next() {
        if l.is_empty() {
            continue;
        }
        let mut r: Vec<_> = Default::default();
        for n in l.split(',') {
            r.push(n.parse().unwrap());
        }
        res.pages.push(r);
    }

    res
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&mut input));
}

fn part1(input: &Input) -> i32 {
    let mut total = 0;

    'line: for line in input.pages.iter() {
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                if input.precedence.contains(&(line[j], line[i])) {
                    continue 'line;
                }
            }
        }
        let mid = line[line.len() / 2];
        total += mid;
    }

    total
}

fn sort_line(line: &mut [i32], precedence: &Precedence) {
    line.sort_by(|a, b| {
        if precedence.contains(&(*a, *b)) {
            return Ordering::Less;
        }
        if precedence.contains(&(*b, *a)) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });
}

fn part2(input: &mut Input) -> i32 {
    let mut total = 0;

    for line in input.pages.iter_mut() {
        'check: for i in 0..line.len() {
            for j in i..line.len() {
                if input.precedence.contains(&(line[j], line[i])) {
                    sort_line(line, &input.precedence);
                    let mid = line[line.len() / 2];
                    total += mid;
                    break 'check;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 143);
    }

    #[test]
    fn test_p2() {
        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp);

        assert_eq!(res, 123);
    }
}
