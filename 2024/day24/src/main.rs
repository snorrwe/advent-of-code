use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Op {
    Xor,
    And,
    Or,
}

impl Op {
    pub fn execute(self, lhs: u8, rhs: u8) -> u8 {
        match self {
            Op::Xor => (lhs ^ rhs) & 1,
            Op::And => lhs & rhs,
            Op::Or => lhs | rhs,
        }
    }
}

struct Input<'a> {
    initial: HashMap<&'a str, u8>,
    dependencies: HashMap<&'a str, (&'a str, &'a str, Op)>,
}

fn parse(input: &str) -> Input {
    let mut initial: HashMap<_, _> = Default::default();
    let mut lines = input.lines();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        let (a, b) = line.split_once(": ").unwrap();
        initial.insert(a, b.parse().unwrap());
    }
    let mut dependencies: HashMap<&str, (&str, &str, Op)> = Default::default();
    let re = Regex::new(r"(\w+) ([A-Z]+) (\w+) -> (\w+)").unwrap();
    for line in lines {
        let Some(m) = re.captures(line) else {
            break;
        };

        let (_, [lhs, op, rhs, res]) = m.extract();
        let op = match op {
            "AND" => Op::And,
            "XOR" => Op::Xor,
            "OR" => Op::Or,
            _ => unreachable!(),
        };

        dependencies.insert(res, (lhs, rhs, op));
    }
    Input {
        initial,
        dependencies,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(input));
}

fn resolve(k: &str, input: &Input) -> u8 {
    if let Some(v) = input.initial.get(k) {
        return *v;
    }
    let (lhs, rhs, op) = input.dependencies[k];
    let lhs = resolve(lhs, input);
    let rhs = resolve(rhs, input);

    op.execute(lhs, rhs)
}

fn part1(input: &Input) -> u64 {
    number_with_prefix('z', input)
}

fn number_with_prefix(prefix: char, input: &Input) -> u64 {
    let mut values = Vec::new();
    for k in input
        .dependencies
        .keys()
        .filter(|k| k.starts_with(prefix))
        .copied()
    {
        values.push((k, resolve(k, input)));
    }
    let mut res = 0;
    for (k, v) in values {
        let q: u64 = k.trim_start_matches(prefix).parse().unwrap();
        res |= (v as u64) << q;
    }
    res
}

fn resolve_deps(k: &str, input: &Input, out: &mut HashMap<String, u64>) {
    *out.entry(k.to_string()).or_default() += 1;

    if let Some((a, b, _)) = input.dependencies.get(k) {
        resolve_deps(*a, input, out);
        resolve_deps(*b, input, out);
    }
}

fn remove_deps(k: &str, input: &mut Input) {
    if let Some((a, b, _)) = input.dependencies.remove(k) {
        remove_deps(a, input);
        remove_deps(b, input);
    }
}

fn emit_connections(k: &str, input: &Input, color: &str) {
    if let Some((a, b, op)) = input.dependencies.get(k) {
        emit_connections(a, input, color);
        emit_connections(b, input, color);

        println!("{k} -> {a} [color={color}] [label=\"{op:?} {b}\"];");
        println!("{k} -> {b} [color={color}] [label=\"{op:?} {a}\"];");
    }
}

fn part2(mut input: Input) -> u64 {
    let x = number_with_prefix('x', &input);
    let y = number_with_prefix('y', &input);
    let z = number_with_prefix('z', &input);

    let s = (x + y) ^ z;
    // incorrect bits are 1, correct bits are 0
    println!("{} 1 bits are incorrect {s:0b}", s.count_ones());

    println!("digraph {{");
    for i in 0..64 {
        let k = format!("z{:02}", i);
        let color = if s & (1 << i) == 0 { "blue" } else { "red" };
        emit_connections(&k, &input, color);
    }
    println!("}}");

    if x + y == z {
        todo!("win")
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(inp);

        assert_eq!(res, 42);
    }
}
