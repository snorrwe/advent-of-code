use std::collections::HashSet;
use utils::IVec2;

fn solve<const N: usize>(input: &str) -> usize {
    debug_assert!(N >= 1);
    let mut visited: HashSet<IVec2> = [IVec2::ZERO].into();
    let mut rope = [IVec2::ZERO; N];

    for line in input.lines() {
        let mut line = line.split(" ");
        let delta = match line.next().unwrap() {
            "U" => IVec2::Y,
            "D" => -IVec2::Y,
            "L" => -IVec2::X,
            "R" => IVec2::X,
            _ => unreachable!(),
        };

        for _ in 0..line.next().unwrap().parse::<i32>().unwrap() {
            let old_head = rope[0];
            rope[0] = old_head + delta;

            for i in 1..N {
                if rope[i - 1].chebyshev(rope[i]) > 1 {
                    let d = (rope[i - 1] - rope[i]).as_direction();
                    rope[i] = rope[i] + d;
                } else {
                    break; // not necessary just an optimization
                }
            }
            visited.insert(rope[N - 1]);
        }
    }
    visited.len()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let res = solve::<2>(&input);
    println!("part1: {res}");
    let res = solve::<10>(&input);
    println!("part2: {res}");
}

#[test]
fn part1_test() {
    let res = solve::<2>(
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
    );

    assert_eq!(13, res)
}
#[test]
fn part2_simple_test() {
    let res = solve::<10>(
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
    );

    assert_eq!(1, res)
}

#[test]
fn part2_test() {
    let res = solve::<10>(
        r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
    );

    assert_eq!(36, res)
}
