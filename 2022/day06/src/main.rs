fn solve(line: &[u8], window_size: usize) -> usize {
    'a: for (i, window) in line.windows(window_size).enumerate() {
        for (i, a) in window.iter().enumerate() {
            for (j, b) in window.iter().enumerate() {
                if i == j {
                    continue;
                }
                if *a == *b {
                    continue 'a;
                }
            }
        }
        return i + window_size;
    }

    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let p1 = solve(input.as_bytes(), 4);
    println!("p1: {}", p1);
    let p2 = solve(input.as_bytes(), 14);
    println!("p2: {}", p2);
}

#[test]
fn part1_test() {
    let res = solve(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
    assert_eq!(res, 7);

    let res = solve(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
    assert_eq!(res, 5);

    let res = solve(b"nppdvjthqldpwncqszvftbrmjlhg", 4);
    assert_eq!(res, 6);

    let res = solve(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
    assert_eq!(res, 10);
    let res = solve(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
    assert_eq!(res, 11);
}

#[test]
fn part2_test() {
    let res = solve(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
    assert_eq!(res, 19);

    let res = solve(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
    assert_eq!(res, 23);

    let res = solve(b"nppdvjthqldpwncqszvftbrmjlhg", 14);
    assert_eq!(res, 23);

    let res = solve(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
    assert_eq!(res, 29);
    let res = solve(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
    assert_eq!(res, 26);
}
