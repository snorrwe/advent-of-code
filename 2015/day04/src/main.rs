fn parse(input: &str) -> &str {
    input.trim()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut n = 0;
    loop {
        let key = format!("{input}{n}");
        let hash = md5::compute(key.as_bytes());
        if format!("{hash:?}").starts_with("00000") {
            return n;
        }
        n += 1;
    }
}

fn part2(input: &str) -> i32 {
    let mut n = 0;
    loop {
        let key = format!("{input}{n}");
        let hash = md5::compute(key.as_bytes());
        if format!("{hash:?}").starts_with("000000") {
            return n;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"abcdef"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 609043);

        let res = part1("pqrstuv");

        assert_eq!(res, 1048970);
    }
}
