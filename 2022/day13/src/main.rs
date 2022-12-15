#[derive(Debug, Eq, PartialEq, serde::Deserialize, Ord, Clone)]
#[serde(untagged)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.partial_cmp(y),
            (Packet::Int(x), Packet::List(_)) => {
                Packet::List(vec![Packet::Int(*x)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Int(y)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Int(*y)]))
            }
            (Packet::List(x), Packet::List(y)) => {
                for (x, y) in x.iter().zip(y.iter()) {
                    let ord = x.partial_cmp(y)?;
                    match ord {
                        std::cmp::Ordering::Equal => { /* continue */ }
                        std::cmp::Ordering::Less | std::cmp::Ordering::Greater => return Some(ord),
                    }
                }
                x.len().partial_cmp(&y.len())
            }
        }
    }
}

fn parse(input: &str) -> Vec<Packet> {
    let mut res = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        res.push(serde_json::from_str(line).unwrap());
    }
    res
}

fn part1(p: &[Packet]) -> usize {
    let mut count = 0;
    for (i, chunk) in p.chunks_exact(2).enumerate() {
        let lhs = &chunk[0];
        let rhs = &chunk[1];
        let cmp = lhs.partial_cmp(rhs).unwrap();
        match cmp {
            std::cmp::Ordering::Less => {
                count += i + 1;
            }
            _ => {}
        }
    }
    count
}

fn part2(mut p: Vec<Packet>) -> usize {
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    p.push(divider1.clone());
    p.push(divider2.clone());

    p.sort_unstable();

    let mut i1 = 0;
    let mut i2 = 0;

    for (i, p) in p.iter().enumerate() {
        if p == &divider1 {
            i1 = i+1;
        }
        if p == &divider2 {
            i2 = i+1;
        }
    }

    i1 * i2
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let packets = parse(&input);

    let res = part1(&packets);
    println!("p1: {res}");
    let res = part2(packets);
    println!("p2: {res}");
}

#[test]
fn part1_test() {
    const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    let packets = parse(INPUT);

    let res = part1(&packets);

    assert_eq!(13, res);
}

#[test]
fn part2_test() {
    const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    let packets = parse(INPUT);

    let res = part2(packets);

    assert_eq!(140, res);
}
