use std::collections::HashMap;

fn main() {
    let inp = std::fs::read_to_string("input.txt").unwrap();
    let inp = parse(inp.as_str());

    println!("{}", part1(&inp));
}

fn part1(inp: &Input) -> i64 {
    let mut steps = 0;
    let mut current = "AAA";
    let mut i = 0;

    loop {
        steps += 1;
        let c = inp.instructions[i];
        i = (i + 1) % inp.instructions.len();
        let i = match c {
            b'R' => 1,
            b'L' => 0,
            _ => unreachable!(),
        };

        current = inp.graph[current][i];
        if current == "ZZZ" {
            return steps;
        }
    }
}

fn parse(s: &str) -> Input {
    let mut lines = s.lines();
    let instructions = lines.next().unwrap().as_bytes();

    let mut graph = HashMap::new();

    let instr_re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for line in lines {
        let Some(cap) = instr_re.captures(line) else {
            continue;
        };

        let (_, [node, left, right]) = cap.extract();

        graph.insert(node, [left, right]);
    }

    Input {
        instructions,
        graph,
    }
}

struct Input<'a> {
    instructions: &'a [u8],
    graph: HashMap<&'a str, [&'a str; 2]>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn test_p11() {
        let inp = parse(INPUT1);

        assert_eq!(part1(&inp), 2);
    }

    #[test]
    fn test_p12() {
        let inp = parse(INPUT2);

        assert_eq!(part1(&inp), 6);
    }
}
