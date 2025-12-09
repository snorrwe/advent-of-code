use itertools::Itertools;
use utils::{AABB2, IVec2};

type Input = Vec<IVec2>;

fn parse(input: &'_ str) -> Input {
    input
        .lines()
        .filter_map(|l| l.trim().split_once(','))
        .map(|(x, y)| IVec2::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    input
        .iter()
        .array_combinations()
        .map(|[a, b]| {
            let c = *b - *a;
            (c.x.abs() as u64 + 1) * (c.y.abs() as u64 + 1)
        })
        .max()
        .unwrap()
}

fn intersect_segment_box((p0, p1): (IVec2, IVec2), bbox: &AABB2) -> bool {
    let c = (bbox.min + bbox.max) / 2;
    let e = bbox.max - c;
    let m = (p0 + p1) / 2;

    let d = p1 - m;

    let m = m - c;

    let adx = d.x.abs();
    if m.x.abs() > e.x + adx {
        return false;
    }
    let ady = d.y.abs();
    if m.y.abs() > e.y + ady {
        return false;
    }
    true
}

fn part2(input: &Input) -> usize {
    let line_segments: Vec<(IVec2, IVec2)> = input
        .iter()
        .chain(input.first())
        .zip(input.iter().skip(1))
        .map(|(a, b)| (*a, *b))
        .collect();

    // part 1 but check if they are in intersected by a line
    input
        .iter()
        .array_combinations()
        .filter_map(|pts: [&IVec2; 2]| {
            let bbox = AABB2::from_points(pts.iter().map(|p| **p));

            let mut cbox = bbox;
            cbox.shrink(IVec2::ONE);
            line_segments
                .iter()
                .all(|s| !intersect_segment_box(*s, &cbox))
                .then_some(bbox)
        })
        .map(|bbox| bbox.width() * bbox.height())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 50);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 24);
    }
}
