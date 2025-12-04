type Input<'a> = &'a str;

fn parse(input: &'_ str) -> Input<'_> {
    input
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i32 {
    todo!()
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 42);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
