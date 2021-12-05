#![feature(test)]
extern crate test;

use std::cmp::Ordering;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Moons {
    pub pos: [[i16; 3]; 4],
    pub vel: [[i16; 3]; 4],
}

impl Moons {
    pub fn energy(&self) -> i16 {
        let abssum = |p: &[i16; 3]| p.iter().map(|a: &i16| a.abs()).sum::<i16>();
        self.pos
            .iter()
            .map(abssum)
            .zip(self.vel.iter().map(abssum))
            .map(|(potential, kinetic)| potential * kinetic)
            .sum()
    }
}

fn adjust(a: i16, b: i16) -> (i16, i16) {
    match a.cmp(&b) {
        Ordering::Less => (1, -1),
        Ordering::Greater => (-1, 1),
        Ordering::Equal => (0, 0),
    }
}

fn tick(moons: &mut Moons) {
    let len = moons.pos.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            let p1 = &moons.pos[i];
            let p2 = &moons.pos[j];
            let (x1, x2) = adjust(p1[0], p2[0]);
            let (y1, y2) = adjust(p1[1], p2[1]);
            let (z1, z2) = adjust(p1[2], p2[2]);

            moons.vel[i][0] += x1;
            moons.vel[i][1] += y1;
            moons.vel[i][2] += z1;

            moons.vel[j][0] += x2;
            moons.vel[j][1] += y2;
            moons.vel[j][2] += z2;
        }
    }
    for (p, v) in moons.pos.iter_mut().zip(moons.vel.iter()) {
        for i in 0..3 {
            p[i] += v[i];
        }
    }
}

fn part2(moons: &mut Moons, seen: &Moons) -> usize {
    let mut iterations = 0;
    let mut periods = [0; 3];
    loop {
        tick(moons);
        iterations += 1;

        'a: for j in 0..3 {
            for i in 0..4 {
                if moons.pos[i][j] != seen.pos[i][j] || moons.vel[i][j] != seen.vel[i][j] {
                    continue 'a;
                }
            }
            // if all 4 matches the original position on axis j
            periods[j] = iterations - periods[j];
        }

        if periods.iter().all(|x| *x > 0) {
            return num_integer::lcm(num_integer::lcm(periods[0], periods[1]), periods[2]);
        }
    }
}

fn main() {
    let mut moons = Moons::default();
    moons.pos = [[8, 0, 8], [0, -5, -10], [16, 10, -5], [19, -10, -7]];

    let seen = moons.clone();

    for _ in 0..1000 {
        tick(&mut moons);
    }
    println!("{}", moons.energy());
    let mut moons = seen.clone();
    let iterations = part2(&mut moons, &seen);
    println!("Boiii {}", iterations);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_tick(b: &mut Bencher) {
        let mut moons = Moons::default();
        moons.pos = [[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]];
        let seen = moons.clone();

        b.iter(move || {
            tick(&mut moons);
            test::black_box(&moons);
            moons == seen
        })
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let mut moons = Moons::default();
        moons.pos = [[8, 0, 8], [0, -5, -10], [16, 10, -5], [19, -10, -7]];

        let seen = moons.clone();

        b.iter(move || {
            part2(&mut moons, &seen);
            test::black_box(&moons);
            moons == seen
        })
    }

    #[test]
    fn simple_example() {
        let mut moons = Moons::default();
        moons.pos = [[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]];

        for _ in 0..10 {
            tick(&mut moons);
        }

        println!("{:#?}", moons);

        assert_eq!(moons.energy(), 179);
    }

    #[test]
    fn part2_simple_example() {
        let mut moons = Moons::default();
        moons.pos = [[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]];

        let seen = moons.clone();

        let res = part2(&mut moons, &seen);

        assert_eq!(res, 2772);
    }

    #[test]
    fn part2_long_example() {
        let mut moons = Moons::default();
        moons.pos = [[-8, -10, 0], [5, 5, 10], [2, -7, 3], [9, -8, -3]];

        let seen = moons.clone();

        let res = part2(&mut moons, &seen);

        assert_eq!(res, 4686774924);
    }

    #[test]
    fn simple_example2() {
        let mut moons = Moons::default();
        moons.pos = [[-8, -10, 0], [5, 5, 10], [2, -7, 3], [9, -8, -3]];

        for _ in 0..100 {
            tick(&mut moons);
        }

        println!("{:#?}", moons);

        assert_eq!(moons.energy(), 1940);
    }
}
