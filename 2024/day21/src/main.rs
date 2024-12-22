use utils::IVec2;

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

fn resolve_numeric(c: u8) -> IVec2 {
    match c {
        b'A' => IVec2::new(2, 3),
        b'0' => IVec2::new(1, 3),
        b'1' => IVec2::new(0, 2),
        b'2' => IVec2::new(1, 2),
        b'3' => IVec2::new(2, 2),
        b'4' => IVec2::new(0, 1),
        b'5' => IVec2::new(1, 1),
        b'6' => IVec2::new(2, 1),
        b'7' => IVec2::new(0, 0),
        b'8' => IVec2::new(1, 0),
        b'9' => IVec2::new(2, 0),
        _ => {
            unreachable!()
        }
    }
}

fn resolve_directional(c: u8) -> IVec2 {
    match c {
        b'^' => IVec2::new(1, 0),
        b'A' => IVec2::new(2, 0),
        b'<' => IVec2::new(0, 1),
        b'v' => IVec2::new(1, 1),
        b'>' => IVec2::new(2, 1),
        _ => {
            unreachable!()
        }
    }
}

fn plan_button_press(from: IVec2, to: IVec2, gap: IVec2, plan: &mut Vec<u8>) -> bool {
    let d = to - from;

    let dx = IVec2::new(d.x, 0);
    let dy = IVec2::new(0, d.y);

    let mut delta = [dx, dy];
    if from.y == gap.y {
        // if we're on the same row, start by going vertically first to avoid the gap
        delta.swap(0, 1);
    }

    for d in delta {
        if d.x != 0 {
            let horizontal = if d.x < 0 { b'<' } else { b'>' };
            for _ in 0..d.x.abs() {
                plan.push(horizontal);
            }
        } else if d.y != 0 {
            let vertical = if d.y < 0 { b'^' } else { b'v' };
            for _ in 0..d.y.abs() {
                plan.push(vertical);
            }
        }
    }
    plan.push(b'A');

    true
}

fn numeric_path(seq: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();

    let mut current = resolve_numeric(b'A');

    for to in seq.iter().copied().map(resolve_numeric) {
        let found = plan_button_press(current, to, IVec2::new(0, 3), &mut res);
        assert!(found);
        current = to;
    }

    res
}

/// take a plan and produce a plan on a directional keyboard
fn path_plan(path: &[u8]) -> Vec<u8> {
    let mut plan = Vec::new();

    let mut current = resolve_directional(b'A');
    for t in path {
        let to = resolve_directional(*t);
        let res = plan_button_press(current, to, IVec2::ZERO, &mut plan);
        assert!(res);
        current = to;
    }

    plan
}

fn shortest_path(line: &str) -> Vec<u8> {
    let mut plan = numeric_path(line.trim().as_bytes());
    for _ in 0..2 {
        plan = path_plan(&plan);
    }
    plan
}

fn part1(input: &Input) -> usize {
    let mut solution = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let plan = shortest_path(line);
        let code: usize = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        solution += plan.len() * code;
    }
    solution
}

fn part2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const INPUT: &str = r#"029A
980A
179A
456A
379A
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp);

        assert_eq!(res, 126384);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT.to_string());
        let res = part2(&inp);

        assert_eq!(res, 42);
    }

    #[test]
    fn test_numeric_path() {
        let plan = numeric_path(b"029A");
        let plan = std::str::from_utf8(&plan).unwrap();
        dbg!(plan);
        let valid: HashSet<_> = ["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"]
            .into_iter()
            .collect();

        assert!(valid.contains(plan));
    }

    #[test]
    fn test_path_plan() {
        let path = numeric_path(b"029A");
        let plan = path_plan(&path);
        let plan = std::str::from_utf8(&plan).unwrap();
        dbg!(plan);

        assert_eq!(plan.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
    }

    macro_rules! shortest_path_test {
        ($name: ident, $inp: expr, $exp: expr) => {
            #[test]
            fn $name() {
                let input = $inp;
                let expected = $exp;
                let plan = shortest_path(input);
                let plan = std::str::from_utf8(&plan).unwrap();
                dbg!(input, plan, expected);
                assert_eq!(plan.len(), expected.len(),);
            }
        };
    }

    shortest_path_test!(
        test_029a,
        "029A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
    );
    shortest_path_test!(
        test_980a,
        "980A",
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"
    );
    shortest_path_test!(
        test_179a,
        "179A",
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    );
    shortest_path_test!(
        test_456a,
        "456A",
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"
    );
    shortest_path_test!(
        test_379a,
        "379A",
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    );
}
