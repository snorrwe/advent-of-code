type Input = Vec<Vec<i32>>;

fn parse(input: String) -> Input {
    let mut res = Vec::new();
    for line in input.lines() {
        let nums = line
            .split(|c: char| c.is_whitespace())
            .filter_map(|c| c.parse().ok())
            .collect::<Vec<i32>>();
        res.push(nums);
    }
    res
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = parse(input);

    let (p1, p2) = solve(&mut input);
    println!("{p1} {p2}");
}

fn is_safe(nums: &[i32]) -> bool {
    let Some((lhs, rhs)) = nums.iter().zip(&nums[1..]).next() else {
        return false;
    };
    let d = rhs - lhs;
    let sign = d.signum();
    let d = d.abs();
    if d < 1 || 3 < d {
        return false;
    }

    for (lhs, rhs) in nums[1..].iter().zip(&nums[2..]) {
        let d = rhs - lhs;
        let delta = d.abs();
        if delta < 1 || 3 < delta || d.signum() != sign {
            return false;
        }
    }
    true
}

fn solve(input: &mut Input) -> (i32, i32) {
    let mut n_safe1 = 0;
    let mut n_safe2 = 0;
    'a: for nums in input {
        if is_safe(nums) {
            n_safe1 += 1;
            n_safe2 += 1;
            continue;
        }
        for i in 0..nums.len() {
            let x = nums.remove(i);
            if is_safe(&nums) {
                n_safe2 += 1;
                continue 'a;
            } else {
                nums.insert(i, x);
            }
        }
    }
    (n_safe1, n_safe2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test() {
        let mut inp = parse(INPUT.to_string());
        let (p1, p2) = solve(&mut inp);

        assert_eq!(p1, 2);
        assert_eq!(p2, 4);
    }
}
