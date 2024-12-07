use rayon::iter::{ParallelBridge as _, ParallelIterator};

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

fn check_nums_v1(expected: i64, result: i64, nums: &[i64]) -> bool {
    assert!(!nums.is_empty());
    if nums.len() == 1 {
        return nums[0] * result == expected || nums[0] + result == expected;
    }
    return check_nums_v1(expected, result * nums[0], &nums[1..])
        || check_nums_v1(expected, result + nums[0], &nums[1..]);
}

fn part1(input: &Input) -> i64 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let Some((lhs, rhs)) = line.split_once(':') else {
                return 0;
            };
            let result: i64 = lhs.trim().parse().unwrap();
            let nums: Vec<i64> = rhs
                .split(|c: char| c.is_whitespace())
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.trim().parse().unwrap())
                .collect();

            let n = nums.len();
            assert!(n > 0);

            if check_nums_v1(result, nums[0], &nums[1..]) {
                return result;
            }

            0
        })
        .sum()
}

fn concat(mut a: i64, b: i64) -> i64 {
    let mut c = b;
    while c != 0 {
        c /= 10;
        a *= 10;
    }
    a + b
}

fn check_nums_v2(expected: i64, result: i64, nums: &[i64]) -> bool {
    assert!(!nums.is_empty());
    let con: i64 = concat(result, nums[0]);
    if nums.len() == 1 {
        return nums[0] * result == expected || nums[0] + result == expected || expected == con;
    }
    return check_nums_v2(expected, result * nums[0], &nums[1..])
        || check_nums_v2(expected, result + nums[0], &nums[1..])
        || check_nums_v2(expected, con, &nums[1..]);
}

fn part2(input: &Input) -> i64 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let Some((lhs, rhs)) = line.split_once(':') else {
                return 0;
            };
            let result: i64 = lhs.trim().parse().unwrap();
            let nums: Vec<i64> = rhs
                .split(|c: char| c.is_whitespace())
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.trim().parse().unwrap())
                .collect();
            let n = nums.len();
            assert!(n > 0);
            if check_nums_v2(result, nums[0], &nums[1..]) {
                return result;
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 3749);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 11387);
    }
}
