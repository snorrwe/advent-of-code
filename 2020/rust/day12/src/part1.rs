use std::mem::swap;

pub fn rotate(deg: i32, dir: char, [mut x, mut y]: [i32; 2]) -> [i32; 2] {
    match dir {
        'L' => {
            for _ in 0..deg / 90 {
                swap(&mut x, &mut y);
                x *= -1;
            }
        }
        'R' => {
            for _ in 0..deg / 90 {
                swap(&mut x, &mut y);
                y *= -1;
            }
        }
        _ => unreachable!(),
    }
    [x, y]
}

pub fn part1(inp: &str) -> u32 {
    let mut vel = [1, 0];
    let mut pos = [0, 0];

    for line in inp.lines() {
        if line.len() < 2 {
            continue;
        }
        let mut chrs = line.chars();
        let magnitude: Option<i32> = line[1..].parse().ok();
        if let Some(c) = chrs.next() {
            match c {
                'F' => {
                    let magnitude = magnitude.unwrap();
                    let [vx, vy] = [vel[0] * magnitude, vel[1] * magnitude];
                    pos[0] += vx;
                    pos[1] += vy;
                }
                'E' => {
                    pos[0] += magnitude.unwrap();
                }
                'W' => {
                    pos[0] -= magnitude.unwrap();
                }
                'N' => {
                    pos[1] += magnitude.unwrap();
                }
                'S' => {
                    pos[1] -= magnitude.unwrap();
                }
                c @ 'L' | c @ 'R' => {
                    vel = rotate(magnitude.unwrap(), c, vel);
                }
                _ => { /*skip*/ }
            }
        }
    }

    // manhattan distance from (0,0)
    (pos[0].abs() + pos[1].abs()) as u32
}

#[test]
fn rotation_1() {
    let xy = [1, 0];
    let a = rotate(90, 'L', xy);
    let b = rotate(90, 'R', xy);

    assert_eq!(a, [0, 1]);
    assert_eq!(b, [0, -1]);
}

#[test]
fn test_part1() {
    let res = part1(
        r#"
F10
N3
F7
R90
F11
"#,
    );

    assert_eq!(res, 25);
}
