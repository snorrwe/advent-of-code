type Input = [Vec<i32>; 2];

fn parse(inp: &str) -> Input {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in inp.lines() {
        let Some((l, r)) = line.split_once(|c: char| c.is_whitespace()) else {
            continue;
        };
        a.push(l.parse().unwrap());
        b.push(r.trim().parse().unwrap());
    }
    a.sort_unstable();
    b.sort_unstable();
    [a, b]
}

fn p1(inp: &Input) -> u32 {
    let mut total = 0;
    for (a, b) in inp[0].iter().zip(inp[1].iter()) {
        total += a.abs_diff(*b);
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input = parse(&input);

    println!("p1: {}", p1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        const INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

        let input = parse(INPUT);

        assert_eq!(p1(&input), 11);
    }
}
