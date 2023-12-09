fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", part1(input.as_str()));
    println!("{}", part2(input.as_str()));
}

fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let Some(cap) = re.captures(line) else {
            continue;
        };

        let (_, [x, y, z]) = cap.extract();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        let z: i32 = z.parse().unwrap();

        let a1 = x * y;
        let a2 = y * z;
        let a3 = z * x;

        total += (a1 + a2 + a3) * 2 + [a1, a2, a3].into_iter().min().unwrap();
    }
    total
}

fn part2(input: &str) -> i32 {
    let re = regex::Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let Some(cap) = re.captures(line) else {
            continue;
        };

        let (_, [x, y, z]) = cap.extract();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        let z: i32 = z.parse().unwrap();

        let mut xyz = [x, y, z];
        xyz.sort();

        total += (xyz[0] + xyz[1]) * 2 + xyz.into_iter().product::<i32>();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let res = part1("2x3x4");

        assert_eq!(res, 58);
    }

    #[test]
    fn test_p2() {
        let res = part2("2x3x4");

        assert_eq!(res, 34);
    }
}
