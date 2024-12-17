use regex::Regex;

#[derive(Debug)]
struct Input {
    a: i64,
    b: i64,
    c: i64,

    program: Vec<i64>,
}

fn parse(input: &str) -> Input {
    let re = Regex::new(
        r#"(?ms)^Register A: (\d+)$
^Register B: (\d+)
^Register C: (\d+)
^$
^Program: (.*)$"#,
    )
    .unwrap();

    let m = re.captures(input).unwrap();

    let (_, [a, b, c, prog]) = m.extract();

    Input {
        a: a.parse().unwrap(),
        b: b.parse().unwrap(),
        c: c.parse().unwrap(),
        program: prog.trim().split(',').map(|c| c.parse().unwrap()).collect(),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> String {
    let mut a = input.a;
    let mut b = input.b;
    let mut c = input.c;
    let program = input.program.as_slice();

    let mut ip = 0;
    let mut output = Vec::new();

    loop {
        if program.len() <= ip + 1 {
            break;
        }
        let opcode = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            0 | 1 | 2 | 3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => {
                unreachable!()
            }
        };

        match opcode {
            0 => {
                a = a / 2i64.pow(combo as u32);
            }
            1 => {
                b = b ^ literal;
            }
            2 => {
                b = combo & 0x7;
            }
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push(combo & 0x7);
            }
            6 => {
                b = a / 2i64.pow(combo as u32);
            }
            7 => {
                c = a / 2i64.pow(combo as u32);
            }
            _ => {
                unreachable!()
            }
        }
        ip += 2;
    }

    output
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn run(input: &Input, initial_a: i64) -> Vec<i64> {
    let program = &input.program;
    let mut output = Vec::new();
    let mut a = initial_a;
    let mut b = input.b;
    let mut c = input.c;
    let mut ip = 0;

    loop {
        if program.len() <= ip + 1 {
            break;
        }
        let opcode = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            0 | 1 | 2 | 3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => {
                unreachable!()
            }
        };

        match opcode {
            0 => {
                a = a / 2i64.pow(combo as u32);
            }
            1 => {
                b = b ^ literal;
            }
            2 => {
                b = combo & 0x7;
            }
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push(combo & 0x7);
            }
            6 => {
                b = a / 2i64.pow(combo as u32);
            }
            7 => {
                c = a / 2i64.pow(combo as u32);
            }
            _ => {
                unreachable!()
            }
        }
        ip += 2;
    }
    output
}

/// the numbers in the output have a repeating pattern,
/// the ith number repeats every 8**i iterations
/// use this fact to skip large portions of the search space
fn part2(input: &Input) -> i64 {
    let program = input.program.as_slice();
    assert!(program.len() >= 2);

    let search_start = 8i64.pow(program.len() as u32 - 1);
    let search_end = 8i64.pow(program.len() as u32);

    let mut x = search_start;
    while x < search_end {
        let output = run(input, x);
        if output == program {
            return x;
        }
        let mut skip = 0;
        for (i, (p, o)) in program.iter().zip(output.iter()).enumerate().rev() {
            if p != o {
                skip = 8i64.pow(i as u32);
                break;
            }
        }
        x += skip.max(1);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

    #[test]
    fn test_p1_simple_v1() {
        let inp = parse(
            r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
        "#,
        );
        let res = part1(&inp);

        assert_eq!(res, "0,1,2");
    }
    #[test]
    fn test_p1_simple_v2() {
        let input = Input {
            a: 2024,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let res = part1(&input);

        assert_eq!(res, "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_p2() {
        let mut inp = parse(
            r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#,
        );
        inp.a = 117440;
        let res = part2(&inp);

        assert_eq!(res, 117440);
    }
}
