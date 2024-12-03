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
    let re = regex::Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    let mut n = 0;
    for m in re.captures_iter(input) {
        let (_, [a, b]) = m.extract();
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();
        n += a * b;
    }

    n
}

fn part2(input: &Input) -> i32 {
    let re = regex::Regex::new(r#"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))"#).unwrap();

    let mut n = 0;
    let mut enabled = true;
    for m in re.captures_iter(input) {
        if m.get(3).is_some() {
            enabled = true;
            continue;
        }
        if m.get(4).is_some() {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        let a = m.get(1).unwrap();
        let b = m.get(2).unwrap();

        let a: i32 = a.as_str().parse().unwrap();
        let b: i32 = b.as_str().parse().unwrap();
        n += a * b;
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 161);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 48);
    }
}
