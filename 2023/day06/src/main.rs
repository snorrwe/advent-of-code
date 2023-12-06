fn main() {
    let s = parse_v1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("{}", solve(&s));
    let s = parse_v2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("{}", solve(&s));
}

fn solve(s: &Stats) -> usize {
    let mut res = 1;
    for (i, time) in s.time.iter().enumerate() {
        let dist = s.distance[i];

        let c = (0..*time)
            .map(|t| {
                let travel = time - t;
                travel * t
            })
            .filter(|d| *d > dist)
            .count();
        res *= c;
    }
    res
}

#[derive(Debug)]
struct Stats {
    time: Vec<i64>,
    distance: Vec<i64>,
}

fn parse_v1(input: &str) -> Stats {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let (_, numbers) = line.split_once(|c: char| c.is_whitespace()).unwrap();

    let mut time = Vec::new();
    for number in numbers.split_whitespace() {
        time.push(number.parse().unwrap());
    }

    let line = lines.next().unwrap();
    let (_, numbers) = line.split_once(|c: char| c.is_whitespace()).unwrap();

    let mut distance = Vec::new();
    for number in numbers.split_whitespace() {
        distance.push(number.parse().unwrap());
    }

    assert_eq!(distance.len(), time.len());

    Stats { distance, time }
}

fn parse_v2(input: &str) -> Stats {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let (_, numbers) = line.split_once(|c: char| c.is_whitespace()).unwrap();

    let mut time = Vec::new();
    for number in numbers.split_whitespace() {
        time.push(number);
    }

    let line = lines.next().unwrap();
    let (_, numbers) = line.split_once(|c: char| c.is_whitespace()).unwrap();

    let mut distance = Vec::new();
    for number in numbers.split_whitespace() {
        distance.push(number);
    }

    let time = time.join("").parse().unwrap();
    let distance = distance.join("").parse().unwrap();

    Stats {
        distance: vec![distance],
        time: vec![time],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn test_p1() {
        let stats = parse_v1(INPUT);

        let res = solve(&stats);

        assert_eq!(res, 288);
    }

    #[test]
    fn test_p2() {
        let stats = parse_v2(INPUT);

        let res = solve(&stats);

        assert_eq!(res, 71503);
    }
}
