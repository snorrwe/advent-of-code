fn interesting(time: isize) -> bool {
    (time - 20) % 40 == 0
}

fn part1(input: &str) -> isize {
    let mut x = 1;
    let mut time = 1;
    let mut signal = 0;
    for line in input.lines() {
        let mut opargs = line.split(" ");
        match opargs.next().unwrap() {
            "noop" => {
                if interesting(time) {
                    signal += x * time;
                }
                time += 1;
            }
            "addx" => {
                let arg: isize = opargs.next().unwrap().parse().unwrap();
                if interesting(time) {
                    signal += x * time;
                }
                if interesting(time + 1) {
                    signal += x * (time + 1);
                }
                time += 2;
                x += arg;
            }
            _ => unreachable!(),
        }
    }
    signal
}

fn main() {
    let program = std::fs::read_to_string("input.txt").unwrap();

    let res = part1(&program);
    println!("p1: {res}");
}

#[test]
fn part1_test() {
    let res = part1(
        r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#,
    );

    assert_eq!(13140, res);
}
