use itertools::Itertools;
use utils::IVec2;

type Input = Vec<IVec2>;

fn parse(input: &'_ str) -> Input {
    input
        .lines()
        .filter_map(|l| l.trim().split_once(','))
        .map(|(x, y)| IVec2::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    input
        .iter()
        .array_combinations()
        .map(|[a, b]| {
            let c = *b - *a;
            (c.x.abs() as u64 + 1) * (c.y.abs() as u64 + 1)
        })
        .max()
        .unwrap()
}

fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 50);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 42);
    }
}
