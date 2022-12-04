fn solve(input: &str) -> [i32; 2] {
    let mut result = [0; 2];
    for line in input.lines() {
        let elves = line.split(',');
        let mut ranges = [(0, 0); 2];

        for (i, elf) in elves.enumerate() {
            let mut range = elf.split('-');
            let from: i32 = range.next().unwrap().parse().unwrap();
            let to: i32 = range.next().unwrap().parse().unwrap();

            ranges[i] = (from, to);
        }

        let [a, b] = ranges;

        if (a.0 <= b.0 && b.1 <= a.1) || (b.0 <= a.0 && a.1 <= b.1) {
            result[0] += 1;
        }
        if a.1 >= b.0 && a.0 <= b.1 {
            result[1] += 1;
        }
    }
    result
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let [p1, p2] = solve(&input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}

#[test]
fn basic_test() {
    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    let [p1, p2] = solve(INPUT);

    assert_eq!(p1, 2);
    assert_eq!(p2, 4);
}
