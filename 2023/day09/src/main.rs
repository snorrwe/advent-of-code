fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        let sequence: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();

        let delta = part1_recursive(&sequence);
        res += *sequence.last().unwrap() + delta;
    }
    res
}

fn part2(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        let sequence: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        let delta = part2_recursive(&sequence);
        res += sequence[0] - delta;
    }
    res
}

fn part1_recursive(sequence: &[i64]) -> i64 {
    let mut zeros = 0;
    let sequence: Vec<i64> = sequence
        .iter()
        .copied()
        .zip(sequence.iter().copied().skip(1))
        .map(|(a, b)| b - a)
        .inspect(|x| {
            if *x == 0 {
                zeros += 1;
            }
        })
        .collect();

    if zeros == sequence.len() {
        return 0;
    }

    let delta = part1_recursive(&sequence);

    *sequence.last().unwrap() + delta
}

fn part2_recursive(sequence: &[i64]) -> i64 {
    let mut zeros = 0;
    let sequence: Vec<i64> = sequence
        .iter()
        .copied()
        .zip(sequence.iter().copied().skip(1))
        .map(|(a, b)| b - a)
        .inspect(|x| {
            if *x == 0 {
                zeros += 1;
            }
        })
        .collect();

    if zeros == sequence.len() {
        return 0;
    }

    let delta = part2_recursive(&sequence);

    sequence[0] - delta
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);

        assert_eq!(res, 114);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);

        assert_eq!(res, 2);
    }
}
