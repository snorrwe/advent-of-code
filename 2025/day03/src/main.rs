type Input = Vec<Vec<u8>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().map(|x| x - b'0').collect())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input.as_str());

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn max(line: &[u8]) -> (usize, i64) {
    assert!(!line.is_empty());
    let mut max_idx = 0;
    for i in 1..line.len() {
        if line[max_idx] < line[i] {
            max_idx = i;
        }
    }
    (max_idx, line[max_idx] as i64)
}

fn part1(input: &Input) -> i64 {
    let mut total = 0;
    for line in input {
        // .iter().max() returns the last position, I need the first
        let (i, a) = max(&line[..line.len() - 1]);
        let b = line[i + 1..].iter().max().unwrap();

        let c = (a * 10) + *b as i64;
        total += c;
    }
    total
}

fn part2(input: &Input) -> i64 {
    let mut total = 0;
    for line in input {
        let mut i = 0;
        for c in (0..12).rev() {
            // .iter().max() returns the last position, I need the first
            let (j, a) = max(&line[i..line.len() - c]);
            total += a * 10i64.pow(c as u32);
            i += j + 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 357);
    }

    #[test]
    fn test_p1_simple() {
        let inp = parse("9891");
        let res = part1(&inp);

        assert_eq!(res, 99);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 3121910778619);
    }
}
