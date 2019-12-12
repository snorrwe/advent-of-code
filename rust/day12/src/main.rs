#![feature(test)]
extern crate test;

mod point;
use point::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Moons {
    pub pos: [Point; 4],
    pub vel: [Point; 4],
}

impl Moons {
    pub fn energy(&self) -> i16 {
        let abssum = |p: &Point| p.x.abs() + p.y.abs() + p.z.abs();
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
            let (x1, x2) = adjust(p1.x, p2.x);
            let (y1, y2) = adjust(p1.y, p2.y);
            let (z1, z2) = adjust(p1.z, p2.z);

            moons.vel[i].x += x1;
            moons.vel[i].y += y1;
            moons.vel[i].z += z1;

            moons.vel[j].x += x2;
            moons.vel[j].y += y2;
            moons.vel[j].z += z2;
        }
    }
    for (p, v) in moons.pos.iter_mut().zip(moons.vel.iter()) {
        *p += *v;
    }
}

fn part2(moons: &mut Moons, seen: &Moons) -> usize {
    let mut iterations = 0;
    loop {
        tick(moons);
        iterations += 1;

        if moons == seen {
            return iterations;
        }
    }
}

fn main() {
    let mut moons = Moons::default();
    moons.pos = [
        Point::new(8, 0, 8),
        Point::new(0, -5, -10),
        Point::new(16, 10, -5),
        Point::new(19, -10, -7),
    ];

    let seen = moons.clone();

    for _ in 0..1000 {
        tick(&mut moons);
    }
    println!("{}", moons.energy());
    let iterations = part2(&mut moons, &seen) + 1000;
    println!("Boiii {}", iterations);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_tick(b: &mut Bencher) {
        let mut moons = Moons::default();
        moons.pos = [
            Point::new(-1, 0, 2),
            Point::new(2, -10, -7),
            Point::new(4, -8, 8),
            Point::new(3, 5, -1),
        ];
        let seen = moons.clone();

        b.iter(move || {
            tick(&mut moons);
            test::black_box(&moons);
            moons == seen
        })
    }

    #[test]
    fn simple_example() {
        let mut moons = Moons::default();
        moons.pos = [
            Point::new(-1, 0, 2),
            Point::new(2, -10, -7),
            Point::new(4, -8, 8),
            Point::new(3, 5, -1),
        ];

        for _ in 0..10 {
            tick(&mut moons);
        }

        println!("{:#?}", moons);

        assert_eq!(moons.energy(), 179);
    }

    #[test]
    fn part2_simple_example() {
        let mut moons = Moons::default();
        moons.pos = [
            Point::new(-1, 0, 2),
            Point::new(2, -10, -7),
            Point::new(4, -8, 8),
            Point::new(3, 5, -1),
        ];

        let seen = moons.clone();

        let res = part2(&mut moons, &seen);

        assert_eq!(res, 2772);
    }

    #[test]
    fn part2_long_example() {
        let mut moons = Moons::default();
        moons.pos = [
            Point::new(-8, -10, 0),
            Point::new(5, 5, 10),
            Point::new(2, -7, 3),
            Point::new(9, -8, -3),
        ];

        let seen = moons.clone();

        let res = part2(&mut moons, &seen);

        assert_eq!(res, 4686774924);
    }

    #[test]
    fn simple_example2() {
        let mut moons = Moons::default();
        moons.pos = [
            Point::new(-8, -10, 0),
            Point::new(5, 5, 10),
            Point::new(2, -7, 3),
            Point::new(9, -8, -3),
        ];

        for _ in 0..100 {
            tick(&mut moons);
        }

        println!("{:#?}", moons);

        assert_eq!(moons.energy(), 1940);
    }
}
