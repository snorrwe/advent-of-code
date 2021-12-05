fn is_valid(numbers: &[i64]) -> bool {
    assert!(numbers.len() >= 3);
    let last = *numbers.last().unwrap();

    let numbers = &numbers[..numbers.len() - 1];

    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers.iter().enumerate() {
            if i != j && a + b == last {
                return true;
            }
        }
    }
    false
}

fn part1(preamble: usize, numbers: &[i64]) -> i64 {
    for i in 0..numbers.len() - preamble {
        if !is_valid(&numbers[i..=i + preamble]) {
            return numbers[i + preamble];
        }
    }
    unreachable!()
}

fn part2(target: i64, numbers: &[i64]) -> i64 {
    for winsize in 2..=numbers.len() {
        // slide
        for startind in 0..numbers.len() - winsize {
            let window = &numbers[startind..startind + winsize];

            if window.iter().cloned().sum::<i64>() == target {
                let min = window.iter().min().unwrap();
                let max = window.iter().max().unwrap();
                return min + max;
            }
        }
    }

    unreachable!()
}

fn parse(inp: &str) -> Vec<i64> {
    inp.lines().filter_map(|x| x.parse().ok()).collect()
}

fn main() {
    let mut input = String::new();

    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let nums = parse(input.as_str());

    let res = part1(25, nums.as_slice());

    println!("{}", res);

    let res = part2(res, nums.as_slice());

    println!("{}", res);
}

#[test]
fn test_p2() {
    let nums = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let target = 127;

    let res = part2(target, &nums);

    assert_eq!(res, 62);
}
