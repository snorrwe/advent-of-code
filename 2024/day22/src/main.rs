use std::collections::HashMap;

type Input = Vec<i64>;

fn parse(input: String) -> Input {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn next(mut n: i64) -> i64 {
    let m = n * 64;
    n = (n ^ m) % 16777216;
    let m = n / 32;
    n = (n ^ m) % 16777216;
    let m = n * 2048;
    (n ^ m) % 16777216
}

fn part1(input: &Input) -> i64 {
    let mut total = 0;
    for num in input {
        let mut num = *num;
        for _ in 0..2000 {
            num = next(num);
        }
        total += num;
    }
    total
}

fn part2(input: &Input) -> i64 {
    let mut sequence_wins: HashMap<[i64; 4], i64> = HashMap::new();
    let mut tmp_costs = HashMap::new();
    for num in input {
        let mut sequence = [0; 4];
        let mut num = *num;
        let mut last = 0;
        for i in 0..4 {
            num = next(num);
            let p = num % 10;
            let delta = p - last;
            sequence[i] = delta;
            last = p;
        }
        let p = num % 10;
        tmp_costs.insert(sequence, p);

        for _ in 4..2000 {
            num = next(num);
            let p = num % 10;
            let delta = p - last;
            sequence.rotate_left(1);
            sequence[3] = delta;
            if !tmp_costs.contains_key(&sequence) {
                tmp_costs.insert(sequence, p);
            }
            last = p;
        }

        for (k, v) in tmp_costs.drain() {
            *sequence_wins.entry(k).or_default() += v;
        }
    }
    sequence_wins.values().copied().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1
10
100
2024
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 37327623);
    }

    #[test]
    fn test_p2() {
        let inp = parse(
            r#"1
2
3
2024
"#
            .to_string(),
        );
        let res = part2(&inp);

        assert_eq!(res, 23);
    }

    #[test]
    fn test_secrets() {
        let mut num = 123;

        let nums: Vec<_> = (0..10)
            .map(|_| {
                num = next(num);
                num
            })
            .collect();

        assert_eq!(
            nums,
            &[
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }
}
