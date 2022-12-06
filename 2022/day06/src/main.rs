fn part1(line: &str) -> usize {
    'a: for i in 0..line.len() - 4 {
        let p = line.get(i..i + 4).unwrap();
        for (i, a) in p.chars().enumerate() {
            for (j, b) in p.chars().enumerate() {
                if i == j {
                    continue;
                }
                if a == b {
                    continue 'a;
                }
            }
        }
        return i + 4;
    }
    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let p1 = part1(&input);
    println!("p1: {}", p1);
}

#[test]
fn part1_test() {
    let res = part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    assert_eq!(res, 7);

    let res = part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
    assert_eq!(res, 5);

    let res = part1("nppdvjthqldpwncqszvftbrmjlhg");
    assert_eq!(res, 6);

    let res = part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    assert_eq!(res, 10);
    let res = part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    assert_eq!(res, 11);
}
