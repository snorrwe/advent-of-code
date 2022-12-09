use std::collections::HashSet;
use utils::IVec2;

fn cheby_dist(a: [i32; 2], b: [i32; 2]) -> i32 {
    (a[0] - b[0]).abs().max((a[1] - b[1]).abs())
}

fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut head = [0; 2];
    let mut tail = [0; 2];
    visited.insert(tail);

    for line in input.lines() {
        let mut line = line.split(" ");
        let dir = line.next().unwrap();
        let amount = line.next().unwrap().parse::<i32>().unwrap();
        let delta = match dir {
            "U" => [0, 1],
            "D" => [0, -1],
            "L" => [-1, 0],
            "R" => [1, 0],
            _ => unreachable!(),
        };

        for _ in 0..amount {
            let new_head = [head[0] + delta[0], head[1] + delta[1]];
            if cheby_dist(new_head, tail) > 1 {
                tail = head;
                visited.insert(tail);
            }
            head = new_head;
        }
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [IVec2::ZERO; 10];
    visited.insert(IVec2::ZERO);

    for line in input.lines() {
        let mut line = line.split(" ");
        let dir = line.next().unwrap();
        let amount = line.next().unwrap().parse::<i32>().unwrap();
        let delta: IVec2 = match dir {
            "U" => IVec2::Y,
            "D" => -IVec2::Y,
            "L" => -IVec2::X,
            "R" => IVec2::X,
            _ => unreachable!(),
        };

        for _ in 0..amount {
            let old_head = rope[0];
            rope[0] = old_head + delta;

            for i in 1..10 {
                let new_head = rope[i - 1];
                let tail = rope[i];
                if new_head.chebyshev(tail) > 1 {
                    let d = (new_head - tail).as_direction();
                    rope[i] = tail + d;
                } else {
                    break;
                }
            }
            visited.insert(rope[9]);
        }
    }
    visited.len()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let res = part1(&input);
    println!("part1: {res}");
    let res = part2(&input);
    println!("part2: {res}");
}

#[test]
fn part1_test() {
    let res = part1(
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
    let res = part2(
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
    let res = part2(
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
