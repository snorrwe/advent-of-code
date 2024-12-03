fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (p1, p2) = solve(&input);
    println!("{} {}", p1, p2);
}

fn solve(input: &str) -> (i32, i32) {
    let re = regex::Regex::new(r#"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))"#).unwrap();

    let mut n1 = 0;
    let mut n2 = 0;
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
        let a = m.get(1).unwrap();
        let b = m.get(2).unwrap();
        let a: i32 = a.as_str().parse().unwrap();
        let b: i32 = b.as_str().parse().unwrap();
        let n = a * b;
        n1 += n;
        if enabled {
            n2 += n;
        }
    }

    (n1, n2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_solve() {
        let (p1, p2) = solve(&INPUT);

        assert_eq!(p1, 161);
        assert_eq!(p2, 48);
    }
}
