type Input = Vec<Machine>;

#[derive(Debug, Default, Clone, Copy)]
struct Machine {
    button_a: [i64; 2],
    button_b: [i64; 2],
    prize: [i64; 2],
}

fn parse(input: String) -> Input {
    let machine_re = regex::Regex::new(
        r#"(?ms)Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)
^$"#,
    )
    .unwrap();

    machine_re
        .captures_iter(&input)
        .map(|m| Machine {
            button_a: [
                m.get(1).unwrap().as_str().parse().unwrap(),
                m.get(2).unwrap().as_str().parse().unwrap(),
            ],
            button_b: [
                m.get(3).unwrap().as_str().parse().unwrap(),
                m.get(4).unwrap().as_str().parse().unwrap(),
            ],
            prize: [
                m.get(5).unwrap().as_str().parse().unwrap(),
                m.get(6).unwrap().as_str().parse().unwrap(),
            ],
        })
        .collect()
}

fn cramer(button_a: [i64; 2], button_b: [i64; 2], prize: [i64; 2]) -> [i64; 2] {
    let denom = button_a[0] * button_b[1] - button_a[1] * button_b[0];
    if denom == 0 {
        return [-1; 2];
    }
    let deta = prize[0] * button_b[1] - prize[1] * button_b[0];
    let detb = button_a[0] * prize[1] - button_a[1] * prize[0];

    [deta / denom, detb / denom]
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i64 {
    let mut total = 0;
    for m in input {
        let res = cramer(m.button_a, m.button_b, m.prize);
        if res[0] > 100
            || res[1] > 100
            || res[0] < 0
            || res[1] < 0
            || res[0] * m.button_a[0] + res[1] * m.button_b[0] != m.prize[0]
            || res[0] * m.button_a[1] + res[1] * m.button_b[1] != m.prize[1]
        {
            continue;
        }
        total += res[0] * 3 + res[1];
    }
    total
}

fn part2(input: &Input) -> i64 {
    let mut total = 0;
    for m in input {
        let res = cramer(
            m.button_a,
            m.button_b,
            [m.prize[0] + 10000000000000, m.prize[1] + 10000000000000],
        );
        if res[0] < 0
            || res[1] < 0
            || res[0] * m.button_a[0] + res[1] * m.button_b[0] != m.prize[0] + 10000000000000
            || res[0] * m.button_a[1] + res[1] * m.button_b[1] != m.prize[1] + 10000000000000
        {
            continue;
        }
        total += res[0] * 3 + res[1];
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 480);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);
        dbg!(res);

        todo!()
    }
}
