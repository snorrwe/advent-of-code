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

    println!("{}", part1(&input));
    println!("{}", part2(&mut input));
}

fn part1(input: &Input) -> usize {
    input.iter().filter(|x| is_safe(x)).count()
}

fn is_safe(nums: &[i32]) -> bool {
    let Some((last, n)) = nums.iter().zip(&nums[1..]).next() else {
        return false;
    };
    let increasing = last < n;
    let d = last.abs_diff(*n);
    if d < 1 || 3 < d {
        return false;
    }

    for (last, n) in nums[1..].iter().zip(&nums[2..]) {
        let d = last.abs_diff(*n);
        if d < 1 || 3 < d || (increasing && n < last) || (!increasing && last < n) {
            return false;
        }
    }
    true
}

fn part2(input: &mut Input) -> i32 {
    let mut n_safe = 0;
    'a: for nums in input {
        for i in 0..nums.len() {
            let x = nums.remove(i);
            if is_safe(&nums) {
                n_safe += 1;
                continue 'a;
            } else {
                nums.insert(i, x);
            }
        }
    }
    n_safe
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
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_p2() {
        let mut inp = parse(INPUT.to_string());
        let res = part2(&mut inp);

        assert_eq!(res, 4);
    }
}
