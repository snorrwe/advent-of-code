use std::collections::HashSet;

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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let res = part1(&input);

    println!("part1: {res}");
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
