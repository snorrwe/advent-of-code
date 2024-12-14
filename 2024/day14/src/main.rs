use std::usize;

use regex::Regex;
use utils::{
    image::{self, ImageEncoder},
    IVec2,
};

type Input = Vec<Robot>;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: IVec2,
    vel: IVec2,
}

fn parse(input: String) -> Input {
    let re = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();

    let mut res = Vec::new();
    for line in input.lines() {
        if let Some(m) = re.captures(line) {
            res.push(Robot {
                pos: IVec2::new(
                    m.get(1).unwrap().as_str().parse().unwrap(),
                    m.get(2).unwrap().as_str().parse().unwrap(),
                ),
                vel: IVec2::new(
                    m.get(3).unwrap().as_str().parse().unwrap(),
                    m.get(4).unwrap().as_str().parse().unwrap(),
                ),
            });
        }
    }
    res
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(input);

    println!("{}", part1(&input, 101, 103));
    // doesn't give the solution but gives the loop period
    println!("{}", part2(&input, 101, 103));
}

fn part1(input: &Input, width: i32, height: i32) -> i32 {
    let mut a = input.clone();
    for _ in 0..100 {
        for b in a.iter_mut() {
            b.pos = b.pos + b.vel;
            while b.pos.x < 0 {
                b.pos.x = b.pos.x + width;
            }
            b.pos.x %= width;
            while b.pos.y < 0 {
                b.pos.y = b.pos.y + height;
            }
            b.pos.y %= height;
        }
    }

    let mut quads = [0; 4];

    let w2 = width / 2;
    let h2 = height / 2;
    for b in a.iter() {
        let x = b.pos.x;
        let y = b.pos.y;

        if x < w2 && y < h2 {
            quads[2] += 1;
        } else if x < w2 && y > h2 {
            quads[1] += 1;
        } else if x > w2 && y < h2 {
            quads[3] += 1;
        } else if x > w2 && y > h2 {
            quads[0] += 1;
        }
    }

    quads.into_iter().product()
}

fn part2(input: &Input, width: i32, height: i32) -> i32 {
    // lcm
    let loop_period = width * height / gcd(width, height);
    let mut minsize = std::usize::MAX;
    let mut minidx = 0;

    let mut data = Vec::new();
    let mut a = input.clone();
    for i in 0..=loop_period {
        for b in a.iter_mut() {
            b.pos = b.pos + b.vel;
            while b.pos.x < 0 {
                b.pos.x = b.pos.x + width;
            }
            b.pos.x %= width;
            while b.pos.y < 0 {
                b.pos.y = b.pos.y + height;
            }
            b.pos.y %= height;
        }

        let mut grid = utils::Grid::new(width as usize, height as usize);
        grid.fill(0);

        for b in a.iter() {
            grid[b.pos] = 0xFFu8;
        }

        // the result image has a considerably lower size when png encoded than the rest
        data.clear();
        image::codecs::png::PngEncoder::new(&mut data)
            .write_image(
                &grid.data,
                grid.width as u32,
                grid.height as u32,
                image::ColorType::L8,
            )
            .unwrap();

        if minsize > data.len() {
            minsize = data.len();
            minidx = i;
        }
    }

    minidx + 1
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == b {
        return a;
    }
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT.to_string());
        let res = part1(&inp, 11, 7);

        assert_eq!(res, 12);
    }
}
