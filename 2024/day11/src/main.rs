type Cache = rustc_hash::FxHashMap<(u32, u64), usize>;

type Input = Vec<u64>;

fn parse(input: String) -> Input {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn count_digits(mut a: u64) -> u32 {
    if a == 0 {
        return 1;
    }
    let mut c = 0;
    while a != 0 {
        a /= 10;
        c += 1
    }
    c
}

fn reduce(depth: u32, n: u64, cache: &mut Cache) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(x) = cache.get(&(depth, n)) {
        return *x;
    }

    let dig = count_digits(n);
    let y = match n {
        0 => reduce(depth - 1, 1, cache),
        _ if dig % 2 == 0 => {
            let d = 10u64.pow(dig / 2);

            reduce(depth - 1, n / d, cache) + reduce(depth - 1, n % d, cache)
        }
        _ => reduce(depth - 1, n * 2024, cache),
    };

    cache.insert((depth, n), y);
    y
}

fn run(n: u32, a: &[u64]) -> usize {
    let mut total = 0;
    let mut cache = Default::default();
    for x in &a[..] {
        total += reduce(n, *x, &mut cache);
    }
    total
}

fn part1(input: &Input) -> usize {
    run(25, &input)
}

fn part2(input: &Input) -> usize {
    run(75, &input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"125 17"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 55312);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(8), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(1003), 4);
        assert_eq!(count_digits(1000), 4);
        assert_eq!(count_digits(9999), 4);
    }
}
