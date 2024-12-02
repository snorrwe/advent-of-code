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

fn part1(input: &Input) -> i32 {
    let mut n_safe = 0;
    'a: for line in input.lines() {
        let mut it = line.split(|c: char| c.is_whitespace());
        let Some(n) = it.next() else {
            continue;
        };
        let mut last: i32 = n.parse().unwrap();
        let Some(n) = it.next() else {
            continue;
        };
        let n = n.parse().unwrap();
        let d = last.abs_diff(n);
        if d < 1 || 3 < d {
            continue 'a;
        }
        let increasing = last < n;
        last = n;

        for n in it {
            let n = n.parse().unwrap();
            let d = last.abs_diff(n);
            if d < 1 || 3 < d || (increasing && n < last) || (!increasing && last < n) {
                continue 'a;
            }
            last = n;
        }
        n_safe += 1;
    }
    n_safe
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
