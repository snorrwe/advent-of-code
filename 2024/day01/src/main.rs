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
    [a, b]
}

fn p1([mut a, mut b]: Input) -> u32 {
    a.sort_unstable();
    b.sort_unstable();
    let mut total = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        total += a.abs_diff(*b);
    }
    total
}

fn p2([a, b]: &Input) -> usize {
    let mut total = 0;
    for i in a {
        total += *i as usize * b.iter().filter(|x| *x == i).count();
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input = parse(&input);

    println!("p1: {}", p1(input.clone()));
    println!("p2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn test_p1() {
        let input = parse(INPUT);
        assert_eq!(p1(input), 11);
    }

    #[test]
    fn test_p2() {
        let input = parse(INPUT);
        assert_eq!(p2(&input), 31);
    }
}
