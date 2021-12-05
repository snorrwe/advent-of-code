mod part1;

fn part2(inp: &str) -> u32 {
    let mut vel = [10, 1];
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
                    vel[0] += magnitude.unwrap();
                }
                'W' => {
                    vel[0] -= magnitude.unwrap();
                }
                'N' => {
                    vel[1] += magnitude.unwrap();
                }
                'S' => {
                    vel[1] -= magnitude.unwrap();
                }
                c @ 'L' | c @ 'R' => {
                    vel = part1::rotate(magnitude.unwrap(), c, vel);
                }
                _ => { /*skip*/ }
            }
        }
    }

    // manhattan distance from (0,0)
    (pos[0].abs() + pos[1].abs()) as u32
}

fn main() {
    let mut input = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();

    let p1 = part1::part1(input.as_str());
    println!("{}", p1);
    let p2 = part2(input.as_str());
    println!("{}", p2);
}

#[test]
fn test_part2() {
    let res = part2(
        r#"
F10
N3
F7
R90
F11
"#,
    );

    assert_eq!(res, 286);
}
