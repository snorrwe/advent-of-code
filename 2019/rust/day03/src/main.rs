use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn manhatten(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, other: f32) -> Point {
        Point {
            x: (self.x as f32 * other).round() as i32,
            y: (self.y as f32 * other).round() as i32,
        }
    }
}

/// Test AB segment and C point intersection
fn test_point_segment(a: Point, b: Point, c: Point) -> bool {
    // We know that these lines are on a manhatten grid so we can start with a
    // separating axis test
    if (a.x != c.x && a.y != c.y) || (b.x != c.x && b.y != c.y) {
        return false;
    }
    let ab = b - a;
    let ac = c - a;
    // either x or y is 0 in both
    let f = (ac.x + ac.y) as f32 / (ab.x + ab.y) as f32;
    // If f < 0 then ab and ac point in different directions
    // If f > 1 then C is farther from A than B is
    0.0 <= f && f <= 1.0
}

fn signed_triangle_area(a: Point, b: Point, c: Point) -> f32 {
    let [ax, ay, bx, by, cx, cy] = [
        a.x as f32, a.y as f32, b.x as f32, b.y as f32, c.x as f32, c.y as f32,
    ];

    (ax - cx) * (by - cy) - (ay - cy) * (bx - cx)
}

/// Test AB and CD segment intersection
/// Return the point where they intersect if any
fn test_segment_segment(a: Point, b: Point, c: Point, d: Point) -> Option<Point> {
    let abd = signed_triangle_area(a, b, d);
    let abc = signed_triangle_area(a, b, c);
    if abd * abc < 0.0 {
        // c and d are on different sides of ab
        let cda = signed_triangle_area(c, d, a);
        let cdb = cda + abc - abd;
        if cda * cdb < 0.0 {
            // a and b are on different sides of cd
            let t = cda / (cda - cdb);
            return Some(a + (b - a) * t);
        }
    }
    None
}

/// Calculate the intersections of two wires
/// Return the lines of the two wires and the intersection points
fn intersections(input: &str) -> ([Vec<(Point, Point)>; 2], Vec<Point>) {
    let mut paths = [vec![], vec![]];
    for (i, wire) in input.split("\n").enumerate() {
        let mut current = Point { x: 0, y: 0 };
        'a: for step in wire.split(',') {
            let last = current;
            let x = 0;
            let y = 0;
            let mut p = match &step[0..1] {
                "R" => Point { x: x + 1, y },
                "L" => Point { x: x - 1, y },
                "U" => Point { x, y: y - 1 },
                "D" => Point { x, y: y + 1 },
                _ => unreachable!(),
            };
            let num = step[1..].parse::<i32>().unwrap();
            p = p * num;
            current = current + p;
            paths[i].push((last, current));
        }
    }
    // Intersections are any point for which l1 intersects l2 for every (l1, l2) pair in W1 × W2
    let ints = paths[0]
        .iter()
        .cloned()
        .cartesian_product(paths[1].iter().cloned())
        .filter(|(a, b)| a != b)
        .filter_map(|(a, b)| test_segment_segment(a.0, a.1, b.0, b.1))
        .collect();
    (paths, ints)
}

fn part1(input: &str) -> i32 {
    intersections(input)
        .1
        .into_iter()
        .map(|p: Point| p.manhatten(&Point { x: 0, y: 0 }))
        .min()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let (wires, intersections) = intersections(input);
    let mut dists = intersections
        .iter()
        .map(|p| (*p, [0, 0]))
        .collect::<HashMap<_, _>>();
    for (i, wire) in wires.iter().enumerate() {
        let mut steps = 0;
        for line in wire {
            for inter in intersections
                .iter()
                .filter(|p| test_point_segment(line.0, line.1, **p))
            {
                // 1 line of 1 wire may only cross a point once
                assert_eq!(
                    dists.get_mut(&inter).expect("dists")[i],
                    0,
                    "line: {:?} point: {:?}",
                    line,
                    inter
                );
                dists.get_mut(&inter).expect("dists")[i] = steps + inter.manhatten(&line.0);
            }
            steps += line.0.manhatten(&line.1)
        }
    }
    dists
        .iter()
        .map(|(p, [x, y])| {
            assert!(*x > 0 && *y > 0, "Intersection point {:?} not covered!", p);
            x + y
        })
        .min()
        .expect("p2 min")
}

fn main() {
    let input = r"R1008,U428,L339,U16,R910,U221,R53,D546,L805,U376,L19,U708,R493,D489,L443,D567,R390,D771,R270,U737,R926,U181,L306,D456,L668,D79,L922,U433,L701,U472,R914,U903,L120,U199,L273,D206,L967,U711,R976,U976,R603,U8,L882,U980,R561,D697,L224,D620,L483,U193,R317,D588,L932,D990,R658,U998,L136,U759,R463,U890,L297,U648,R163,U250,R852,U699,R236,D254,L173,U720,L259,U632,L635,U426,R235,D699,R411,U650,L258,D997,L781,D209,L697,D306,L793,U657,L936,U317,R549,D798,L951,D80,R591,D480,R835,U292,L722,U987,L775,U173,R353,U404,L250,U652,L527,D282,L365,D657,L141,D301,R128,D590,L666,U478,L85,D822,L716,U68,R253,D186,R81,U741,L316,D615,L570,U407,L734,D345,L652,U362,L360,D791,R358,U190,L823,U274,L279,D998,L16,D644,L201,D469,R213,D487,L251,D395,R130,U902,L398,U201,L56,D850,R541,D661,R921,U647,R309,D550,L307,D68,R894,U653,L510,D375,R20,U86,R357,D120,L978,D200,L45,D247,R906,U334,L242,D466,R418,U548,R698,D158,R582,U469,L968,U736,R196,U437,R87,D722,R811,U625,L425,D675,L904,D331,R693,D491,R559,U540,L120,D975,R180,U224,R610,U260,L769,D486,R93,D300,L230,U181,L60,U910,L60,D554,L527,U37,R69,D649,R768,D835,L581,U932,L746,U170,L733,U40,L497,D957,R12,U686,R85,D461,R796,D142,R664,U787,R636,D621,R824,D421,R902,D686,L202,D839,R567,D129,L77,D917,L200,D106,R3,D546,L101,D762,R780,U334,L410,D190,R431,D828,L816,D529,R648,D449,L845,U49,R750,U864,L133,D822,R46,U475,L229,U929,L676,D793,R379,U71,L243,U640,L122,U183,R528,U22,R375,D928,R292,U796,R259,U325,L921,U489,L246,D153,L384,D684,L243,U65,L342,U662,R707
L1008,D243,L602,D497,L395,U81,R589,U94,R640,D965,L397,D781,R464,U642,L130,D740,R938,D260,L106,D323,L626,U869,L495,U450,R640,D675,R602,D449,L542,U917,L244,U702,L644,D809,R902,U163,R118,U982,L867,D795,R546,U194,R397,D877,L354,D255,L477,U45,L707,D624,R806,U642,L926,D233,L800,U691,L990,D979,L431,U999,L423,D794,L238,U25,R986,U595,L167,U480,L555,U831,R496,U799,L897,D895,L993,D11,R486,U176,L90,U842,R499,U792,R787,U859,L100,U169,R170,D89,R297,D944,R362,D460,R147,U831,L45,U972,R582,D90,L934,U284,R555,U235,L138,U837,R965,U915,R928,U982,R157,D566,L953,U653,L629,U460,L335,D912,R355,D683,L710,D562,R392,D44,R707,D979,L749,D223,L776,D723,R735,D356,R49,U567,L563,D220,L868,U223,R448,D505,L411,U662,L188,D536,R55,U718,L108,D289,R435,U98,R775,U933,L127,D84,R253,D523,L2,D905,R266,U675,R758,D331,R122,U988,R215,D500,R89,U306,R833,U763,R570,D818,L985,U127,L87,D210,R355,D532,R870,U196,R695,U633,R170,D540,R506,U708,L663,U566,L633,U306,L452,U180,R463,D21,L220,D268,R608,U986,L493,D598,L957,D116,L454,D584,L405,U651,R352,U681,R807,U767,L988,U692,R474,U710,R607,U313,R711,U12,R371,D561,R72,U522,R270,U904,L49,U525,R562,U895,L232,D319,R902,D236,L601,D816,R836,U419,R610,U644,L733,U355,L836,U228,L895,D39,L44,D848,L965,U475,R56,U62,L458,U99,R236,D763,R912,U295,R515,U179,R20,D777,R511,D906,R903,U855,L507,D512,L63,D630,R442,U595,L701,U654,R238,D35,L31,D469,R6,D222,R132,D837,R921,U838,R986,D441,L950,D530,L397,U41,L81,D60,L245,D75,R620,D455,L937,D180,R215,D684,R724,U561,R479,D353,L501";

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_0() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let res = part1(input);
        assert_eq!(res, 6);
    }

    #[test]
    fn part1_1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let res = part1(input);
        assert_eq!(res, 159);
    }

    #[test]
    fn part1_2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let res = part1(input);
        assert_eq!(res, 135);
    }

    #[test]
    fn part2_0() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let res = part2(input);
        assert_eq!(res, 30);
    }

    #[test]
    fn part2_1() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let res = part2(input);
        assert_eq!(res, 410);
    }

    #[test]
    fn part2_2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let res = part2(input);
        assert_eq!(res, 610);
    }
}
