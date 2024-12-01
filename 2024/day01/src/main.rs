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

/// return solution for p1, p2
fn solve([a, b]: &Input) -> (u32, i32) {
    let mut total_1 = 0;
    let mut total_2 = 0;
    let mut hb = 0;
    let mut part = 0;
    while a[0] == b[hb] {
        hb += 1;
        part += a[0];
    }
    total_1 += a[0].abs_diff(b[0]);
    total_2 += part;
    for ha in 1..a.len() {
        total_1 += a[ha].abs_diff(b[ha]);
        if a[ha - 1] == a[ha] {
            total_2 += part;
        } else {
            part = 0;
            while b[hb] < a[ha] {
                hb += 1;
            }
            while a[ha] == b[hb] && hb < b.len() {
                part += a[ha];
                hb += 1;
            }
            total_2 += part;
        }
    }
    (total_1, total_2)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input = parse(&input);
    let (p1, p2) = solve(&input);

    println!("p1: {}\np2: {}", p1, p2);
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
    fn test_fused() {
        let input = parse(INPUT);
        let (p1, p2) = solve(&input);
        assert_eq!(p1, 11);
        assert_eq!(p2, 31);
    }
}
