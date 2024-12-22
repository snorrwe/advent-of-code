use std::usize;

use itertools::Itertools;
use utils::IVec2;

type Input = String;

const NUMERIC_GAP: IVec2 = IVec2::new(0, 3);

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

fn plan_cost(plan: &[u8], depth: usize) -> usize {
    let mut cost = button_press_cost(b'A', plan[0], depth, NUMERIC_GAP, resolve_numeric).unwrap();
    let mut from = plan[0];
    for to in &plan[1..] {
        cost += button_press_cost(from, *to, depth, NUMERIC_GAP, resolve_numeric).unwrap();
        from = *to;
    }
    cost
}

fn dir_to_label(dir: IVec2) -> u8 {
    match dir {
        IVec2::NEG_X => b'<',
        IVec2::X => b'>',
        IVec2::NEG_Y => b'^',
        IVec2::Y => b'v',
        _ => unreachable!(),
    }
}

fn button_press_cost(
    from: u8,
    to: u8,
    depth: usize,
    // identify the keyboard by the position of the gap + the resolve function
    gap: IVec2,
    resolve: impl Fn(u8) -> IVec2,
) -> Option<usize> {
    if from == to {
        return Some(1);
    }
    let from = resolve(from);
    let to = resolve(to);
    if depth == 0 {
        return Some(from.manhatten(to) as usize + 1);
    }
    let d = to - from;

    let mut min_cost = usize::MAX;

    let horizontal = if d.x < 0 { -IVec2::X } else { IVec2::X };
    let vertical = if d.y < 0 { -IVec2::Y } else { IVec2::Y };
    // for each possible plan
    'outer: for mut test in itertools::repeat_n(horizontal, d.x.abs() as usize)
        .chain(itertools::repeat_n(vertical, d.y.abs() as usize))
        .permutations((d.x.abs() + d.y.abs()) as usize)
    {
        let mut current = from;
        current += test[0];
        if current == gap {
            continue;
        }
        let mut from = dir_to_label(test[0]);
        let Some(mut cost) =
            button_press_cost(b'A', from, depth - 1, IVec2::ZERO, resolve_directional)
        else {
            continue 'outer;
        };
        for to in test.drain(1..) {
            current += to;
            if current == gap {
                continue 'outer;
            }
            let to = dir_to_label(to);
            let Some(c) = button_press_cost(from, to, depth - 1, IVec2::ZERO, resolve_directional)
            else {
                continue 'outer;
            };
            from = to;
            cost += c;
        }
        // all paths end with a press
        let Some(c) = button_press_cost(from, b'A', depth - 1, IVec2::ZERO, resolve_directional)
        else {
            continue 'outer;
        };
        cost += c;
        min_cost = min_cost.min(cost);
    }

    (min_cost != usize::MAX).then_some(min_cost)
}

fn part1(input: &Input) -> usize {
    let mut solution = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let cost = plan_cost(line.trim().as_bytes(), 2);
        let code = line.trim_end_matches('A');
        let code: usize = code.parse().unwrap();
        solution += cost * code;
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

    macro_rules! shortest_path_test {
        ($name: ident, $inp: expr, $exp: expr) => {
            #[test]
            fn $name() {
                let input = $inp;
                let expected = $exp;
                let plan = plan_cost(input.as_bytes(), 2);
                assert_eq!(plan, expected.len());
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
