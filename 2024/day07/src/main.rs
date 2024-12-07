use itertools::Itertools as _;
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

            's: for ops in vec![[b'+', b'*']; nums.len() - 1]
                .into_iter()
                .flatten()
                .combinations(nums.len() - 1)
            {
                let mut tmp = nums[0];
                for (op, x) in ops.into_iter().zip(nums[1..].iter().copied()) {
                    match op {
                        b'*' => {
                            tmp *= x;
                        }
                        b'+' => {
                            tmp += x;
                        }
                        _ => {}
                    }
                    if tmp > result {
                        continue 's;
                    }
                }
                if result == tmp {
                    return result;
                }
            }
            0
        })
        .sum()
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
            's: for ops in vec![[b'+', b'*', b'|']; nums.len() - 1]
                .into_iter()
                .flatten()
                .combinations(nums.len() - 1)
            {
                let mut tmp = nums[0];
                for (op, x) in ops.into_iter().zip(nums[1..].iter().copied()) {
                    match op {
                        b'*' => {
                            tmp *= x;
                        }
                        b'+' => {
                            tmp += x;
                        }
                        b'|' => {
                            tmp = format!("{tmp}{x}").parse().unwrap();
                        }
                        _ => {}
                    }
                    if tmp > result {
                        continue 's;
                    }
                }
                if result == tmp {
                    return result;
                }
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
