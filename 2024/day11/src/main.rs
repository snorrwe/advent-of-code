type Input = Vec<u64>;

fn parse(input: String) -> Input {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn count_digits(mut a: u64) -> u32 {
    if a == 0 {
        return 1;
    }
    let mut c = 0;
    while a != 0 {
        a /= 10;
        c += 1
    }
    c
}

fn run(n: i32, a: &mut Vec<u64>, b: &mut Vec<u64>) {
    for _ in 0..n {
        b.clear();
        for x in &a[..] {
            let dig = count_digits(*x);
            match x {
                0 => {
                    b.push(1);
                }
                _ if dig % 2 == 0 => {
                    let d = 10u64.pow(dig / 2);
                    b.push(x / d);
                    b.push(x % d);
                }
                _ => {
                    b.push(x * 2024);
                }
            }
        }
        std::mem::swap(a, b);
    }
}

fn part1(input: &Input) -> usize {
    let mut a = input.clone();
    let mut b = Vec::with_capacity(input.len());

    run(25, &mut a, &mut b);
    a.len().max(b.len())
}

fn part2(input: &Input) -> usize {
    let mut a = input.clone();
    let mut b = Vec::with_capacity(input.len());

    run(75, &mut a, &mut b);
    a.len().max(b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"125 17"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 55312);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 42);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(8), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(1003), 4);
        assert_eq!(count_digits(1000), 4);
        assert_eq!(count_digits(9999), 4);
    }
}
