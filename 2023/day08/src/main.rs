use rustc_hash::FxHashMap as HashMap;

fn main() {
    let inp = std::fs::read_to_string("input.txt").unwrap();
    let inp = parse(inp.as_str());

    println!("{}", part1(&inp));
    println!("{}", part2(&inp));
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

fn part2(inp: &Input) -> i64 {
    let mut steps = 0;
    let mut current = inp
        .graph
        .keys()
        .copied()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();
    let mut period_len = vec![0; current.len()];
    let mut i = 0;

    let mut done = 0;
    loop {
        steps += 1;
        let c = inp.instructions[i];
        i = (i + 1) % inp.instructions.len();
        let i = match c {
            b'R' => 1,
            b'L' => 0,
            _ => unreachable!(),
        };

        for (j, current) in current.iter_mut().enumerate() {
            *current = inp.graph[*current][i];
            if current.ends_with("Z") {
                if period_len[j] == 0 {
                    period_len[j] = steps;
                    done += 1;
                }
            }
        }

        if done == current.len() {
            println!("{:?}", period_len);
            todo!()
        }
    }
}

fn parse(s: &str) -> Input {
    let mut lines = s.lines();
    let instructions = lines.next().unwrap().as_bytes();

    let mut graph = HashMap::default();

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

#[derive(Debug)]
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

    const INPUT3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
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

    #[test]
    fn test_p21() {
        let inp = parse(INPUT3);

        assert_eq!(part2(&inp), 6);
    }
}
