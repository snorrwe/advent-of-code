fn interesting(time: i32) -> bool {
    (time - 20) % 40 == 0
}

fn time_to_crt(time: usize) -> [usize; 2] {
    let row = time / 40;
    let col = time - row * 40;
    [col, row]
}

fn draw_sprite(crt: &mut [[u8; 40]; 6], time: i32, x: i32) {
    let [col, row] = time_to_crt(time as usize);
    let mut visible = false;
    for i in -1..=1 {
        if x + i == col as i32 {
            visible = true;
            break;
        }
    }
    if visible {
        crt[row][col] = b'#';
    }
}

fn run(input: &str) -> (i32, String) {
    let mut x = 1;
    let mut time = 0;
    let mut signal = 0;
    let mut crt = [[b'.'; 40]; 6];
    for line in input.lines() {
        let mut opargs = line.split(" ");
        match opargs.next().unwrap() {
            "noop" => {
                draw_sprite(&mut crt, time, x);
                if interesting(time) {
                    signal += x * time;
                }
                time += 1;
            }
            "addx" => {
                for d in 0..2 {
                    let time = time + d;
                    if interesting(time) {
                        signal += x * time;
                    }
                    draw_sprite(&mut crt, time, x);
                }
                let arg: i32 = opargs.next().unwrap().parse().unwrap();
                x += arg;
                time += 2;
            }
            _ => unreachable!(),
        }
    }
    let crt = crt
        .into_iter()
        .flat_map(|x| {
            x.into_iter()
                .map(|x| x as char)
                .chain(std::iter::once('\n'))
        })
        .collect();
    (signal, crt)
}

fn main() {
    let program = std::fs::read_to_string("input.txt").unwrap();

    let (p1, p2) = run(&program);
    println!("p1: {p1}");
    println!("p2:\n{p2}");
}

const _PROGRAM: &str = r#"addx 15
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
noop"#;

#[test]
fn part1_test() {
    let (res, _) = run(_PROGRAM);

    assert_eq!(13140, res);
}

#[test]
fn part2_test() {
    let (_, p2) = run(_PROGRAM);

    let exp = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

    assert_eq!(exp, p2, "\n\nexp:\n{exp}\nactual:\n{p2}");
}
