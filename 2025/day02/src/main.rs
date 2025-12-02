type Input<'a> = Vec<(&'a str, &'a str)>;

fn parse(input: &'_ str) -> Input<'_> {
    input
        .trim()
        .split(',')
        .filter_map(|row| row.split_once('-'))
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &Input) -> i64 {
    input
        .iter()
        .flat_map(|(from, to)| {
            let from: i64 = from.parse().unwrap();
            let to: i64 = to.parse().unwrap();
            from..=to
        })
        .filter(|x| {
            let mut s = x.to_string();
            let r = s.split_off(s.len() / 2);
            s == r
        })
        .sum()
}

fn part2(input: &Input) -> i64 {
    input
        .iter()
        .flat_map(|(from, to)| {
            let from: i64 = from.parse().unwrap();
            let to: i64 = to.parse().unwrap();
            from..=to
        })
        .filter(|x| {
            let s = x.to_string();
            for i in 1..=s.len() / 2 {
                let pat = &s[0..i];
                if s.split(pat).filter(|s| !s.is_empty()).count() == 0 {
                    return true;
                }
            }
            false
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_p1() {
        let inp = parse(INPUT);
        let res = part1(&inp);

        assert_eq!(res, 1227775554);
    }

    #[test]
    fn test_p2() {
        let inp = parse(INPUT);
        let res = part2(&inp);

        assert_eq!(res, 4174379265);
    }
}
